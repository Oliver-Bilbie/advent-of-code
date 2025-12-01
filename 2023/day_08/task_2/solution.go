package main

import (
	"bufio"
	"fmt"
	"strings"
)

// This is a LOT easier than in Rust!
type Node struct {
	name  string
	left  *Node
	right *Node
}

type TraversalData struct {
	node_name       string
	direction_index int
}

type CycleData struct {
	first_z_index int
	cycle_length  int
}

func generate_network(node_details []string) map[string]*Node {
	var nodes_map = make(map[string]*Node)
	for _, line := range node_details {
		var node_name = line[0:3]
		nodes_map[node_name] = &Node{name: node_name, left: nil, right: nil}
	}
	for _, line := range node_details {
		var node_name = line[0:3]
		var left_node_name = line[7:10]
		var right_node_name = line[12:15]
		nodes_map[node_name].left = nodes_map[left_node_name]
		nodes_map[node_name].right = nodes_map[right_node_name]
	}
	return nodes_map
}

func generate_cycle_data(start_nodes []Node, directions string) []CycleData {
	var cycle_data []CycleData
	for _, node := range start_nodes {
		var current_node = node
		var direction_index = 0
		var initial_traversal_data TraversalData = TraversalData{node_name: current_node.name, direction_index: direction_index}
		var traversal_data []TraversalData = []TraversalData{initial_traversal_data}
		var cycle_found = false

		for !cycle_found {
			if directions[direction_index] == 'L' {
				current_node = *current_node.left
			} else {
				current_node = *current_node.right
			}

			cycle_found = check_cycle_found(traversal_data, direction_index, &current_node)

			traversal_data = append(traversal_data, TraversalData{node_name: current_node.name, direction_index: direction_index})
			direction_index = (direction_index + 1) % len(directions)
		}

		// Here we exploit that the input is designed such that there is exactly one "Z node" per cycle
		var first_z_index = 0
		for i, step := range traversal_data {
			if step.node_name[2] == 'Z' {
				first_z_index = i
				break
			}
		}
		var cycle_length = len(traversal_data) - first_z_index - 1

		cycle_data = append(cycle_data, CycleData{first_z_index: first_z_index, cycle_length: cycle_length})
	}
	return cycle_data
}

func generate_start_nodes(nodes_map map[string]*Node) []Node {
	var start_nodes []Node
	for _, node := range nodes_map {
		if node.name[2] == 'A' {
			start_nodes = append(start_nodes, *node)
		}
	}
	return start_nodes
}

func check_cycle_found(traversal_data []TraversalData, current_direction_index int, current_node *Node) bool {
	// Here we exploit that the input is designed such that there is exactly one "Z node" per cycle
	if current_node.name[2] == 'Z' {
		for _, step := range traversal_data {
			if step.node_name == current_node.name {
				return true
			}
		}
	}
	return false
}

func all_paths_finished(cycle_data []CycleData, step_index int) bool {
	for _, path := range cycle_data {
		var cycle_position = (step_index - path.first_z_index) % path.cycle_length
		if cycle_position != 0 {
			return false
		}
	}
	return true
}

func Solve(input string) string {
	scanner := bufio.NewScanner(strings.NewReader(input))

	var lines []string
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	var directions = lines[0]
	var nodes_map = generate_network(lines[2:])

	var start_nodes = generate_start_nodes(nodes_map)
	var cycle_data = generate_cycle_data(start_nodes, directions)

	// We will now iterate over each "Z node" occurance in one of the paths
	// until all paths have reached a "Z node"
	var number_of_steps = cycle_data[0].first_z_index
	for all_paths_finished(cycle_data, number_of_steps) == false {
		number_of_steps += cycle_data[0].cycle_length
	}

	return fmt.Sprintln("Total steps: ", number_of_steps)
}
