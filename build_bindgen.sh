# Install wasm-pack CLI
cargo install wasm-pack

# Compile the project to web assembly
wasm-pack build --target web ./rust_bindgen

# Copy the resulting package
cp -r ./rust_bindgen/pkg .

# Copy the wasm file (to run from Python)
cp ./rust_bindgen/pkg/wasm_example_bindgen_bg.wasm ./wasm_example_bindgen.wasm
