package main

import (
	"log"
	"sort"
	"strconv"
	"strings"
)

func Part2_CalcSumTopThree(content string) int {
	sum := 0
	sumArr := make([]int, 0)
	splitString := strings.Split(content, "\n")
	for i, line := range splitString {
		line := Part1_CleanInput(line)
		if len(line) == 0 {
			sumArr = append(sumArr, sum)
			sum = 0
			continue
		}
		integer, err := strconv.ParseInt(line, 10, 32)
		if err != nil {
			log.Fatalln(err, " ", line)
		}
		sum += int(integer)
		if i == len(splitString)-1 {
			sumArr = append(sumArr, sum)
		}
	}

	sort.Slice(sumArr, func(i, j int) bool {
		return sumArr[i] > sumArr[j]
	})

	t := 0
	for i := 0; i < 3; i++ {
		t += sumArr[i]
	}
	return t
}
