package main

import (
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

type Hand struct {
	cards string
	bid   string
}

func strToMap(str string) map[string]int {
	m := map[string]int{}
	for _, v := range str {
		m[string(v)] += 1
	}
	return m
}

func strToMapP2(str string) map[string]int {
	m := map[string]int{}
	jCount := 0
	for _, v := range str {
		if string(v) == "J" {
			jCount++
			continue
		}
		m[string(v)] += 1
	}
	if jCount > 0 {
		maxKey, maxValue := "", 0
		for k, v := range m {
			if v > maxValue {
				maxKey, maxValue = k, v
			}
		}
		m[maxKey] += jCount
	}
	return m
}

func sortByType(input string, useJolly bool) [][]Hand {

	/*
		Sort the hand by type by making a map of
		the cards and then checking the map length
		or keys value to find the type:

		len == 1 -> Five of a Kind
		len == 2 -> one key == 1 ? -> y -> Four of a kind
		                              n -> Full house
		len == 3 -> any key == 2 ? -> y -> Two pair
		                              n -> Three of a kind
		len == 4 -> One pair
		len == 5 -> High card
	*/

	var HC, OP, TP, ThreeK, FH, FourK, FiveK []Hand
	for _, line := range strings.Split(input, "\n") {
		b := strings.Split(line, " ")
		hand := Hand{
			cards: b[0],
			bid:   b[1],
		}

		var handMap map[string]int
		if useJolly {
			handMap = strToMapP2(hand.cards)
		} else {
			handMap = strToMap(hand.cards)
		}

	typeSort:
		switch len(handMap) {
		case 1:
			FiveK = append(FiveK, hand)
		case 2:
			for _, v := range handMap {
				if v == 1 {
					FourK = append(FourK, hand)
					break typeSort
				}
			}
			FH = append(FH, hand)
		case 3:
			for _, v := range handMap {
				if v == 2 {
					TP = append(TP, hand)
					break typeSort
				}
			}
			ThreeK = append(ThreeK, hand)
		case 4:
			OP = append(OP, hand)
		case 5:
			HC = append(HC, hand)
		}
	}
	return [][]Hand{HC, OP, TP, ThreeK, FH, FourK, FiveK}
}

func sortByCard(s [][]Hand, useJolly bool) [][]Hand {
	valueMap := map[string]int{
		"A": 14,
		"K": 13,
		"Q": 12,
		"J": 11,
		"T": 10,
		"9": 9,
		"8": 8,
		"7": 7,
		"6": 6,
		"5": 5,
		"4": 4,
		"3": 3,
		"2": 2,
	}
	if useJolly {
		valueMap["J"] = 1
	}
	for _, v := range s {
		sort.SliceStable(v, func(i, j int) bool {
			charIndex := 0
			for index, char := range v[i].cards {
				if string(char) != string(v[j].cards[index]) {
					charIndex = index
					break
				}
			}
			return valueMap[string(v[i].cards[charIndex])] < valueMap[string(v[j].cards[charIndex])]
		})
	}
	return s
}

func solvePart1(input string) int {
	typeSorted := sortByType(input, false)
	fullSorted := sortByCard(typeSorted, false)
	sum := 0
	rank := 1
	for _, hands := range fullSorted {
		for _, hand := range hands {
			if n, err := strconv.Atoi(hand.bid); err == nil {
				sum += rank * n
			}
			rank++
		}
	}
	return sum
}

func solvePart2(input string) int {
	typeSorted := sortByType(input, true)
	fullSorted := sortByCard(typeSorted, true)
	sum := 0
	rank := 1
	for _, hands := range fullSorted {
		for _, hand := range hands {
			if n, err := strconv.Atoi(hand.bid); err == nil {
				sum += rank * n
			}
			rank++
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
