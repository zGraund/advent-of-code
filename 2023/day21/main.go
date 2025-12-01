package main

import (
	"fmt"
	"os"
	"strings"
)

type Plot struct {
	row, col int
}

var plots = map[Plot]bool{} // False if plot is unvisited
var start Plot

func (p Plot) getNextPos() []Plot {
	newArr := []Plot{}
	for _, v := range [][]int{{0, 1}, {0, -1}, {1, 0}, {-1, 0}} {
		next := Plot{p.row + v[0], p.col + v[1]}
		if visited, ok := plots[next]; ok && !visited {
			newArr = append(newArr, next)
		}
	}
	plots[p] = true
	return newArr
}

func solvePart1(steps int) int {
	pos := []Plot{start}
	result := map[Plot]bool{}
	for i := 0; i <= steps; i++ { // We do 1 more step to get the last positions
		new := []Plot{}
		if i%2 == 0 {
			for _, v := range pos {
				result[v] = true
			}
		}
		for _, p := range pos {
			if v := plots[p]; v {
				continue
			}
			new = append(new, p.getNextPos()...)
		}
		pos = new
	}
	return len(result)
}

func solvePart2(input string) int {
	return 1
}

func main() {
	i, _ := os.ReadFile("input.txt")
	input := strings.Split(string(i), "\n")

	for row, line := range input {
		for col, p := range strings.Split(line, "") {
			if p == "." {
				plots[Plot{row, col}] = false
			} else if p == "S" {
				plots[Plot{row, col}] = false
				start = Plot{row, col}
			}
		}
	}

	fmt.Println("Part 1:", solvePart1(64))
	// fmt.Println(plots)
	// fmt.Println("Part 2:", solvePart2(input))
}
