use std::{collections::HashMap, fs::read_to_string, isize};

fn part1(data: &str) -> usize {
    let max_len = (data.find("\n").unwrap()) as isize;

    let mut towers: HashMap<char, Vec<[isize; 2]>> = HashMap::new();
    for (row, line) in data.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char != '.' {
                towers
                    .entry(char)
                    .or_default()
                    .push([row as isize, col as isize]);
            }
        }
    }

    let in_bound = |c: [isize; 2]| c[0] >= 0 && c[0] < max_len && c[1] >= 0 && c[1] < max_len;

    let mut antinodes = HashMap::new();

    for towers_group in towers.values() {
        for (t1_ind, t1) in towers_group.iter().enumerate() {
            for t2 in towers_group.iter().skip(t1_ind + 1) {
                let t1_node = [t1[0] + (t1[0] - t2[0]), t1[1] + (t1[1] - t2[1])];
                let t2_node = [t2[0] + (t2[0] - t1[0]), t2[1] + (t2[1] - t1[1])];

                if in_bound(t1_node) {
                    antinodes.insert(t1_node, ());
                }
                if in_bound(t2_node) {
                    antinodes.insert(t2_node, ());
                }
            }
        }
    }

    return antinodes.len();
}

fn part2(data: &str) -> usize {
    let max_len = (data.find("\n").unwrap()) as isize;

    let mut towers: HashMap<char, Vec<[isize; 2]>> = HashMap::new();
    for (row, line) in data.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char != '.' {
                towers
                    .entry(char)
                    .or_default()
                    .push([row as isize, col as isize]);
            }
        }
    }

    let in_bound = |c: [isize; 2]| c[0] >= 0 && c[0] < max_len && c[1] >= 0 && c[1] < max_len;

    let mut antinodes = HashMap::new();

    for towers_group in towers.values() {
        for (t1_ind, t1) in towers_group.iter().enumerate() {
            for t2 in towers_group.iter().skip(t1_ind + 1) {
                let t1_dif = [t1[0] - t2[0], t1[1] - t2[1]];

                let mut antinode_t1 = [t1[0], t1[1]];
                while in_bound(antinode_t1) {
                    antinodes.insert(antinode_t1, ());
                    antinode_t1[0] += t1_dif[0];
                    antinode_t1[1] += t1_dif[1];
                }

                let t2_dif = [t2[0] - t1[0], t2[1] - t1[1]];

                let mut antinode_t2 = [t2[0], t2[1]];
                while in_bound(antinode_t2) {
                    antinodes.insert(antinode_t2, ());
                    antinode_t2[0] += t2_dif[0];
                    antinode_t2[1] += t2_dif[1];
                }
            }
        }
    }

    return antinodes.len();
}

fn main() {
    let data = read_to_string("rsc/input08.txt").expect("Input file not found!");

    println!("Part 1 solution: {}", part1(&data));
    println!("Part 2 solution: {}", part2(&data));
}
