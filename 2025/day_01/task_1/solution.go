package main

import (
	"bufio"
	"fmt"
	"strconv"
	"strings"
)

func adjustLeft(current int, distance int) int {
	return (current - distance%100 + 100) % 100
}

func adjustRight(current int, distance int) int {
	return (current + distance) % 100
}

func Solve(input string) string {
	scanner := bufio.NewScanner(strings.NewReader(input))

	var zeroes uint32 = 0
	var position int = 50

	for scanner.Scan() {
		var instruction string = scanner.Text()

		direction := instruction[0]
		distance, _ := strconv.Atoi(instruction[1:])

		switch direction {
		case 'L':
			position = adjustLeft(position, distance)

		case 'R':
			position = adjustRight(position, distance)

		default:
			panic("Invalid direction found")
		}

		if position == 0 {
			zeroes += 1
		}
	}

	return fmt.Sprintf("Answer: %d\n", zeroes)
}
