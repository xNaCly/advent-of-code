package main

import (
	"os"
)

func readFile(path string) string {
	data, _ := os.ReadFile(path)
	return string(data)
}

func main() {
	// set false to run part 2
	part1 := false
	file_path := "input.txt"

	println("running for file=", file_path)
	content := readFile(file_path)
	if part1 {
		Part1_workFiles(content)
	} else {
		Part2_workFile(content)
	}
}
