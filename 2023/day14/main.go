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

func cycle(c map[Coord]string, len int) int {

	/*
		Make a full north -> west -> south -> east
		cycle and then calculate the load
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
	sum := 0
	for k, v := range c {
		if v == "O" {
			sum += len - k.row
		}
	}
	return sum
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
		in the "cache" and store an array with all the previous
		calculated loads, when an item already exist in the map
		a cycle is discovered, with the cycle length we can then
		calculate where the 1B th cycle will end up and return
		the relative load from the "already seen" array
	*/

	cache, loads := map[string]int{}, []int{}
	for i := 1; i <= 1000000000; i++ {
		sum := cycle(coords, col)
		key := createKey(coords, col, row)
		if v, ok := cache[key]; ok {
			cycleStart, cycleLen := v, i-v
			return loads[cycleStart+((1000000000-cycleStart)%(-cycleLen)-1)]
		}
		loads = append(loads, sum)
		cache[key] = i
	}
	return 0
}

func main() {
	i, _ := os.ReadFile("input.txt")
	input := strings.Split(string(i), "\n")

	coordsP1 := map[Coord]string{}
	coordsP2 := map[Coord]string{}
	for row, l := range input {
		for col, c := range l {
			coordsP1[Coord{row, col}] = string(c)
			coordsP2[Coord{row, col}] = string(c)
		}
	}
	col, row := len(input), len(input[0])

	fmt.Println("Part 1:", solvePart1(coordsP1, len(input)))
	fmt.Println("Part 2:", solvePart2(coordsP2, col, row))
}
