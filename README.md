# Rust Metal Playground

A repo for learning how to parallelize computations in the GPU using Apple's Metal, in Rust.

## Pre-Requisites

- [XCode](https://www.freecodecamp.org/news/how-to-download-and-install-xcode/)
- [Rust](https://www.rust-lang.org/es/tools/install)

At the moment there's one example that multiplies two arrays of unsigned integers. The example was made using [metal-rs](https://github.com/gfx-rs/metal-rs) library.

To run the example, use the following command: 

`cargo run --release`

This command will automatically build the metal related code to generate the `.metallib` file that the rust app use.
