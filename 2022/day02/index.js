import { readFileSync } from "fs";

const data = readFileSync("./input.txt", "utf8");

/**
 * @param {string} data
 * @returns {void}
 **/
function part1(data) {
	const decode = {
		A: "rock",
		X: "rock",
		B: "paper",
		Y: "paper",
		C: "scissor",
		Z: "scissor",
	};

	const pointPerShape = {
		X: 1, // rock
		Y: 2, // paper
		Z: 3, // scissor
	};

	const winMap = {
		rock: "scissor",
		paper: "rock",
		scissor: "paper",
	};

	let sum = 0;
	const rounds = data.split("\n");
	rounds.forEach((round) => {
		const [opp, me] = round.split(" ", 2);
		sum += pointPerShape[me];
		if (decode[opp] === decode[me]) {
			sum += 3;
		} else if (winMap[decode[me]] === decode[opp]) {
			sum += 6;
		}
	});

	console.log(sum);
}

/**
 * @param {string} data
 * @returns {void}
 **/
function part2(data) {
	const points = {
		A: 1,
		B: 2,
		C: 3,
	};

	const results = {
		// lost, draw, win
		A: ["C", "A", "B"],
		B: ["A", "B", "C"],
		C: ["B", "C", "A"],
	};

	let sum = 0;
	const rounds = data.split("\n");
	rounds.forEach((round) => {
		const [opp, res] = round.split(" ", 2);
		if (res === "X") {
			// we need to lose so find the losing move and add the points
			sum += points[results[opp][0]];
		} else if (res === "Y") {
			// we need to draw so find the draw move and add the points and the draw score
			sum += points[results[opp][1]] + 3;
		} else {
			// we need to win so find the winning move and add the points and the win score
			sum += points[results[opp][2]] + 6;
		}
	});

	console.log(sum);
}

part1(data);
part2(data);
