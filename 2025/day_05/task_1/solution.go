package main

import (
	"bufio"
	"fmt"
	"strconv"
	"strings"
)

type Range struct {
	first uint
	last  uint
}

func readInput(input string) (ranges []Range, ids []uint) {
	scanner := bufio.NewScanner(strings.NewReader(input))
	ranges = []Range{}
	ids = []uint{}

	// Read ranges
	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			break
		}

		first_str, last_str, found := strings.Cut(line, "-")
		if !found {
			panic(fmt.Sprintf("%s is not a valid range (no separator)", line))
		}
		first, first_err := strconv.ParseUint(first_str, 10, 0)
		last, last_err := strconv.ParseUint(last_str, 10, 0)
		if first_err != nil || last_err != nil {
			panic(fmt.Sprintf("%s is not a valid range (not integers)", line))
		}

		ranges = append(ranges, Range{uint(first), uint(last)})
	}

	// read ids
	for scanner.Scan() {
		line := scanner.Text()

		id, err := strconv.ParseUint(line, 10, 0)
		if err != nil {
			panic(fmt.Sprintf("%s is not a valid id", line))
		}

		ids = append(ids, uint(id))
	}

	err := scanner.Err()
	if err != nil {
		fmt.Printf("[Warning] %s\n", err)
	}

	return ranges, ids
}

func result(input string) uint64 {
	ranges, ids := readInput(input)
	total_fresh := 0

	for _, id := range ids {
		for _, rng := range ranges {
			if id >= rng.first && id <= rng.last {
				total_fresh += 1
				break
			}
		}
	}

	return uint64(total_fresh)
}

func Solve(input string) string {
	return fmt.Sprintf("Answer: %d\n", result(input))
}
