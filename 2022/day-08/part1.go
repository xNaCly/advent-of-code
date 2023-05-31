package main

import (
	"fmt"
	"io"
	"strings"
)

func Visible(m [][]int, i int, j int) bool {
	cc := m[i][j]

	dir := map[rune][]int{
		'w': m[i][:j],
		'e': m[i][j+1:],
	}

	n := make([]int, 0)
	for y := 0; y < i; y++ {
		n = append(n, m[y][j])
	}
	dir['n'] = n

	s := make([]int, i)
	for y := i; y < len(m); y++ {
		s = append(s, m[y][j])
	}
	dir['s'] = s

	dirV := map[rune]bool{
		'w': true,
		'e': true,
		'n': true,
		's': true,
	}

	for k, v := range dir {
		for _, i := range v {
			if cc <= i {
				dirV[k] = false
				break
			}
		}
	}

	return dirV['w'] || dirV['e'] || dirV['n'] || dirV['s']
}

func Part(reader io.Reader) int {
	var r int

	out, _ := io.ReadAll(reader)

	s := strings.TrimSpace(string(out))
	rclen := strings.IndexRune(s, '\n')

	m := make([][]int, rclen)

	// TODO: this could work, idk
	// its a square, so we calculate the trees at the edge, minus the 4 edges
	// r += rclen*4 - 4

	for i, l := range strings.Split(s, "\n") {
		m[i] = make([]int, rclen)
		for j, c := range l {
			m[i][j] = int(c - '0')
		}
	}

	for i := range m {
		if i == 0 || i+1 == rclen {
			// TODO: this was a continue
			r++
		}
		for j := range m[i] {
			if j == 0 || j+1 == rclen {
				// TODO: this was a continue
				r++
			} else if Visible(m, i, j) {
				fmt.Print(m[i][j], " ")
				r++
			} else {
				fmt.Print("x ")
			}
		}
		fmt.Println()
	}

	return r - 1
}
