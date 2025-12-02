//go:build !js

package main

import (
	"os"
	"testing"
)

func TestSolvesTheExample(t *testing.T) {
	got := getPassword(readTestInput(t))
	var want uint32 = 6

	if got != want {
		t.Fatalf("MyFunction() = %d; want %d", got, want)
	}
}

func readTestInput(t *testing.T) string {
	t.Helper()
	input, err := os.ReadFile("../test_input.txt")
	if err != nil {
		t.Fatal("cannot read the test input data")
	}
	return string(input)
}
