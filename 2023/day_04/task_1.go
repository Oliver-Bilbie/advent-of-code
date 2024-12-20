package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"strconv"
)

const WINNING_NUMBERS_LENGTH int = 10
const CARD_NUMBERS_LENGTH int = 25

const WINNING_NUMBERS_START = 10
const WINNING_NUMBERS_END = 39

const CARD_NUMBERS_START = 42
const CARD_NUMBERS_END = 116

const (
	WINNING_NUMBERS bool = true
	CARD_NUMBERS         = false
)

func get_number_set(card_values string, number_set bool) []uint8 {
	var length int
	var start uint8
	var end uint8

	switch number_set {
	case WINNING_NUMBERS:
		length = WINNING_NUMBERS_LENGTH
		start = WINNING_NUMBERS_START
		end = WINNING_NUMBERS_END
	case CARD_NUMBERS:
		length = CARD_NUMBERS_LENGTH
		start = CARD_NUMBERS_START
		end = CARD_NUMBERS_END
	}

	// Since the number positions are constant, we can just hardcode them
	number_set_string := card_values[start:end]

	var number_set_values []uint8 = make([]uint8, length)
	for i := 0; i < length; i++ {
		number, err := strconv.ParseUint(number_set_string[3*i:3*i+2], 10, 16)
		if err != nil {
			// Single digit numbers will throw an error so we need to parse them differently
			number, _ = strconv.ParseUint(number_set_string[3*i+1:3*i+2], 10, 16)
		}
		number_set_values[i] = uint8(number)
	}

	return number_set_values
}

func get_card_score(card_values string) uint32 {
	winning_numbers := get_number_set(card_values, WINNING_NUMBERS)
	card_numbers := get_number_set(card_values, CARD_NUMBERS)

	// Evaluate the score
	var score uint32 = 0
	for i := 0; i < len(winning_numbers); i++ {
		winning_number := winning_numbers[i]
		if slices.Contains(card_numbers, winning_number) {
			if score == 0 {
				score = 1
			} else {
				score *= 2
			}
		}
	}

	return score
}

func main() {
	file, _ := os.Open("input.txt")
	defer file.Close()
	scanner := bufio.NewScanner(file)

	var total_points uint32 = 0

	for scanner.Scan() {
		var line_data string = scanner.Text()
		total_points += get_card_score(line_data)
	}

	fmt.Printf("Total points: %d\n", total_points)
}
