package main

import (
	"strings"
)

var pointMapResults = map[string]int{
	"lost": 0,
	"draw": 3,
	"win":  6,
}

type sign struct {
	name     string
	strongTo string
	weakTo   string
	worth    int
}

var charSignMap = map[rune]sign{
	'A': {name: "rock", strongTo: "scissors", weakTo: "paper", worth: 1},
	'B': {name: "paper", strongTo: "rock", weakTo: "scissors", worth: 2},
	'C': {name: "scissors", strongTo: "paper", weakTo: "rock", worth: 3},
	'X': {name: "rock", strongTo: "scissors", weakTo: "paper", worth: 1},
	'Y': {name: "paper", strongTo: "rock", weakTo: "scissors", worth: 2},
	'Z': {name: "scissors", strongTo: "paper", weakTo: "rock", worth: 3},
}

func Part1_calcIfWon(sign1 sign, sign2 sign) int {
	if sign2.strongTo == sign1.name {
		return pointMapResults["win"]
	} else if sign2.weakTo == sign1.name {
		return pointMapResults["lost"]
	} else {
		return pointMapResults["draw"]
	}
}

func Part1_calculatePoints(line string) int {
	signs := strings.Split(strings.TrimSpace(line), " ")
	firstSign := charSignMap[rune(signs[0][0])]
	secondSign := charSignMap[rune(signs[1][0])]
	signSum := secondSign.worth
	resultSum := Part1_calcIfWon(firstSign, secondSign)
	return resultSum + signSum
}

func Part1_workFiles(content string) {
	sum := 0
	lines := strings.Split(content, "\n")
	for _, line := range lines {
		if len(line) < 3 {
			continue
		}
		value := Part1_calculatePoints(line)
		sum += value
	}
	println(sum)
}
