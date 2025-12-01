package main

import (
	"unsafe"
)

// We store the result as a global variable to prevent it being
// lost to GC before the caller can read it
var result []byte

//export solve
func solve(ptr uint32, size uint32) uint64 {
	inputBytes := unsafe.Slice((*byte)(unsafe.Pointer(uintptr(ptr))), size)
	input := string(inputBytes)

	result = []byte(Solve(input))

	resultPtr := uintptr(unsafe.Pointer(&result[0]))
	resultLen := uint64(len(result))

	return (uint64(resultPtr) << 32) | resultLen
}
