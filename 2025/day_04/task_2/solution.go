package main

import (
	"bufio"
	"fmt"
	"strings"
)

func safeGet[T any](s []T, i int) (T, bool) {
	if i < 0 || i >= len(s) {
		var zero T
		return zero, false
	}
	return s[i], true
}

func countAdjacent(row int, col int, warehouse [][]bool) int {
	total := -1 // offset counting the current tile

	for i := row - 1; i <= row+1; i++ {
		warehouseRow, ok := safeGet(warehouse, i)
		if !ok {
			continue
		}
		for j := col - 1; j <= col+1; j++ {
			isRoll, ok := safeGet(warehouseRow, j)
			if !ok {
				continue
			}

			if isRoll {
				total += 1
			}
		}
	}

	return total
}

func readWarehouse(input string) [][]bool {
	scanner := bufio.NewScanner(strings.NewReader(input))
	warehouse := [][]bool{}

	for scanner.Scan() {
		line := scanner.Text()
		warehouseRow := []bool{}
		for _, c := range line {
			warehouseRow = append(warehouseRow, c == '@')
		}
		warehouse = append(warehouse, warehouseRow)
	}

	err := scanner.Err()
	if err != nil {
		fmt.Printf("[Warning] %s\n", err)
	}

	return warehouse
}

func result(input string) uint64 {
	warehouse := readWarehouse(input)

	var total int64 = 0
	var previous_total int64 = -1

	for total != previous_total {
		previous_total = total
		for row := range warehouse {
			for col := range warehouse[0] {
				if warehouse[row][col] && countAdjacent(row, col, warehouse) < 4 {
					warehouse[row][col] = false
					total += 1
				}
			}
		}
	}

	return uint64(total)
}

func Solve(input string) string {
	return fmt.Sprintf("%d rolls of paper can be removed by the Elves\n", result(input))
}
