package main

import (
	"bufio"
	"fmt"
	"strings"
)

func result(input string) uint64 {
	scanner := bufio.NewScanner(strings.NewReader(input))
	var total_joltage uint64 = 0

	for scanner.Scan() {
		var bank string = scanner.Text()
		first := numericCharToInt(bank[len(bank)-2])
		second := numericCharToInt(bank[len(bank)-1])

		for i := len(bank) - 3; i >= 0; i-- {
			value := numericCharToInt(bank[i])

			if value >= first {
				if first > second {
					second = first
				}
				first = value
			}
		}

		total_joltage += uint64(10*first + second)
	}

	return total_joltage
}

func numericCharToInt(c byte) uint64 {
	return uint64(c - '0') // this is yucky but fast
}

func Solve(input string) string {
	return fmt.Sprintf("Answer: %d\n", result(input))
}
