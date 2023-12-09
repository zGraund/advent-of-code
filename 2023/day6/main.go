package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func getNumbers(str string) []int {
	re := regexp.MustCompile(`\b\d+\b`)
	s := re.FindAllString(str, -1)
	result := make([]int, len(s))
	for i, v := range s {
		if j, err := strconv.Atoi(v); err == nil {
			result[i] = j
		}
	}
	return result
}

func getNumbersPart2(str string) int {
	re := regexp.MustCompile(`\b\d+\b`)
	s := re.FindAllString(str, -1)
	st := strings.Join(s, "")
	if j, err := strconv.Atoi(st); err == nil {
		return j
	}
	return 0
}

func getUpLimit(t int, d int) int {
	for x := 0; x < t; x++ {
		if x*(t-x) > d {
			return x
		}
	}
	return 0
}

func getDownLimit(t int, d int) int {
	for x := t; x >= 0; x-- {
		if x*(t-x) > d {
			return x + 1
		}
	}
	return 0
}

/*
	!Note:
	The upper and lower bound of the
	winning time are mirrored so technically
	we only need 1 of them to calculate the
	result, but i figured that out at the end
	so i'm just gonna write this note
*/

func solvePart1(t string, d string) int {
	time := getNumbers(t)
	destination := getNumbers(d)

	result := 1
	for i, v := range time {
		up := getUpLimit(v, destination[i])
		down := getDownLimit(v, destination[i])
		result *= down - up
	}
	return result
}

func solvePart2(t string, d string) int {
	time := getNumbersPart2(t)
	destination := getNumbersPart2(d)

	up := getUpLimit(time, destination)
	down := getDownLimit(time, destination)

	return down - up
}

func main() {
	i, _ := os.ReadFile("input.txt")
	input := string(i)
	lines := strings.Split(input, "\n")

	fmt.Println("Part 1:", solvePart1(lines[0], lines[1]))
	fmt.Println("Part 2:", solvePart2(lines[0], lines[1]))
}
