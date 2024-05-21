use std::fs;
use std::path::Path;

fn main() {
    //set this to where you want the protos file to be saved
    let out_dir = "G:\\servers\\protos";
    println!("Output directory: {}", out_dir);

    // Ensure the output directory exists
    if !Path::new(out_dir).exists() {
        fs::create_dir_all(out_dir).expect("Failed to create output directory");
    }

    let proto_file = "src/_.proto";
    println!("Compiling protobuf file: {}", proto_file);

    // Compile the Protobuf definitions
    match prost_build::Config::new().out_dir(out_dir).compile_protos(&[proto_file], &["src/"]) {
        Ok(_) => println!("Protobuf file compiled successfully"),
        Err(e) => println!("Failed to compile protobuf file: {:?}", e),
    }

    // Confirm the file was generated
    let generated_file = Path::new(out_dir).join("_.rs");
    if generated_file.exists() {
        println!("Generated file exists: {:?}", generated_file);
    } else {
        println!("Generated file does NOT exist: {:?}", generated_file);
    }
}
