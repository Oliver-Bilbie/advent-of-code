package main

import (
	"bufio"
	"fmt"
	"log"
	"strconv"
	"strings"
)

const TOTAL_RED uint64 = 12
const TOTAL_GREEN uint64 = 13
const TOTAL_BLUE uint64 = 14

const r_ascii byte = 114
const g_ascii byte = 103
const b_ascii byte = 98

func get_contribution(input string) uint8 {
	// Returns 0 if the game is impossible
	// Otherwise returns the game ID
	components := strings.Split(input, ": ")
	id_component := components[0]
	game_component := components[1]

	game_items := strings.Split(game_component, " ")
	for i := 0; i < len(game_items)-1; i += 2 {
		number_of_cubes, err := strconv.ParseUint(game_items[i], 10, 64)
		if err != nil {
			log.Fatal(err)
		}

		color_char := game_items[i+1][0]

		switch color_char {
		case r_ascii:
			if number_of_cubes > TOTAL_RED {
				return 0
			}
		case g_ascii:
			if number_of_cubes > TOTAL_GREEN {
				return 0
			}
		case b_ascii:
			if number_of_cubes > TOTAL_BLUE {
				return 0
			}
		}
	}

	game_id, err := strconv.ParseUint(id_component[5:], 10, 64)
	if err == nil {
		return uint8(game_id)
	} else {
		log.Fatal(err)
		return 0
	}
}

func Solve(input string) string {
	scanner := bufio.NewScanner(strings.NewReader(input))

	var possible_game_ids_sum uint64 = 0

	for scanner.Scan() {
		var line_data string = scanner.Text()
		possible_game_ids_sum += uint64(get_contribution(line_data))
	}

	return fmt.Sprintf("Sum of the IDs of possible games: %d\n", possible_game_ids_sum)
}
