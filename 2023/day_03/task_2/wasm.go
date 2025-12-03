//go:build js

package main

import "syscall/js"

var SolverName string

func main() {
	js.Global().Set(SolverName, js.FuncOf(func(this js.Value, args []js.Value) any {
		input := args[0].String()
		return Solve(input)
	}))

	select {}
}
