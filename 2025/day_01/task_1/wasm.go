//go:build js

package main

import (
	"syscall/js"
)

func main() {
	js.Global().Set("solve", js.FuncOf(func(this js.Value, args []js.Value) interface{} {
		input := args[0].String()
		return Solve(input)
	}))

	select {}
}
