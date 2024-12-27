use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
    time::Instant,
};

/**
* Keypad
* +---+---+---+
* | 7 | 8 | 9 |
* +---+---+---+
* | 4 | 5 | 6 |
* +---+---+---+
* | 1 | 2 | 3 |
* +---+---+---+
*     | 0 | A |
*     +---+---+
*
* Robot controlls
*     +---+---+
*     | ^ | A |
* +---+---+---+
* | < | v | > |
* +---+---+---+
*/

const DIRS: [(isize, isize, &str); 4] = [(1, 0, "v"), (-1, 0, "^"), (0, 1, ">"), (0, -1, "<")];

const KEYPAD: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    ['\0', '0', 'A'],
];

const ARROWS: [[char; 3]; 2] = [
    ['\0', '^', 'A'], //
    ['<', 'v', '>'],
];

fn get_paths(map: &[[char; 3]], dests: &str, res: &mut HashMap<String, Vec<String>>) {
    for dest in dests.chars() {
        for (row, line) in map.iter().enumerate() {
            for (col, char) in line.iter().enumerate() {
                if *char == '\0' {
                    continue;
                }
                let mut min_len = usize::MAX;
                let mut q = VecDeque::new();
                q.push_back((row as isize, col as isize, "".to_owned()));
                // Simple BFS without a visited map but a min len as a break condition
                while q.len() > 0 {
                    let (r, c, p) = q.pop_front().unwrap();
                    if p.len() > min_len {
                        break;
                    }
                    if map[r as usize][c as usize] == dest {
                        min_len = p.len();
                        res.entry(String::from_iter([*char, dest]))
                            .or_default()
                            .push(p.clone() + "A");
                    }
                    for dir in DIRS {
                        let (n_row, n_col) = (r as isize + dir.0, c as isize + dir.1);
                        if n_row < 0
                            || n_row >= map.len() as isize
                            || n_col < 0
                            || n_col >= line.len() as isize
                            || map[n_row as usize][n_col as usize] == '\0'
                        {
                            continue;
                        }
                        q.push_back((n_row, n_col, p.clone() + dir.2));
                    }
                }
            }
        }
    }
}

fn part1(data: &str) -> usize {
    // Pre compute all paths from an arrow to the all others
    // the key is a String of len 2 that is from x to y:
    // "A<" -> from A to <
    let mut paths: HashMap<String, Vec<String>> = HashMap::new();
    get_paths(&ARROWS, "<v>^A", &mut paths);

    fn solve(moves: &str, paths: &HashMap<String, Vec<String>>, depth: usize) -> usize {
        if depth == 1 {
            return moves.len();
        };

        let mut res = 0;
        let mut pos = 'A';
        for mv in moves.chars() {
            // Check all paths from current position to destination and find the shortest one
            res += paths
                .get(&String::from_iter([pos, mv]))
                .unwrap()
                .iter()
                .map(|n_move| solve(n_move, paths, depth - 1))
                .min()
                .unwrap_or(0);

            pos = mv;
        }

        res
    }

    data.trim()
        .lines()
        .map(|line| {
            let mut paths_k: HashMap<String, Vec<String>> = HashMap::new();
            get_paths(&KEYPAD, line, &mut paths_k);
            let mut pos = 'A';
            line.chars()
                .map(|n_pos| {
                    let n_pos_r = paths_k
                        .get(&String::from_iter([pos, n_pos]))
                        .unwrap()
                        .iter()
                        .map(|p| solve(p, &paths, 3))
                        .min()
                        .unwrap();
                    pos = n_pos;
                    n_pos_r
                })
                .sum::<usize>()
                * line[..line.len() - 1].parse::<usize>().unwrap()
        })
        .sum()
}

fn part2(data: &str) -> usize {
    // Pre compute all paths from an arrow to the all others
    // the key is a String of len 2 that is from x to y:
    // "A<" -> from A to <
    let mut paths: HashMap<String, Vec<String>> = HashMap::new();
    get_paths(&ARROWS, "<v>^A", &mut paths);

    fn solve(
        moves: &str,
        paths: &HashMap<String, Vec<String>>,
        depth: usize,
        c: &mut HashMap<(String, usize), usize>,
    ) -> usize {
        if depth == 1 {
            return moves.len();
        };
        let mut res = 0;
        let mut pos = 'A';

        let c_res = c.get(&(String::from(moves), depth));
        if c_res.is_some() {
            return *c_res.unwrap();
        }

        for mv in moves.chars() {
            // Check all paths from current position to destination and find the shortest one
            res += paths
                .get(&String::from_iter([pos, mv]))
                .unwrap()
                .iter()
                .map(|n_move| solve(n_move, paths, depth - 1, c))
                .min()
                .unwrap_or(0);

            pos = mv;
        }

        c.insert((String::from(moves), depth), res);
        res
    }

    // Same as part 1 but with memoization
    let mut cache = HashMap::new();
    data.trim()
        .lines()
        .map(|line| {
            let mut paths_k: HashMap<String, Vec<String>> = HashMap::new();
            get_paths(&KEYPAD, line, &mut paths_k);
            let mut pos = 'A';
            line.chars()
                .map(|n_pos| {
                    let n_pos_r = paths_k
                        .get(&String::from_iter([pos, n_pos]))
                        .unwrap()
                        .iter()
                        .map(|p| solve(p, &paths, 26, &mut cache))
                        .min()
                        .unwrap();
                    pos = n_pos;
                    n_pos_r
                })
                .sum::<usize>()
                * line[..line.len() - 1].parse::<usize>().unwrap()
        })
        .sum()
}

fn main() {
    let data = read_to_string("rsc/input21.txt").expect("Input file not found!");

    let p1 = Instant::now();
    println!("Part 1 solution: {}", part1(&data));
    println!(" ∟ Elapsed time: {:.2?}", p1.elapsed());

    let p2 = Instant::now();
    println!("Part 2 solution: {}", part2(&data));
    println!(" ∟ Elapsed time: {:.2?}", p2.elapsed());
}
