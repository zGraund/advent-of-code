package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Coord struct {
	row, col int
}

func nextCoord(d Coord, dir string, steps int) Coord {
	switch dir {
	case "U":
		fallthrough
	case "3":
		d.row -= steps
	case "D":
		fallthrough
	case "1":
		d.row += steps
	case "L":
		fallthrough
	case "2":
		d.col -= steps
	case "R":
		fallthrough
	case "0":
		d.col += steps
	}
	return d
}

func solvePart1(input []string) int {

	/*
		To solve this problem we can use the same approach
		as day 10 by using shoelace formula to calculate
		the internal points and then add the perimeter to
		get the result

		A = i + (b / 2) - 1

		solving for i:
		i = A - (b / 2) + 1

		Solution = A + i

		where A = area, b = perimeter and i = interior points
	*/

	vertices := []Coord{}
	excavator := Coord{0, 0}
	perimeter := 0
	for _, line := range input {
		t := strings.Split(line, " ")
		dir, dist := t[0], 0

		if n, err := strconv.Atoi(t[1]); err == nil {
			dist = n
		}

		excavator = nextCoord(excavator, dir, dist)
		vertices = append(vertices, excavator)
		perimeter += dist
	}

	area := 0
	for i := 0; i < len(vertices); i++ {
		p1, p2 := vertices[i], vertices[(i+1)%len(vertices)]
		area += (p1.col * p2.row) - (p1.row * p2.col)
	}

	// The area in divided by 2 because the shoelace formula
	// calculate 2A
	internalPoints := (area / 2) - (perimeter / 2) + 1
	return internalPoints + perimeter
}

func solvePart2(input []string) int {

	/*
		Same solution as part 1 but with
		the parsing of the right distance
		value
	*/

	vertices := []Coord{}
	excavator := Coord{0, 0}
	perimeter := 0
	for _, line := range input {
		t := strings.Split(line, " ")[2]
		dir, dist := string(t[len(t)-2]), 0

		if n, err := strconv.ParseInt(t[2:len(t)-2], 16, 64); err == nil {
			dist = int(n)
		}

		excavator = nextCoord(excavator, dir, dist)
		vertices = append(vertices, excavator)
		perimeter += dist
	}

	area := 0
	for i := 0; i < len(vertices); i++ {
		p1, p2 := vertices[i], vertices[(i+1)%len(vertices)]
		area += (p1.col * p2.row) - (p1.row * p2.col)
	}

	internalPoints := (area / 2) - (perimeter / 2) + 1
	return internalPoints + perimeter
}

func main() {
	i, _ := os.ReadFile("input.txt")
	input := strings.Split(string(i), "\n")

	fmt.Println("Part 1:", solvePart1(input))
	fmt.Println("Part 2:", solvePart2(input))
}
