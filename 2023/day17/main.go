package main

import (
	"container/heap"
	"fmt"
	"os"
	"strconv"
	"strings"
)

/*
	Solution inspired from HyperNeutrino
	https://www.youtube.com/watch?v=2pDSooPLLkI
*/

type Node struct {
	hLoss, row, col, dRow, dCol, sameDir, index int
}

type VisitedNode struct {
	row, col, dRow, dCol, sameDir int
}

// Boilerplate code for the priority queue
// https://pkg.go.dev/container/heap#example-package-PriorityQueue

type PriorityQueue []*Node

func (pq PriorityQueue) Len() int { return len(pq) }

func (pq PriorityQueue) Less(i, j int) bool { return pq[i].hLoss < pq[j].hLoss }

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
	pq[i].index = i
	pq[j].index = j
}

func (pq *PriorityQueue) Push(x any) {
	n := len(*pq)
	item := x.(*Node)
	item.index = n
	*pq = append(*pq, item)
}

func (pq *PriorityQueue) Pop() any {
	old := *pq
	n := len(old)
	item := old[n-1]
	old[n-1] = nil
	item.index = -1
	*pq = old[0 : n-1]
	return item
}

/*
	For this solution we can use Dijkstra's algorithm
	to find the shortest path while respecting the
	problem requirements
*/

func solvePart1(grid [][]int, rowLen, colLen int) int {

	// Create a priority queue and insert the
	// starting node, ordered by heat loss
	pq := make(PriorityQueue, 1)

	pq[0] = &Node{0, 0, 0, 0, 0, 0, 0}
	heap.Init(&pq)

	seen := map[VisitedNode]bool{}
	for {
		node := heap.Pop(&pq).(*Node)

		// Return if we are in the bottom right node
		// this will always be the least amount of heat
		// thanks to the priority queue
		if node.row == rowLen-1 && node.col == colLen-1 {
			return node.hLoss
		}

		// Skip node if it's in the "already seen" map
		if _, ok := seen[VisitedNode{node.row, node.col, node.dRow, node.dCol, node.sameDir}]; ok {
			continue
		}
		seen[VisitedNode{node.row, node.col, node.dRow, node.dCol, node.sameDir}] = true

		// If we haven't move 3 times in the same direction we can check the next
		// node in the same direction, also ignore if we are not moving
		if node.sameDir < 3 && (node.dRow != 0 || node.dCol != 0) {
			nextRow := node.row + node.dRow
			nextCol := node.col + node.dCol

			// Check if we are still inside the grid and
			// push the node into the heap
			if 0 <= nextRow && nextRow < rowLen && 0 <= nextCol && nextCol < colLen {
				heap.Push(&pq,
					&Node{
						hLoss:   node.hLoss + grid[nextRow][nextCol],
						row:     nextRow,
						col:     nextCol,
						dRow:    node.dRow,
						dCol:    node.dCol,
						sameDir: node.sameDir + 1,
					},
				)
			}
		}

		// Regardless of the previous step we can check all adjacent node
		// except the one we came from
		for _, v := range [][]int{{0, 1}, {0, -1}, {1, 0}, {-1, 0}} {
			row, col := v[0], v[1]

			// Ignore the node in the same current direction since
			// we already check it in the previous step, also ignore
			// the node we came from
			if (row != node.dRow || col != node.dCol) && (row != -(node.dRow) || col != -(node.dCol)) {
				nextRow := node.row + row
				nextCol := node.col + col

				// Check if we are still inside the grid
				// push the node into the heap
				if 0 <= nextRow && nextRow < rowLen && 0 <= nextCol && nextCol < colLen {
					heap.Push(&pq,
						&Node{
							hLoss:   node.hLoss + grid[nextRow][nextCol],
							row:     nextRow,
							col:     nextCol,
							dRow:    row,
							dCol:    col,
							sameDir: 1, // Set to 1 since we just changed direction
						},
					)
				}
			}
		}
	}
}

func solvePart2(grid [][]int, rowLen, colLen int) int {

	// Part 2 is like part 1 but with some extra checks
	pq := make(PriorityQueue, 1)

	pq[0] = &Node{0, 0, 0, 0, 0, 0, 0}
	heap.Init(&pq)

	seen := map[VisitedNode]bool{}
	for {
		node := heap.Pop(&pq).(*Node)

		// Here we also check if we traveled at least 4 times
		// in the same direction before stopping
		if node.row == rowLen-1 && node.col == colLen-1 && node.sameDir >= 4 {
			return node.hLoss
		}

		if _, ok := seen[VisitedNode{node.row, node.col, node.dRow, node.dCol, node.sameDir}]; ok {
			continue
		}
		seen[VisitedNode{node.row, node.col, node.dRow, node.dCol, node.sameDir}] = true

		// This time we can move in the same direction 10 times
		if node.sameDir < 10 && (node.dRow != 0 || node.dCol != 0) {
			nextRow := node.row + node.dRow
			nextCol := node.col + node.dCol
			if 0 <= nextRow && nextRow < rowLen && 0 <= nextCol && nextCol < colLen {
				heap.Push(&pq,
					&Node{
						hLoss:   node.hLoss + grid[nextRow][nextCol],
						row:     nextRow,
						col:     nextCol,
						dRow:    node.dRow,
						dCol:    node.dCol,
						sameDir: node.sameDir + 1,
					},
				)
			}
		}

		// Before turning this time we need to check if
		// we moved in the same direction at least 4 times
		if node.sameDir >= 4 || (node.dRow == 0 && node.dCol == 0) {
			for _, v := range [][]int{{0, 1}, {0, -1}, {1, 0}, {-1, 0}} {
				row, col := v[0], v[1]
				if (row != node.dRow || col != node.dCol) && (row != -(node.dRow) || col != -(node.dCol)) {
					nextRow := node.row + row
					nextCol := node.col + col
					if 0 <= nextRow && nextRow < rowLen && 0 <= nextCol && nextCol < colLen {
						heap.Push(&pq,
							&Node{
								hLoss:   node.hLoss + grid[nextRow][nextCol],
								row:     nextRow,
								col:     nextCol,
								dRow:    row,
								dCol:    col,
								sameDir: 1,
							},
						)
					}
				}
			}
		}
	}
}

func main() {
	i, _ := os.ReadFile("input.txt")
	input := strings.Split(string(i), "\n")

	grid := make([][]int, len(input))
	for il, l := range input {
		t := make([]int, len(l))
		for i, c := range l {
			if num, err := strconv.Atoi(string(c)); err == nil {
				t[i] = num
			}
		}
		grid[il] = t
	}
	rowLen, colLen := len(grid), len(grid[0])

	fmt.Println("Part 1:", solvePart1(grid, rowLen, colLen))
	fmt.Println("Part 2:", solvePart2(grid, rowLen, colLen))
}
