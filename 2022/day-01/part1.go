package main

import (
	"log"
	"os"
	"strconv"
	"strings"
)

func Part1_CleanInput(line string) string {
	return strings.TrimFunc(line, func(r rune) bool {
		res := false
		switch r {
		case '\n', '\r', ' ', '\t':
			res = true
		}
		return res
	})
}

func Part1_ReadFile(path string) string {
	data, _ := os.ReadFile(path)
	return string(data)
}

func Part1_WorkFile(content string) {
	sum := 0
	max := 0
	for _, line := range strings.Split(content, "\n") {
		// trim whitespaces
		line := Part1_CleanInput(line)
		if len(line) == 0 {
			if max < sum {
				max = sum
			}
			sum = 0
			continue
		}
		integer, err := strconv.ParseInt(line, 10, 32)
		if err != nil {
			log.Fatalln(err, " ", line)
		}
		sum += int(integer)
	}
	println(max)
}
