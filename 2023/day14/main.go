package main

import (
	"fmt"
	"os"
	"strings"
)

type Coord struct {
	row, col int
}

func createKey(m map[Coord]string, row, col int) string {

	/*
		Flatten the map into a string that respect
		the input order
	*/

	temp := make([][]string, row)
	for i := range temp {
		temp[i] = make([]string, col)
	}
	for k, v := range m {
		temp[k.row][k.col] = v + temp[k.row][k.col]
	}
	result := ""
	for _, v := range temp {
		result += strings.Join(v, "")
	}
	return result
}

func cycle(c map[Coord]string, len int) {

	/*
		Make a full north -> west -> south -> east
		cycle
	*/

	for _, dir := range [4]Coord{{-1, 0}, {0, -1}, {1, 0}, {0, 1}} {
		done := false
		for !done {
			done = true
			for k, v := range c {
				if v != "O" {
					continue
				}
				if spot, ok := c[Coord{k.row + dir.row, k.col + dir.col}]; ok && spot == "." {
					c[Coord{k.row + dir.row, k.col + dir.col}] = "O"
					c[k] = "."
					done = false
				}
			}
		}
	}
}

func calculateLoad(m map[string]int, i, row, col int) int {
	sum := 0
	for k, v := range m {
		if v == i {
			for ind, char := range k {
				if char == 'O' {
					sum += row - (ind / col)
				}
			}
			return sum
		}
	}
	return 0
}

func solvePart1(coords map[Coord]string, len int) int {

	/*
		Make a north tilt and then
		calculate the load
	*/

	done := false
	for !done {
		done = true
		for k, v := range coords {
			if v != "O" {
				continue
			}
			if up, ok := coords[Coord{k.row - 1, k.col}]; ok && up == "." {
				coords[Coord{k.row - 1, k.col}] = "O"
				coords[k] = "."
				done = false
			}
		}
	}

	sum := 0
	for k, v := range coords {
		if v == "O" {
			sum += len - k.row
		}
	}
	return sum
}

func solvePart2(coords map[Coord]string, col, row int) int {

	/*
		Start a loop and keep track of the seen stone combinations
		in the "cache", when an item already exist in the map
		a cycle is discovered, with the cycle length we can then
		calculate where the 1Bth cycle will end up and calculate
		the load using the key of the relative cache entry
	*/

	cache := map[string]int{}
	for i := 1; i <= 1000000000; i++ {
		cycle(coords, col)
		key := createKey(coords, col, row)
		if v, ok := cache[key]; ok {
			cycleStart, cycleLen := v, i-v
			i := cycleStart + ((1000000000 - cycleStart) % (-cycleLen))
			return calculateLoad(cache, i, row, col)
		}
		cache[key] = i
	}
	return 0
}

func main() {
	i, _ := os.ReadFile("input.txt")
	input := strings.Split(string(i), "\n")

	// Make 2 copies so we can modify them in place
	coordsP1 := map[Coord]string{}
	coordsP2 := map[Coord]string{}
	for row, l := range input {
		for col, c := range l {
			coordsP1[Coord{row, col}] = string(c)
			coordsP2[Coord{row, col}] = string(c)
		}
	}
	row, col := len(input), len(input[0])

	fmt.Println("Part 1:", solvePart1(coordsP1, row))
	fmt.Println("Part 2:", solvePart2(coordsP2, row, col))
}
