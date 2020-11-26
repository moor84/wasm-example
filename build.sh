# Install wasm target (if not already installed)
rustup target add wasm32-unknown-unknown

# Compile the project to web assembly
cargo build --manifest-path ./rust/Cargo.toml --target wasm32-unknown-unknown --release

# Optimise and minify the .wasm file
wasm-gc ./rust/target/wasm32-unknown-unknown/release/wasm_example.wasm ./web/wasm_example.wasm
cp ./web/wasm_example.wasm ./
