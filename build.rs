use tonic_build;
use std::fs;
use std::process::Command;
use std::env;
use std::path::PathBuf;
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let debug = env::var("DEBUG").unwrap_or_else(|_| "false".to_string()) == "true";

    if debug {
        // Perform debug-specific checks
        println!("cargo:rerun-if-changed=proto/arsmedicatech/fhir_sync.proto");

        if !fs::metadata("proto/arsmedicatech/fhir_sync.proto").is_ok() {
            panic!("File not found: proto/arsmedicatech/fhir_sync.proto");
        } else {
            println!("Found proto file, proceeding with build.");
        }

        let out = Command::new(std::env::var("PROTOC").unwrap_or_else(|_| "protoc".into()))
            .arg("--proto_path=proto")
            .arg("--proto_path=.")
            .arg("--descriptor_set_out=descriptor.pb")
            .arg("proto/arsmedicatech/fhir_sync.proto")
            .output()?;  // This gives us access to both stdout and stderr

        if !out.status.success() {
            eprintln!("protoc failed with status: {}", out.status);
            eprintln!("stdout:\n{}", String::from_utf8_lossy(&out.stdout));
            eprintln!("stderr:\n{}", String::from_utf8_lossy(&out.stderr));
            panic!("protoc invocation failed");
        }

        println!("protoc succeeded, proceeding to compile with tonic_build");
    }

    // Force set protoc path
    let protoc_path = "/usr/bin/protoc"; // or `which protoc` output
    env::set_var("PROTOC", protoc_path);

    println!("Using protoc at: {}", protoc_path);

    let proto_root = PathBuf::from("proto");

    // Recursively collect all .proto files
    let proto_files: Vec<PathBuf> = WalkDir::new(&proto_root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|f| f.path().extension().map(|e| e == "proto").unwrap_or(false))
        .map(|f| f.into_path())
        .collect();

    // Optional: dump what you're compiling
    for file in &proto_files {
        println!("Compiling: {}", file.display());
    }

    tonic_build::configure()
        .out_dir("src/proto")          // generated modules live here
        .compile_protos(
            &proto_files.iter().map(|p| p.to_str().unwrap()).collect::<Vec<_>>(),
            //&["proto"]
            &["proto", "/usr/include"]
        )?;
    Ok(())
}
