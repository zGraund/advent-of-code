use regex::Regex;
use std::fs::read_to_string;

fn part1(data: &str) -> i32 {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut res = 0;

    for line in data.lines() {
        let mult: Vec<(i32, i32)> = regex
            .captures_iter(line)
            .map(|nums| {
                (
                    // YOLO
                    nums[1].parse::<i32>().unwrap(),
                    nums[2].parse::<i32>().unwrap(),
                )
            })
            .collect();

        for nums in mult {
            res += nums.0 * nums.1
        }
    }

    return res;
}

fn part2(data: &str) -> i32 {
    // We can match all 3 operation and toggle a status when we meet a do() or don't(), then
    // process the next multiplications if the status is active
    let regex = Regex::new(r"mul\((\d+),(\d+)\)|don't\(\)|do\(\)").unwrap();
    let mut res = 0;
    let mut active = true;

    for line in data.lines() {
        let mult: Vec<(i32, i32)> = regex
            .captures_iter(line)
            .filter_map(|capt| {
                // Use the expression to toggle the active status and early return
                let exp = capt.get(0).unwrap().as_str();
                if exp == "do()" {
                    active = true;
                    return None;
                }
                if exp == "don't()" {
                    active = false;
                    return None;
                }

                // Here the capture is a mul(...) operation so we can safely parse the groups
                if active {
                    let n1 = capt.get(1).unwrap().as_str().parse::<i32>().unwrap();
                    let n2 = capt.get(2).unwrap().as_str().parse::<i32>().unwrap();
                    return Some((n1, n2));
                }

                return None;
            })
            .collect();

        for nums in &mult {
            res += nums.0 * nums.1
        }
    }

    return res;
}

fn main() {
    let data = read_to_string("rsc/input03.txt").expect("Input file not found!");

    println!("Part 1 solution: {}", part1(&data));
    println!("Part 2 solution: {}", part2(&data));
}
