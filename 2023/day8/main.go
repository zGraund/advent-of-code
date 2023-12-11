package main

import (
	"fmt"
	"os"
	"regexp"
	"strings"
)

type Node struct {
	left  string
	right string
}

func gcd(a, b int) int {
	for b != 0 {
		a, b = b, a%b
	}
	return a
}

func lcm(a, b int, integers ...int) int {
	result := a * b / gcd(a, b)
	for i := 0; i < len(integers); i++ {
		result = lcm(result, integers[i])
	}
	return result
}

func mapNodes(input string) (string, []string, map[string]Node) {
	instructions := strings.Split(input, "\n")[0]
	n := strings.Split(input, "\n")[2:]

	re := regexp.MustCompile(`[A-Z]{3}`)

	var startingNodes []string
	nodes := make(map[string]Node, len(n))
	for _, line := range n {
		p := strings.Split(line, " = ")
		lr := re.FindAllString(p[1], -1)
		nodes[p[0]] = Node{left: lr[0], right: lr[1]}
		if strings.HasSuffix(p[0], "A") {
			startingNodes = append(startingNodes, p[0])
		}
	}
	return instructions, startingNodes, nodes
}

func solvePart1(instructions string, nodes map[string]Node) int {
	index, count := 0, 0
	curNode := "AAA"
	insLen := len(instructions)
	for {
		ins := string(instructions[index])
		if ins == "L" {
			curNode = nodes[curNode].left
		} else {
			curNode = nodes[curNode].right
		}
		count++
		if curNode == "ZZZ" {
			break
		}
		if index == insLen-1 {
			index = 0
		} else {
			index++
		}
	}
	return count
}

func solvePart2(instructions string, startingNodes []string, nodes map[string]Node) int {

	/*
		Part 2 solved using lcm that worked because the loops are closed
		and always the same
	*/

	insLen := len(instructions)

	var ranges []int
	for _, v := range startingNodes {
		index, count := 0, 0
		for {
			ins := string(instructions[index])
			if ins == "L" {
				v = nodes[v].left
			} else {
				v = nodes[v].right
			}
			count++
			if strings.HasSuffix(v, "Z") {
				ranges = append(ranges, count)
				break
			}
			if index == insLen-1 {
				index = 0
			} else {
				index++
			}
		}
	}
	return lcm(ranges[0], ranges[1], ranges[2:]...)
}

func main() {
	i, _ := os.ReadFile("input.txt")
	input := string(i)

	instructions, startingNodes, nodes := mapNodes(input)

	fmt.Println("Part 1:", solvePart1(instructions, nodes))
	fmt.Println("Part 2:", solvePart2(instructions, startingNodes, nodes))
}
