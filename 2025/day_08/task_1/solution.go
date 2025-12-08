package main

import (
	"bufio"
	"container/heap"
	"fmt"
	"strconv"
	"strings"
)

type Point struct {
	x uint32
	y uint32
	z uint32
}

type Pair struct {
	a Point
	b Point
}

const UINT64_MAX = 1<<64 - 1

func readPoints(input string) []Point {
	scanner := bufio.NewScanner(strings.NewReader(input))
	points := []Point{}

	for scanner.Scan() {
		line := scanner.Text()
		coordinates := [3]uint32{}

		for i, field := range strings.Split(line, ",") {
			val, err := strconv.ParseUint(field, 10, 32)
			if err != nil {
				panic(fmt.Sprintf("%s is not an integer\n", field))
			}
			coordinates[i] = uint32(val)
		}

		p := Point{coordinates[0], coordinates[1], coordinates[2]}
		points = append(points, p)
	}

	err := scanner.Err()
	if err != nil {
		fmt.Printf("[Warning] %s\n", err)
	}

	return points
}

func findConnections(points []Point, n int) *PairHeap {
	h := &PairHeap{}
	heap.Init(h)

	for i, p1 := range points {
		for _, p2 := range points[i+1:] {
			heap.Push(h, Pair{p1, p2})
			if h.Len() > n {
				heap.Pop(h)
			}
		}
	}

	return h
}

func findGroups(connections *PairHeap) []map[Point]struct{} {
	groups := []map[Point]struct{}{}

	for connections.Len() > 0 {
		pair := heap.Pop(connections).(Pair)

		// First pass: find all groups that match
		toMerge := []int{}

		for i, g := range groups {
			_, containsA := g[pair.a]
			_, containsB := g[pair.b]
			if containsA || containsB {
				toMerge = append(toMerge, i)
			}
		}

		if len(toMerge) == 0 {
			g := map[Point]struct{}{
				pair.a: {},
				pair.b: {},
			}
			groups = append(groups, g)
			continue
		}

		// Merge all matched groups into the first one
		target := toMerge[0]
		g := groups[target]
		g[pair.a] = struct{}{}
		g[pair.b] = struct{}{}

		// Merge others into g
		for i := 1; i < len(toMerge); i++ {
			idx := toMerge[i]
			for p := range groups[idx] {
				g[p] = struct{}{}
			}
		}

		// Remove extra groups (reverse order so indices don't shift)
		for i := len(toMerge) - 1; i >= 1; i-- {
			idx := toMerge[i]
			groups = append(groups[:idx], groups[idx+1:]...)
		}
	}

	return groups
}

func prodOfLargest(groups []map[Point]struct{}) uint64 {
	h := &IntHeap{}
	heap.Init(h)

	for _, g := range groups {
		heap.Push(h, len(g))
		if h.Len() > 3 {
			heap.Pop(h)
		}
	}

	var prod uint64 = 1
	for h.Len() > 0 {
		size := heap.Pop(h).(int)
		prod *= uint64(size)
	}
	return prod
}

func result(input string, n int) uint64 {
	points := readPoints(input)
	connections := findConnections(points, n)
	groups := findGroups(connections)
	return prodOfLargest(groups)
}

func Solve(input string) string {
	return fmt.Sprintf("Answer: %d\n", result(input, 1000))
}
