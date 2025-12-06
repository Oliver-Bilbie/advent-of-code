package main

import (
	"fmt"
	"strconv"
	"strings"
)

func charIsNumeric(c byte) bool {
	return c >= '0' && c <= '9'
}

func charIsOperator(c byte) bool {
	return c == '+' || c == '*'
}

func readInputs(input string) (values [][]uint16, operations []byte) {
	lines := strings.Split(input, "\n")
	values = [][]uint16{}
	operations = []byte{}

	// The first char of the final line will be an operator
	lineCount := 0
	for lineNum, content := range lines {
		if len(content) > 0 && charIsOperator(content[0]) {
			lineCount = lineNum + 1
		}
	}
	if lineCount == 0 {
		panic("Unable to read the input")
	}

	lineLength := len(lines[0])

	currentValues := []uint16{}

	for col := range lineLength {
		valStr := ""
		for row := range lineCount - 1 {
			if charIsNumeric(lines[row][col]) {
				valStr = valStr + string(lines[row][col])
			}
		}

		if len(valStr) > 0 {
			val, err := strconv.ParseUint(valStr, 10, 16)
			if err != nil {
				panic(fmt.Sprintf("%s is not an integer", valStr))
			}
			currentValues = append(currentValues, uint16(val))
		}

		opChar := lines[lineCount-1][col]
		if charIsOperator(opChar) {
			operations = append(operations, opChar)
		}

		if valStr == "" && opChar == ' ' {
			values = append(values, currentValues)
			currentValues = []uint16{}
		}
	}

	values = append(values, currentValues)

	return values, operations
}

func result(input string) uint64 {
	values, operations := readInputs(input)
	var total uint64 = 0

	for i := range operations {
		switch operations[i] {
		case '+':
			var sum uint64 = 0
			for _, val := range values[i] {
				sum += uint64(val)
			}
			total += sum
		case '*':
			var product uint64 = 1
			for _, val := range values[i] {
				product *= uint64(val)
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
