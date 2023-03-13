# Metal playground in rust
Made for learning how to parallelize computations in the GPU using Apple's Metal, in Rust, via the [metal](https://crates.io/crates/metal) crate.

## Overview
The source code will contain documented examples of use, growing in complexity, aimed at a final objective of parallelize a Fast Fourier Transform algorithm.

### Contents
1. **dotprod**: learn the basics of Metal and metal-rs, implement a simple dot product between two vectors represented as uint arrays.
2. **matrixprod**: a more complex example to learn about grid size and thread groups, implement a product between square matrices.
3. **memory**: example to show how to shared memory between CPU and GPU. This example simply creates a vector to be modified from the GPU.

## Pre-requisites

- [XCode](https://www.freecodecamp.org/news/how-to-download-and-install-xcode/)
- [Rust](https://www.rust-lang.org/es/tools/install)

## Running the examples

To run the examples, use the following command: 

`make example EXAMPLE={example}`

> For the `memory` example some nightly features are needed so `+nightly` has to be added to the command above.

To re-build all the necessary `.metallib` files, you can use 

`make build_metal EXAMPLE={example}`

where `{example}` is the name of the example to run in both commands.

## References
- [Apple's Metal documentation](https://developer.apple.com/documentation/metal): we recommend to start with "Performing Calculations on a GPU". Note that these docs are in Swift/Obj-C.
- [miniSTARK](https://github.com/andrewmilson/ministark): A minimal STARK library built in Rust and gpu-accelerated with Metal.
- [metal-rs examples](https://github.com/gfx-rs/metal-rs)
