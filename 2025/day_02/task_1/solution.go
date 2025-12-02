package main

import (
	"fmt"
	"strconv"
	"strings"
)

func isValidId(id uint64) bool {
	// If the id is of the form "abcabc" then it will be exactly
	// divisible by "1001"

	digit_count := getDigitCount(id)
	if digit_count%2 > 0 {
		return true
	}

	return id%getDivisor(digit_count) != 0
}

func getDigitCount(num uint64) int {
	count := 0
	for num > 0 {
		count++
		num /= 10
	}
	return count
}

func getDivisor(digit_count int) uint64 {
	return pow10(digit_count/2) + 1
}

func pow10(n int) uint64 {
	var v uint64 = 1
	for range n {
		v *= 10
	}
	return v
}

func strToInt(id string) uint64 {
	id_int, err := strconv.ParseUint(id, 10, 64)
	if err != nil {
		panic(fmt.Sprintf("ID is not an integer: %s\n", id))
	}
	return id_int
}

func result(input string) uint64 {
	input = strings.TrimSpace(input)

	var sum uint64 = 0

	for r := range strings.SplitSeq(input, ",") {
		values := strings.Split(r, "-")
		first := strToInt(values[0])
		last := strToInt(values[1])

		for i := first; i <= last; i++ {
			if !isValidId(i) {
				sum += uint64(i)
			}
		}
	}

	return sum
}

func Solve(input string) string {
	return fmt.Sprintf("Answer: %d\n", result(input))
}
