use std::process::Command;

const METAL_IN: &str = "src/metal/dot_product.metal";
const METAL_OUT: &str = "src/metal/dot_product.metallib";

fn main() {
    Command::new("xcrun")
        .args(&["-sdk macosx", "metal"])
        .args(&[METAL_IN])
        .args(&["-o", METAL_OUT])
        .status().unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", METAL_IN);
}
