use tls_api::TlsConnector;
use tls_api_stub;
use std::sync::Arc;
use uuid::Uuid;
use std::any::Any;
use std::collections::HashMap;

/// Client 对象必须保存数据..生命周期也应该是全局的.不是配置的
#[derive(Clone)]
pub struct NodeConf {
    pub tls: Tls,
    pub authorized:Option<String>,
    pub host: String,
    pub port: Option<u16>,
    pub id: String,
    pub envs: HashMap<String,String>
}

//好像tlb_api有问题
impl NodeConf {
    /// Create default configuration.
    pub fn new_plain(host: &str, port: u16) -> Self {
        NodeConf {
            tls: Tls::None,
            host: String::from(host),
            port: Some(port),
            authorized:None,
            id:String::from(Uuid::new_v4().to_hyphenated().to_string()),
            envs:HashMap::new()
        }
    }

    pub fn new_tls(host: &str, port: u16,tls:Tls,authorized:&str)->Self{
        NodeConf{
            tls,
            host:String::from(host),
            port:Some(port),
            authorized:Some(String::from(authorized)),
            id:String::from(Uuid::new_v4().to_hyphenated().to_string()),
            envs:HashMap::new()
        }
    }

    pub fn set_id(mut self,id:String)->Self
    {
        self.id=id;
        self
    }

}

#[derive(Clone, Debug)]
pub enum ClientAddress {
    Tcp { port: u16, host: String },
    Unix { socket: String },
}

//Ｔ是为了满足TLS的多态。。。
//好像必须实现多态
//TLS到时候也是配置实现
#[derive(Clone)]
pub enum Tls {
    Native(Arc<tls_api_native_tls::TlsConnector>),
    OpenSsl(Arc<tls_api_native_tls::TlsConnector>),
    RustLS(Arc<tls_api_native_tls::TlsConnector>),
    None,
}

pub fn get_client_address(host: &str, port: u16) -> ClientAddress {
    return ClientAddress::Tcp { host: String::from(host), port };
}

pub type NodeConfs= Vec<NodeConf>;