import { readFileSync } from "fs";

const data = readFileSync("./input.txt", "utf8");

/**
  Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
**/

/**
 * @typedef  {Object}   Monkey
 * @property {number[]} items
 * @property {string[]}	operation
 * @property {number}   divisibleBy
 * @property {number}   isTrue
 * @property {number}   isFalse
 * @property {number}   activity
 */

/**
 * @param {string} data
 * @returns {void}
 **/
function part1(data) {
	/**
	 * @type {Monkey[]}
	 */
	const monkeys = [];

	data.split("\n\n").forEach((monkey) => {
		const att = monkey.split("\n");
		monkeys.push({
			items: att[1].match(/\d+/g).map((n) => parseInt(n)),
			operation: att[2].split(" = ")[1].split(" "),
			divisibleBy: parseInt(att[3].match(/\d+/g)),
			isTrue: parseInt(att[4].match(/\d+/g)),
			isFalse: parseInt(att[5].match(/\d+/g)),
			activity: 0,
		});
	});

	for (let round = 0; round < 20; round++) {
		monkeys.forEach((monkey) => {
			monkey.items.forEach((item) => {
				const left = parseInt(monkey.operation[0]) || item;
				const right = parseInt(monkey.operation[2]) || item;
				const worry = Math.floor((monkey.operation[1] === "*" ? left * right : left + right) / 3);
				if (worry % monkey.divisibleBy === 0) {
					monkeys[monkey.isTrue].items.push(worry);
				} else {
					monkeys[monkey.isFalse].items.push(worry);
				}
				monkey.activity++;
			});
			monkey.items = [];
		});
	}

	monkeys.sort((a, b) => b.activity - a.activity);
	console.log(monkeys[0].activity * monkeys[1].activity);
}

/**
 * @param {string} data
 * @returns {void}
 **/
function part2(data) {
	/**
	 * @type {Monkey[]}
	 */
	const monkeys = [];

	data.split("\n\n").forEach((monkey) => {
		const att = monkey.split("\n");
		monkeys.push({
			items: att[1].match(/\d+/g).map((n) => parseInt(n)),
			operation: att[2].split(" = ")[1].split(" "),
			divisibleBy: parseInt(att[3].match(/\d+/g)),
			isTrue: parseInt(att[4].match(/\d+/g)),
			isFalse: parseInt(att[5].match(/\d+/g)),
			activity: 0,
		});
	});

	let mod = 1;
	monkeys.forEach((monkey) => (mod *= monkey.divisibleBy));

	for (let round = 0; round < 10000; round++) {
		monkeys.forEach((monkey) => {
			monkey.items.forEach((item) => {
				item %= mod;
				const left = parseInt(monkey.operation[0]) || item;
				const right = parseInt(monkey.operation[2]) || item;
				const worry = monkey.operation[1] === "*" ? left * right : left + right;
				if (worry % monkey.divisibleBy === 0) {
					monkeys[monkey.isTrue].items.push(worry);
				} else {
					monkeys[monkey.isFalse].items.push(worry);
				}
				monkey.activity++;
			});
			monkey.items = [];
		});
	}

	monkeys.sort((a, b) => b.activity - a.activity);
	console.log(monkeys[0].activity * monkeys[1].activity);
}

part1(data);
part2(data);
