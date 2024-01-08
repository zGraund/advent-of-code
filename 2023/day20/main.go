package main

import (
	"fmt"
	"os"
	"slices"
	"strings"
)

/*
	This is a very lazy and unpolished implementation of a solution
	with a lot of assumption made from other users in the AoC sub-reddit
*/

type Module struct {
	prefix       string
	inputModules map[string]int8
	destinations []string
	status       int8
}

type Destination struct {
	module string
	pulse  int8
}

// Very basic queue implementation
type Queue []Destination

func (q *Queue) Push(d Destination) {
	*q = append(*q, d)
}

func (q *Queue) Pop() (Destination, bool) {
	old := *q
	if len(old) == 0 {
		return Destination{}, false
	}
	*q = old[1:]
	return old[0], true
}

var queue = make(Queue, 0)
var modules = map[string]Module{}
var p2Modules = make([]string, 0) // The modules name that send pulse to the input of rx
var loops = make([]int, 0)        // The cycles of the p2 modules
var count int = 0                 // Number of button presses for p2

func processPulse() (int, int) {

	// This function process a full button press

	hi, lo := 0, 1 // lo start at 1 because it include the button pulse
	count++
	for len(queue) != 0 {
		dest, _ := queue.Pop()
		if dest.pulse == 0 {
			lo += 1
		} else {
			hi += 1
		}
		if module, ok := modules[dest.module]; ok {
			var out int8
			if module.prefix == "%" {
				// if the module is a flip-flop
				if dest.pulse >= 1 {
					// ignore high pulse
					continue
				}
				// switch status and set outbound value
				module.status = 1 - module.status
				modules[dest.module] = module
				out = module.status
			} else {
				// if the module is a conjunction
				for _, inp := range module.inputModules {
					// check if all inputs are high and set outbound value
					if inp == 0 {
						out = 1
						break
					}
					out = 0
				}
			}
			for y, m := range p2Modules {
				// for each of the remaining modules in p2Modules
				// check if they sent a high pulse
				if modules[dest.module].inputModules[m] == 1 {
					// if they did append the button presses to loops
					// and remove the module from p2Modules
					loops = append(loops, count)
					p2Modules = slices.Delete(p2Modules, y, y+1)
				}
			}
			for _, d := range module.destinations {
				// push the new destinations in the queue
				queue.Push(Destination{module: d, pulse: out})
				if m, ok := modules[d]; ok && m.prefix == "&" {
					// update the input for conjunctions modules
					modules[d].inputModules[dest.module] = out
				}
			}
		}
	}
	return hi, lo
}

func solvePart1(d []Destination) int {

	// Brute force approach that simply loop 1000 times

	hi, lo := 0, 0
	for i := 0; i < 1000; i++ {
		// at each iteration restore the queue
		queue = slices.Clone(d)
		h, l := processPulse()
		hi += h
		lo += l
	}
	return hi * lo
}

func solvePart2(d []Destination) int {

	/*
		In this part i did a lot of observations and assumptions:
		o1) there is only 1 module that sent to rx
		o2) the rx input module is a conjunction (iM)
		o3) all the inputs of iM are all flip-flops

		a1) all the iM inputs send a high pulse at the same interval
		a2) the cycles are all prime numbers
	*/

	finalModule := "rx"
	var finalModuleInput string

	for k, m := range modules {
		// find the rx input
		if m.destinations[0] == finalModule {
			finalModuleInput = k
			break
		}

		// Here i should have reset the status of all modules
		// to default but if i do that the final result is wrong
		// no idea why

	}

	// create the array of inputs of finalModuleInput
	for i := range modules[finalModuleInput].inputModules {
		p2Modules = append(p2Modules, i)
	}

	// loop until we found all input modules
	for len(p2Modules) > 0 {
		queue = slices.Clone(d)
		processPulse()
	}

	// since all cycles are prime we ca just multiply them together
	// instead of using the LCM
	sum := 1
	for _, n := range loops {
		sum *= n
	}
	return sum
}

func main() {
	i, _ := os.ReadFile("input.txt")
	input := strings.Split(string(i), "\n")

	// relatively simple input parser
	broadcasterDest := []Destination{}
	for _, line := range input {
		t := strings.Split(line, " -> ")
		if t[0] == "broadcaster" {
			for _, d := range strings.Split(t[1], ", ") {
				broadcasterDest = append(broadcasterDest, Destination{module: d, pulse: 0})
			}
		} else {
			modules[t[0][1:]] = Module{
				status:       0,
				prefix:       string(t[0][0]),
				destinations: strings.Split(t[1], ", "),
				inputModules: make(map[string]int8),
			}
		}
	}

	for k, ff := range modules {
		for _, d := range ff.destinations {
			if v, ok := modules[d]; ok && v.prefix == "&" {
				v.inputModules[k] = 0
			}
		}
	}

	fmt.Println("Part 1:", solvePart1(broadcasterDest))
	fmt.Println("Part 2:", solvePart2(broadcasterDest))
}
