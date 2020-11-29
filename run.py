import wasmtime.loader

# Import `wasm_example_rust.wasm` from the current dirctory
import wasm_example_rust

fib_rust = wasm_example_rust.fibonacci(10)
print(f'Result from Rust: {fib_rust}')

# Import `wasm_example_bindgen.wasm` from the current dirctory
import wasm_example_bindgen

fib_rust_b = wasm_example_bindgen.fibonacci(10)
print(f'Result from Rust (wasm-pack): {fib_rust_b}')

print(wasm_example_bindgen.will_return_string(0))
