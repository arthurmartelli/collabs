package parser_test

import (
	"os"
	"squire/pkg/parser"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestParse(t *testing.T) {
	t.Run("it parses a valid story", func(t *testing.T) {
		/*
			% You're probably going to die.
			% Jeffrey Chupp

			# Something isn't right here. {#intro}

			You hear a phone ringing. _Something_ makes you suspicious of it.

			- [pick up phone](#phone)
			- [do not answer](#ignore-phone)
			- [jump in a nearby lion's mouth](#lion)

			# You pick up the phone... {#phone}

			It is your grandmother. **You die.**

			- [start over](#intro)

			# You ignore the phone... {#ignore-phone}

			It was your grandmother. **You die.**

			- [start over](#intro)

			# You jump in a nearby lion's mouth... {#lion}

			Strangely, you don't die. Guess you better start getting ready for school.

			- [pick up backpack and head out](#backpack)
			- [decide to skip school](#skip)

			# You decide to skip school {#skip}

			A wild herd of dinosaurs bust in and kill you. Guess you'll never get to tell your friends about how you're immune to lions... or that you met live dinosaurs :(

			- [start over](#intro)

			# Going to school {#backpack} !!

			You're on your way to school when a meteor lands on you. You gain super powers and institute world peace.

			You win.
		*/

		content, err := os.ReadFile("test_data/valid.md")

		if err != nil {
			t.Fatal(err)
		}

		story, err := parser.Parse(string(content))

		if err != nil {
			t.Fatal(err)
		}

		// Story
		assert.Equal(t, "You're probably going to die.", story.Title)
		assert.Equal(t, "Jeffrey Chupp", story.Author)
		assert.Equal(t, 6, len(story.Chapters))

		// First chapter
		firstChapter := story.Chapters[0]
		assert.Equal(t, "Something isn't right here.", firstChapter.Title)
		assert.Equal(t, "intro", firstChapter.ID)
		assert.Equal(t, []parser.Choice{
			{Text: "pick up phone", ChapterID: "phone"},
			{Text: "do not answer", ChapterID: "ignore-phone"},
			{Text: "jump in a nearby lion's mouth", ChapterID: "lion"},
		}, firstChapter.Choices)

		// Last chapter
		lastChapter := story.Chapters[5]
		assert.Equal(t, "You decide to skip school", lastChapter.Title)
		assert.Equal(t, "skip", lastChapter.ID)
		assert.Equal(t, 0, len(lastChapter.Choices))

		t.Skip("TODO")
	})

	t.Run("it returns an error when parsing an invalid story", func(t *testing.T) {
		_, err := os.ReadFile("test_data/valid.md")

		if err != nil {
			t.Fatal(err)
		}

		t.Skip("TODO")
	})
}
