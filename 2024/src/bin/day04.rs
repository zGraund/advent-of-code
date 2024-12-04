use std::{char, fs::read_to_string, usize};

fn part1(data: &str) -> i32 {
    let matrix: Vec<Vec<char>> = data
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let tot_len = matrix.len();

    let dirs = [
        [-1, -1],
        [-1, 0],
        [-1, 1],
        [0, 1],
        [0, -1],
        [1, 1],
        [1, 0],
        [1, -1],
    ];

    fn check_dir(mut row: i32, mut col: i32, dir: [i32; 2], matrix: &Vec<Vec<char>>) -> bool {
        let tot_len = matrix.len() as i32;
        let word = ['M', 'A', 'S'];
        let mut i = 0;
        while i < word.len() {
            row += dir[0];
            col += dir[1];

            if row >= 0 && row < tot_len && col >= 0 && col < tot_len {
                if matrix[row as usize][col as usize] != word[i] {
                    return false;
                }
            } else {
                return false;
            }
            i += 1
        }
        return true;
    }

    let mut res = 0;
    let mut row = 0;
    while row < tot_len {
        let mut col = 0;
        while col < tot_len {
            if matrix[row][col] == 'X' {
                for dir in dirs {
                    if check_dir(row as i32, col as i32, dir, &matrix) {
                        res += 1
                    };
                }
            }
            col += 1
        }
        row += 1
    }

    return res;
}

fn part2(data: &str) -> i32 {
    let matrix: Vec<Vec<char>> = data
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let tot_len = matrix.len();

    fn check_pair(p1: char, p2: char) -> bool {
        return (p1 == 'M' && p2 == 'S') || (p1 == 'S' && p2 == 'M');
    }

    let mut res = 0;

    let mut row = 1;
    while row < tot_len {
        let mut col = 1;
        while col < tot_len {
            if matrix[row][col] == 'A' {
                if row <= 0 || row >= tot_len - 1 || col <= 0 || col >= tot_len - 1 {
                    col += 1;
                    continue;
                }
                let pair1 = (matrix[row - 1][col - 1], matrix[row + 1][col + 1]);
                let pair2 = (matrix[row - 1][col + 1], matrix[row + 1][col - 1]);
                if check_pair(pair1.0, pair1.1) && check_pair(pair2.0, pair2.1) {
                    res += 1
                }
            }
            col += 1
        }
        row += 1
    }

    return res;
}

fn main() {
    let data = read_to_string("rsc/input04.txt").expect("Input file not found!");

    println!("Part 1 solution: {}", part1(&data));
    println!("Part 2 solution: {}", part2(&data));
}
