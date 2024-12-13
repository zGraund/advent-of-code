use std::{collections::HashSet, fs::read_to_string, isize, time::Instant, usize};

const DIRS: [[isize; 2]; 4] = [
    [0, 1],  // Right
    [1, 0],  // Down
    [0, -1], // Left
    [-1, 0], // Up
];

fn in_bound(row: isize, col: isize, len: isize) -> bool {
    return row >= 0 && col >= 0 && row < len && col < len;
}

fn part1(data: &str) -> usize {
    let map: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    let mut visited: HashSet<[usize; 2]> = HashSet::new();

    fn bfs(
        row: usize,
        col: usize,
        map: &Vec<Vec<char>>,
        visited: &mut HashSet<[usize; 2]>,
    ) -> (usize, usize) {
        if visited.contains(&[row, col]) {
            return (0, 0);
        }

        let l = map.len() as isize;
        let (mut area, mut perimeter) = (1, 0);

        visited.insert([row, col]);

        for dir in DIRS {
            let n_row = row as isize + dir[0];
            let n_col = col as isize + dir[1];
            if in_bound(n_row, n_col, l) && map[row][col] == map[n_row as usize][n_col as usize] {
                // If the next plot is inside the map and is part of the same
                // region sum the area and perimeter of the next node
                let (a, p) = bfs(n_row as usize, n_col as usize, map, visited);
                area += a;
                perimeter += p;
            } else {
                // If it's outside or it's not part of the same region
                // we can count that direction as a perimeter segment
                perimeter += 1;
            }
        }

        return (area, perimeter);
    }

    map.iter()
        .enumerate()
        .map(|(row, line)| {
            line.iter()
                .enumerate()
                .map(|(col, _)| {
                    let (a, p) = bfs(row, col, &map, &mut visited);
                    a * p
                })
                .sum::<usize>()
        })
        .sum()
}

fn part2(data: &str) -> usize {
    let map: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    let mut visited: HashSet<[usize; 2]> = HashSet::new();

    fn check_corners(row: isize, col: isize, map: &Vec<Vec<char>>) -> usize {
        // It wasn't that hard to come up with this solution
        // but it was kind of a pain to implement
        let l = map.len();
        let curr = map[row as usize][col as usize];
        let mut corners = 0;

        // Take 2 directions at 90 degree angle and check if
        // the current node is a corner
        for i in 0..DIRS.len() {
            let j = (i + 1) % DIRS.len();

            // n1 n3
            // n0 n2
            let node1_row = row + DIRS[i][0];
            let node1_col = col + DIRS[i][1];

            let node2_row = row + DIRS[j][0];
            let node2_col = col + DIRS[j][1];

            let node3_row = row + DIRS[i][0] + DIRS[j][0];
            let node3_col = col + DIRS[i][1] + DIRS[j][1];

            if in_bound(node1_row, node1_col, l as isize)
                && in_bound(node2_row, node2_col, l as isize)
            {
                // If they are both inside the map...
                let node1 = map[node1_row as usize][node1_col as usize];
                let node2 = map[node2_row as usize][node2_col as usize];

                // ...and they are both different from the current node...
                if node1 != curr && node2 != curr {
                    // External corner
                    // 0 0
                    // 1 0
                    corners += 1;
                }

                // ...or they are both equal to the current node...
                if node1 == curr && node2 == curr {
                    // (node3 is in bound since the map is a square)
                    let node3 = map[node3_row as usize][node3_col as usize];
                    // ...end the 3rd node is different...
                    if node3 != curr {
                        // Internal corner
                        // 1 0
                        // 1 1
                        corners += 1;
                    }
                }
            } else if in_bound(node1_row, node1_col, l as isize) {
                // If only node 1 is inside the map...
                let node1 = map[node1_row as usize][node1_col as usize];

                // ...and it's different from current node...
                if node1 != curr {
                    // Corner at edge of map
                    // _ _ _
                    // 1 1 0
                    corners += 1;
                }
            } else if in_bound(node2_row, node2_col, l as isize) {
                // If only node 2 is inside the map...
                let node2 = map[node2_row as usize][node2_col as usize];

                // ...and it's different from current node...
                if node2 != curr {
                    // Corner at edge of map
                    // _ _ _
                    // 1 1 0
                    corners += 1;
                }
            } else {
                // If both nodes are outside of the map...
                // Corner at the adge
                // _ _
                // 1 _
                corners += 1
            };
        }

        return corners;
    }

    fn bfs(
        row: usize,
        col: usize,
        map: &Vec<Vec<char>>,
        visited: &mut HashSet<[usize; 2]>,
    ) -> (usize, usize) {
        if visited.contains(&[row, col]) {
            return (0, 0);
        }

        let l = map.len() as isize;
        let (mut area, mut sides) = (1, check_corners(row as isize, col as isize, map));

        visited.insert([row, col]);

        for dir in DIRS {
            let n_row = row as isize + dir[0];
            let n_col = col as isize + dir[1];
            if in_bound(n_row, n_col, l) && map[row][col] == map[n_row as usize][n_col as usize] {
                let (a, s) = bfs(n_row as usize, n_col as usize, map, visited);
                area += a;
                sides += s;
            }
        }

        return (area, sides);
    }

    map.iter()
        .enumerate()
        .map(|(row, line)| {
            line.iter()
                .enumerate()
                .map(|(col, _)| {
                    let (a, p) = bfs(row, col, &map, &mut visited);
                    a * p
                })
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    let data = read_to_string("rsc/input12.txt").expect("Input file not found!");

    let p1 = Instant::now();
    println!("Part 1 solution: {}", part1(&data));
    println!(" ∟ Elapsed time: {:.2?}", p1.elapsed());

    let p2 = Instant::now();
    println!("Part 2 solution: {}", part2(&data));
    println!(" ∟ Elapsed time: {:.2?}", p2.elapsed());
}
