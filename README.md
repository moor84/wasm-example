# Experimenting with WebAssembly

## Installation

Install Rust

Install Go

cargo install wasm-gc miniserve

## Build
`./build.sh` - Build pure Rust

`./build_bindgen.sh` - Build rust-bindgen project

`./build_go.sh` - Build Go project

## Serve

`miniserve . --index index.html` - pure Rust

`miniserve . --index index_bindgen.html` - Rust-bindgen

`miniserve . --index index_go.html` - Serve Go project
