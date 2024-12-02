use std::{collections::HashMap, fs::read_to_string};

fn part1(data: &str) -> i32 {
    let mut c1: Vec<i32> = vec![];
    let mut c2: Vec<i32> = vec![];

    for line in data.lines() {
        let ids: Vec<&str> = line.split("   ").collect();
        c1.push(ids[0].parse().unwrap());
        c2.push(ids[1].parse().unwrap());
    }

    c1.sort();
    c2.sort();

    let mut sum = 0;
    for (first, second) in c1.iter().zip(c2.iter()) {
        sum += (first - second).abs()
    }

    return sum;
}

fn part2(data: &str) -> i32 {
    let mut occur: HashMap<i32, i32> = HashMap::new();
    let mut list: Vec<i32> = vec![];

    for line in data.lines() {
        let ids: Vec<&str> = line.split("   ").collect();
        *occur.entry(ids[1].parse().unwrap()).or_insert(0) += 1;
        list.push(ids[0].parse().unwrap());
    }

    let mut sum = 0;
    for num in list {
        sum += num * occur.get(&num).unwrap_or(&0);
    }

    return sum;
}

fn main() {
    let data = read_to_string("rsc/input01.txt").expect("Input file not found!");

    println!("Part 1 solution: {}", part1(&data));
    println!("Part 2 solution: {}", part2(&data));
}
