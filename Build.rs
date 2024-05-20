use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("OUT_DIR = {}", out_dir);

    let proto_file = "src/_.proto";
    println!("Compiling protobuf file: {}", proto_file);

    // Compile the Protobuf definitions
    match prost_build::compile_protos(&[proto_file], &["src/"]) {
        Ok(_) => println!("Protobuf file compiled successfully"),
        Err(e) => println!("Failed to compile protobuf file: {:?}", e),
    }

    // Confirm the file was generated
    let generated_file = Path::new(&out_dir).join("_.rs");
    if generated_file.exists() {
        println!("Generated file exists: {:?}", generated_file);
    } else {
        println!("Generated file does NOT exist: {:?}", generated_file);
    }
}
