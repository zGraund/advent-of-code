import { readFileSync } from "fs";

const data = readFileSync("./input.txt", "utf8");

/**
 * @param {string} data
 * @returns {void}
 **/
function part1(data) {
	// Brute force with BFS is enough no need to use a fancy algorithm

	const grid = data.split("\n");

	let col = -1,
		row = -1;
	for (row = 0; row < grid.length; row++) {
		col = grid[row].search("S");
		if (col !== -1) break;
	}
	const q = [[row, col]];
	const visited = new Set().add(q[0].join(","));
	let res = 0;

	const gridVal = (cords) => {
		const char = grid[cords[0]][cords[1]];
		if (char === "S") return "a".charCodeAt(0);
		if (char === "E") return "z".charCodeAt(0);
		return char.charCodeAt(0);
	};

	const dirs = [
		[0, 1],
		[1, 0],
		[0, -1],
		[-1, 0],
	];

	out: while (q.length > 0) {
		res++;
		for (const curr of Array.from(q)) {
			q.shift();
			if (grid[curr[0]][curr[1]] === "E") {
				break out;
			}
			for (const dir of dirs) {
				const next = [curr[0] + dir[0], curr[1] + dir[1]];
				if (!visited.has(next.join(",")) && next[0] >= 0 && next[0] < grid.length && next[1] >= 0 && next[1] < grid[0].length) {
					if (gridVal(curr) >= gridVal(next) || gridVal(curr) === gridVal(next) - 1) {
						visited.add(next.join(","));
						q.push(next);
					}
				}
			}
		}
	}

	// -1 because in the while loop we are counting the last node E
	console.log(res - 1);
}

/**
 * @param {string} data
 * @returns {void}
 **/
function part2(data) {
	// Even in part 2 a brute force solution is enough, this finishes in ~350 ms on my machine

	const grid = data.split("\n");
	let res = Infinity;

	const starts = [];

	for (let row = 0; row < grid.length; row++) {
		for (let col = 0; col < grid[row].length; col++) {
			const char = grid[row][col];
			if (char === "S" || char === "a") starts.push([row, col]);
		}
	}

	const gridVal = (cords) => {
		const char = grid[cords[0]][cords[1]];
		if (char === "S") return "a".charCodeAt(0);
		if (char === "E") return "z".charCodeAt(0);
		return char.charCodeAt(0);
	};

	const dirs = [
		[0, 1],
		[1, 0],
		[0, -1],
		[-1, 0],
	];

	start: for (const start of starts) {
		const q = [start];
		const visited = new Set().add(q[0].join(","));
		let steps = 0;

		out: while (q.length > 0) {
			steps++;
			for (const curr of Array.from(q)) {
				q.shift();
				if (grid[curr[0]][curr[1]] === "E") {
					res = Math.min(res, steps - 1);
					break out;
				}
				for (const dir of dirs) {
					const next = [curr[0] + dir[0], curr[1] + dir[1]];
					if (!visited.has(next.join(",")) && next[0] >= 0 && next[0] < grid.length && next[1] >= 0 && next[1] < grid[0].length) {
						if (gridVal(curr) >= gridVal(next) || gridVal(curr) === gridVal(next) - 1) {
							visited.add(next.join(","));
							q.push(next);
						}
					}
				}
			}
			if (steps >= res) continue start;
		}
	}

	console.log(res);
}

part1(data);
part2(data);
