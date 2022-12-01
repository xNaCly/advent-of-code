package main

import (
	"os"
)

func main() {
	// set false to run part 2
	part1 := false
	file_path := "input.txt"

	println("running for file=", file_path)
	content := Part1_ReadFile(file_path)
	if part1 {
		Part1_WorkFile(content)
		os.Exit(0)
	} else {
		res := Part2_CalcSumTopThree(content)
		println(res)
	}
}
