package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

const r_ascii byte = 114
const g_ascii byte = 103
const b_ascii byte = 98

func get_power(input string) uint64 {
	components := strings.Split(input, ": ")
	game_component := components[1]
	game_items := strings.Split(game_component, " ")

	var min_red uint64 = 0
	var min_green uint64 = 0
	var min_blue uint64 = 0

	for i := 0; i < len(game_items)-1; i += 2 {
		number_of_cubes, err := strconv.ParseUint(game_items[i], 10, 64)
		if err != nil {
			log.Fatal(err)
		}

		color_char := game_items[i+1][0]

		switch color_char {
		case r_ascii:
			if number_of_cubes > min_red {
				min_red = number_of_cubes
			}
		case g_ascii:
			if number_of_cubes > min_green {
				min_green = number_of_cubes
			}
		case b_ascii:
			if number_of_cubes > min_blue {
				min_blue = number_of_cubes
			}
		}
	}

	return min_red * min_green * min_blue
}

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)

	var powers_sum uint64 = 0

	for scanner.Scan() {
		var line_data string = scanner.Text()
		powers_sum += uint64(get_power(line_data))
	}

	fmt.Printf("Sum of powers: %d\n", powers_sum)

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}
