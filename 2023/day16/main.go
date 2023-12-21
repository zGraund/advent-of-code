package main

import (
	"fmt"
	"os"
	"strings"
	"time"
)

var beamCounter = 0

type Coord struct {
	row, col int
}

type Beam struct {
	coord Coord
	dir   string
}

func getNextPos(beam Beam, curNode string) []Beam {

	/*
		Modify the coordinates and the directions
		of the current beam based on the current
		node and the current direction, if the beam
		meet a splitter return a second beam
	*/

	beams := []Beam{}
	dirMap := map[string]Coord{
		"up":    {-1, 0},
		"down":  {1, 0},
		"right": {0, 1},
		"left":  {0, -1},
	}

	switch curNode {
	case ".":
		break
	case "|":
		if beam.dir == "left" || beam.dir == "right" {
			beam2 := beam

			beam.dir = "up"
			beam2.dir = "down"
			beams = append(beams, beam2)
		}
	case "-":
		if beam.dir == "up" || beam.dir == "down" {
			beam2 := beam

			beam.dir = "right"
			beam2.dir = "left"
			beams = append(beams, beam2)
		}
	case "\\":
		switch beam.dir {
		case "up":
			beam.dir = "left"
		case "down":
			beam.dir = "right"
		case "left":
			beam.dir = "up"
		case "right":
			beam.dir = "down"
		}
	case "/":
		switch beam.dir {
		case "up":
			beam.dir = "right"
		case "down":
			beam.dir = "left"
		case "left":
			beam.dir = "down"
		case "right":
			beam.dir = "up"
		}
	default:
		panic("No valid node provided")
	}
	beams = append(beams, beam)
	for i := range beams {
		beams[i].coord.row += dirMap[beams[i].dir].row
		beams[i].coord.col += dirMap[beams[i].dir].col
	}
	return beams
}

func iterate(b Beam, grid map[Coord]string, visitedNodes map[Coord]string) {

	/*
		Move the current beam to the next node, if 2 beams are returned
		from the getNextPos function, iterate over that beam too,
		keep track of the visited node with the relative direction to
		avoid infinite loops
	*/

	if _, ok := grid[b.coord]; !ok {
		return
	}
	if v, ok := visitedNodes[b.coord]; ok {
		if v == b.dir {
			return
		}
	}
	beamCounter++
	visitedNodes[b.coord] = b.dir
	beams := getNextPos(b, grid[b.coord])
	if len(beams) == 2 {
		iterate(beams[1], grid, visitedNodes)
	}
	iterate(beams[0], grid, visitedNodes)
}

func solvePart1(grid map[Coord]string) int {

	// The len of the visited map is the number of
	// energized nodes
	visited := make(map[Coord]string)
	iterate(Beam{Coord{0, 0}, "right"}, grid, visited)
	return len(visited)
}

func solvePart2(grid map[Coord]string, rowLen, colLen int) int {

	// Same as part 1 but check all the beams
	max := 0
	for row := 0; row < rowLen; row++ {
		visited1 := make(map[Coord]string)
		iterate(Beam{Coord{row, 0}, "right"}, grid, visited1)

		visited2 := make(map[Coord]string)
		iterate(Beam{Coord{row, colLen}, "left"}, grid, visited2)

		if len(visited1) > max {
			max = len(visited1)
		}
		if len(visited2) > max {
			max = len(visited2)
		}
	}
	for col := 0; col < colLen; col++ {
		visited1 := make(map[Coord]string)
		iterate(Beam{Coord{0, col}, "down"}, grid, visited1)

		visited2 := make(map[Coord]string)
		iterate(Beam{Coord{rowLen, col}, "up"}, grid, visited2)

		if len(visited1) > max {
			max = len(visited1)
		}
		if len(visited2) > max {
			max = len(visited2)
		}
	}
	return max
}

func main() {
	file, _ := os.ReadFile("input.txt")
	input := strings.Split(string(file), "\n")

	grid := make(map[Coord]string)
	for row, line := range input {
		for col, char := range line {
			grid[Coord{row, col}] = string(char)
		}
	}

	// I was curious about the performance so i added
	// a counter and a timer
	start := time.Now()
	fmt.Println("Part 1:", solvePart1(grid))
	elapsed := time.Since(start)
	fmt.Println("Completed in:", elapsed)
	fmt.Println()

	start = time.Now()
	fmt.Println("Part 2:", solvePart2(grid, len(input)-1, len(input[0])-1))
	elapsed = time.Since(start)
	fmt.Println("Total light beams:", beamCounter)
	fmt.Println("Time to complete: ", elapsed)
}
