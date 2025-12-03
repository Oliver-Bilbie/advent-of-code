package main

import (
	"bufio"
	"fmt"
	"strings"
)

const BATTERY_COUNT = 12

// since we only need a few values these are hard-coded to improve performance
var pow10 = []uint64{1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000, 10000000000, 100000000000}

func result(input string) uint64 {
	scanner := bufio.NewScanner(strings.NewReader(input))
	var total uint64 = 0

	for scanner.Scan() {
		bank := scanner.Text()

		batteries := [BATTERY_COUNT]uint64{}
		for i := range BATTERY_COUNT {
			batteries[i] = numericCharToInt(bank[len(bank)-BATTERY_COUNT+i])
		}

		for i := len(bank) - BATTERY_COUNT - 1; i >= 0; i-- {
			jolts := numericCharToInt(bank[i])

			for position, current_jolts := range batteries {
				if jolts < current_jolts {
					break
				}

				batteries[position] = jolts
				jolts = current_jolts
			}
		}

		for position, jolts := range batteries {
			total += jolts * pow10[BATTERY_COUNT-position-1]
		}
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
