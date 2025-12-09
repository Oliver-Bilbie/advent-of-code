package main

import (
	"bufio"
	"fmt"
	"strconv"
	"strings"
)

type Point struct {
	x uint32
	y uint32
}

func readRedTiles(input string) []Point {
	scanner := bufio.NewScanner(strings.NewReader(input))
	redTiles := []Point{}

	for scanner.Scan() {
		line := scanner.Text()
		values := strings.SplitN(line, ",", 2)

		x, err := strconv.ParseUint(values[1], 10, 32)
		if err != nil {
			panic(fmt.Sprintf("%s is not an integer\n", values[1]))
		}
		y, err := strconv.ParseUint(values[0], 10, 32)
		if err != nil {
			panic(fmt.Sprintf("%s is not an integer\n", values[1]))
		}

		redTiles = append(redTiles, Point{uint32(x), uint32(y)})
	}

	err := scanner.Err()
	if err != nil {
		fmt.Printf("[Warning] %s\n", err)
	}

	return redTiles
}

func largestRectangle(points []Point) uint64 {
	var largest uint64 = 0

	for i, p1 := range points {
		for _, p2 := range points[i+1:] {
			area := getArea(p1, p2)
			if area > largest {
				largest = area
			}
		}
	}

	return largest
}

func getArea(p1, p2 Point) uint64 {
	width := abs(int64(p1.x)-int64(p2.x)) + 1
	height := abs(int64(p1.y)-int64(p2.y)) + 1
	return uint64(width) * uint64(height)
}

func abs(x int64) int64 {
	if x < 0 {
		return -x
	}
	return x
}

func result(input string) uint64 {
	redTiles := readRedTiles(input)
	return largestRectangle(redTiles)
}

func Solve(input string) string {
	return fmt.Sprintf("Answer: %d\n", result(input))
}
