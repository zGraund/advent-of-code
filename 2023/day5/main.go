package main

import (
	"fmt"
	"os"
	"regexp"
	"sort"
	"strconv"
	"strings"
	"sync"
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

func sortMatrix(matrix [][]int) [][]int {
	sort.SliceStable(matrix, func(i, j int) bool {
		return matrix[i][1] < matrix[j][1]
	})
	return matrix
}

func parseInput(input string) ([]int, [][][]int) {

	/*
		Get 2 slices
		1 with all the seeds
		1 with all the maps as a 2d array
		convert the string numbers to int
		and sort the matrix based on the source number
	*/

	var seeds []int
	var maps [][][]int

	t := strings.Split(input, ":")
	seeds = getNumbers(t[1])
	for _, v := range t[2:] {
		var matrix [][]int
		for _, line := range strings.Split(v, "\n") {
			if numbers := getNumbers(line); len(numbers) > 0 {
				matrix = append(matrix, numbers)
			}
		}
		maps = append(maps, sortMatrix(matrix))
	}
	return seeds, maps
}

func findDestinationNumber(n int, m [][]int) int {

	/*
		Since the map is sorted we can loop over it
		and find the range that contain the source
		if there is no valid range return the source
		if the range is found we can do a bit of math
		to find the destination number
	*/

	var rang []int
	for i, v := range m {
		if v[1] > n {
			if i == 0 {
				return n
			}
			rang = m[i-1]
			break
		}
		if i == len(m)-1 {
			rang = v
			break
		}
	}
	if rang[1]+rang[2] > n {
		return rang[0] + (n - rang[1])
	}
	return n
}

func solvePart1(seeds []int, maps [][][]int) int {

	/*
		Iterate over every seed and
		found the final destination,
		this should not be resource
		intensive since the maps are
		sorted at the start once

		Detailed explanation in each
		function
	*/

	result := 999999999
	for _, source := range seeds {
		for _, m := range maps {
			source = findDestinationNumber(source, m)
		}
		if result > source {
			result = source
		}
	}
	return result
}

func solvePart2(seeds []int, maps [][][]int) int {

	/*
		Same thing as part one but
		this time we iterate over the
		seeds to get the full list

		THIS IS A VERY BAD BRUTEFORCE SOLUTION
		~6min 30s to complete on an i5-4690
	*/

	result := 999999999
	for i := 0; i < len(seeds); i += 2 {
		for x := 0; x < seeds[i+1]; x++ {
			source := seeds[i] + x
			for _, m := range maps {
				source = findDestinationNumber(source, m)
			}
			if result > source {
				result = source
			}
		}
	}
	return result
}

// Solution 2 using go routines

func mapSeed(seed int, range_ int, maps [][][]int, wg *sync.WaitGroup, ch chan int) {
	defer wg.Done()
	result := 999999999
	for x := 0; x < range_; x++ {
		source := seed + x
		for _, m := range maps {
			source = findDestinationNumber(source, m)
		}
		if result > source {
			result = source
		}
	}
	ch <- result
}

func solvePart2GoRoutine(seeds []int, maps [][][]int) int {

	/*
		Same thing as part one but
		this time we iterate over the
		seeds to get the full list

		Still brute force but with routines
		~3min 15s to complete on an i5-4690
	*/

	ch := make(chan int)
	var wg sync.WaitGroup

	for i := 0; i < len(seeds); i += 2 {
		wg.Add(1)
		go mapSeed(seeds[i], seeds[i+1], maps, &wg, ch)
	}

	go func() {
		wg.Wait()
		close(ch)
	}()

	smallest := <-ch
	for result := range ch {
		if result < smallest {
			smallest = result
		}
	}
	return smallest
}

func main() {
	i, _ := os.ReadFile("input.txt")
	input := string(i)
	seeds, maps := parseInput(input)

	fmt.Println("Part 1:", solvePart1(seeds, maps))
	// fmt.Println("Part 2:", solvePart2(seeds, maps))
	fmt.Println("Part 2:", solvePart2GoRoutine(seeds, maps))

}
