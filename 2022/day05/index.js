import { readFileSync } from "fs";

const data = readFileSync("./input.txt", "utf8");

/**
 * @param {string} data
 * @returns {void}
 **/
function part1(data) {
	const [crt, inst] = data.split("\n\n", 2);

	const lineLength = crt.indexOf("\n") + 1;
	const crates = [[]];
	for (let i = 1; i < crt.length - lineLength; i += 4) {
		// Very reasonable way to keep tract of the right pile lmao
		const pile = crt[crt.length - lineLength + (i % lineLength) + 1];

		if (!crates[pile]) crates[pile] = [];
		if (crt[i] !== " ") crates[pile].unshift(crt[i]);
	}

	inst.split("\n").forEach((line) => {
		const instructions = line.match(/\d+/g);
		for (let i = 0; i < instructions[0]; i++) {
			crates[instructions[2]].push(crates[instructions[1]].pop());
		}
	});

	console.log(crates.map((crate) => crate[crate.length - 1]).join(""));
}

/**
 * @param {string} data
 * @returns {void}
 **/
function part2(data) {
	const [crt, inst] = data.split("\n\n", 2);

	const lineLength = crt.indexOf("\n") + 1;
	const crates = [[]];
	for (let i = 1; i < crt.length - lineLength; i += 4) {
		// Very reasonable way to keep tract of the right pile lmao
		const pile = crt[crt.length - lineLength + (i % lineLength) + 1];

		if (!crates[pile]) crates[pile] = [];
		if (crt[i] !== " ") crates[pile].unshift(crt[i]);
	}

	inst.split("\n").forEach((line) => {
		const instructions = line.match(/\d+/g);
		const toMove = [];
		for (let i = 0; i < instructions[0]; i++) {
			toMove.unshift(crates[instructions[1]].pop());
		}
		crates[instructions[2]].push(...toMove);
	});

	console.log(crates.map((crate) => crate[crate.length - 1]).join(""));
}

part1(data);
part2(data);
