use std::process::Command;
use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let zig_path = "vendor/trinos-golden-float";
    
    println!("Building crypto library...");
    let status = Command::new("zig")
        .args(&["build", "-Doptimize=ReleaseFast", "-Dshared=true"])
        .current_dir(zig_path)
        .status()
        .expect("Failed to build zig library");
    
    assert!(status.success());
    
    println!("cargo:rustc-link-search=native={}/zig-out/lib", zig_path);
    println!("cargo:rustc-link-lib=static=crypto", zig_path);
    println!("cargo:rerun-if-changed={}/src", zig_path);
}
