package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Rule struct {
	key  string
	sym  string
	val  int
	dest string
}

type WorkFlow struct {
	rules []Rule
	def   string
}

func copyMap(m map[string][]int) map[string][]int {
	cp := make(map[string][]int)
	for k, v := range m {
		cp[k] = v
	}
	return cp
}

func (wf WorkFlow) checkCondition(p map[string]int) string {
	for _, r := range wf.rules {
		if r.sym == "<" && p[r.key] < r.val {
			return r.dest
		}
		if r.sym == ">" && p[r.key] > r.val {
			return r.dest
		}
	}
	return wf.def
}

func parseWorkflows(wfs string) map[string]WorkFlow {
	workflows := map[string]WorkFlow{}
	for _, wf := range strings.Split(wfs, "\n") {
		t := strings.Split(wf, "{")
		key := t[0]
		rules := strings.Split(t[1][:len(t[1])-1], ",")

		wfRules := []Rule{}
		for _, r := range rules {
			if strings.Contains(r, ":") {
				temp := strings.Split(r, ":")
				val, _ := strconv.Atoi(temp[0][2:])
				rule := Rule{
					key:  string(temp[0][0]),
					sym:  string(temp[0][1]),
					val:  val,
					dest: temp[1],
				}
				wfRules = append(wfRules, rule)
			}
		}

		workflows[key] = WorkFlow{
			rules: wfRules,
			def:   rules[len(rules)-1],
		}
	}
	return workflows
}

func parseParts(p string) []map[string]int {
	parts := []map[string]int{}
	for _, part := range strings.Split(p, "\n") {
		values := strings.Split(part[1:len(part)-1], ",")
		r := make(map[string]int)
		for _, v := range values {
			t := strings.Split(v, "=")
			if n, err := strconv.Atoi(t[1]); err == nil {
				r[t[0]] = n
			}
		}
		parts = append(parts, r)
	}
	return parts
}

func iterate(p map[string]int, name string, wfs map[string]WorkFlow) int {
	sum := 0
	dest := wfs[name].checkCondition(p)

	if dest == "R" {
		return 0
	}
	if dest == "A" {
		for _, n := range p {
			sum += n
		}
		return sum
	}

	sum += iterate(p, dest, wfs)

	return sum
}

func iterateP2(ranges map[string][]int, name string, wfs map[string]WorkFlow) int {
	if name == "R" {
		return 0
	}
	if name == "A" {
		p := 1
		for _, v := range ranges {
			lo, hi := v[0], v[1]
			p *= hi - lo + 1
		}
		return p
	}

	wf := wfs[name]

	sum := 0
	for _, r := range wf.rules {
		lo, hi := ranges[r.key][0], ranges[r.key][1]
		var rT, rF [2]int
		if r.sym == "<" {
			rT[0], rT[1] = lo, min(r.val-1, hi)
			rF[0], rF[1] = max(r.val, lo), hi
		} else {
			rT[0], rT[1] = max(r.val+1, lo), hi
			rF[0], rF[1] = lo, min(r.val, hi)
		}
		if rT[0] <= rT[1] {
			c := copyMap(ranges)
			c[r.key] = rT[:]
			sum += iterateP2(c, r.dest, wfs)
		}
		if rF[0] <= rF[1] {
			ranges[r.key] = rF[:]
		} else {
			break
		}
	}
	sum += iterateP2(ranges, wf.def, wfs)
	return sum
}

func solvePart1(workflows map[string]WorkFlow, parts []map[string]int) int {

	/*
		Relatively easy solution, parse the parts and workflows
		and then walk them with each part
	*/

	sum := 0
	for _, part := range parts {
		sum += iterate(part, "in", workflows)
	}
	return sum
}

func solvePart2(workflows map[string]WorkFlow) int {

	/*
		Part 2 was more complex, we start with a range of xmas values, walk
		the workflows and split each range in 2 with a "true" and "false" part,
		the true range (if not empty) is the fed back into the same function,
		the false range (if not empty) is used to update the map with all the ranges
		if the workflow is out of rules or no one applied to the range we proceed
		to the next workflow using the default destination
	*/

	sum := 0
	ranges := map[string][]int{
		"x": {1, 4000},
		"m": {1, 4000},
		"a": {1, 4000},
		"s": {1, 4000},
	}
	sum += iterateP2(ranges, "in", workflows)
	return sum
}

func main() {
	i, _ := os.ReadFile("input.txt")
	input := strings.Split(string(i), "\n\n")

	wfs, p := input[0], input[1]

	workflows := parseWorkflows(wfs)
	parts := parseParts(p)

	fmt.Println("Part 1:", solvePart1(workflows, parts))
	fmt.Println("Part 2:", solvePart2(workflows))
}
