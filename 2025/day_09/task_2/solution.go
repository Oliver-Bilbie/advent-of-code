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

func largestRectangle(points []Point, xBounds, yBounds map[uint32][2]uint32) uint64 {
	var largest uint64 = 0

	for i, p1 := range points {
		for _, p2 := range points[i+1:] {
			area := getArea(p1, p2)
			if area > largest && isValidRectange(p1, p2, xBounds, yBounds) {
				largest = area
			}
		}
	}

	return largest
}

func findBoundaries(points []Point) (x map[uint32][2]uint32, y map[uint32][2]uint32) {
	// boundary on x==key between y==val[0] and y==val[1] inclusive
	x = make(map[uint32][2]uint32)
	y = make(map[uint32][2]uint32)

	for i, p1 := range points {
		for j, p2 := range points {
			if i == j {
				continue
			}

			if p1.x == p2.x {
				if p1.y <= p2.y {
					x[p1.x] = [2]uint32{p1.y, p2.y}
				} else {
					x[p1.x] = [2]uint32{p2.y, p1.y}
				}
			}

			if p1.y == p2.y {
				if p1.x <= p2.x {
					y[p1.y] = [2]uint32{p1.x, p2.x}
				} else {
					y[p1.y] = [2]uint32{p2.x, p1.x}
				}
			}
		}
	}

	return x, y
}

func isValidRectange(p1, p2 Point, xBounds, yBounds map[uint32][2]uint32) bool {
	start := p1
	end := p2

	if int64(p2.x)-int64(p1.x) < 0 {
		start.x = p2.x
		end.x = p1.x
	}
	if int64(p2.y)-int64(p1.y) < 0 {
		start.y = p2.y
		end.y = p1.y
	}

	for x := start.x + 1; x < end.x; x++ {
		yRange, isBoundary := xBounds[x]
		if isBoundary && start.y >= yRange[0] && start.y < yRange[1] {
			return false
		}

		yRange, isBoundary = xBounds[x]
		if isBoundary && end.y > yRange[0] && end.y <= yRange[1] {
			return false
		}
	}

	for y := start.y + 1; y < end.y; y++ {
		xRange, isBoundary := yBounds[y]
		if isBoundary && start.x >= xRange[0] && start.x < xRange[1] {
			return false
		}

		xRange, isBoundary = yBounds[y]
		if isBoundary && end.x > xRange[0] && end.x <= xRange[1] {
			return false
		}
	}

	return true
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
	xBounds, yBounds := findBoundaries(redTiles)
	return largestRectangle(redTiles, xBounds, yBounds)
}

func Solve(input string) string {
	return fmt.Sprintf("Answer: %d\n", result(input))
}
