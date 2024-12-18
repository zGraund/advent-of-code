use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    fs::read_to_string,
    isize,
    time::Instant,
    usize,
};

const DIRS: [[isize; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];

fn part1(data: &str) -> usize {
    let mut s: [isize; 2] = [0, 0];
    let map: Vec<Vec<char>> = data
        .trim()
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, char)| match char {
                    'S' => {
                        s = [row as isize, col as isize];
                        char
                    }
                    _ => char,
                })
                .collect()
        })
        .collect();

    let mut visited = HashSet::new();

    // 0     1    2    3        4
    // cost, row, col, dir_row, dir_col
    let mut pq = BinaryHeap::new();
    pq.push(Reverse([0, s[0], s[1], 0, 1]));

    let mut res = 0;
    while pq.len() > 0 {
        let node = pq.pop().unwrap();
        let [cost, row, col, dr, dc] = node.0;

        if map[row as usize][col as usize] == 'E' {
            res = cost as usize;
            break;
        }

        visited.insert([row, col, dr, dc]);

        for dir in DIRS {
            // If direction is opposite of current dir
            if dir[0] == dr * -1 && dir[1] == dc * -1 {
                continue;
            }
            let c_inc = if dr == dir[0] && dc == dir[1] {
                1 // same dir
            } else {
                1001 // any turn
            };
            let nrow = row + dir[0];
            let ncol = col + dir[1];
            let ncost = cost + c_inc;
            if map[nrow as usize][ncol as usize] == '#' {
                continue;
            }
            if visited.contains(&[nrow, ncol, dir[0], dir[1]]) {
                continue;
            }
            pq.push(Reverse([ncost, nrow, ncol, dir[0], dir[1]]));
        }
    }

    res
}

fn part2(data: &str) -> usize {
    let mut s: [isize; 2] = [0, 0];
    let map: Vec<Vec<char>> = data
        .trim()
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, char)| match char {
                    'S' => {
                        s = [row as isize, col as isize];
                        char
                    }
                    _ => char,
                })
                .collect()
        })
        .collect();

    let mut visited = HashSet::new();

    // 0     1    2    3        4        5
    // cost, row, col, dir_row, dir_col, path_len
    let mut pq = BinaryHeap::new();
    pq.push((Reverse([0, s[0], s[1], 0, 1]), vec![[s[0], s[1]]]));

    let mut max_cost = isize::MAX;

    // NOTE: this runs in ~50s on my old pc (i5 4690), a more efficient solution would have been to
    // save all the preceding nodes of each node and then backtrack once Dijkstra finishes but I'm
    // already behind in the AoC challenge so this solution is goon enough for now
    let mut res = HashSet::new();
    while pq.len() > 0 {
        let node = pq.pop().unwrap();
        let [cost, row, col, dr, dc] = node.0 .0;
        let mut path = node.1;

        if map[row as usize][col as usize] == 'E' {
            if cost > max_cost {
                break;
            }
            res.extend(path.clone());
            max_cost = cost;
        }

        visited.insert([row, col, dr, dc]);

        for dir in DIRS {
            // If direction is opposite of current dir
            if dir[0] == dr * -1 && dir[1] == dc * -1 {
                continue;
            }
            let c_inc = if dr == dir[0] && dc == dir[1] {
                1 // same dir
            } else {
                1001 // any turn
            };
            let nrow = row + dir[0];
            let ncol = col + dir[1];
            let ncost = cost + c_inc;
            if map[nrow as usize][ncol as usize] == '#' {
                continue;
            }
            if visited.contains(&[nrow, ncol, dir[0], dir[1]]) {
                continue;
            }
            path.push([row, col]);
            pq.push((Reverse([ncost, nrow, ncol, dir[0], dir[1]]), path.clone()));
        }
    }

    // +1 to add the last node
    res.len() + 1
}

fn main() {
    let data = read_to_string("rsc/input16.txt").expect("Input file not found!");

    let p1 = Instant::now();
    println!("Part 1 solution: {}", part1(&data));
    println!(" ∟ Elapsed time: {:.2?}", p1.elapsed());

    let p2 = Instant::now();
    println!("Part 2 solution: {}", part2(&data));
    println!(" ∟ Elapsed time: {:.2?}", p2.elapsed());
}
