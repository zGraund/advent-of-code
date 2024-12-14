use std::{collections::HashMap, fs::read_to_string, isize, time::Instant};

fn part1(data: &str) -> usize {
    fn solve(
        ba: &[isize],
        bb: &[isize],
        gx: isize,
        gy: isize,
        cost: usize,
        cache: &mut HashMap<[isize; 2], (bool, usize)>,
    ) -> (bool, usize) {
        let c = cache.get(&[gx, gy]);
        if c.is_some() {
            return *c.unwrap();
        }
        if gx == 0 && gy == 0 {
            return (true, cost);
        }
        let mut res = (false, usize::MAX);
        if gx < 0 || gy < 0 {
            return res;
        }
        let ba_res = solve(ba, bb, gx - ba[0], gy - ba[1], cost + 3, cache);
        let bb_res = solve(ba, bb, gx - bb[0], gy - bb[1], cost + 1, cache);
        if ba_res.0 && bb_res.0 {
            res = (true, ba_res.1.min(bb_res.1))
        } else if ba_res.0 {
            res = (true, ba_res.1)
        } else if bb_res.0 {
            res = (true, bb_res.1)
        };
        cache.insert([gx, gy], res);
        return res;
    }

    let mut res = 0;
    for machine in data.trim().split("\n\n") {
        let mut cache: HashMap<[isize; 2], (bool, usize)> = HashMap::new();
        let line: Vec<isize> = machine
            .lines()
            .flat_map(|l| {
                l.split_once(": ")
                    .unwrap()
                    .1
                    .split(", ")
                    .map(|v| v.split_at(2).1.parse::<isize>().unwrap())
            })
            .collect();

        let button_a = &line[0..2];
        let button_b = &line[2..4];
        let goal = &line[4..];
        if (button_a[0] * 100) + (button_b[0] * 100) < goal[0]
            && (button_a[1] * 100) + (button_b[1] * 100) < goal[1]
        {
            continue;
        }

        // This runs in 2s on my machine and it's obviously the wrong approach
        // but i'm keeping it in, the proper solution is in part 2
        let r = solve(button_a, button_b, goal[0], goal[1], 0, &mut cache);
        if r.0 {
            res += r.1;
        }
    }

    return res;
}

fn part2(data: &str) -> usize {
    let mut res = 0;
    for machine in data.trim().split("\n\n") {
        let line: Vec<f64> = machine
            .lines()
            .flat_map(|l| {
                l.split_once(": ")
                    .unwrap()
                    .1
                    .split(", ")
                    .map(|v| v.split_at(2).1.parse::<f64>().unwrap())
            })
            .collect();

        // Button A
        let ax = line[0];
        let ay = line[1];
        // Button B
        let bx = line[2];
        let by = line[3];
        // Prize
        let gx = line[4] + 10_000_000_000_000.0;
        let gy = line[5] + 10_000_000_000_000.0;

        // We can think of this problem as a mathematical equation with 2 variables
        // A and B are how many times we need to press each button
        // to get to the destination
        // (A * ax) + (B * bx) = gx
        // (A * ay) + (B * by) = gy

        // We can use Cramer's rule to solve for A and B
        // A = ((gx * by) - (gy * bx)) / ((ax * by) - (ay * bx))
        // B = ((ax * gy) - (ay * gx)) / ((ax * by) - (ay * bx))
        let a = ((gx * by) - (gy * bx)) / ((ax * by) - (ay * bx));
        let b = ((ax * gy) - (ay * gx)) / ((ax * by) - (ay * bx));

        // If A or B are not integers there are no combinations of buttons that
        // can reach the prize so we skip them
        if a.fract() != 0.0 || b.fract() != 0.0 {
            continue;
        }

        res += ((a * 3.0) + b) as usize;
    }

    return res;
}

fn main() {
    let data = read_to_string("rsc/input13.txt").expect("Input file not found!");

    let p1 = Instant::now();
    println!("Part 1 solution: {}", part1(&data));
    println!(" ∟ Elapsed time: {:.2?}", p1.elapsed());

    let p2 = Instant::now();
    println!("Part 2 solution: {}", part2(&data));
    println!(" ∟ Elapsed time: {:.2?}", p2.elapsed());
}
