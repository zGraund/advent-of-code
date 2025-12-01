import { readFileSync } from "fs";

const data = readFileSync("./input.txt", "utf8");

/**
 * @param {string} data
 * @returns {void}
 **/
function part1(data) {
	const scan = new Set();

	// Borders
	let left = Infinity,
		right = -1,
		down = -1;

	data.trim()
		.split("\n")
		.forEach((line) => {
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
					scan.add([start[0], start[1]].join(","));
				}
				scan.add([end[0], end[1]].join(","));
			}
		});

	let res = 0;
	let sand = [500, -1];
	while (true) {
		if (sand[0] < left || sand[0] > right || sand[1] > down) {
			break;
		}
		if (!scan.has([sand[0], sand[1] + 1].join(","))) {
			// Move down left if empty
			sand[1]++;
			continue;
		} else if (!scan.has([sand[0] - 1, sand[1] + 1].join(","))) {
			// Move diagonal left if empty
			sand[0]--;
			sand[1]++;
			continue;
		} else if (!scan.has([sand[0] + 1, sand[1] + 1].join(","))) {
			// Move diagonal right if empty
			sand[0]++;
			sand[1]++;
			continue;
		}
		scan.add(sand.join(","));
		res++;
		sand = [500, -1];
	}

	console.log(res);
}

/**
 * @param {string} data
 * @returns {void}
 **/
function part2(data) {
	const scan = new Set();

	// Borders
	let lastFreeSpace = -1;

	data.trim()
		.split("\n")
		.forEach((line) => {
			const coords = line.split(" -> ").map((coord) => {
				const [x, y] = coord.split(",");
				lastFreeSpace = Math.max(lastFreeSpace, y);
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
					scan.add([start[0], start[1]].join(","));
				}
				scan.add([end[0], end[1]].join(","));
			}
		});

	lastFreeSpace = lastFreeSpace + 1;
	let res = 0;
	let sand = [500, -1];
	while (true) {
		if (scan.has("500,0")) {
			break;
		}
		if (!scan.has([sand[0], sand[1] + 1].join(",")) && sand[1] < lastFreeSpace) {
			// Move down if empty
			sand[1]++;
			continue;
		} else if (!scan.has([sand[0] - 1, sand[1] + 1].join(",")) && sand[1] < lastFreeSpace) {
			// Move diagonal left if empty
			sand[0]--;
			sand[1]++;
			continue;
		} else if (!scan.has([sand[0] + 1, sand[1] + 1].join(",")) && sand[1] < lastFreeSpace) {
			// Move diagonal right if empty
			sand[0]++;
			sand[1]++;
			continue;
		}
		scan.add(sand.join(","));
		res++;
		sand = [500, -1];
	}

	console.log(res);
}

console.time("Part 1 time");
part1(data);
console.timeEnd("Part 1 time");
console.time("Part 2 time");
part2(data);
console.timeEnd("Part 2 time");
