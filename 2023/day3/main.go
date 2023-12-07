package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type PartNumber struct {
	startIndex,
	endIndex,
	number int
}

func findNumbers(str string) []PartNumber {
	result := []PartNumber{}
	part := PartNumber{-1, -1, 0}

	// Custom made super janky algorithm to find all numbers in a line

	for indexValue, value := range strings.Split(str, "") {
		if i, err := strconv.Atoi(value); err == nil {
			part.number = part.number*10 + i
			if part.startIndex == -1 {
				part.startIndex = indexValue
			}
			if indexValue == len(str)-1 {
				part.endIndex = indexValue
				result = append(result, part)
				part = PartNumber{-1, -1, 0}
			}
		} else {
			if part.startIndex != -1 {
				part.endIndex = indexValue - 1
				result = append(result, part)
				part = PartNumber{-1, -1, 0}
			}
		}
	}
	return result
}

func isValidSymbol(c byte) bool {
	symbols := "@#$%&*/+-="
	return strings.ContainsAny(string(c), symbols)
}

func isPartNumber(part PartNumber, line, previousLine, nextLine string) bool {

	// Check if the number has any symbols around (again super janky)

	if part.startIndex > 0 && isValidSymbol(line[part.startIndex-1]) {
		return true
	}
	if part.endIndex < len(line)-1 && isValidSymbol(line[part.endIndex+1]) {
		return true
	}
	for i := part.startIndex - 1; i <= part.endIndex+1; i++ {
		if i >= 0 && i <= len(previousLine)-1 {
			if isValidSymbol(previousLine[i]) {
				return true
			}
		}
	}
	for i := part.startIndex - 1; i <= part.endIndex+1; i++ {
		if i >= 0 && i <= len(nextLine)-1 {
			if isValidSymbol(nextLine[i]) {
				return true
			}
		}
	}

	return false
}

func isGear(parts []PartNumber, index int) []int {

	// Check if the asterisk is touching any number

	result := []int{}
	for _, v := range parts {
		if (index >= v.startIndex-1) && (index <= v.endIndex+1) {
			result = append(result, v.number)
		}
	}
	return result
}

func solvePart1(input string) int {
	lines := strings.Split(input, "\n")
	sum := 0
	previousLine := ""
	for indexLine, line := range lines {
		// Find all numbers in the line
		numbers := findNumbers(line)
		nextLine := ""
		for _, v := range numbers {
			if indexLine < len(lines)-1 {
				nextLine = lines[indexLine+1]
			}
			// Check if the number is a part
			if isPartNumber(v, line, previousLine, nextLine) {
				sum += v.number
			}
		}
		previousLine = line
	}
	return sum
}

func solvePart2(input string) int {
	lines := strings.Split(input, "\n")
	sum := 0
	previousLine := ""
	for indexLine, line := range lines {
		nextLine := ""
		if indexLine < len(lines)-1 {
			nextLine = lines[indexLine+1]
		}
		// Get all asterisks in the line
		var asterisks = []int{}
		for i, v := range line {
			if string(v) == "*" {
				asterisks = append(asterisks, i)
			}
		}
		if len(asterisks) == 0 {
			previousLine = line
			continue
		}

		// Get all numbers that are on the same line and one line above and below
		pNum := findNumbers(previousLine)
		cNum := findNumbers(line)
		nNum := findNumbers(nextLine)
		parts := append(append(pNum, cNum...), nNum...)
		for _, v := range asterisks {
			// Check if the asterisk is touching two numbers
			gears := isGear(parts, v)
			if len(gears) == 2 {
				sum += gears[0] * gears[1]
			}
		}
		previousLine = line
	}
	return sum
}

func main() {
	i, _ := os.ReadFile("input.txt")
	input := string(i)

	fmt.Println("Part 1:", solvePart1(input))
	fmt.Println("Part 2:", solvePart2(input))
}
