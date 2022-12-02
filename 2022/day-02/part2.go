package main

import (
	"strings"
)

var decisionMap = map[rune]string{
	'X': "lose",
	'Y': "draw",
	'Z': "win",
}

var reverseSignToCharMap = map[string]rune{
	"rock":     'A',
	"paper":    'B',
	"scissors": 'C',
}

func Part2_selectCorrectResponse(opSign sign, decision string) sign {
	signChar := ' '

	if decision == "draw" {
		signChar = reverseSignToCharMap[opSign.name]
	} else if decision == "win" {
		signChar = reverseSignToCharMap[opSign.weakTo]
	} else if decision == "lose" {
		signChar = reverseSignToCharMap[opSign.strongTo]
	}
	return charSignMap[signChar]
}

func Part2_workFile(content string) {
	sum := 0
	lines := strings.Split(content, "\n")
	for _, line := range lines {
		if len(line) < 3 {
			continue
		}
		signs := strings.Split(strings.TrimSpace(line), " ")
		firstSign := charSignMap[rune(signs[0][0])]
		decision := decisionMap[rune(signs[1][0])]
		secondSign := Part2_selectCorrectResponse(firstSign, decision)
		sum += Part1_calculatePoints(firstSign, secondSign)
	}
	println(sum)
}
