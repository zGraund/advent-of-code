use std::{collections::HashMap, fs::read_to_string};

// Pretty basic brute force, make a map with all the rules and then
// check if each one is respected
fn part1(data: &str) -> i32 {
    let (rul, updates) = data.split_once("\n\n").unwrap();

    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();

    for rule in rul.lines() {
        let (r1, r2) = rule.split_once("|").unwrap();
        rules
            .entry(r1.parse().unwrap())
            .or_insert(Vec::new())
            .push(r2.parse().unwrap());
    }

    let mut res = 0;
    'line: for line in updates.lines() {
        let update: Vec<i32> = line.split(",").map(|n| n.parse::<i32>().unwrap()).collect();

        for i in 1..update.len() {
            if rules.contains_key(&update[i]) {
                let printed = &update[..i];
                for rule in rules.get(&update[i]).unwrap().iter() {
                    for val in printed {
                        if val == rule {
                            // Rule is broken
                            continue 'line;
                        }
                    }
                }
            }
        }

        res += update[(update.len() - 1) / 2]
    }

    return res;
}

// This is also a relatively simple brute force, make a reverse map of the rules and then
// order the number array by swapping them around
fn part2(data: &str) -> i32 {
    let (rul, updates) = data.split_once("\n\n").unwrap();

    // Since we are gonna order the update by swapping the values that are in front of the current
    // number we are gonna need a reversed rule list meaning that the number in the key goes after
    // he numbers in the value array
    let mut rules_reversed: HashMap<i32, Vec<i32>> = HashMap::new();

    for rule in rul.lines() {
        let (r1, r2) = rule.split_once("|").unwrap();
        rules_reversed
            .entry(r2.parse().unwrap())
            .or_insert(Vec::new())
            .push(r1.parse().unwrap());
    }

    let mut res = 0;
    for line in updates.lines() {
        let mut adjusted = false;
        let mut update: Vec<i32> = line.split(",").map(|n| n.parse::<i32>().unwrap()).collect();

        let mut i = 0;
        // We can skip the last number
        'update: while i < update.len() - 1 {
            if !rules_reversed.contains_key(&update[i]) {
                i += 1;
                continue;
            }
            for rule in rules_reversed.get(&update[i]).unwrap().iter() {
                // Find the first number that break the rule, we are skipping the first i numbers
                let j = update.iter().enumerate().skip(i).find_map(|(ind, n)| {
                    if n == rule {
                        return Some(ind);
                    }
                    None
                });

                if j != None {
                    update.swap(i, j.unwrap());
                    adjusted = true;
                    continue 'update;
                }
            }
            i += 1
        }

        if adjusted {
            res += update[(update.len() - 1) / 2]
        }
    }

    return res;
}

fn main() {
    let data = read_to_string("rsc/input05.txt").expect("Input file not found!");

    println!("Part 1 solution: {}", part1(&data));
    println!("Part 2 solution: {}", part2(&data));
}
