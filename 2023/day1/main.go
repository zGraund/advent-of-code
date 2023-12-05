package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
	"unicode"
)

func extractNumbers(str string) int {
	var numbers []string
	for _, char := range str {
		if unicode.IsDigit(char) {
			numbers = append(numbers, string(char))
		}
	}
	calibration := strings.Join([]string{numbers[0], numbers[len(numbers)-1]}, "")
	i, _ := strconv.Atoi(calibration)
	return i
}

func solvePart1(str string) int {
	sum := 0
	for _, line := range strings.Fields(str) {
		sum += extractNumbers(line)
	}
	return sum
}

func solvePart2(str string) int {
	// Replace only the inner letters to account for the edge case of overlapping
	replacer := strings.NewReplacer(
		"one", "o1e",
		"two", "t2o",
		"three", "t3e",
		"four", "f4r",
		"five", "f5e",
		"six", "s6x",
		"seven", "s7n",
		"eight", "e8t",
		"nine", "n9e",
	)
	sum := 0
	for _, line := range strings.Fields(str) {
		// Replacing 2 times to avoid overlapping edge cases
		// eightwo => 82
		line = replacer.Replace(replacer.Replace(line))
		sum += extractNumbers(line)
	}
	return sum
}

func main() {
	i, _ := os.ReadFile("input.txt")
	input := string(i)

	fmt.Println("Part 1:", solvePart1(input))
	fmt.Println("Part 2:", solvePart2(input))
}
