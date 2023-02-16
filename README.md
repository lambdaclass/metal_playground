# Metal playground in rust
Made for learning how to parallelize computations in the GPU using Apple's Metal, in Rust, via the [metal](https://crates.io/crates/metal) crate.

## Overview
The source code will contain documented examples of use, growing in complexity, aimed at a final objective of parallelize a Fast Fourier Transform algorithm.

### Contents
1. **dotprod**: learn the basics of Metal and metal-rs, implement a simple dot product between two vectors represented as uint arrays.

## Pre-requisites

- [XCode](https://www.freecodecamp.org/news/how-to-download-and-install-xcode/)
- [Rust](https://www.rust-lang.org/es/tools/install)

To run the example, use the following command: 

`cargo run --example dotprod`

This command will automatically build the metal related code to generate the `.metallib` file that the rust app uses.

## References
- [Apple's Metal documentation](https://developer.apple.com/documentation/metal): we recommend to start with "Performing Calculations on a GPU". Note that these docs are in Swift/Obj-C.
- [miniSTARK](https://github.com/andrewmilson/ministark): A minimal STARK library built in Rust and gpu-accelerated with Metal.
- [metal-rs examples](https://github.com/gfx-rs/metal-rs)
