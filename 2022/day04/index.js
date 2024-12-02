import { readFileSync } from "fs";

const data = readFileSync("./input.txt", "utf8");

/**
 * @param {string} data
 * @returns {void}
 **/
function part1(data) {
	let res = 0;

	const fullOverlap = (start1, end1, start2, end2) => {
		return (start1 <= start2 && end1 >= end2) || (start2 <= start1 && end2 >= end1);
	};

	data.split("\n").forEach((line) => {
		const ranges = line.split(/,|-/).map((n) => Number(n));
		if (fullOverlap(...ranges)) {
			res++;
		}
	});

	console.log(res);
}

/**
 * @param {string} data
 * @returns {void}
 **/
function part2(data) {
	let res = 0;

	const overlap = (start1, end1, start2, end2) => {
		return start1 <= end2 && start2 <= end1;
	};

	data.split("\n").forEach((line) => {
		const ranges = line.split(/,|-/).map((n) => Number(n));
		if (overlap(...ranges)) {
			res++;
		}
	});

	console.log(res);
}

part1(data);
part2(data);
