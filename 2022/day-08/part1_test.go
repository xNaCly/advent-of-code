package main

import (
	"fmt"
	"os"
	"strings"
	"testing"
)

func TestPart(t *testing.T) {
	o := Part(strings.NewReader(`
30373
25512
65332
33549
35390`))
	if o != 21 {
		t.Fatalf("got %d, wanted 21", o)
	}
}

// TODO: as always with aoc, the example is right, but somehow when working with the code on the full puzzle input, its incorrect
func TestPartFinal(t *testing.T) {
	f, _ := os.Open("i.txt")
	o := Part(f)
	fmt.Println("output: ", o)
	if o == 0 {
		t.Fatalf("got %d, wanted 21", o)
	}
}
