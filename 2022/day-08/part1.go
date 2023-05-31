package main

import (
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

	for i, l := range strings.Split(s, "\n") {
		m[i] = make([]int, rclen)
		for j, c := range l {
			m[i][j] = int(c - '0')
		}
	}

	for i := range m {
		if i == 0 || i+1 == len(m) {
			r += len(m[i])
			continue
		}
		for j := range m[i] {
			if j == 0 || j+1 == len(m[i]) {
				r++
			} else if Visible(m, i, j) {
				r++
			}
		}
	}

	return r
}
