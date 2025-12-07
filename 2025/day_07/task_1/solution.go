package main

import (
	"bufio"
	"fmt"
	"strings"
)

func dedupSorted[T comparable](slice []T) []T {
	if len(slice) == 0 {
		return slice
	}

	writeIndex := 1

	for i := 1; i < len(slice); i++ {
		if slice[i] != slice[writeIndex-1] {
			if i != writeIndex {
				slice[writeIndex] = slice[i]
			}
			writeIndex++
		}
	}

	return slice[:writeIndex]
}

func result(input string) uint64 {
	scanner := bufio.NewScanner(strings.NewReader(input))
	beams := []uint8{}
	var splits uint64 = 0

	scanner.Scan()
	topRow := scanner.Text()
	for col, val := range topRow {
		if val == 'S' {
			beams = []uint8{uint8(col)}
		}
	}

	var rowCount uint8 = uint8(len(topRow))

	for scanner.Scan() {
		row := scanner.Text()
		nextBeams := []uint8{}

		for _, col := range beams {
			switch row[col] {
			case '.':
				nextBeams = append(nextBeams, col)
			case '^':
				splits += 1
				if col > 0 {
					nextBeams = append(nextBeams, col-1)
				}
				if col+1 < rowCount {
					nextBeams = append(nextBeams, col+1)
				}
			default:
				panic(fmt.Sprintf("%c is not a valid tile state\n", row[col]))
			}
		}

		beams = dedupSorted(nextBeams)
	}

	err := scanner.Err()
	if err != nil {
		fmt.Printf("[Warning] %s\n", err)
	}

	return splits
}

func Solve(input string) string {
	return fmt.Sprintf("The beam will be split %d times\n", result(input))
}
