package main

import (
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

func iterate(str string, nums []int, cache map[string]int) int {

	/*
		! This iterative function has been blatantly copied from
		! https://youtu.be/g3Ms5e7Jdqo?si=I9xO-FP4k4dhMjoF

		i have no idea on how to even start solving part 1

		for part 2 since it's basically the same input times 5
		we can just add caching on this same function
		124.594 cache hit in part 2
	*/

	if str == "" {
		if len(nums) == 0 {
			return 1
		} else {
			return 0
		}
	}

	if len(nums) == 0 {
		if strings.Contains(str, "#") {
			return 0
		} else {
			return 1
		}
	}

	key := str
	for _, n := range nums {
		key += strconv.Itoa(n)
	}
	if val, ok := cache[key]; ok {
		cache["hit"]++
		return val
	}

	result := 0

	if str[0] == '.' || str[0] == '?' {
		result += iterate(str[1:], nums, cache)
	}

	if str[0] == '#' || str[0] == '?' {
		if nums[0] <= len(str) && !strings.Contains(str[:nums[0]], ".") && (nums[0] == len(str) || str[nums[0]] != '#') {
			if nums[0]+1 > len(str) {
				result += iterate("", nums[1:], cache)
			} else {
				result += iterate(str[nums[0]+1:], nums[1:], cache)
			}
		}
	}

	cache[key] = result
	return result
}

func unfold(str string, nums []int) (string, []int) {
	slices.Grow(nums, len(nums)*5)
	newStr, newNums := str, nums
	for i := 0; i < 4; i++ {
		newStr += "?" + str
		newNums = append(newNums, nums...)
	}
	return newStr, newNums
}

func solvePart1(input []string) int {
	sum, cHit := 0, 0
	for _, l := range input {
		cache := map[string]int{
			"hit": 0,
		}
		t := strings.Split(l, " ")
		str, nums := t[0], []int{}
		for _, s := range strings.Split(t[1], ",") {
			if n, err := strconv.Atoi(s); err == nil {
				nums = append(nums, n)
			}
		}
		sum += iterate(str, nums, cache)
		cHit += cache["hit"]
	}
	fmt.Println("cache hits:", cHit)
	return sum
}

func solvePart2(input []string) int {
	sum, cHit := 0, 0
	for _, l := range input {
		cache := map[string]int{
			"hit": 0,
		}
		t := strings.Split(l, " ")
		str, nums := t[0], []int{}
		for _, s := range strings.Split(t[1], ",") {
			if n, err := strconv.Atoi(s); err == nil {
				nums = append(nums, n)
			}
		}
		str, nums = unfold(str, nums)
		sum += iterate(str, nums, cache)
		cHit += cache["hit"]
	}
	fmt.Println("cache hits:", cHit)
	return sum
}

func main() {
	i, _ := os.ReadFile("input.txt")
	input := strings.Split(string(i), "\n")

	fmt.Println("Part 1:", solvePart1(input))
	fmt.Println("Part 2:", solvePart2(input))
}
