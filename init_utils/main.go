package main

import (
	"fmt"

	tea "github.com/charmbracelet/bubbletea"
)

func main() {
	if !isRootDir() {
		fmt.Println("\n[Error] init utils can only run from its root directory")
		return
	}

	m := values{
		list:  newYearList(),
		phase: selectYear,
	}

	finalModel, err := tea.NewProgram(m, tea.WithAltScreen()).Run()
	if err != nil {
		fmt.Println("\n[Error]", err)
		return
	}

	inputs := finalModel.(values)

	if inputs.language == "" {
		fmt.Println("\n[Info] initialization was aborted")
		return
	}

	fmt.Printf(
		"\n[Info] initializing %d day %d for %s\n",
		inputs.year,
		inputs.day,
		inputs.language,
	)

	err = makeDirectory(inputs.year, inputs.day, inputs.language)
	if err != nil {
		fmt.Println("\n[Error]", err)
		return
	}

	fmt.Printf("[Info] initialization was successful %s\n", getFestiveEmoji())
}
