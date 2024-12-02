package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type mapping struct {
	source_range_start      uint64
	destination_range_start uint64
	range_length            uint64
}

type seed struct {
	range_start  uint64
	range_length uint64
}

func apply_mapping(section_mappings []mapping, source_value uint64) uint64 {
	var destination_value uint64 = source_value
	for _, mapping := range section_mappings {
		if source_value >= mapping.source_range_start && source_value < mapping.source_range_start+mapping.range_length {
			destination_value = mapping.destination_range_start + (source_value - mapping.source_range_start)
		}
	}
	return destination_value
}

func string_to_reverse_mapping(input string) mapping {
	values := strings.Split(input, " ")
	source_range_start, _ := strconv.ParseUint(values[0], 10, 32)
	destination_range_start, _ := strconv.ParseUint(values[1], 10, 32)
	range_length, _ := strconv.ParseUint(values[2], 10, 32)

	return mapping{source_range_start, destination_range_start, range_length}
}

func string_to_seeds(input string) []seed {
	values := strings.Split(input, " ")
	var seeds []seed
	for i := 1; i < len(values); i = i + 2 {
		range_start, _ := strconv.ParseUint(values[i], 10, 32)
		range_length, _ := strconv.ParseUint(values[i+1], 10, 32)
		seeds = append(seeds, seed{range_start, range_length})
	}
	return seeds
}

func get_destinations_from_mappings(section_mappings []mapping) []uint64 {
	var destinations []uint64
	for _, mapping := range section_mappings {
		// Confusingly we actually need the sorce range because the mappings are reversed
		destinations = append(destinations, mapping.source_range_start)
	}
	return destinations
}

func is_seed(seed_id uint64, seeds []seed) bool {
	for _, seed := range seeds {
		if seed_id >= seed.range_start && seed_id < seed.range_start+seed.range_length {
			return true
		}
	}
	return false
}

func main() {
	file, _ := os.Open("input.txt")
	defer file.Close()
	scanner := bufio.NewScanner(file)

	// Read seed values
	scanner.Scan()
	var seeds []seed = string_to_seeds(scanner.Text())
	scanner.Scan()
	scanner.Scan()

	// Read mappings (with source and target reversed, so we can find the seed for a given location)
	var sections [][]mapping

	var section_mappings []mapping
	for scanner.Scan() {
		var line_data string = scanner.Text()
		if line_data != "" {
			section_mappings = append(section_mappings, string_to_reverse_mapping(line_data))
		} else {
			sections = append(sections, section_mappings)
			section_mappings = nil
			scanner.Scan() // Skip the following header line
		}
	}
	sections = append(sections, section_mappings)

	// Evaluate the seed for each location until we find one that exists
	var location_index int = 0
	var seed_exists bool = false
	for seed_exists == false {
    var mapped_item uint64 = uint64(location_index)
		for mapping_index := len(sections) - 1; mapping_index >= 0; mapping_index-- {
			var section_mappings []mapping = sections[mapping_index]
      mapped_item = apply_mapping(section_mappings, mapped_item)
		}
    seed_exists = is_seed(mapped_item, seeds)
    location_index++
	}

  fmt.Printf("Closest location: %d\n", location_index-1)
}
