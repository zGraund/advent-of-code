import { readFileSync } from "fs";

const data = readFileSync("./input.txt", "utf8");

// Convert letters to priority
const lToN = (c) => {
	const code = c.charCodeAt(0);
	if (code >= 97 && code <= 122) {
		// lowercase letters 'a' to 'z'
		return code - 96;
	} else if (code >= 65 && code <= 90) {
		// uppercase letters 'A' to 'Z'
		return code - 38;
	}
};

/**
 * @param {string} data
 * @returns {void}
 **/
function part1(data) {
	let sum = 0;

	data.split("\n").forEach((line) => {
		const comp1 = {};
		const comp2 = {};
		for (let i = 0, j = line.length / 2; i < line.length / 2; i++, j++) {
			const p1 = line.charAt(i);
			const p2 = line.charAt(j);
			comp1[p1] = 1;
			comp2[p2] = 1;
			if (comp1[p2]) {
				sum += lToN(p2);
				break;
			}
			if (comp2[p1]) {
				sum += lToN(p1);
				break;
			}
		}
	});

	console.log(sum);
}

/**
 * @param {string} data
 * @returns {void}
 **/
function part2(data) {
	let sum = 0;

	const lines = data.split("\n");
	for (let i = 2; i < lines.length; i += 3) {
		const elf1 = new Set(lines[i - 2].split(""));
		const elf2 = new Set(lines[i - 1].split(""));
		for (const badge of lines[i]) {
			if (elf1.has(badge) && elf2.has(badge)) {
				sum += lToN(badge);
				break;
			}
		}
	}

	console.log(sum);
}

part1(data);
part2(data);
