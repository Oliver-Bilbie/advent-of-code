package main

import (
	"fmt"
	"io"
	"strings"
	"time"

	"github.com/charmbracelet/bubbles/list"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
)

const listHeight = 14

var (
	titleStyle        = lipgloss.NewStyle().MarginLeft(2)
	itemStyle         = lipgloss.NewStyle().PaddingLeft(4)
	selectedItemStyle = lipgloss.NewStyle().PaddingLeft(2).Foreground(lipgloss.Color("2"))
	paginationStyle   = list.DefaultStyles().PaginationStyle.PaddingLeft(4)
	helpStyle         = list.DefaultStyles().HelpStyle.PaddingLeft(4).PaddingBottom(1)
	quitTextStyle     = lipgloss.NewStyle().Margin(1, 0, 2, 4)
)

type item string

func (i item) FilterValue() string { return "" }

type itemDelegate struct{}

func (d itemDelegate) Height() int                             { return 1 }
func (d itemDelegate) Spacing() int                            { return 0 }
func (d itemDelegate) Update(_ tea.Msg, _ *list.Model) tea.Cmd { return nil }

func (d itemDelegate) Render(w io.Writer, m list.Model, index int, listItem list.Item) {
	i, ok := listItem.(item)
	if !ok {
		return
	}

	str := string(i)

	fn := itemStyle.Render
	if index == m.Index() {
		fn = func(s ...string) string {
			return selectedItemStyle.Render("> " + strings.Join(s, " "))
		}
	}

	fmt.Fprint(w, fn(str))
}

type phase int

const (
	selectYear phase = iota
	selectDay
	selectLang
	done
)

type values struct {
	list  list.Model
	phase phase

	year     int
	day      int
	language string

	quitting bool
}

func (m values) Init() tea.Cmd { return nil }

func (m values) Update(msg tea.Msg) (tea.Model, tea.Cmd) {

	switch msg := msg.(type) {

	case tea.WindowSizeMsg:
		m.list.SetWidth(msg.Width)
		return m, nil

	case tea.KeyMsg:
		switch msg.String() {

		case "q", "ctrl+c":
			m.quitting = true
			return m, tea.Quit

		case "enter":
			selected, ok := m.list.SelectedItem().(item)
			if !ok {
				return m, nil
			}

			switch m.phase {

			case selectYear:
				fmt.Sscanf(string(selected), "%d", &m.year)
				m.phase = selectDay
				m.list = newDayList()

			case selectDay:
				fmt.Sscanf(string(selected), "%d", &m.day)
				m.phase = selectLang
				m.list = newLangList()

			case selectLang:
				m.language = string(selected)
				m.phase = done
				return m, tea.Quit
			}
		}
	}

	var cmd tea.Cmd
	m.list, cmd = m.list.Update(msg)
	return m, cmd
}

func (m values) View() string {
	if m.quitting {
		return quitTextStyle.Render("Quit.")
	}
	return "\n" + m.list.View()
}

func baseList(items []list.Item, title string) list.Model {
	const defaultWidth = 20

	l := list.New(items, itemDelegate{}, defaultWidth, listHeight)
	l.Title = title
	l.SetShowStatusBar(false)
	l.SetFilteringEnabled(false)

	l.Styles.Title = titleStyle
	l.Styles.PaginationStyle = paginationStyle
	l.Styles.HelpStyle = helpStyle

	return l
}

func newYearList() list.Model {
	latest_year := time.Now().Year()
	if time.Now().Month() != time.December {
		latest_year -= 1
	}

	years := []list.Item{}
	for y := latest_year; y >= 2015; y-- {
		years = append(years, item(fmt.Sprintf("%d", y)))
	}

	return baseList(years, "Select Year")
}

func newDayList() list.Model {
	days := []list.Item{}
	for d := 1; d <= 25; d++ {
		days = append(days, item(fmt.Sprintf("%d", d)))
	}

	l := baseList(days, "Select Day")
	selected := 1

	now := time.Now()
	if now.Month() == time.December && now.Day() <= 12 {
		selected = now.Day()
	}

	l.Select(selected)

	return l
}

func newLangList() list.Model {
	langs := []list.Item{
		item("Rust 🦀"),
		item("Go 🐹"),
	}
	return baseList(langs, "Select Language")
}
