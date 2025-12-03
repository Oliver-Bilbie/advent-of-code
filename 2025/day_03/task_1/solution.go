package main

import (
	"bufio"
	"fmt"
	"strings"
)

func result(input string) uint64 {
	scanner := bufio.NewScanner(strings.NewReader(input))
	var total uint64 = 0

	for scanner.Scan() {
		bank := scanner.Text()
		first := numericCharToInt(bank[len(bank)-2])
		second := numericCharToInt(bank[len(bank)-1])

		for i := len(bank) - 3; i >= 0; i-- {
			jolts := numericCharToInt(bank[i])

			if jolts >= first {
				if first > second {
					second = first
				}
				first = jolts
			}
		}

		total += uint64(10*first + second)
	}

	err := scanner.Err()
	if err != nil {
		fmt.Printf("[Warning] %s\n", err)
	}

	return total
}

func numericCharToInt(c byte) uint64 {
	result := int(c - '0')
	if result < 0 || result > 9 {
		panic(fmt.Sprintf("[Error] unable to convert %c to an integer\n", c))
	}
	return uint64(result)
}

func Solve(input string) string {
	return fmt.Sprintf("The total joltage is: %d\n", result(input))
}
