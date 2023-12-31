package main

import (
	"fmt"
	"os"
	"regexp"
	"strings"
)

/*
	Another solution was to not split the winning and other numbers
	and just check if they appear 2 times in the string
	the number of numbers that appeared multiple times
	is the number of winning numbers
*/

func solvePart1(input string) int {
	sum := 0
	for _, line := range strings.Split(input, "\n") {
		nums := strings.Split(strings.Split(line, ":")[1], "|")

		re := regexp.MustCompile(`\b\d+\b`)
		winningNumbers := re.FindAllString(nums[0], -1)
		numbersYouHave := re.FindAllString(nums[1], -1)

		value := 0
		// (Here i could have sorted and used a binary search to get better performance)
		for _, w := range winningNumbers {
			for _, n := range numbersYouHave {
				if w == n {
					if value > 0 {
						value *= 2
					} else {
						value = 1
					}
				}
			}
		}
		sum += value
	}
	return sum
}

func solvePart2(input string) int {
	sum := 0
	totalCards := map[int]int{}
	for indexLine, line := range strings.Split(input, "\n") {
		nums := strings.Split(strings.Split(line, ":")[1], "|")

		re := regexp.MustCompile(`\b\d+\b`)
		winningNumbers := re.FindAllString(nums[0], -1)
		numbersYouHave := re.FindAllString(nums[1], -1)

		match := 0
		// (Here i could have sorted and used a binary search to get better performance)
		for _, w := range winningNumbers {
			for _, n := range numbersYouHave {
				if w == n {
					match += 1
				}
			}
		}

		// Use the number of matches to add extra cards to the next
		// using the numbers of current cards as value
		cardNumber := totalCards[indexLine] + 1
		for i := 1; i <= match; i++ {
			totalCards[indexLine+i] += cardNumber
		}
		sum += cardNumber
	}
	return sum
}

func main() {
	i, _ := os.ReadFile("input.txt")
	input := string(i)

	fmt.Println("Part 1:", solvePart1(input))
	fmt.Println("Part 2:", solvePart2(input))
}
