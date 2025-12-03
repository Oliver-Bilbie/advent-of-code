package main

import (
	"fmt"
	"os"
	"regexp"
	"sort"
	"strings"

	toml "github.com/pelletier/go-toml/v2"
)

func doRustSetup(cargo_path string, year int, day int) error {
	rawBytes, err := os.ReadFile(cargo_path)
	if err != nil {
		return fmt.Errorf("read error: %w", err)
	}
	raw := string(rawBytes)

	var cargo struct {
		Workspace struct {
			Members []string `toml:"members"`
		} `toml:"workspace"`
	}

	if err := toml.Unmarshal(rawBytes, &cargo); err != nil {
		return fmt.Errorf("toml parse error: %w", err)
	}

	members := cargo.Workspace.Members

	new1 := fmt.Sprintf("%d/day_%02d/task_1", year, day)
	new2 := fmt.Sprintf("%d/day_%02d/task_2", year, day)
	members = append(members, new1, new2)

	sort.Strings(members)
	members = dedupSortedArray(members)

	var b strings.Builder
	b.WriteString("members = [\n")
	for _, m := range members {
		b.WriteString(fmt.Sprintf("  %q,\n", m))
	}
	b.WriteString("]")

	newMembersBlock := b.String()

	re := regexp.MustCompile(`members\s*=\s*\[[^\]]*\]`)
	if !re.MatchString(raw) {
		return fmt.Errorf("could not find members array in Cargo.toml")
	}

	newRaw := re.ReplaceAllString(raw, newMembersBlock)

	return os.WriteFile(cargo_path, []byte(newRaw), 0644)
}

func dedupSortedArray[T comparable](nums []T) []T {
	if len(nums) <= 1 {
		return nums
	}

	i := 0
	for j := 1; j < len(nums); j++ {
		if nums[j] != nums[i] {
			i++
			nums[i] = nums[j]
		}
	}
	return nums[:i+1]
}
