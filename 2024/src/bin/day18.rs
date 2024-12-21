use core::panic;
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

fn part2(data: &str) -> String {
    // NOTE: change byte value for corresponding input
    let bytes = 1024;

    let bytes_list: Vec<[isize; 2]> = data
        .trim()
        .lines()
        .map(|b| {
            let c = b.split_once(",").unwrap();
            [c.1.parse().unwrap(), c.0.parse().unwrap()]
        })
        .collect();

    let mut map: HashSet<[isize; 2]> = bytes_list.iter().take(bytes).map(|l| *l).collect();

    fn check_path(map: &HashSet<[isize; 2]>) -> bool {
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
        found
    }

    for b in bytes_list.iter().skip(bytes) {
        map.insert(*b);
    }

    // Very lazy semi brute force solution, fill the map and then iter in reverse
    // (since the right byte is probably lower in the list and if the map is full
    // the bfs is faster) and check each time if we can reach the end, the code
    // runs in ~40ms and it's way faster than implementing Dijkstra
    for b in bytes_list.iter().skip(bytes).rev() {
        map.remove(b);
        if check_path(&map) {
            return format!("{},{}", b[1], b[0]);
        }
    }

    panic!("No node found")
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
