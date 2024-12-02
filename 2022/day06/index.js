import { readFileSync } from "fs";

const data = readFileSync("./input.txt", "utf8");

/*
 *
 *	BRUTEFORCE FTW
 *
 */

/**
 * @param {string} data
 * @returns {void}
 **/
function part1(data) {
	for (let i = 4; i < data.length; i++) {
		const m = new Set(data.substring(i - 4, i));
		if (m.size === 4) {
			console.log(i);
			break;
		}
	}
}

/**
 * @param {string} data
 * @returns {void}
 **/
function part2(data) {
	for (let i = 14; i < data.length; i++) {
		const m = new Set(data.substring(i - 14, i));
		if (m.size === 14) {
			console.log(i);
			break;
		}
	}
}

part1(data);
part2(data);
