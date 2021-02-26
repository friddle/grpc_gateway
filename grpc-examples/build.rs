extern crate protoc_rust_grpc;

fn main() {
    protoc_rust_grpc::Codegen::new()
        .out_dir("src/proto_gen/")
        .include("src/proto")
        .inputs(vec!["src/proto/helloworld.proto","src/proto/route_guide.proto"])
        .rust_protobuf(true)
        .run()
        .expect("protoc-rust-grpc");
}
