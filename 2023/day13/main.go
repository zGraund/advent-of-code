package main

import (
	"fmt"
	"os"
	"slices"
	"strings"
)

func invertMatrix(m []string) []string {
	t := make([]string, len(m[0]))
	for _, line := range m {
		for i, col := range line {
			t[i] = string(col) + t[i]
		}
	}
	return t
}

func differByOne(s1, s2 []string) bool {

	/*
		Count how many differences there are
		between 2 array of string and return
		true if there is only one
	*/

	count := 0
	if len(s1) != len(s2) {
		return false
	}
	for i, str := range s1 {
		for n, char := range str {
			if char != rune(s2[i][n]) {
				count++
			}
		}
	}
	return count == 1
}

func getValue(mirror string) int {

	/*
		Get 2 arrays based on a central point,
		and check if they are equal
	*/

	lines := strings.Split(mirror, "\n")
	for i := 1; i < len(lines); i++ {
		up := make([]string, 0)
		up = append(up, lines[:i]...)
		slices.Reverse(up)
		down := lines[i:]
		if slices.Equal(down[:min(len(up), len(down))], up[:min(len(down), len(up))]) {
			return i
		}
	}
	return 0
}
func getValueWithSmudge(mirror string) int {

	/*
		Same of the other func but check if
		they differ by only 1 point
	*/

	lines := strings.Split(mirror, "\n")
	for i := 1; i < len(lines); i++ {
		up := make([]string, 0)
		up = append(up, lines[:i]...)
		slices.Reverse(up)
		down := lines[i:]
		if differByOne(down[:min(len(up), len(down))], up[:min(len(down), len(up))]) {
			return i
		}
	}
	return 0
}

func solvePart1(input []string) int {
	sum := 0
	for _, mirror := range input {
		sum += getValue(mirror) * 100
		sum += getValue(strings.Join(invertMatrix(strings.Split(mirror, "\n")), "\n")) * 1
	}
	return sum
}

func solvePart2(input []string) int {
	sum := 0
	for _, mirror := range input {
		sum += getValueWithSmudge(mirror) * 100
		sum += getValueWithSmudge(strings.Join(invertMatrix(strings.Split(mirror, "\n")), "\n"))
	}
	return sum
}

func main() {
	i, _ := os.ReadFile("input.txt")
	input := strings.Split(string(i), "\n\n")

	fmt.Println("Part 1:", solvePart1(input))
	fmt.Println("Part 2:", solvePart2(input))
}
