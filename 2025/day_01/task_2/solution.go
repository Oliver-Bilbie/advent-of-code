package main

import (
	"bufio"
	"fmt"
	"strconv"
	"strings"
)

func adjustLeft(current int, distance int) (position int, zeroes uint32) {
	position = (current - distance%100 + 100) % 100

	if current == 0 {
		// We must avoid counting the starting zero
		zeroes = uint32(distance / 100)
	} else {
		zeroes = uint32((100 + distance - current) / 100)
	}

	return
}

func adjustRight(current int, distance int) (position int, zeroes uint32) {
	position = (current + distance) % 100
	zeroes = uint32((current + distance) / 100)

	return
}

func Solve(input string) string {
	scanner := bufio.NewScanner(strings.NewReader(input))

	var zeroes uint32 = 0
	var add_zeroes uint32 = 0
	var position int = 50

	for scanner.Scan() {
		var instruction string = scanner.Text()

		direction := instruction[0]
		distance, _ := strconv.Atoi(instruction[1:])

		switch direction {
		case 'L':
			position, add_zeroes = adjustLeft(position, distance)

		case 'R':
			position, add_zeroes = adjustRight(position, distance)

		default:
			panic("Invalid direction found")
		}

		zeroes += add_zeroes
	}

	return fmt.Sprintf("Answer: %d\n", zeroes)
}
