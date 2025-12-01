package main

import (
	"bufio"
	"fmt"
	"slices"
	"strconv"
	"strings"
)

const NUM_OF_CARDS uint8 = 198

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

func update_card_totals(card_totals []uint32, card_values string) {
	// We need to convert the card id from a string to a uint8
	// This somehow takes nine lines in Go...
	// (This may be a skill issue)
	var card_id uint8
	card_id_u64, err := strconv.ParseUint(card_values[5:8], 10, 16)
	if err != nil {
		card_id_u64, err = strconv.ParseUint(card_values[6:8], 10, 16)
		if err != nil {
			card_id_u64, _ = strconv.ParseUint(card_values[7:8], 10, 16)
		}
	}
	card_id = uint8(card_id_u64)

	winning_numbers := get_number_set(card_values, WINNING_NUMBERS)
	card_numbers := get_number_set(card_values, CARD_NUMBERS)

	var num_of_matches uint8 = 0

	for i := 0; i < len(winning_numbers); i++ {
		winning_number := winning_numbers[i]
		if slices.Contains(card_numbers, winning_number) {
			num_of_matches++
		}
	}

	var max_id_to_increase uint8
	if card_id+num_of_matches-1 < NUM_OF_CARDS-1 {
		max_id_to_increase = card_id - 1 + num_of_matches
	} else {
		max_id_to_increase = NUM_OF_CARDS - 1
	}

	for id_to_increase := card_id; id_to_increase <= max_id_to_increase; id_to_increase++ {
		card_totals[id_to_increase] += card_totals[card_id-1]
	}
}

func Solve(input string) string {
	scanner := bufio.NewScanner(strings.NewReader(input))

	var card_totals []uint32 = make([]uint32, NUM_OF_CARDS)
	for i := range card_totals {
		card_totals[i] = 1
	}

	for scanner.Scan() {
		var line_data string = scanner.Text()
		update_card_totals(card_totals, line_data)
	}

	var total_cards uint32 = 0
	for i := 0; i < int(NUM_OF_CARDS); i++ {
		total_cards += card_totals[i]
	}

	return fmt.Sprintf("Total points: %d\n", total_cards)
}
