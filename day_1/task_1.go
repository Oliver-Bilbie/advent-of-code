package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func find_first_num(input string) uint8 {
	var first_num uint8

	for i := 0; i < len(input); i++ {
		current_char := input[i]
		current_num, err := strconv.ParseUint(string(current_char), 10, 64)
		if err == nil {
			first_num = uint8(current_num)
			break
		}
	}

	return first_num
}

func find_last_num(input string) uint8 {
	var last_num uint8

	for i := len(input) - 1; i >= 0; i-- {
		current_char := input[i]
		current_num, err := strconv.ParseUint(string(current_char), 10, 64)
		if err == nil {
			last_num = uint8(current_num)
			break
		}
	}

	return last_num
}

func task_1() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)

	var calibration_value_sum uint64 = 0

	for scanner.Scan() {
		var line_data string = scanner.Text()
		var first_number uint8 = find_first_num(line_data)
		var last_number uint8 = find_last_num(line_data)
		var calibration_value uint8 = 10*first_number + last_number
		calibration_value_sum += uint64(calibration_value)
	}

	fmt.Printf("Sum of calibration values: %d\n", calibration_value_sum)

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}

func main() {
	task_1()
}
