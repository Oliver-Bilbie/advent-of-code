package main

import (
	"bufio"
	"fmt"
	"strings"
)

type Point struct {
	col      uint8
	inNPaths uint64
}

func mergePoints(points []Point) []Point {
	if len(points) == 0 {
		return points
	}

	writeIndex := 1

	for i := 1; i < len(points); i++ {
		if points[i].col != points[writeIndex-1].col {
			if i != writeIndex {
				points[writeIndex] = points[i]
			}
			writeIndex++
		} else {
			points[writeIndex-1].inNPaths += points[i].inNPaths
		}
	}

	return points[:writeIndex]
}

func result(input string) uint64 {
	scanner := bufio.NewScanner(strings.NewReader(input))
	beams := []Point{}

	scanner.Scan()
	topRow := scanner.Text()
	for col, val := range topRow {
		if val == 'S' {
			beams = append(beams, Point{uint8(col), 1})
		}
	}

	var rowCount uint8 = uint8(len(topRow))

	for scanner.Scan() {
		row := scanner.Text()
		nextBeams := []Point{}

		for _, p := range beams {
			switch row[p.col] {
			case '.':
				nextBeams = append(nextBeams, Point{p.col, p.inNPaths})
			case '^':
				if p.col > 0 {
					nextBeams = append(nextBeams, Point{p.col - 1, p.inNPaths})
				}
				if p.col+1 < rowCount {
					nextBeams = append(nextBeams, Point{p.col + 1, p.inNPaths})
				}
			default:
				panic(fmt.Sprintf("%c is not a valid tile state\n", row[p.col]))
			}
		}

		beams = mergePoints(nextBeams)
	}

	err := scanner.Err()
	if err != nil {
		fmt.Printf("[Warning] %s\n", err)
	}

	var result uint64 = 0
	for _, p := range beams {
		result += p.inNPaths
	}

	return result
}

func Solve(input string) string {
	return fmt.Sprintf("The tachyon particle will end up on %d timelines\n", result(input))
}
