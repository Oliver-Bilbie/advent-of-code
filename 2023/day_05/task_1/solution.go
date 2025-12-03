package main

import (
	"bufio"
	"fmt"
	"strconv"
	"strings"
)

type mapping struct {
	source_range_start      uint64
	destination_range_start uint64
	range_length            uint64
}

func apply_mappings(section_mappings []mapping, source_values []uint64) {
	for i, source_value := range source_values {
		for _, mapping := range section_mappings {
			if source_value >= mapping.source_range_start && source_value < mapping.source_range_start+mapping.range_length {
				source_values[i] = mapping.destination_range_start + (source_value - mapping.source_range_start)
			}
		}
	}
}

func string_to_mapping(input string) mapping {
	values := strings.Split(input, " ")
	source_range_start, _ := strconv.ParseUint(values[1], 10, 32)
	destination_range_start, _ := strconv.ParseUint(values[0], 10, 32)
	range_length, _ := strconv.ParseUint(values[2], 10, 32)

	return mapping{source_range_start, destination_range_start, range_length}
}

func string_to_seeds(input string) []uint64 {
	values := strings.Split(input, " ")
	var seeds []uint64
	for i := 1; i < len(values); i++ {
		seed, _ := strconv.ParseUint(values[i], 10, 32)
		seeds = append(seeds, seed)
	}
	return seeds
}

func get_minimum_value(values []uint64) uint64 {
	var minimum uint64 = values[0]
	for i := 1; i < len(values); i++ {
		if values[i] < minimum {
			minimum = values[i]
		}
	}
	return minimum
}

func Solve(input string) string {
	scanner := bufio.NewScanner(strings.NewReader(input))

	// Read seed values
	scanner.Scan()
	var seeds []uint64 = string_to_seeds(scanner.Text())
	scanner.Scan()
	scanner.Scan()

	var section_mappings []mapping
	for scanner.Scan() {
		var line_data string = scanner.Text()
		if line_data != "" {
			section_mappings = append(section_mappings, string_to_mapping(line_data))
		} else {
			apply_mappings(section_mappings, seeds)

			section_mappings = nil
			scanner.Scan() // Skip the following header line
		}
	}
	apply_mappings(section_mappings, seeds)

	closest_seed := get_minimum_value(seeds)

	return fmt.Sprintf("closest seed: %d\n", closest_seed)
}
