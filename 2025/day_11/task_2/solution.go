package main

import (
	"bufio"
	"fmt"
	"strings"
)

type Device struct {
	name        string
	connections []*Device
}

type SearchState struct {
	node     *Device
	foundDAC bool
	foundFFT bool
}

func readDevices(input string) *Device {
	scanner := bufio.NewScanner(strings.NewReader(input))
	devices := make(map[string]*Device)

	for scanner.Scan() {
		line := scanner.Text()
		fields := strings.Fields(line)

		name := fields[0][:3]
		connections := []*Device{}

		for _, connStr := range fields[1:] {
			d, ok := devices[connStr]
			if !ok {
				d = &Device{connStr, nil}
				devices[connStr] = d
			}
			connections = append(connections, d)
		}

		d, ok := devices[name]
		if ok {
			d.connections = connections
		} else {
			d = &Device{name, connections}
			devices[name] = d
		}
	}

	err := scanner.Err()
	if err != nil {
		fmt.Printf("[Warning] %s\n", err)
	}

	return devices["svr"]
}

func dfs(search SearchState, cache map[SearchState]uint64) uint64 {
	val, isCached := cache[search]
	if isCached {
		return val
	}

	if search.node.name == "out" {
		if search.foundDAC && search.foundFFT {
			return 1
		}
		return 0
	}

	if search.node.name == "dac" {
		search.foundDAC = true
	}
	if search.node.name == "fft" {
		search.foundFFT = true
	}

	var total uint64 = 0
	for _, next := range search.node.connections {
		nextSearch := SearchState{next, search.foundDAC, search.foundFFT}
		total += dfs(nextSearch, cache)
	}

	cache[search] = total
	return total
}

func result(input string) uint64 {
	svr := readDevices(input)
	cache := make(map[SearchState]uint64)
	return dfs(SearchState{svr, false, false}, cache)
}

func Solve(input string) string {
	return fmt.Sprintf("There are %d different paths\n", result(input))
}
