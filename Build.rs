use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    // Set the default output directory if the environment variable is not provided
    let out_dir = match env::var("PROTO_OUTPUT_DIR") {
        Ok(val) => val,
        Err(_) => {
            println!("PROTO_OUTPUT_DIR environment variable not provided, using default.");
            "G:\\servers\\protos".to_string() // Default output directory
        }
    };

    println!("Output directory: {}", out_dir);

    // Ensure the output directory exists or create it if it doesn't
    if !Path::new(&out_dir).exists() {
        fs::create_dir_all(&out_dir).expect("Failed to create output directory");
        println!("Output directory created: {}", out_dir);
    }

    let proto_file = "src/_.proto";
    println!("Compiling protobuf file: {}", proto_file);

    // Compile the Protobuf definitions
    match prost_build::Config::new().out_dir(&out_dir).compile_protos(&[proto_file], &["src/"]) {
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
