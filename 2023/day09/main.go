package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func getDiff(line []int) ([]int, bool) {
	result, found := []int{}, false
	for i := 1; i <= len(line)-1; i++ {
		result = append(result, line[i]-line[i-1])
	}
	for _, v := range result {
		if v != 0 {
			found = false
			break
		}
		found = true
	}
	return result, found
}

func solvePart1(input string) int {
	sum := 0
	for _, line := range strings.Split(input, "\n") {
		curLine := []int{}
		for _, n := range strings.Split(line, " ") {
			if x, e := strconv.Atoi(n); e == nil {
				curLine = append(curLine, x)
			}
		}

		finalVal := []int{curLine[len(curLine)-1]}
		done := false
		for !done {
			curLine, done = getDiff(curLine)
			finalVal = append([]int{curLine[len(curLine)-1]}, finalVal...)
		}
		for i := 1; i <= len(finalVal)-1; i++ {
			sum += finalVal[i]
		}
	}
	return sum
}

func solvePart2(input string) int {
	sum := 0
	for _, line := range strings.Split(input, "\n") {
		curLine := []int{}
		for _, n := range strings.Split(line, " ") {
			if x, e := strconv.Atoi(n); e == nil {
				curLine = append(curLine, x)
			}
		}

		// To solve part 2 we just need to reverse the first line

		for i2, j := 0, len(curLine)-1; i2 < j; i2, j = i2+1, j-1 {
			curLine[i2], curLine[j] = curLine[j], curLine[i2]
		}

		finalVal := []int{curLine[len(curLine)-1]}
		done := false
		for !done {
			curLine, done = getDiff(curLine)
			finalVal = append([]int{curLine[len(curLine)-1]}, finalVal...)
		}
		for i := 1; i <= len(finalVal)-1; i++ {
			sum += finalVal[i]
		}
	}
	return sum
}

func main() {
	i, _ := os.ReadFile("input.txt")
	input := string(i)

	fmt.Println("Part 1:", solvePart1(input))
	fmt.Println("Part 2:", solvePart2(input))
}
