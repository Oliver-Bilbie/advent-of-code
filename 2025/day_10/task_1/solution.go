package main

import (
	"bufio"
	"fmt"
	"strings"
)

type State uint16

func readMachine(machine string) (lights State, schematics []State) {
	fields := strings.Fields(machine)

	lightsStr := fields[0]
	lightsStr = lightsStr[1 : len(lightsStr)-1]

	for i, c := range lightsStr {
		if c == '#' {
			lights |= 1 << i
		}
	}

	n := len(lightsStr)

	schematics = []State{}
	for _, schemaStr := range fields[1 : len(fields)-1] {
		schemaStr = schemaStr[1 : len(schemaStr)-1]

		var s State
		i := 0

		for i < len(schemaStr) {
			idx := int(schemaStr[i] - '0')
			if idx >= 0 && idx < n {
				s |= 1 << idx
			}
			i += 2 // Step over the comma to the next integer
		}

		schematics = append(schematics, s)
	}

	return lights, schematics
}

func applySchematic(lights State, schematic State) State {
	return lights ^ schematic
}

func bfs(wantLights State, schematics []State) uint64 {
	startLights := State(0)
	heads := []State{startLights}
	visited := make(map[State]struct{})
	var depth uint64 = 1

	if startLights == wantLights {
		return 0
	}

	for len(heads) > 0 {
		nextHeads := []State{}

		for _, haveLights := range heads {
			for _, s := range schematics {
				newLights := applySchematic(haveLights, s)
				if newLights == wantLights {
					return depth
				}
				_, isVisited := visited[newLights]
				if !isVisited {
					nextHeads = append(nextHeads, newLights)
					visited[newLights] = struct{}{}
				}
			}
		}

		heads = nextHeads
		depth += 1
	}

	panic("The required light state is unreachable")
}

func result(input string) uint64 {
	scanner := bufio.NewScanner(strings.NewReader(input))
	var total uint64 = 0

	for scanner.Scan() {
		wantLights, schematics := readMachine(scanner.Text())
		total += bfs(wantLights, schematics)
	}

	err := scanner.Err()
	if err != nil {
		fmt.Printf("[Warning] %s\n", err)
	}

	return total
}

func Solve(input string) string {
	return fmt.Sprintf("Indicator lights are configured after %d button presses\n", result(input))
}
