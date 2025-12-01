import { readFileSync } from "fs";

const data = readFileSync("./input.txt", "utf8");
const diagonals = [
	[1, 1],
	[-1, 1],
	[-1, -1],
	[1, -1],
];

/**
 * @param {string} data
 * @returns {void}
 **/
function part1(data) {
	// const lineTarget = 10;
	const lineTarget = 2000000;

	let beacons = new Set(),
		l = 0,
		h = 0;

	data.trim()
		.split("\n")
		.forEach((line) => {
			const coords = line.match(/-?\d+/gm).map((n) => parseInt(n));
			let sensor = [coords[0], coords[1]],
				beacon = [coords[2], coords[3]];

			if (beacon[1] === lineTarget) {
				beacons.add(beacon.join(","));
			}

			// If we calculate the manhattan distance and the distance to the target line
			// we can calculate the column that the sensor reach on that particular row
			const mhDistance = Math.abs(sensor[0] - beacon[0]) + Math.abs(sensor[1] - beacon[1]);
			const dToTarget = Math.abs(sensor[1] - lineTarget);

			l = Math.min(l, sensor[0] - mhDistance + dToTarget);
			h = Math.max(h, sensor[0] + mhDistance - dToTarget);
		});

	console.log(Math.abs(l - h) + 1 - beacons.size);
}

/**
 * @param {string} data
 * @returns {void}
 **/
function part2(data) {
	// const maxCoord = 20;
	const maxCoord = 4000000;

	// [][]int -> [col, row, mhDistance]
	const sensors = [];

	data.trim()
		.split("\n")
		.forEach((line) => {
			const coords = line.match(/-?\d+/gm).map((n) => parseInt(n));
			let sensor = [coords[0], coords[1]],
				beacon = [coords[2], coords[3]];
			const mhDistance = Math.abs(sensor[0] - beacon[0]) + Math.abs(sensor[1] - beacon[1]);
			sensors.push([...sensor, mhDistance]);
		});

	// Look at all points just outside of the sensor perimeter and check if
	// they are inside any other sensor area, if one ot them is not we got the answer
	for (const [i, sensor] of sensors.entries()) {
		const [col, row, dis] = sensor;
		// Upper vertex, just outside of sensor perimeter
		const curr = [col, row - dis - 1];
		// Loop in all 4 directions
		for (const diagonal of diagonals) {
			// Loop mh distance + 2 to get all points outside the perimeter
			for (let n = 0; n < dis + 2; n++) {
				if (curr[0] < 0 || curr[0] > maxCoord || curr[1] < 0 || curr[1] > maxCoord) {
					curr[0] += diagonal[0];
					curr[1] += diagonal[1];
					continue;
				}
				let found = true;
				// Check all other sensors area
				for (const [j, nsensor] of sensors.entries()) {
					if (j === i) {
						continue;
					}
					const mhDist2 = Math.abs(nsensor[0] - curr[0]) + Math.abs(nsensor[1] - curr[1]);
					// If it's inside we can skip this coordinate
					if (mhDist2 <= nsensor[2]) {
						found = false;
						break;
					}
				}
				if (found) {
					console.log(curr[0] * 4000000 + curr[1]);
					return;
				}
				curr[0] += diagonal[0];
				curr[1] += diagonal[1];
			}
		}
	}

	console.log("No valid spot found!");
}

part1(data);
part2(data);
