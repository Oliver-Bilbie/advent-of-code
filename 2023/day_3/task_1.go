package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func is_symbol(char byte) bool {
	var non_symbols = [11]byte{'.', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0'}
	for _, non_symbol := range non_symbols {
		if char == non_symbol {
			return false
		}
	}
	return true
}

func is_numeric(char byte) bool {
	var numerics = [10]byte{'1', '2', '3', '4', '5', '6', '7', '8', '9', '0'}
	for _, numeric := range numerics {
		if char == numeric {
			return true
		}
	}
	return false
}

func find_line_symbols(input string) []int16 {
	var symbol_locations []int16

	for i := 0; i < len(input); i++ {
		if is_symbol(input[i]) {
			symbol_locations = append(symbol_locations, int16(i))
		}
	}

	return symbol_locations
}

func get_line_sum(input string, symbol_locations []int16) uint64 {
	var line_sum uint64 = 0
	var start_column int16 = 0
	var end_column int16 = 0

	for start_column < int16(len(input)) {
		if is_numeric(input[start_column]) {
      end_column = start_column + 1
			for end_column < int16(len(input)) {
				if is_numeric(input[end_column]) {
					end_column++
				} else {
					break
				}
			}

      // Check whether the number is adjacent to a symbol
      for _, symbol_location := range symbol_locations {
        if symbol_location >= start_column - 1 && symbol_location <= end_column {
          // If the number is adjacent to a symbol, add it to the sum
          var part_number_str string = input[start_column:end_column]
          part_number, err := strconv.ParseUint(part_number_str, 10, 64)
          if err != nil {
            log.Fatal(err)
          }

          line_sum += part_number
          break
        }
      }

      start_column = end_column
		} else {
			start_column++
		}
	}

  return line_sum
}

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	scanner := bufio.NewScanner(file)

	var part_numbers_sum uint64 = 0
	var line_number int16 = 0
	var symbol_locations [][]int16

	for scanner.Scan() {
		var line_data string = scanner.Text()
		symbol_locations = append(symbol_locations, find_line_symbols(line_data))
		line_number++
	}

	file.Seek(0, 0)
	scanner = bufio.NewScanner(file)
  line_number = 0
	for scanner.Scan() {
		var line_data string = scanner.Text()
    var nearby_symbols []int16

    if line_number == 0 {
      nearby_symbols = append(symbol_locations[0], symbol_locations[1]...)
    } else if line_number == int16(len(symbol_locations) - 1) {
      nearby_symbols = append(symbol_locations[line_number - 1], symbol_locations[line_number]...)
    } else {
      nearby_symbols = append(append(symbol_locations[line_number - 1], symbol_locations[line_number]...), symbol_locations[line_number + 1]...)
    }

    part_numbers_sum += get_line_sum(line_data, nearby_symbols)

    line_number++
	}

	fmt.Printf("Sum of part numbers: %d\n", part_numbers_sum)

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	file.Close()
}
