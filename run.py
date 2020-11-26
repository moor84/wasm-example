import wasmtime.loader

# Assuming `wasm_example.wasm` is in the python load path...
import wasm_example

fib = wasm_example.fibonacci(10)
print(fib)
