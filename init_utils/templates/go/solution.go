package main

import (
	"bufio"
	"fmt"
	"strings"
)

func result(input string) uint64 {
	scanner := bufio.NewScanner(strings.NewReader(input))

	for scanner.Scan() {
		var line string = scanner.Text()
	}

	return 0
}

func Solve(input string) string {
	return fmt.Sprintf("Answer: %d\n", result(input))
}
