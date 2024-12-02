use std::fs::read_to_string;

fn part1(data: &str) -> u32 {
    let mut sum = 0;

    'line: for line in data.lines() {
        let report: Vec<i32> = line
            .split_whitespace()
            .map(|c| c.parse::<i32>().unwrap())
            .collect();

        let mut inc = false;
        let mut dec = false;
        for i in 1..report.len() {
            let diff = (report[i - 1] - report[i]).abs();
            if diff < 1 || diff > 3 {
                continue 'line;
            }
            if report[i - 1] > report[i] {
                dec = true
            }
            if report[i - 1] < report[i] {
                inc = true
            }
        }

        if dec != inc {
            sum += 1
        }
    }

    return sum;
}

fn part2(data: &str) -> i32 {
    let mut sum = 0;

    fn is_stable(report: Vec<i32>) -> bool {
        let mut inc = false;
        let mut dec = false;
        for i in 1..report.len() {
            let diff = (report[i - 1] - report[i]).abs();
            if diff < 1 || diff > 3 {
                return false;
            }
            if report[i - 1] > report[i] {
                dec = true
            }
            if report[i - 1] < report[i] {
                inc = true
            }
        }

        if dec != inc {
            return true;
        }
        return false;
    }

    for line in data.lines() {
        let report: Vec<i32> = line
            .split_whitespace()
            .map(|c| c.parse::<i32>().unwrap())
            .collect();

        // Bruteforce solution, i don't like this but i hate DP even more
        for i in 0..report.len() {
            if is_stable([&report[..i], &report[i + 1..]].concat()) {
                sum += 1;
                break;
            }
        }
    }

    return sum;
}

fn main() {
    let data = read_to_string("rsc/input02.txt").expect("Input file not found!");

    println!("Part 1 solution: {}", part1(&data));
    println!("Part 2 solution: {}", part2(&data));
}
