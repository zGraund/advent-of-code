use std::{collections::HashMap, fs::read_to_string};

const DIRS: [[isize; 2]; 4] = [[-1, 0], [0, 1], [1, 0], [0, -1]];

fn part1(data: &str) -> isize {
    fn dfs(
        map: &Vec<Vec<isize>>,
        row: isize,
        col: isize,
        peaks: &mut HashMap<[isize; 2], ()>,
    ) -> isize {
        let curr_val = map[row as usize][col as usize];
        if curr_val == 9 && !peaks.contains_key(&[row, col]) {
            peaks.insert([row, col], ());
            return 1;
        }

        let mut res = 0;
        for dir in DIRS {
            let next_row = row + dir[0];
            let next_col = col + dir[1];
            if next_row < 0
                || next_row as usize >= map.len()
                || next_col < 0
                || next_col as usize >= map.len()
            {
                continue;
            }
            let next_val = map[next_row as usize][next_col as usize];
            if curr_val + 1 == next_val {
                res += dfs(map, next_row, next_col, peaks);
            }
        }

        return res;
    }

    let mut trailheads = vec![];
    let map: Vec<Vec<isize>> = data
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, char)| {
                    if char == '0' {
                        trailheads.push([row, col]);
                    }
                    char.to_digit(10).unwrap() as isize
                })
                .collect()
        })
        .collect();

    return trailheads
        .iter()
        .map(|th| dfs(&map, th[0] as isize, th[1] as isize, &mut HashMap::new()))
        .sum::<isize>();
}

fn part2(data: &str) -> isize {
    fn dfs(map: &Vec<Vec<isize>>, row: isize, col: isize) -> isize {
        let curr_val = map[row as usize][col as usize];
        if curr_val == 9 {
            return 1;
        }

        let mut res = 0;
        for dir in DIRS {
            let next_row = row + dir[0];
            let next_col = col + dir[1];
            if next_row < 0
                || next_row as usize >= map.len()
                || next_col < 0
                || next_col as usize >= map.len()
            {
                continue;
            }
            let next_val = map[next_row as usize][next_col as usize];
            if curr_val + 1 == next_val {
                res += dfs(map, next_row, next_col);
            }
        }

        return res;
    }

    let mut trailheads = vec![];
    let map: Vec<Vec<isize>> = data
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, char)| {
                    if char == '0' {
                        trailheads.push([row, col]);
                    }
                    char.to_digit(10).unwrap() as isize
                })
                .collect()
        })
        .collect();

    return trailheads
        .iter()
        .map(|th| dfs(&map, th[0] as isize, th[1] as isize))
        .sum::<isize>();
}

fn main() {
    let data = read_to_string("rsc/input10.txt").expect("Input file not found!");

    println!("Part 1 solution: {}", part1(&data));
    println!("Part 2 solution: {}", part2(&data));
}
