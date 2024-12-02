package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

type gear struct {
	line     int64
	column   int64
	number_1 int64
	number_2 int64
}

func is_asterix(char byte) bool {
	return char == 42
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

func find_line_gears(input string, line_number int64) []gear {
	var line_gears []gear

	for i := 0; i < len(input); i++ {
		if is_asterix(input[i]) {
			line_gears = append(line_gears, gear{line: line_number, column: int64(i), number_1: 0, number_2: 0})
		}
	}

	return line_gears
}

func evaluate_line_numbers(input string, line_number int64, gears []gear) []gear {
	var start_column int64 = 0
	var end_column int64 = 0

	for start_column < int64(len(input)) {
		if is_numeric(input[start_column]) {
			end_column = start_column + 1
			for end_column < int64(len(input)) {
				if is_numeric(input[end_column]) {
					end_column++
				} else {
					break
				}
			}

			// Check whether the number is adjacent to a gear
			// This is a bit inefficient but I don't have more time to commit to this
      for i := 0; i < len(gears); i++ {
				// If the number is adjacent to a gear, attach it to the gear
				if gears[i].line >= line_number-1 && gears[i].line <= line_number+1 && gears[i].column >= start_column-1 && gears[i].column <= end_column {
					if gears[i].number_1 == 0 {
						gears[i].number_1, _ = strconv.ParseInt(input[start_column:end_column], 10, 16)
					} else if gears[i].number_2 == 0 {
						gears[i].number_2, _ = strconv.ParseInt(input[start_column:end_column], 10, 16)
					} else {
            log.Fatal("Gear has more than 2 numbers")
					}
				}
			}

			start_column = end_column
		} else {
			start_column++
		}
	}

	return gears
}

func evaluate_gear_ratio_sum(gears []gear) int64 {
  var gear_ratios_sum int64 = 0
  for i := 0; i < len(gears); i++ {
    gear_ratios_sum += gears[i].number_1 * gears[i].number_2
  }
  return gear_ratios_sum
}

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	scanner := bufio.NewScanner(file)

	var line_number int64 = 0
	var gears []gear

	for scanner.Scan() {
		var line_data string = scanner.Text()
		gears = append(gears, find_line_gears(line_data, line_number)...)
		line_number++
	}

	file.Seek(0, 0)
	scanner = bufio.NewScanner(file)
	line_number = 0
	for scanner.Scan() {
		var line_data string = scanner.Text()

    gears = evaluate_line_numbers(line_data, line_number, gears)

		line_number++
	}

  gear_ratios_sum := evaluate_gear_ratio_sum(gears)
	fmt.Printf("Sum of gear ratios: %d\n", gear_ratios_sum)

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	file.Close()
}
