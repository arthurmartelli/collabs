package parser

import (
	"regexp"
	"strings"
)

type Story struct {
	Title    string
	Author   string
	Chapters []Chapter
}

type Chapter struct {
	Title     string
	ID        string
	Body      string
	IsDeadEnd bool
	Choices   []Choice
}

type Choice struct {
	Text      string
	ChapterID string
}

var (
	// FrontMatter is a line that starts with %
	FrontMatter = "% "
	// ChapterTitle is a line that starts with #
	ChapterTitle = "# "
	// ChapterFrontRegex is a regex that matches a title ID and if it's a dead end
	// Ending with !! means it's a dead end
	// # You jump in a nearby lion's mouth... {#lion}
	// # You jump in a nearby lion's mouth... {#lion} !
	ChapterFrontRegex = regexp.MustCompile(`^# ([^{]+){([^}]+)} ?(!)?$`)
	// Parses the choice of the chapter which are a list of links
	// - [pick up backpack and head out](#backpack)
	// - [decide to skip school](#skip)
	ChoiceRegex = regexp.MustCompile(`^- \[([^\]]+)\]\(#([^\)]+)\)`)
)

// Parse a story or return an error
func Parse(content string) (Story, error) {
	story := Story{}
	chapter := Chapter{}

	lines := strings.Split(content, "\n")

	for _, line := range lines {
		switch {
		case isFrontMatter(line):
			if story.Title == "" {
				story.Title = getFrontMatter(line)
			} else {
				story.Author = getFrontMatter(line)
			}
		case isNewChapter(line):
			if chapter.ID != "" {
				story.Chapters = append(story.Chapters, chapter)
				chapter = Chapter{}
			}

			title, ID, isDeadEnd := parseChapterFront(line)

			if title != "" {
				chapter.Title = title
			}

			if ID != "" {
				chapter.ID = ID
			}

			if isDeadEnd {
				chapter.IsDeadEnd = isDeadEnd
			}

			story.Chapters = append(story.Chapters, chapter)
		case isChoice(line):
			choice := parseChoice(line)
			chapter.Choices = append(chapter.Choices, choice)
		}
	}

	// Add the last chapter
	if chapter.ID != "" {
		story.Chapters = append(story.Chapters, chapter)
	}

	println(len(story.Chapters))

	return story, nil
}

func isChoice(line string) bool {
	return ChoiceRegex.MatchString(line)
}

func parseChoice(line string) Choice {
	match := ChoiceRegex.FindStringSubmatch(line)

	if len(match) == 3 {
		return Choice{
			Text:      match[1],
			ChapterID: match[2],
		}
	}

	// No match found
	return Choice{}
}

// Checks for the chapter's title, ID, and if it's a Dead end
// A chapter will always have a Title and ID
func parseChapterFront(line string) (string, string, bool) {
	match := ChapterFrontRegex.FindStringSubmatch(line)

	if len(match) == 3 {
		return match[1], match[2], (match[2] == "!")
	}

	// No match found
	return "", "", false

}

func isFrontMatter(line string) bool {
	return strings.HasPrefix(line, FrontMatter)
}

func getFrontMatter(line string) string {
	return strings.TrimSpace(strings.TrimPrefix(line, FrontMatter))
}

func isNewChapter(line string) bool {
	return strings.HasPrefix(line, ChapterTitle)
}
