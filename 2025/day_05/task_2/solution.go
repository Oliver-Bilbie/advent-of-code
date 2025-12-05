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

func readRanges(input string) []Range {
	scanner := bufio.NewScanner(strings.NewReader(input))
	ranges := []Range{}

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

	err := scanner.Err()
	if err != nil {
		fmt.Printf("[Warning] %s\n", err)
	}

	return ranges
}

func result(input string) uint64 {
	ranges := readRanges(input)

	var total_fresh uint64 = 0
	sortRanges(ranges)

	current := ranges[0]
	for _, rng := range ranges[1:] {
		if rng.first > current.last {
			total_fresh += uint64(current.last - current.first + 1)
			current = rng
		} else {
			if rng.last > current.last {
				current.last = rng.last
			}
		}
	}
	total_fresh += uint64(current.last - current.first + 1)

	return total_fresh
}

func Solve(input string) string {
	return fmt.Sprintf("Answer: %d\n", result(input))
}
