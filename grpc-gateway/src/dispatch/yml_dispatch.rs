use serde::{Serialize, Deserialize};
use serde_yaml;
use crate::{GrpcDispatchHandler, LoadBalanceType, Headers, Header, Metadata, NodeConfs, NodeConf, ServerBuilder, Server, ServerConf};
use std::collections::HashMap;
use crate::common::node_conf::Tls;
use httpbis::ClientTlsOption;
use tls_api::{TlsConnectorBuilder, TlsConnector, TlsAcceptor,TlsAcceptorBuilder};
use std::sync::Arc;
use regex::Regex;
use std::io::{BufReader, Read};
use std::fs::File;
use std::fs::read_to_string;
use bytes::{Bytes, BytesMut};

#[derive(Debug, PartialEq, Serialize, Deserialize,Clone)]
pub enum SSlAuthType{
    none,
    ssl,
    openssl,
    native_ssl,
    rust_ssl
}


#[derive(Debug, PartialEq, Serialize, Deserialize,Clone)]
pub struct DispatchOption{
    pub envs:HashMap<String,String>,
    pub load_balance: Option<LoadBalanceType>,
    pub log_env_type:Option<String>
}

impl Default for DispatchOption{
    fn default() -> Self {
        return DispatchOption{
            envs:HashMap::new(),
            load_balance:Some(LoadBalanceType::Default),
            log_env_type:Some("info".to_string()),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize,Clone)]
pub struct DispatchAuthOption{
    pub auth_file:String,
    pub authorized:Option<String>,
    pub pass:Option<String>
}

#[derive(Debug, PartialEq, Serialize, Deserialize,Clone)]
pub struct DispatchNode{
    pub host:String,
    pub port:u16,
    pub auth_option:Option<DispatchAuthOption>,
    pub auth:SSlAuthType,
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DispatchListen{
    pub port:u16,
    pub auth:SSlAuthType,
    pub auth_option:Option<DispatchAuthOption>,
    pub option:Option<DispatchOption>
}

#[derive(Debug, PartialEq, Serialize, Deserialize,Clone)]
pub struct DispatchProxy{
    pub name:String,
    pub methods:Vec<String>,
    pub nodes:Vec<DispatchNode>,
    pub weight:Option<u32>,
    pub option:Option<DispatchOption>
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DispatchYaml
{
    pub listen:DispatchListen,
    pub proxys:Vec<DispatchProxy>
}

#[test]
fn runtime_read_bytes()
{
    let bytes=include_bytes!("../../../grpc-examples/src/root-ca.der").to_vec();
    let mut f=BufReader::new(File::open("../grpc-examples/src/root-ca.der").expect("read file not success"));
    let mut new_bytes:Vec<u8>=Vec::new();
    f.read_to_end(&mut new_bytes);
    println!("bytes: {:?}  \rruntime bytes: {:?}",bytes,new_bytes);
    assert_eq!(bytes,new_bytes);
}


fn generate_tls_connector<T:TlsConnector>(ss_type:&SSlAuthType,option:&DispatchAuthOption)->T{
    if !option.auth_file.ends_with(&"der") {
        panic!("only support der file location");
    }
    let mut f=BufReader::new(File::open(option.auth_file.clone())
        .expect(&format!("pem not exists:{}",option.auth_file)));
    let mut root_ca =Vec::new();
    f.read_to_end(&mut root_ca);
    let root_ca = tls_api::Certificate::from_der(root_ca.to_vec());
    let mut builder = T::builder().unwrap();
    builder
        .add_root_certificate(root_ca)
        .expect("add_root_certificate");
    builder.build().unwrap()
}

fn generate_native_tls_acceptor(ss_type:&SSlAuthType,option:&DispatchAuthOption)->tls_api_native_tls::TlsAcceptor{
    let mut file=BufReader::new(File::open(option.auth_file.clone())
        .expect(&format!("pem not exists:{}",option.auth_file)));
    let mut pksc12:Vec<u8>=Vec::new();
    file.read_to_end(&mut pksc12);
    let str=String::new();
    let pass=option.pass.as_ref().unwrap_or(&str);
    let builder = tls_api_native_tls::TlsAcceptorBuilder::from_pkcs12(&pksc12,pass).unwrap();
    builder.build().unwrap()
}


pub struct DispatchYamlStarter<T:Server>{
    pub yaml:DispatchYaml,
    pub server:Option<T>,
}


impl DispatchProxy{
    fn from_node_to_confs(node:&DispatchNode)->NodeConf{
        let node_conf=match node.auth{
            SSlAuthType::none=>{
                NodeConf::new_plain(&node.host,node.port)
            },
            SSlAuthType::ssl=>{
                let tls=Tls::Native(Arc::new(generate_tls_connector::<tls_api_native_tls::TlsConnector>
                    (&node.auth,&node.auth_option.as_ref().unwrap())));
                let authorized=node.auth_option.as_ref().unwrap().authorized.as_ref().unwrap_or_else(|| &node.host);
                NodeConf::new_tls(&node.host,node.port,tls,&authorized)
            }
            _=>{
                panic!("暂时不支持的格式")
            }
        };
        node_conf
    }
}

impl GrpcDispatchHandler for DispatchProxy{
    fn is_match(&self, path: &str, metadata: &Metadata) -> bool{
        let mut is_fit=false;
        for (_,method) in self.methods.iter().enumerate(){
            let regex=Regex::new(method).unwrap();
            is_fit=regex.is_match(path)||is_fit;
        };
        is_fit
    }
    fn get_balance_type(&self) -> &LoadBalanceType{
        let load_balance=match &self.option{
            Some(option)=>option.load_balance.as_ref().unwrap_or(&LoadBalanceType::Default),
            None=>&LoadBalanceType::Default
        };
        return &load_balance
    }
    fn get_clients(&self) -> NodeConfs{
        return self.nodes.iter().map(
            |node|Self::from_node_to_confs(node)
        ).collect()
    }
}



impl <T:Server>DispatchYamlStarter<T>{

