import { readFileSync } from "fs";

const data = readFileSync("./input.txt", "utf8");

/**
 * @param {string} data
 * @returns {void}
 **/
function part1(data) {
	let cycle = 0,
		x = 1,
		strength = 0;

	const checkStrength = (cycle, x) => {
		if ((cycle - 20) % 40 === 0) return x * cycle;
		return 0;
	};

	data.split("\n").forEach((line) => {
		const [inst, num] = line.split(" ", 2);
		if (inst === "noop") {
			cycle++;
			strength += checkStrength(cycle, x);
		} else {
			cycle++;
			strength += checkStrength(cycle, x);
			cycle++;
			strength += checkStrength(cycle, x);
			x += parseInt(num);
		}
	});

	console.log(strength);
}

/**
 * @param {string} data
 * @returns {void}
 **/
function part2(data) {
	let sprite = 1,
		res = [];

	const draw = (sprite, res) => {
		if (res.length % 40 >= sprite - 1 && res.length % 40 <= sprite + 1) {
			res.push("#");
		} else {
			// A space makes the result easier to read
			res.push(" ");
		}
	};

	data.split("\n").forEach((line) => {
		const [inst, num] = line.split(" ", 2);
		if (inst === "noop") {
			draw(sprite, res);
		} else {
			draw(sprite, res);
			draw(sprite, res);
			sprite += parseInt(num);
		}
	});

	console.log(res.join("").match(/.{40}/g).join("\n"));
}

part1(data);
part2(data);
