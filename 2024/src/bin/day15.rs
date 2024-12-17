use core::panic;
use std::{collections::HashMap, fs::read_to_string, time::Instant};

fn part1(data: &str) -> usize {
    let (raw_map, moves) = data.trim().split_once("\n\n").unwrap();
    let mut robot: [isize; 2] = [0; 2];
    let mut map: HashMap<[isize; 2], char> = raw_map
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(col, char)| match char {
                    '@' => {
                        robot = [row as isize, col as isize];
                        None
                    }
                    '.' => None,
                    _ => Some(([row as isize, col as isize], char)),
                })
                .collect::<Vec<([isize; 2], char)>>()
        })
        .collect();

    let m_len = raw_map.find("\n").unwrap() as isize;

    let ib = |p: [isize; 2]| p[0] >= 0 && p[1] >= 0 && p[0] < m_len && p[1] < m_len;

    for mv in moves.chars() {
        if mv == '\n' {
            continue;
        }
        let dir: [isize; 2] = match mv {
            '^' => [-1, 0],
            '>' => [0, 1],
            'v' => [1, 0],
            '<' => [0, -1],
            _ => panic!("Unrecognize move"),
        };

        let mut next_free = [robot[0], robot[1]];
        let mut pushing = false;
        while ib(next_free) {
            next_free[0] += dir[0];
            next_free[1] += dir[1];
            let c = map.get(&next_free);
            if let Some(c) = c {
                match c {
                    '#' => break,
                    'O' => {
                        pushing = true;
                        continue;
                    }
                    _ => panic!("Unrecognize char in map"),
                }
            } else {
                if pushing {
                    map.remove(&[robot[0] + dir[0], robot[1] + dir[1]]);
                    map.insert(next_free, 'O');
                };
                robot[0] += dir[0];
                robot[1] += dir[1];
                break;
            }
        }
        // display_map(&map, m_len as usize, robot, mv);
    }

    map.iter()
        .filter_map(|(coords, char)| {
            if *char == 'O' {
                Some((coords[0] * 100 + coords[1]) as usize)
            } else {
                None
            }
        })
        .sum()
}

fn part2(data: &str) -> usize {
    let (raw_map, moves) = data.trim().split_once("\n\n").unwrap();
    let mut robot: [isize; 2] = [-1; 2];
    let mut map: Vec<Vec<char>> = raw_map
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .flat_map(|(col, char)| match char {
                    '@' => {
                        robot = [row as isize, (col * 2) as isize];
                        ['@', '.']
                    }
                    'O' => ['[', ']'],
                    _ => [char; 2],
                })
                .collect()
        })
        .collect();

    fn can_move_o(row: isize, col: isize, dc: isize, map: &Vec<Vec<char>>) -> (bool, isize) {
        let n_col = col + dc;
        let n_char = map[row as usize][n_col as usize];
        match n_char {
            '[' | ']' => can_move_o(row, n_col, dc, map),
            '.' => (true, n_col),
            _ => (false, n_col),
        }
    }

    fn can_move_v(
        row: isize,
        col: isize,
        dr: isize,
        map: &Vec<Vec<char>>,
        to_move: &mut HashMap<[isize; 2], char>,
    ) -> bool {
        let n_row = row + dr;
        let n_col = col;
        let n_char = map[n_row as usize][col as usize];
        let char = map[row as usize][col as usize];
        to_move.insert([row, col], char);
        match n_char {
            '[' | ']' => {
                let p = if n_char == '[' { 1 } else { -1 };
                return can_move_v(n_row, n_col, dr, map, to_move)
                    && can_move_v(n_row, n_col + p, dr, map, to_move);
            }
            '.' => true,
            _ => false,
        }
    }

    for mv in moves.chars() {
        if mv == '\n' {
            continue;
        }
        let dir: isize = match mv {
            '^' => -1,
            'v' => 1,
            '>' => 1,
            '<' => -1,
            _ => panic!("Unrecognize move"),
        };
        if mv == '<' || mv == '>' {
            let (can_move, mut dest) = can_move_o(robot[0], robot[1], dir, &map);
            if can_move {
                while dest != robot[1] {
                    map[robot[0] as usize][dest as usize] =
                        map[robot[0] as usize][(dest - dir) as usize];
                    dest -= dir;
                }
                // Remove old robot and update position
                map[robot[0] as usize][robot[1] as usize] = '.';
                robot[1] += dir;
            }
        } else {
            let mut to_move: HashMap<[isize; 2], char> = HashMap::new();
            if can_move_v(robot[0], robot[1], dir, &map, &mut to_move) {
                // First we remove all item that need to be moved...
                for item in to_move.iter() {
                    let dest = item.0;
                    map[dest[0] as usize][dest[1] as usize] = '.';
                }
                // ...then we add them back, otherwise, since they are not ordered
                // new one might delete already placed ones
                for item in to_move {
                    let dest = item.0;
                    let ch = item.1;
                    map[(dest[0] + dir) as usize][dest[1] as usize] = ch;
                }
                // Remove old robot and update position
                map[robot[0] as usize][robot[1] as usize] = '.';
                robot[0] += dir;
            }
        }
    }

    map.iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter().enumerate().filter_map(move |(col, ch)| {
                if *ch == '[' {
                    Some(row * 100 + col)
                } else {
                    None
                }
            })
        })
        .sum::<usize>()
}

fn main() {
    let data = read_to_string("rsc/input15.txt").expect("Input file not found!");

    let p1 = Instant::now();
    println!("Part 1 solution: {}", part1(&data));
    println!(" ∟ Elapsed time: {:.2?}", p1.elapsed());

    let p2 = Instant::now();
    println!("Part 2 solution: {}", part2(&data));
    println!(" ∟ Elapsed time: {:.2?}", p2.elapsed());
}

#[allow(dead_code)]
// Debug function to print the map
fn display_map(map: &HashMap<[isize; 2], char>, len: usize, robot: [isize; 2], m: char) {
    let mut visual: Vec<Vec<char>> = vec![vec!['.'; len]; len];
    // If the robot is not visible something is wrong
    visual[robot[0] as usize][robot[1] as usize] = '@';
    for (k, v) in map.iter() {
        visual[k[0] as usize][k[1] as usize] = *v;
    }
    println!(
        "Move: {}\n{}",
        m,
        visual
            .iter()
            .map(|line| line.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    );
}
