use core::f64;
use std::{fs::read_to_string, isize};

fn part1(data: &str) -> isize {
    fn solve(mut nums: Vec<isize>, cur: f64) -> bool {
        if nums.len() <= 0 {
            return cur == 0.0 || cur == 1.0;
        };
        let l = nums.pop().unwrap() as f64;
        return solve(nums.clone(), cur - l) || solve(nums.clone(), cur / l);
    }

    let mut res = 0;
    for line in data.lines() {
        let (t, n) = line.split_once(": ").unwrap();
        let tot = t.parse::<isize>().unwrap();
        let nums: Vec<isize> = n.split(" ").map(|n| n.parse().unwrap()).collect();
        if solve(nums.clone(), tot as f64) {
            res += tot;
        }
    }
    return res;
}

fn part2(data: &str) -> u64 {
    fn solve(nums: &[u64], cur: u64, tot: u64) -> bool {
        if nums.len() <= 0 {
            return cur == tot;
        };
        if cur > tot {
            return false;
        }
        let n = nums[0];
        let nums = &nums[1..];
        let cur_c = cur * 10u64.pow(n.ilog10() + 1) + n;
        return solve(nums, cur + n, tot) || solve(nums, cur * n, tot) || solve(nums, cur_c, tot);
    }

    let mut res = 0;
    for line in data.lines() {
        let (t, n) = line.split_once(": ").unwrap();
        let tot = t.parse::<u64>().unwrap();
        let nums: Vec<u64> = n.split(" ").map(|n| n.parse().unwrap()).collect();
        if solve(&nums[1..], nums[0], tot) {
            res += tot;
        }
    }
    return res;
}

fn main() {
    let data = read_to_string("rsc/input07.txt").expect("Input file not found!");

    println!("Part 1 solution: {}", part1(&data));
    println!("Part 2 solution: {}", part2(&data));
}
