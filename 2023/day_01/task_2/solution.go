package main

import (
	"bufio"
	"fmt"
	"strconv"
	"strings"
)

func find_first_num(input string) uint8 {
	var first_num_str int8
	var first_num_str_index int8 = int8(len(input) - 1)
	var first_num int8
	var first_num_index int8 = int8(len(input) - 1)

	// Find the first number encoded as a word
	var numbers_as_words = [9]string{"one", "two", "three", "four", "five", "six", "seven", "eight", "nine"}
	for i, current_num_str := range numbers_as_words {
		current_index := int8(strings.Index(input, current_num_str))
		if current_index > -1 { // If substring exists
			if current_index < first_num_str_index {
				first_num_str = int8(i + 1)
				first_num_str_index = current_index
			}
		}
	}

	// Find the first number
	for i := 0; i < len(input); i++ {
		current_char := input[i]
		current_num, err := strconv.ParseUint(string(current_char), 10, 64)
		if err == nil {
			first_num = int8(current_num)
			first_num_index = int8(i)
			break
		}
	}

	// Return the value with the smallest index
	if first_num_str_index < first_num_index {
		return uint8(first_num_str)
	} else {
		return uint8(first_num)
	}
}

func find_last_num(input string) uint8 {
	var last_num_str int8
	var last_num_str_index int8 = 0
	var last_num int8
	var last_num_index int8 = 0

	// Find the last number encoded as a word
	var numbers_as_words = [9]string{"one", "two", "three", "four", "five", "six", "seven", "eight", "nine"}
	for i, current_num_str := range numbers_as_words {
		current_index := int8(strings.LastIndex(input, current_num_str))
		if current_index > -1 { // If substring exists
			if current_index > last_num_str_index {
				last_num_str = int8(i + 1)
				last_num_str_index = current_index
			}
		}
	}

	// Find the last number
	for i := len(input) - 1; i >= 0; i-- {
		current_char := input[i]
		current_num, err := strconv.ParseUint(string(current_char), 10, 64)
		if err == nil {
			last_num = int8(current_num)
			last_num_index = int8(i)
			break
		}
	}

	// Return the value with the largest index
	if last_num_str_index > last_num_index {
		return uint8(last_num_str)
	} else {
		return uint8(last_num)
	}
}

func Solve(input string) string {
	scanner := bufio.NewScanner(strings.NewReader(input))

	var calibration_value_sum uint64 = 0

	for scanner.Scan() {
		var line_data string = scanner.Text()
		var first_number uint8 = find_first_num(line_data)
		var last_number uint8 = find_last_num(line_data)
		var calibration_value uint8 = 10*first_number + last_number
		fmt.Printf("%s has been evaluated to %d\n", line_data, calibration_value)
		calibration_value_sum += uint64(calibration_value)
	}

	return fmt.Sprintf("Sum of calibration values: %d\n", calibration_value_sum)
}
