use std::{collections::HashMap, fs::read_to_string, isize};

const DIRS: [[isize; 2]; 4] = [
    [-1, 0], // Up
    [0, 1],  // Right
    [1, 0],  // Down
    [0, -1], // Left
];

fn part1(data: &str) -> usize {
    let mut dir = 0;
    let mut pos = [-1, -1];
    let mut max_len = 0;

    let mut obstacles: HashMap<[isize; 2], _> = HashMap::new();
    for (row, line) in data.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char == '#' {
                obstacles.insert([row as isize, col as isize], ());
            }
            if char == '^' {
                pos = [row as isize, col as isize]
            }
        }
        max_len += 1;
    }

    let mut visited: HashMap<[isize; 2], _> = HashMap::new();
    visited.insert(pos, ());
    loop {
        let next_pos = [(pos[0] + DIRS[dir][0]), (pos[1] + DIRS[dir][1])];

        // Out of bound
        if next_pos[0] < 0 || next_pos[0] >= max_len || next_pos[1] < 0 || next_pos[1] >= max_len {
            break;
        }

        if !obstacles.contains_key(&next_pos) {
            pos = next_pos;
            visited.insert(next_pos, ());
        } else {
            dir = (dir + 1) % DIRS.len();
        }
    }

    return visited.len();
}

fn part2(data: &str) -> usize {
    let mut dir = 0;
    let mut pos = [-1, -1];
    let mut max_len = 0;

    let mut obstacles: HashMap<[isize; 2], _> = HashMap::new();
    for (row, line) in data.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char == '#' {
                obstacles.insert([row as isize, col as isize], ());
            }
            if char == '^' {
                pos = [row as isize, col as isize]
            }
        }
        max_len += 1;
    }

    fn is_loop(
        obstacles: &HashMap<[isize; 2], ()>,
        mut visited: HashMap<[isize; 3], ()>,
        mut pos: [isize; 2],
        mut dir: usize,
        max_len: isize,
    ) -> bool {
        loop {
            let next_pos = [(pos[0] + DIRS[dir][0]), (pos[1] + DIRS[dir][1])];

            // Out of bound
            if next_pos[0] < 0
                || next_pos[0] >= max_len
                || next_pos[1] < 0
                || next_pos[1] >= max_len
            {
                return false;
            }

            if visited.contains_key(&[next_pos[0], next_pos[1], dir as isize]) {
                return true;
            }
            if !obstacles.contains_key(&next_pos) {
                pos = next_pos;
                visited.insert([next_pos[0], next_pos[1], dir as isize], ());
            } else {
                dir = (dir + 1) % DIRS.len();
            }
        }
    }

    let mut visited: HashMap<[isize; 3], _> = HashMap::new();
    visited.insert([pos[0], pos[1], dir as isize], ());

    let mut tested: HashMap<[isize; 2], _> = HashMap::new();

    // This is a mess but it's the fastest bruteforce method that i could find
    let mut res = 0;
    let guard_pos = pos;
    loop {
        let next_pos = [(pos[0] + DIRS[dir][0]), (pos[1] + DIRS[dir][1])];

        // Out of bound
        if next_pos[0] < 0 || next_pos[0] >= max_len || next_pos[1] < 0 || next_pos[1] >= max_len {
            break;
        }

        if !obstacles.contains_key(&next_pos) {
            obstacles.insert(next_pos, ());
            if !tested.contains_key(&next_pos)
                && next_pos != guard_pos
                && is_loop(
                    &obstacles,
                    visited.clone(),
                    pos.clone(),
                    dir.clone(),
                    max_len,
                )
            {
                res += 1;
            }
            obstacles.remove(&next_pos);
            tested.insert(next_pos, ());
            pos = next_pos;
            visited.insert([next_pos[0], next_pos[1], dir as isize], ());
        } else {
            dir = (dir + 1) % DIRS.len();
        }
    }

    return res;
}

fn main() {
    let data = read_to_string("rsc/input06.txt").expect("Input file not found!");

    println!("Part 1 solution: {}", part1(&data));
    println!("Part 2 solution: {}", part2(&data));
}
