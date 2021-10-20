extern crate protoc_rust;

use protoc_rust::Customize;

fn main() {
    protoc_rust::Codegen::new()
        .customize(Customize {
            serde_derive: Some(true),
            gen_mod_rs: Some(true),
            ..Default::default()
        })
        .out_dir("src/protos")
        .input("protos/bom-1.3.proto")
        .include("protos")
        .run()
        .expect("protoc");
}