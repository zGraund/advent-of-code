import { readFileSync } from "fs";

const data = readFileSync("./input.txt", "utf8");

/**
 * @param {string} data
 * @returns {void}
 **/
function part1(data) {
	const scan = new Map();

	// Borders
	let left = Infinity,
		right = -1,
		down = -1;

	data.split("\n").forEach((line) => {
		const coords = line.split(" -> ").map((coord) => {
			const [x, y] = coord.split(",");
			left = Math.min(left, x);
			right = Math.max(right, x);
			down = Math.max(down, y);
			return [parseInt(x), parseInt(y)];
		});
		for (let i = 1; i < coords.length; i++) {
			const start = coords[i - 1];
			const end = coords[i];
			let dirX = 0,
				dirY = 0;
			if (start[0] < end[0]) dirX = 1;
			if (start[0] > end[0]) dirX = -1;
			if (start[1] > end[1]) dirY = -1;
			if (start[1] < end[1]) dirY = 1;
			for (start[0], start[1]; start[0] !== end[0] || start[1] !== end[1]; start[0] += dirX, start[1] += dirY) {
				scan.set([start[0], start[1]].join(","), "#");
			}
			scan.set([end[0], end[1]].join(","), "#");
		}
	});

	let sandInTheAbyss = false;
	while (!sandInTheAbyss) {
		const sand = [500, -1];
		// FIXME: always check if the sand can go down
		while (!scan.has([sand[0], sand[1] + 1].join(","))) {
			sand[1]++;
		}
		if (!scan.has([sand[0] - 1, sand[1] + 1].join(","))) {
			// Set diagonal left if empty
			scan.set([sand[0] - 1, sand[1] + 1].join(","), "o");
		} else if (!scan.has([sand[0] + 1, sand[1] + 1].join(","))) {
			// Set diagonal right if empty
			scan.set([sand[0] + 1, sand[1] + 1].join(","), "o");
		} else {
			// Set center if right and left are blocked
			scan.set(sand.join(","), "o");
		}
		// console.log(sand);
		if (sand[0] < left || sand[0] > right || sand[1] > down) sandInTheAbyss = true;
		console.log(scan);
	}
	console.log(left, right, down);
}

/**
 * @param {string} data
 * @returns {void}
 **/
function part2(data) {
	console.log("Part 2 not implemented yet");
}

part1(data);
part2(data);
