import { readFileSync } from "fs";

const data = readFileSync("./input.txt", "utf8");

/**
 * @param {string} data
 * @returns {void}
 **/
function part1(data) {
	const grid = data.split("\n");
	const visibles = new Set();
	for (let i = 0; i < grid.length; i++) {
		let maxL = -1,
			maxR = -1,
			maxU = -1,
			maxD = -1;
		for (let l = 0, r = grid.length - 1; l < grid.length; l++, r--) {
			const treeL = grid[i][l];
			const treeR = grid[i][r];
			const treeU = grid[l][i];
			const treeD = grid[r][i];
			if (treeL > maxL) visibles.add(`${i},${l}`);
			if (treeR > maxR) visibles.add(`${i},${r}`);
			if (treeU > maxU) visibles.add(`${l},${i}`);
			if (treeD > maxD) visibles.add(`${r},${i}`);
			maxL = Math.max(treeL, maxL);
			maxR = Math.max(treeR, maxR);
			maxD = Math.max(treeD, maxD);
			maxU = Math.max(treeU, maxU);
		}
	}
	console.log(visibles.size);
}

/**
 * @param {string} data
 * @returns {void}
 **/
function part2(data) {
	const grid = data.split("\n");

	// Classic brute force approach, it's faster than thinking about a proper solution
	const checkVisibility = (col, row, colDir, rowDir) => {
		let res = 0;
		const treeVal = grid[col][row];
		for (col += colDir, row += rowDir; col < grid.length && col >= 0 && row < grid.length && row >= 0; col += colDir, row += rowDir) {
			const tree = grid[col][row];
			if (tree >= treeVal) return res + 1;
			res++;
		}
		return res;
	};

	let res = 0;
	for (let row = 0; row < grid.length; row++) {
		for (let col = 0; col < grid.length; col++) {
			const up = checkVisibility(col, row, -1, 0);
			const down = checkVisibility(col, row, 1, 0);
			const right = checkVisibility(col, row, 0, 1);
			const left = checkVisibility(col, row, 0, -1);
			res = Math.max(res, up * down * right * left);
		}
	}
	console.log(res);
}

part1(data);
part2(data);
