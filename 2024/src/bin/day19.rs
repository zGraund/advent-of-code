use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    time::Instant,
};

fn part1(data: &str) -> usize {
    let (t, combinations) = data.trim().split_once("\n\n").unwrap();
    let towels: HashSet<&str> = t.split(", ").collect();
    let mut c: HashMap<String, bool> = HashMap::new();

    fn check(combo: &str, towels: &HashSet<&str>, c: &mut HashMap<String, bool>) -> bool {
        if combo.len() <= 0 {
            return true;
        }
        let r = c.get(&combo.to_string());
        if r.is_some() {
            return *r.unwrap();
        }
        let mut r = combo.len();
        while r > 0 {
            if towels.contains(&combo[..r]) {
                let res = check(&combo[r..], towels, c);
                c.insert(combo.to_string(), res);
                if res {
                    return true;
                }
            }
            r -= 1;
        }
        false
    }

    combinations
        .lines()
        .map(|l| check(l, &towels, &mut c) as usize)
        .sum()
}

fn part2(data: &str) -> usize {
    let (t, combinations) = data.trim().split_once("\n\n").unwrap();
    let towels: HashSet<&str> = t.split(", ").collect();
    let mut c: HashMap<String, usize> = HashMap::new();

    fn check(combo: &str, towels: &HashSet<&str>, c: &mut HashMap<String, usize>) -> usize {
        if combo.len() <= 0 {
            return 1;
        }
        let r = c.get(&combo.to_string());
        if r.is_some() {
            return *r.unwrap();
        }
        let mut res = 0;
        let mut r = combo.len();
        while r > 0 {
            if towels.contains(&combo[..r]) {
                res += check(&combo[r..], towels, c);
                c.insert(combo.to_string(), res);
            }
            r -= 1;
        }
        res
    }

    combinations
        .lines()
        .map(|l| check(l, &towels, &mut c))
        .sum()
}

fn main() {
    let data = read_to_string("rsc/input19.txt").expect("Input file not found!");

    let p1 = Instant::now();
    println!("Part 1 solution: {}", part1(&data));
    println!(" ∟ Elapsed time: {:.2?}", p1.elapsed());

    let p2 = Instant::now();
    println!("Part 2 solution: {}", part2(&data));
    println!(" ∟ Elapsed time: {:.2?}", p2.elapsed());
}
