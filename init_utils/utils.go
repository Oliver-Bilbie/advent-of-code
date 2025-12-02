package main

import (
	"bytes"
	"fmt"
	"io/fs"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"time"
)

var FESTIVE_EMOJIS = []string{"🎅", "🎄", "❄️", "☃️", "🎁", "🦌"}

func getFestiveEmoji() string {
	return FESTIVE_EMOJIS[time.Now().Second()%len(FESTIVE_EMOJIS)]
}

func isRootDir() bool {
	cwd, err := os.Getwd()
	if err != nil {
		return false
	}

	return strings.HasSuffix(cwd, "advent-of-code/init_utils")
}

func fileExists(path string) bool {
	_, err := os.Stat(path)
	return err == nil
}

func copyDir(src, dst string) error {
	parent := filepath.Dir(dst)
	if err := os.MkdirAll(parent, 0o755); err != nil {
		return fmt.Errorf("failed to create parent directory %s: %w", parent, err)
	}

	cmd := exec.Command("cp", "-R", src, dst)

	var stderr bytes.Buffer
	cmd.Stderr = &stderr

	err := cmd.Run()
	if err != nil {
		msg := stderr.String()
		if msg == "" {
			return fmt.Errorf("cp failed: %w", err)
		}
		return fmt.Errorf("cp failed: %s: %w", strings.TrimSpace(msg), err)
	}

	return nil
}

func findReplace(path, find, replace string) error {
	b, err := os.ReadFile(path)
	if err != nil {
		return err
	}

	content := string(b)
	content = strings.ReplaceAll(content, find, replace)

	return os.WriteFile(path, []byte(content), 0644)
}

func makeDirectory(year int, day int, language string) error {
	// TODO: if rust, add to workspace?

	init_utils_dir, err := os.Getwd()
	if err != nil {
		return err
	}
	root_dir := filepath.Dir(init_utils_dir)
	day_dir := fmt.Sprintf("%s/%d/day_%02d", root_dir, year, day)

	template_dir := init_utils_dir + "/templates"
	switch language {
	case "Rust 🦀":
		template_dir += "/rust"
	case "Go 🐹":
		template_dir += "/go"
	default:
		return fmt.Errorf("invalid language: %s", language)
	}

	for task := 1; task <= 2; task++ {
		var solution_name = fmt.Sprintf("solution_%d_%02d_%d", year, day, task)
		var solution_dir = fmt.Sprintf("%s/task_%d", day_dir, task)

		if fileExists(solution_dir) {
			return fmt.Errorf("a solution already exists at %s", solution_dir)
		}

		err := copyDir(template_dir, solution_dir)
		if err != nil {
			return err
		}

		err = filepath.WalkDir(solution_dir, func(path string, file fs.DirEntry, err error) error {
			if err != nil {
				return err
			}
			if file.IsDir() {
				return nil
			}

			findReplace(path, "__SOLUTION_NAME__", solution_name)
			return nil
		})
		if err != nil {
			return err
		}
	}

	return nil
}
