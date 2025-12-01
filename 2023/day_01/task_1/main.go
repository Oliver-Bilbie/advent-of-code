package main

import (
	"fmt"
	"os"
)

func main() {
	input, err := os.ReadFile("../input.txt")
	if err != nil {
		fmt.Println("Error reading input file:", err)
		return
	}
	fmt.Println(Solve(string(input)))
}
