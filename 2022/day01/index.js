import { readFileSync } from "fs";

const data = readFileSync("./input.txt", "utf8");

/**
 * @param {string} data
 * @returns {void}
 **/
function part1(data) {
	const elfs = data.split("\n\n");
	let max = 0;
	elfs.forEach((elf) => {
		max = Math.max(
			max,
			elf.split("\n").reduce((acc, val) => Number(acc) + Number(val), 0),
		);
	});
	console.log(max);
}

/**
 * @param {string} data
 * @returns {void}
 **/
function part2(data) {
	const elfs = data.split("\n\n");
	const top = [];
	elfs.forEach((elf) => {
		top.push(elf.split("\n").reduce((acc, val) => Number(acc) + Number(val), 0));
		top.sort((a, b) => b - a);
	});
	console.log(top[0] + top[1] + top[2]);
}

part1(data);
part2(data);
