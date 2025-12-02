package main

import (
	"fmt"
	"strconv"
	"strings"
)

func isValidId(id int) bool {
	id_str := strconv.Itoa(id)
	midpoint := len(id_str) / 2
	return id_str[:midpoint] != id_str[midpoint:]
}

func strToInt(id string) int {
	id_int, err := strconv.Atoi(id)
	if err != nil {
		panic(fmt.Sprintf("ID is not an integer: %s\n", id))
	}
	return id_int
}

func result(input string) uint64 {
	var sum uint64 = 0
	ranges := strings.Split(strings.TrimSpace(input), ",")

	for _, r := range ranges {
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
