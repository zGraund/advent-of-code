package main

import (
	"fmt"
	"os"
	"strings"
)

type Cord struct {
	y, x int
}

func getStartDir(maze string, startIndex int, dirToStep map[string]int) string {

	/*
		Get the starting direction by looking around the starting point for
		the first valid move
	*/

	validMoves := map[string]string{
		"left":  "-LF",
		"up":    "|7F",
		"right": "-J7",
		"down":  "|LJ",
	}
	for _, dir := range []string{"left", "up", "right", "down"} {
		if strings.ContainsRune(validMoves[dir], rune(maze[startIndex+dirToStep[dir]])) {
			return dir
		}
	}
	panic("No starting move found")
}

func getNextDir(currentNode string, direction string) string {

	/*
		Get the next direction based on the current direction
		and the current maze node
	*/

	switch currentNode {
	case "|":
		fallthrough
	case "-":
		return direction
	case "L":
		if direction == "down" {
			return "right"
		}
		return "up"
	case "J":
		if direction == "down" {
			return "left"
		}
		return "up"
	case "7":
		if direction == "right" {
			return "down"
		}
		return "left"
	case "F":
		if direction == "left" {
			return "down"
		}
		return "right"
	case "S":
		return "done"
	default:
		panic("No next node detected")
	}
}

func solvePart1(input string, mazeLen int, dirToStep map[string]int) int {

	/*
		I have decided to use a 1d string instead of a 2d
		array to navigate the maze, for no particular reason.
		Go from start to finish of the maze and count
		the steps, then divide by 2 to get the answer
	*/

	curPos := strings.Index(input, "S")
	dir := getStartDir(input, curPos, dirToStep)

	count := 1
	for dir != "done" {
		curPos += dirToStep[dir]
		dir = getNextDir(string(input[curPos]), dir)
		count++
	}
	return count / 2
}

func solvePart2(input string, mazeLen int, dirToStep map[string]int) int {

	/*
		! Solution based on tips taken from the AoC solution megathread

		Get the perimeter with the same approach of part1, while saving
		the 3d coordinates of the current location inside cords
		With all the perimeter coordinates we can calculate the area
		of the maze using the Shoelace formula, with the area and the perimeter
		we can now calculate the number of points inside the area using
		Pick's theorem:

		A = i + (b / 2) - 1

		solving for i:

		i = A - (b / 2) + 1

		where A = area, b = perimeter and i = interior points
	*/

	var cords []Cord

	curPos := strings.Index(input, "S")
	dir := getStartDir(input, curPos, dirToStep)

	perimeter := 1
	for dir != "done" {
		// calculate the 2d coordinates from the 1d input string
		x := curPos % mazeLen
		y := curPos / mazeLen // int division so rounded down
		cords = append(cords, Cord{y, x})
		curPos += dirToStep[dir]
		dir = getNextDir(string(input[curPos]), dir)
		perimeter++
	}

	area := 0
	for i := 0; i < len(cords); i++ {
		var p1, p2 Cord
		p1 = cords[i]
		if i == len(cords)-1 {
			p2 = cords[0]
		} else {
			p2 = cords[i+1]
		}
		area += (p1.x * p2.y) - (p1.y * p2.x)
	}
	area = area / 2
	return area - (perimeter / 2) + 1
}

func main() {
	i, _ := os.ReadFile("input.txt")
	mazeLen := len(strings.SplitN(string(i), "\n", 2)[0])
	input := strings.ReplaceAll(string(i), "\n", "")

	dirToStep := map[string]int{
		"up":    -mazeLen,
		"down":  mazeLen,
		"right": 1,
		"left":  -1,
	}

	fmt.Println("Part 1:", solvePart1(input, mazeLen, dirToStep))
	fmt.Println("Part 2:", solvePart2(input, mazeLen, dirToStep))
}
