package main

import (
	"fmt"
	"math"
	"os"
	"regexp"
	"strings"
)

type Galaxy struct {
	x, y int
}

func intercept(v, x1, x2 int) bool {

	/*
		Check if the empty line is between the 2 points
	*/

	return v < int(math.Max(float64(x1), float64(x2))) && v > int(math.Min(float64(x1), float64(x2)))
}

func invertMatrix(m []string) []string {
	t := []string{}
	for _, line := range m {
		for i, col := range line {
			if len(t) <= i {
				t = append(t, string(col))
			} else {
				t[i] += string(col)
			}
		}
	}
	return t
}

func extraSpaceIndex(input []string) []int {

	/*
		Get the coordinates of the empty columns/rows
	*/

	result := []int{}
	re := regexp.MustCompile(`^\.+$`)
	for i, line := range input {
		if re.MatchString(line) {
			result = append(result, i)
		}
	}
	return result
}

func getExtraSpaceIndexes(input []string) ([]int, []int) {
	y := extraSpaceIndex(input)
	x := extraSpaceIndex(invertMatrix(input))
	return y, x
}

func getGalaxies(input []string) []Galaxy {
	galaxies := []Galaxy{}
	for y, l := range input {
		for x, v := range l {
			if v == '#' {
				galaxies = append(galaxies, Galaxy{x, y})
			}
		}
	}
	return galaxies
}

func calculateResult(galaxies []Galaxy, gapVal int, gapX, gapY []int) int {

	/*
		Loop over every galaxy except the last one and calculate the distance
		with all the next galaxies in the list, then check if the there is
		one or more empty rows/columns between the couple, if there is add
		the number of extra space to the result
	*/

	sum := 0
	for i := 0; i < len(galaxies)-1; i++ {
		g1 := galaxies[i]
		for x := i + 1; x < len(galaxies); x++ {
			g2 := galaxies[x]
			extra := 0
			for _, v := range gapY {
				if intercept(v, g1.y, g2.y) {
					extra += gapVal - 1
				}
			}
			for _, v := range gapX {
				if intercept(v, g1.x, g2.x) {
					extra += gapVal - 1
				}
			}
			tot := int(math.Abs(float64(g2.x-g1.x))) + int(math.Abs(float64(g2.y-g1.y))) + extra
			sum += int(tot)
		}
	}
	return sum
}

/*
	Part 1 and 2 are basically the same, the only difference
	is the number of spaces to add on each empty row/column
*/

func solvePart1(input []string) int {
	emptyRows, emptyColumns := getExtraSpaceIndexes(input)
	galaxies := getGalaxies(input)
	return calculateResult(galaxies, 2, emptyColumns, emptyRows)
}

func solvePart2(input []string) int {
	emptyRows, emptyColumns := getExtraSpaceIndexes(input)
	galaxies := getGalaxies(input)
	return calculateResult(galaxies, 1000000, emptyColumns, emptyRows)
}

func main() {
	i, _ := os.ReadFile("input.txt")
	input := strings.Split(string(i), "\n")

	fmt.Println("Part 1:", solvePart1(input))
	fmt.Println("Part 2:", solvePart2(input))
}
