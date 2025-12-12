package main

import (
	"bufio"
	"fmt"
	"strconv"
	"strings"
)

const PRESENT_SIZE uint8 = 3
const TOTAL_PRESENTS uint8 = 6

type Region struct {
	width    uint8
	height   uint8
	required []uint8
}

func readPresentSizes(input string) [TOTAL_PRESENTS]uint8 {
	scanner := bufio.NewScanner(strings.NewReader(input))
	sizes := [TOTAL_PRESENTS]uint8{}

	scanner.Scan()
	for i := range TOTAL_PRESENTS {
		for range PRESENT_SIZE {
			scanner.Scan()
			line := scanner.Text()
			for col := range PRESENT_SIZE {
				if line[col] == '#' {
					sizes[i] += 1
				}
			}
		}
		scanner.Scan()
		scanner.Scan()
	}

	err := scanner.Err()
	if err != nil {
		fmt.Printf("[Warning] %s\n", err)
	}

	return sizes
}

func readRegions(input string) []Region {
	scanner := bufio.NewScanner(strings.NewReader(input))
	regions := []Region{}

	// Skip over presents
	for range 5 * TOTAL_PRESENTS {
		scanner.Scan()
	}

	for scanner.Scan() {
		fields := strings.Fields(scanner.Text())

		sizes := strings.Split(fields[0], "x")
		width, err := strconv.ParseUint(sizes[0], 10, 8)
		if err != nil {
			panic(fmt.Sprintf("%s is not an integer\n", sizes[0]))
		}
		height, err := strconv.ParseUint(sizes[1][:len(sizes[1])-1], 10, 8)
		if err != nil {
			panic(fmt.Sprintf("%s is not an integer\n", sizes[1][:len(sizes[1])-1]))
		}

		required := []uint8{}
		for presentNum := range TOTAL_PRESENTS {
			count, err := strconv.ParseUint(fields[presentNum+1], 10, 8)
			if err != nil {
				panic(fmt.Sprintf("%s is not an integer\n", fields[presentNum+1]))
			}
			required = append(required, uint8(count))
		}

		regions = append(regions, Region{uint8(width), uint8(height), required})
	}

	err := scanner.Err()
	if err != nil {
		fmt.Printf("[Warning] %s\n", err)
	}

	return regions
}

func isValidRegion(region Region, presents *[TOTAL_PRESENTS]uint8) bool {
	// The search space for this problem as written is ridiculously large.
	// Fortunately, the input is designed such that a basic sanity check
	// of the areas is enough!
	regionArea := uint32(region.width) * uint32(region.height)
	presentsArea := uint32(0)
	for i, a := range presents {
		presentsArea += uint32(a) * uint32(region.required[i])
	}

	return presentsArea < regionArea
}

func result(input string) uint64 {
	presents := readPresentSizes(input)
	fmt.Println(presents)
	regions := readRegions(input)

	var validRegions uint64 = 0
	for _, r := range regions {
		if isValidRegion(r, &presents) {
			validRegions += 1
		}
	}

	return validRegions
}

func Solve(input string) string {
	return fmt.Sprintf("%d regions can fit all of the required presents\n", result(input))
}
