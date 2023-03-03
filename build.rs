use std::process::Command;

const METAL_IN: [&str; 2] = [
    "examples/dotprod/metal/dot_product",
    "examples/fp/metal/fp_ops",
];

fn main() {
    for file in METAL_IN {
        Command::new("xcrun")
            .args(["-sdk", "macosx", "metal"])
            .args([&(file.to_owned() + ".metal")])
            .args(["-o", &(file.to_owned() + ".metallib")])
            .status()
            .expect("Failed to execute xcrun");

        println!("cargo:rerun-if-changed={}", file);
    }

    println!("cargo:rerun-if-changed=build.rs");
}
