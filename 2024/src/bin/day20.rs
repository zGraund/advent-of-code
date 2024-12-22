use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::read_to_string,
    isize,
    time::Instant,
    usize,
};

const DIRS: [[isize; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];

fn part1(data: &str) -> usize {
    let map: Vec<Vec<char>> = data
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let (mut start, mut end) = ([0, 0], [0, 0]);
    for (row, line) in map.iter().enumerate() {
        for (col, char) in line.iter().enumerate() {
            match char {
                'S' => start = [row as isize, col as isize],
                'E' => end = [row as isize, col as isize],
                _ => continue,
            }
        }
    }

    // Pre-compute the path and save the distance from the end
    // of each node
    let mut path: HashMap<[isize; 2], isize> = HashMap::new();
    let mut curr = end.clone();
    let mut d_to_end: isize = 0;
    loop {
        path.insert(curr, d_to_end);
        if curr == start {
            break;
        }
        d_to_end += 1;
        for dir in DIRS {
            let nrow = curr[0] + dir[0];
            let ncol = curr[1] + dir[1];
            if !path.contains_key(&[nrow, ncol]) && map[nrow as usize][ncol as usize] != '#' {
                curr = [nrow, ncol];
                break;
            }
        }
    }

    // Iterate from the start and check if going through a wall will
    // let us skip a part of the path, if it does we calculate the
    // steps saved: Total - (Current + Remaining)
    let mut res = 0;
    for (node, dist) in path.iter() {
        let d_from_start = d_to_end - dist;
        // This code doesn't account for a situation like this:
        //
        //  >>>>v
        //  ^###.##
        //  ^###...>
        //  ^...###
        //   ##.###
        //     ^
        //
        // and assume that all cheats are straight paths, my input work with that assumption
        // but if your result is wrong you need add another loop to go "into" the wall
        // and check all directions from there in the same way
        for dir in DIRS {
            let nrow = node[0] + dir[0] * 2;
            let ncol = node[1] + dir[1] * 2;
            if nrow >= 0
                && nrow < map.len() as isize
                && ncol >= 0
                && ncol < map.len() as isize
                && path.contains_key(&[nrow, ncol])
            {
                //                                       | +2 since we take 2 steps to cheat a wall
                //                                       V
                let s_saved = d_to_end - (d_from_start + 2 + path.get(&[nrow, ncol]).unwrap());
                if s_saved >= 100 {
                    res += 1;
                }
            }
        }
    }

    return res;
}

fn part2(data: &str) -> usize {
    let map: Vec<Vec<char>> = data
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let (mut start, mut end) = ([0, 0], [0, 0]);
    for (row, line) in map.iter().enumerate() {
        for (col, char) in line.iter().enumerate() {
            match char {
                'S' => start = [row as isize, col as isize],
                'E' => end = [row as isize, col as isize],
                _ => continue,
            }
        }
    }

    // Pre-compute the path and save the distance from the end
    // of each node
    let mut path: HashMap<[isize; 2], isize> = HashMap::new();
    let mut curr = end.clone();
    let mut path_len: isize = 0;
    loop {
        path.insert(curr, path_len);
        if curr == start {
            break;
        }
        path_len += 1;
        for dir in DIRS {
            let nrow = curr[0] + dir[0];
            let ncol = curr[1] + dir[1];
            if !path.contains_key(&[nrow, ncol]) && map[nrow as usize][ncol as usize] != '#' {
                curr = [nrow, ncol];
                break;
            }
        }
    }

    let mut cheats: HashSet<[isize; 4]> = HashSet::new();

    // NOTE: 50 for test 100 for full input
    let cheat_threshold = 100;

    // Simple bfs brute force solution that uses the same principle of the part 1 solution
    for (start, dist) in path.iter() {
        let mut q: VecDeque<[isize; 2]> = VecDeque::new();
        q.push_front(*start);

        let mut visited: HashSet<[isize; 2]> = HashSet::new();
        visited.insert(*start);

        let d_from_start = path_len - dist;
        let mut steps = 0;

        while q.len() > 0 && steps <= 20 {
            for _ in 0..q.len() {
                let current = q.pop_front().unwrap();

                if path.contains_key(&current) {
                    let s_saved = path_len - (d_from_start + steps + path.get(&current).unwrap());
                    if s_saved >= cheat_threshold {
                        cheats.insert([start[0], start[1], current[0], current[1]]);
                    }
                }

                for dir in DIRS {
                    let nrow = current[0] + dir[0];
                    let ncol = current[1] + dir[1];
                    if nrow < 0
                        || nrow >= map.len() as isize
                        || ncol < 0
                        || ncol >= map.len() as isize
                        || visited.contains(&[nrow, ncol])
                    {
                        continue;
                    }
                    visited.insert([nrow, ncol]);
                    q.push_back([nrow, ncol]);
                }
            }
            steps += 1;
        }
    }

    return cheats.len();
}

fn main() {
    let data = read_to_string("rsc/input20.txt").expect("Input file not found!");

    let p1 = Instant::now();
    println!("Part 1 solution: {}", part1(&data));
    println!(" ∟ Elapsed time: {:.2?}", p1.elapsed());

    let p2 = Instant::now();
    println!("Part 2 solution: {}", part2(&data));
    println!(" ∟ Elapsed time: {:.2?}", p2.elapsed());
}
