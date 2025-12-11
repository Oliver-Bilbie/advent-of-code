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

	return devices["you"]
}

func dfs(node *Device) uint64 {
	if node.name == "out" {
		return 1
	}

	var total uint64 = 0

	for _, next := range node.connections {
		total += dfs(next)
	}

	return total
}

func result(input string) uint64 {
	you := readDevices(input)
	return dfs(you)
}

func Solve(input string) string {
	return fmt.Sprintf("There are %d different paths\n", result(input))
}
