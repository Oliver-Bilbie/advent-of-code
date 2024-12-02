package main

import (
	"bufio"
	"fmt"
	"os"
)

type Node struct {
	name  string
	left  *Node
	right *Node
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

func main() {
	var lines []string
	file, _ := os.Open("input.txt")
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}
	file.Close()

	var directions = lines[0]
	var direction_index = 0
	var nodes_map = generate_network(lines[2:])
	var steps = 0

	var current_node = nodes_map["AAA"]
	for current_node.name != "ZZZ" {
		if directions[direction_index] == 'L' {
			current_node = current_node.left
		} else {
			current_node = current_node.right
		}

		if direction_index == len(directions)-1 {
			direction_index = 0
		} else {
			direction_index += 1
		}

		steps += 1
	}

	fmt.Println("Getting from AAA to ZZZ took", steps, "steps")
}
