package main

import (
	"bufio"
	"fmt"
	"strconv"
	"strings"
)

func countInputLines(input string) uint8 {
	scanner := bufio.NewScanner(strings.NewReader(input))
	var count uint8 = 0

	for scanner.Scan() {
		count++
	}
	err := scanner.Err()
	if err != nil {
		fmt.Printf("[Warning] %s\n", err)
	}

	return count
}

func readInputs(input string) (values [][]uint16, operations []byte) {
	scanner := bufio.NewScanner(strings.NewReader(input))
	valueCount := countInputLines(input) - 1
	values = [][]uint16{}
	operations = []byte{}

	for range valueCount {
		scanner.Scan()
		valueRow := []uint16{}
		for val_str := range strings.FieldsSeq(scanner.Text()) {
			val, err := strconv.ParseUint(val_str, 10, 16)
			if err != nil {
				panic(fmt.Sprintf("%s is not an integer", val_str))
			}
			valueRow = append(valueRow, uint16(val))
		}
		values = append(values, valueRow)
	}

	scanner.Scan()
	for op_str := range strings.FieldsSeq(scanner.Text()) {
		switch op_str {
		case "+":
			operations = append(operations, '+')
		case "*":
			operations = append(operations, '*')
		default:
			panic(fmt.Sprintf("%s is not a valid operation", op_str))
		}
	}

	err := scanner.Err()
	if err != nil {
		fmt.Printf("[Warning] %s\n", err)
	}

	return values, operations
}

func result(input string) uint64 {
	values, operations := readInputs(input)
	var total uint64 = 0

	for i := range operations {
		switch operations[i] {
		case '+':
			var sum uint64 = 0
			for _, vals := range values {
				sum += uint64(vals[i])
			}
			total += sum
		case '*':
			var product uint64 = 1
			for _, vals := range values {
				product *= uint64(vals[i])
			}
			total += product
		default:
			panic(fmt.Sprintf("%c is not a valid operation", operations[i]))
		}

	}

	return total
}

func Solve(input string) string {
	return fmt.Sprintf("Grand total: %d\n", result(input))
}
