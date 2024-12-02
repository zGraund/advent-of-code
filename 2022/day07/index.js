import { readFileSync } from "fs";

const data = readFileSync("./input.txt", "utf8");

/**
 * @param {string} data
 * @returns {void}
 **/
function part1(data) {
	const wd = [];
	const size = new Map();

	data.split("\n").forEach((line) => {
		if (line.startsWith("$")) {
			const command = line.split(" ");
			// Push the current dir on the stack and join it with the parents to get an "absolute path"
			if (command[1] === "cd") command[2] === ".." ? wd.pop() : wd.push((wd[wd.length - 1] || "") + command[2]);
			if (command[1] === "ls") return;
		} else {
			if (line.startsWith("dir")) return;
			const s = parseInt(line.split(" ")[0]);
			wd.forEach((dir) => size.set(dir, (size.get(dir) || 0) + s));
		}
	});

	let res = 0;
	size.forEach((dir) => {
		if (dir <= 100000) res += dir;
	});

	console.log(res);
}

/**
 * @param {string} data
 * @returns {void}
 **/
function part2(data) {
	const wd = [];
	const size = new Map();

	data.split("\n").forEach((line) => {
		if (line.startsWith("$")) {
			const command = line.split(" ");
			// Push the current dir on the stack and join it with the parents to get an "absolute path"
			if (command[1] === "cd") command[2] === ".." ? wd.pop() : wd.push((wd[wd.length - 1] || "") + command[2]);
			if (command[1] === "ls") return;
		} else {
			if (line.startsWith("dir")) return;
			const s = parseInt(line.split(" ")[0]);
			wd.forEach((dir) => size.set(dir, (size.get(dir) || 0) + s));
		}
	});

	const total = 70000000;
	const needed = 30000000;
	const needToFree = needed - (total - size.get("/"));
	let candidate = total;
	size.forEach((s) => {
		if (s >= needToFree && s < candidate) candidate = s;
	});

	console.log(candidate);
}

part1(data);
part2(data);
