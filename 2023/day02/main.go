package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func getRGB(str string) map[string]int {
	m := map[string]int{
		"red":   0,
		"green": 0,
		"blue":  0,
	}
	for _, v := range []string{"red", "green", "blue"} {
		if strings.HasSuffix(str, v) {
			val := strings.Replace(str, " "+v, "", 1)
			n, _ := strconv.Atoi(val)
			m[v] = n
		}
	}
	return m
}

func solvePart1(input string) int {
	const R int = 12
	const G int = 13
	const B int = 14

	sum := 0
game:
	for i, line := range strings.Split(input, "\n") {
		game := strings.Split(line, ": ")[1]
		round := strings.Split(game, "; ")

		for _, r := range round {
			value := strings.Split(r, ", ")

			for _, v := range value {
				val := getRGB(v)
				if val["red"] > R || val["green"] > G || val["blue"] > B {
					continue game
				}
			}
		}
		sum += (i + 1)
	}
	return sum
}

func solvePart2(input string) int {
	sum := 0

	for _, line := range strings.Split(input, "\n") {
		minR := 0
		minG := 0
		minB := 0

		game := strings.Split(line, ": ")[1]
		round := strings.Split(game, "; ")

		for _, r := range round {
			value := strings.Split(r, ", ")

			for _, v := range value {
				val := getRGB(v)
				if minR < val["red"] {
					minR = val["red"]
				}
				if minG < val["green"] {
					minG = val["green"]
				}
				if minB < val["blue"] {
					minB = val["blue"]
				}
			}
		}
		sum += (minR * minG * minB)
	}
	return sum
}

func main() {
	i, _ := os.ReadFile("input.txt")
	input := string(i)

	fmt.Println("Part 1:", solvePart1(input))
	fmt.Println("Part 2:", solvePart2(input))
}
