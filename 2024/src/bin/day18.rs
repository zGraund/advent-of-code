use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
    isize,
    time::Instant,
};

const DIRS: [[isize; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];

fn part1(data: &str) -> usize {
    // NOTE: change byte value for corresponding input
    let bytes = 1024;

    let map: HashSet<[isize; 2]> = data
        .trim()
        .lines()
        .take(bytes)
        .map(|l| {
            let c = l.split_once(",").unwrap();
            [c.1.parse().unwrap(), c.0.parse().unwrap()]
        })
        .collect();

    let mut visited: HashSet<[isize; 2]> = HashSet::new();
    let mut q: VecDeque<[isize; 2]> = VecDeque::new();
    q.push_front([0, 0]);

    // NOTE: change the end coordinates
    let end_coord = 70;
    let mut res = 0;
    'out: while q.len() > 0 {
        let n = q.len();
        for _ in 0..n {
            let node = q.pop_front().unwrap();
            if node[0] == end_coord && node[1] == end_coord {
                break 'out;
            }
            for dir in DIRS {
                let nrow = node[0] + dir[0];
                let ncol = node[1] + dir[1];
                if nrow < 0
                    || nrow > end_coord
                    || ncol < 0
                    || ncol > end_coord
                    || visited.contains(&[nrow, ncol])
                    || map.contains(&[nrow, ncol])
                {
                    continue;
                }
                visited.insert([nrow, ncol]);
                q.push_back([nrow, ncol]);
            }
        }
        res += 1;
    }
    res
}

fn part2(data: &str) -> &str {
    // NOTE: change byte value for corresponding input
    let bytes = 1024;

    let lines: Vec<&str> = data.trim().lines().collect();
    let mut map: HashSet<[isize; 2]> = lines
        .iter()
        .take(bytes)
        .map(|l| {
            let c = l.split_once(",").unwrap();
            [c.1.parse().unwrap(), c.0.parse().unwrap()]
        })
        .collect();

    fn check_path(map: &HashSet<[isize; 2]>) -> (bool, HashSet<[isize; 2]>) {
        let mut visited: HashSet<[isize; 2]> = HashSet::new();
        let mut q: VecDeque<[isize; 2]> = VecDeque::new();
        q.push_front([0, 0]);

        let mut found = false;

        // NOTE: change the end coordinates
        let end_coord = 70;
        'out: while q.len() > 0 {
            let n = q.len();
            for _ in 0..n {
                let node = q.pop_front().unwrap();
                if node[0] == end_coord && node[1] == end_coord {
                    found = true;
                    break 'out;
                }
                for dir in DIRS {
                    let nrow = node[0] + dir[0];
                    let ncol = node[1] + dir[1];
                    if nrow < 0
                        || nrow > end_coord
                        || ncol < 0
                        || ncol > end_coord
                        || visited.contains(&[nrow, ncol])
                        || map.contains(&[nrow, ncol])
                    {
                        continue;
                    }
                    visited.insert([nrow, ncol]);
                    q.push_back([nrow, ncol]);
                }
            }
        }
        (found, visited)
    }

    // Very lazy semi brute force solution, find the shortest path and then check
    // if the new byte will fall on the path, if it does try to find a new path,
    // if the new path doesn't exist break.
    // Visited is not exactly only the path nodes but the code runs in ~20s and it's faster
    // that implementing Dijkstra
    let (mut complete, mut visited) = check_path(&map);
    for b in lines.iter().skip(bytes) {
        let c = b.split_once(",").unwrap();
        let nbyte = [c.1.parse().unwrap(), c.0.parse().unwrap()];
        map.insert(nbyte);
        if visited.contains(&nbyte) {
            (complete, visited) = check_path(&map)
        }
        if !complete {
            return b;
        }
    }

    "-,-"
}

fn main() {
    let data = read_to_string("rsc/input18.txt").expect("Input file not found!");

    let p1 = Instant::now();
    println!("Part 1 solution: {}", part1(&data));
    println!(" ∟ Elapsed time: {:.2?}", p1.elapsed());

    let p2 = Instant::now();
    println!("Part 2 solution: {}", part2(&data));
    println!(" ∟ Elapsed time: {:.2?}", p2.elapsed());
}
