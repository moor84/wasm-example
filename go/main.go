package main

import "syscall/js"

func fibonacci(n int) int {
	if n <= 1 {
		return n
	}
	return fibonacci(n-1) + fibonacci(n-2)
}

func fibonacciJS(this js.Value, inputs []js.Value) interface{} {
	n := inputs[0].Int()
	js.Global().Get("console").Call("log", n)
	return fibonacci(n)
}

func main() {
	c := make(chan bool)
	js.Global().Set("fibonacci", js.FuncOf(fibonacciJS))
	js.Global().Get("console").Call("log", "Hello from Go!")
	<-c
}
