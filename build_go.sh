cd go
GOOS=js GOARCH=wasm go build -o ../wasm_example_go.wasm
cd ..
wasm-gc wasm_example_go.wasm wasm_example_go.wasm
