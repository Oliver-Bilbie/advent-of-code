package main

import (
	"fmt"
	"strconv"
	"strings"
)

func isValidId(id int) bool {
	id_str := strconv.Itoa(id)

	for chunk_count := 2; chunk_count <= len(id_str); chunk_count++ {
		if len(id_str)%chunk_count == 0 {
			chunk_size := len(id_str) / chunk_count
			chunk := id_str[:chunk_size]

			if strings.Repeat(chunk, chunk_count) == id_str {
				return false
			}
		}
	}

	return true
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
