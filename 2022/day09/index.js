import { readFileSync } from "fs";

const data = readFileSync("./input.txt", "utf8");

const isTouching = (h, t) => {
	return Math.abs(Math.floor(Math.sqrt(Math.pow(h[0] - t[0], 2) + Math.pow(h[1] - t[1], 2)))) <= 1;
};

const dirMap = {
	R: [0, 1],
	L: [0, -1],
	U: [-1, 0],
	D: [1, 0],
};

/**
 * @param {string} data
 * @returns {void}
 **/
function part1(data) {
	const head = [0, 0];
	const tail = [0, 0];
	const visited = new Set().add("0,0");

	data.split("\n").forEach((line) => {
		const [dir, rep] = line.split(" ", 2);
		for (let i = 0; i < rep; i++) {
			const old = [head[0], head[1]];
			head[0] += dirMap[dir][0];
			head[1] += dirMap[dir][1];
			if (isTouching(head, tail)) continue;
			// the tail always go the the old head position when it moves
			tail[0] = old[0];
			tail[1] = old[1];
			visited.add(`${tail[0]},${tail[1]}`);
		}
	});

	console.log(visited.size);
}

/**
 * @param {string} data
 * @returns {void}
 **/
function part2(data) {
	const rope = [[0, 0]];
	for (let i = 0; i < 9; i++) {
		rope.push([0, 0]);
	}
	const visited = new Set().add("0,0");

	data.split("\n").forEach((line) => {
		const [dir, rep] = line.split(" ", 2);
		for (let i = 0; i < rep; i++) {
			// Move the head
			rope[0][0] += dirMap[dir][0];
			rope[0][1] += dirMap[dir][1];
			// Iterate over all the knots and move according to preceding knot
			for (let j = 1; j < rope.length; j++) {
				const head = rope[j - 1];
				const tail = rope[j];
				if (isTouching(head, tail)) break;
				// Calculate the difference in position based on the 2 knots
				tail[0] += Math.sign(head[0] - tail[0]);
				tail[1] += Math.sign(head[1] - tail[1]);
			}
			visited.add(`${rope[rope.length - 1][0]},${rope[rope.length - 1][1]}`);
		}
	});

	console.log(visited.size);
}

part1(data);
part2(data);
