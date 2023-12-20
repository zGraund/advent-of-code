package main

import (
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

/*
	Very straightforward day, just make a fixed array of 256 boxes,
	parse the input and follow the instructions
*/

func ASCIIStringHelperAlgorithm(str string, part2 bool) int {
	result := 0
	for _, char := range str {
		if (char == '-' || char == '=') && part2 {
			return result
		}
		result = ((result + int(char)) * 17) % 256
	}
	return result
}

func HolidayASCIIStringHelperManualArrangementProcedure(str string, boxes [][]string) {
	boxInd := ASCIIStringHelperAlgorithm(str, true)
	box := boxes[boxInd]

	if strings.HasSuffix(str, "-") {
		label := strings.TrimSuffix(str, "-")
		for i, v := range box {
			if strings.HasPrefix(v, label) {
				boxes[boxInd] = slices.Delete(box, i, i+1)
			}
		}
		return
	}

	if strings.Contains(str, "=") {
		label := strings.Replace(str, "=", " ", 1)
		for i, v := range box {
			if strings.HasPrefix(v, strings.Split(label, " ")[0]) {
				box[i] = label
				return
			}
		}
		boxes[boxInd] = append(box, label)
	}
}

func solvePart1(input []string) int {
	sum := 0
	for _, str := range input {
		sum += ASCIIStringHelperAlgorithm(str, false)
	}
	return sum
}

func solvePart2(input []string) int {
	boxes := make([][]string, 256)
	for _, str := range input {
		HolidayASCIIStringHelperManualArrangementProcedure(str, boxes)
	}

	sum := 0
	for boxI, box := range boxes {
		if len(box) == 0 {
			continue
		}
		for lensI, lens := range box {
			if focal, err := strconv.Atoi(strings.Split(lens, " ")[1]); err == nil {
				sum += (boxI + 1) * (lensI + 1) * focal
			}
		}
	}
	return sum
}

func main() {
	i, _ := os.ReadFile("input.txt")
	input := strings.Split(string(i), ",")

	fmt.Println("Part 1:", solvePart1(input))
	fmt.Println("Part 2:", solvePart2(input))
}
