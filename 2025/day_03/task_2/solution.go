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
		var joltages = [12]uint64{
			numericCharToInt(bank[len(bank)-12]),
			numericCharToInt(bank[len(bank)-11]),
			numericCharToInt(bank[len(bank)-10]),
			numericCharToInt(bank[len(bank)-9]),
			numericCharToInt(bank[len(bank)-8]),
			numericCharToInt(bank[len(bank)-7]),
			numericCharToInt(bank[len(bank)-6]),
			numericCharToInt(bank[len(bank)-5]),
			numericCharToInt(bank[len(bank)-4]),
			numericCharToInt(bank[len(bank)-3]),
			numericCharToInt(bank[len(bank)-2]),
			numericCharToInt(bank[len(bank)-1]),
		}

		for i := len(bank) - 13; i >= 0; i-- {
			value := numericCharToInt(bank[i])

			for position, jolts := range joltages {
				if value < jolts {
					break
				}

				// swap := jolts
				joltages[position] = value
				value = jolts
			}
		}

		for position, jolts := range joltages {
			total_joltage += jolts * pow10(11-position)
		}
	}

	return total_joltage
}

func numericCharToInt(c byte) uint64 {
	return uint64(c - '0') // this is yucky but fast
}

func pow10(n int) uint64 {
	var v uint64 = 1
	for range n {
		v *= 10
	}
	return v
}

func Solve(input string) string {
	return fmt.Sprintf("Answer: %d\n", result(input))
}