    pub fn parse_from_file(file:&str)->DispatchYamlStarter<T>{
        let text=read_to_string(file).expect("yml file not exist");
        let obj=Self::parse_from_text(&text);
        obj
    }

    pub fn parse_from_text(text:&str)->DispatchYamlStarter<T>{
        let yml_obj:DispatchYaml=serde_yaml::from_str(&text).expect("");
        return DispatchYamlStarter{
            server:None,
            yaml:yml_obj
        }
    }

    pub fn run(mut self)->Self
    {
        let mut listener:&DispatchListen=&self.yaml.listen;
        let mut server_builder=match listener.auth{
            SSlAuthType::none=>ServerBuilder::new(),
            SSlAuthType::ssl=>ServerBuilder::<tls_api_native_tls::TlsAcceptor>::new().set_tls(
                generate_native_tls_acceptor(
                &listener.auth,
                         &listener.auth_option.as_ref().unwrap())
            ),
            _=>panic!("unsupport auth type")
        };
        let server_conf=ServerConf::new();
        server_builder=server_builder.set_conf(server_conf);
        server_builder=server_builder.set_port(listener.port);
        for proxy in self.yaml.proxys.iter(){
            server_builder=server_builder.add_dispatch(Arc::new(Box::new(proxy.clone())));
        }
        let mut server:T=server_builder.build().expect("");
        self.server=Some(server);
        self
    }

}




#[test]
fn test_simple(){
    let yml_text=r#"
  host: "www.friddle.me"
  port: 4001
  auth_option:
      pem_location: ./test.pem
  "#;
    let yml_obj:DispatchNode=serde_yaml::from_str(&yml_text).expect("");
    assert_eq!(yml_obj.auth_option.expect("sss").auth_file, "./test.pem");
    assert_eq!(yml_obj.port,4001);
}

#[test]
fn test_listen(){
    let yml_text=r#"
       port: 50051
       auth: none
  "#;
    let yml_obj:DispatchListen=serde_yaml::from_str(&yml_text).expect("");
    assert_eq!(yml_obj.auth,SSlAuthType::none);
}




#[test]
fn test_yml_parse(){
    let yml_text=
        r#"
 listen:
       port: 50051
       auth: ssl
       auth_option:
          pem_location: ./test.pem
          authorized: jump.qiuqiuhuiben.com  
 proxys:
     - 
       name: login
       methods:
        - /login/*
        - /account/*
       nodes:
         -
           host: login1.qiuqiu.com
           port: 50051
           auth: none
         -
           host: login2.qiuqiu.com
           port: 50052
           auth: ssl
           auth_option:
             auth_file: ./test.pem
             authorized: jump.qiuqiuhuiben.com  
       options:
           load_balance: Round
           envs: 
              - env=1
              - env=2
     - 
       name: pay
       methods:
        - /payment/
       nodes:
         -
           host: pay.qiuqiu.com:50051
           auth: none
  "#;
    let yml_obj:DispatchYaml=serde_yaml::from_str(&yml_text).expect("");
    assert_eq!(yml_obj.proxys[0].name,"login");
}
