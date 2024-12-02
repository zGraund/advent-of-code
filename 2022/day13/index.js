import { readFileSync } from "fs";

const data = readFileSync("./input.txt", "utf8");

/**
 * @param {string} data
 * @returns {void}
 **/
function part1(data) {
	let res = 0;

	const compare = (first, second) => {
		for (let i = 0; i < Math.min(first.length, second.length); i++) {
			const f = first[i];
			const s = second[i];
			if (typeof f === "number" && typeof s === "number") {
				if (f === s) continue;
				if (f < s) {
					return 1;
				}
				if (f > s) return 0;
			}
			if (Array.isArray(f) && Array.isArray(s)) {
				const t = compare(f, s);
				if (t === -1) continue;
				return t;
			}
			return compare(Array.isArray(f) ? f : [f], Array.isArray(s) ? s : [s]);
		}
		if (first.length === second.length) return -1;
		return first.length > second.length ? 0 : 1;
	};

	data.split("\n\n").forEach((pair, ind) => {
		const [f, s] = pair.split("\n");
		const first = Array.from(eval(f)),
			second = Array.from(eval(s));

		res += compare(first, second) * (ind + 1);
	});

	console.log(res);
}

/**
 * @param {string} data
 * @returns {void}
 **/
function part2(data) {
	const compare = (first, second) => {
		for (let i = 0; i < Math.min(first.length, second.length); i++) {
			const f = first[i];
			const s = second[i];
			if (typeof f === "number" && typeof s === "number") {
				if (f === s) continue;
				if (f < s) {
					return -1;
				}
				if (f > s) return 1;
			}
			if (Array.isArray(f) && Array.isArray(s)) {
				const t = compare(f, s);
				if (t === 0) continue;
				return t;
			}
			const t = compare(Array.isArray(f) ? f : [f], Array.isArray(s) ? s : [s]);
			if (t === 0) continue;
			return t;
		}
		if (first.length === second.length) return 0;
		return first.length > second.length ? 1 : -1;
	};

	const packets = data.split(/[\n]+/).map((line) => Array.from(eval(line)));
	const dividerPackets = [[[2]], [[6]]];
	packets.push(...dividerPackets);

	packets.sort(compare);

	let res = 1;
	dividerPackets.forEach((divider) => (res *= packets.findIndex((packet) => JSON.stringify(divider) === JSON.stringify(packet)) + 1));

	console.log(res);
}

part1(data);
part2(data);
