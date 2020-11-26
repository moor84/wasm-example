import wasmtime.loader

# Import `wasm_example_rust.wasm` from the current dirctory
import wasm_example_rust

fib_rust = wasm_example_rust.fibonacci(10)
print(f'Result from Rust: {fib_rust}')
