use std::{collections::HashSet, fs::read_to_string, time::Instant};

fn part1(data: &str) -> usize {
    // NOTE: the robots are represented as [col, row, d_col, d_row]
    let mut robots: Vec<Vec<isize>> = data
        .trim()
        .lines()
        .map(|line| {
            line.split(" ")
                .flat_map(|s| s.split_at(2).1.split(",").map(|v| v.parse().unwrap()))
                .collect()
        })
        .collect();

    // NOTE: uncomment the right len

    // test
    // let row_len = 7;
    // let col_len = 11;

    // real input
    let row_len = 103;
    let col_len = 101;

    let assign_quadrant = |row: isize, col: isize| -> Option<usize> {
        let r = row_len / 2;
        let c = col_len / 2;
        if row == r || col == c {
            return None;
        }
        if row < r && col < c {
            Some(0)
        } else if row < r && col > c {
            Some(1)
        } else if row > r && col < c {
            Some(2)
        } else {
            Some(3)
        }
    };

    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot[0] = (robot[0] + robot[2]).rem_euclid(col_len);
            robot[1] = (robot[1] + robot[3]).rem_euclid(row_len);
        }
    }

    let mut res: [usize; 4] = [0; 4];
    for robot in robots.iter() {
        match assign_quadrant(robot[1], robot[0]) {
            Some(n) => res[n] += 1,
            None => (),
        }
    }

    return res.iter().product();
}

fn part2(data: &str) -> usize {
    // NOTE: the robots are represented as [col, row, d_col, d_row]
    let mut robots: Vec<Vec<isize>> = data
        .trim()
        .lines()
        .map(|line| {
            line.split(" ")
                .flat_map(|s| s.split_at(2).1.split(",").map(|v| v.parse().unwrap()))
                .collect()
        })
        .collect();

    const ROW_LEN: usize = 103;
    const COL_LEN: usize = 101;

    #[allow(dead_code)]
    fn join_map(map: &Vec<Vec<isize>>) -> String {
        let mut visual: [[&str; COL_LEN]; ROW_LEN] = [[" "; COL_LEN]; ROW_LEN];

        for robot in map.iter() {
            visual[robot[1] as usize][robot[0] as usize] = "X";
        }

        visual
            .iter()
            .map(|l| l.join(" "))
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn overlap(map: &Vec<Vec<isize>>) -> bool {
        let mut seen: HashSet<[isize; 2]> = HashSet::new();
        for r in map.iter() {
            let k = [r[0], r[1]];
            if seen.contains(&k) {
                return true;
            }
            seen.insert(k);
        }
        false
    }

    let mut i = 0;
    while overlap(&robots) {
        for robot in robots.iter_mut() {
            robot[0] = (robot[0] + robot[2]).rem_euclid(COL_LEN as isize);
            robot[1] = (robot[1] + robot[3]).rem_euclid(ROW_LEN as isize);
        }
        i += 1;
    }

    // Uncomment to see the tree!
    // println!("{}", join_map(&robots));

    return i;
}

fn main() {
    let data = read_to_string("rsc/input14.txt").expect("Input file not found!");

    let p1 = Instant::now();
    println!("Part 1 solution: {}", part1(&data));
    println!(" ∟ Elapsed time: {:.2?}", p1.elapsed());

    let p2 = Instant::now();
    println!("Part 2 solution: {}", part2(&data));
    println!(" ∟ Elapsed time: {:.2?}", p2.elapsed());
}
