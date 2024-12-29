use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::read_to_string,
    time::Instant,
};

fn part1(data: &str) -> usize {
    let mut connections: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in data.trim().lines() {
        let (l, r) = line.split_once("-").unwrap();
        connections.entry(l).or_default().insert(r);
        connections.entry(r).or_default().insert(l);
    }
    let mut res = HashSet::new();
    for (pc, con) in connections.iter() {
        for (i, pc2) in con.iter().enumerate() {
            for pc3 in con.iter().skip(i + 1) {
                let con2 = connections.get(pc2).unwrap();
                if !con2.contains(pc3) {
                    continue;
                }
                if pc.starts_with("t") || pc2.starts_with("t") || pc3.starts_with("t") {
                    let mut lan = [pc, pc2, pc3];
                    lan.sort();
                    res.insert(lan);
                }
            }
        }
    }
    return res.len();
}

fn part2(data: &str) -> String {
    let mut connections: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in data.trim().lines() {
        let (l, r) = line.split_once("-").unwrap();
        connections.entry(l).or_default().insert(r);
        connections.entry(r).or_default().insert(l);
    }

    let mut res = vec![];
    for (pc, con) in connections.iter() {
        if con.len() < res.len() {
            continue;
        }
        for i in 0..con.len() {
            let mut q = VecDeque::from_iter(con.iter().skip(i));
            let mut line_res: Vec<&str> = vec![pc];
            while q.len() > 0 {
                let pc2 = q.pop_front().unwrap();
                for _ in 0..q.len() {
                    let pc3 = q.pop_front().unwrap();
                    if connections.get(pc2).unwrap().contains(pc3) {
                        q.push_back(pc3);
                    }
                }
                line_res.push(pc2);
            }
            if res.len() < line_res.len() {
                res = line_res
            }
        }
    }

    res.sort();
    res.join(",")
}

fn main() {
    let data = read_to_string("rsc/input23.txt").expect("Input file not found!");

    let p1 = Instant::now();
    println!("Part 1 solution: {}", part1(&data));
    println!(" ∟ Elapsed time: {:.2?}", p1.elapsed());

    let p2 = Instant::now();
    println!("Part 2 solution: {}", part2(&data));
    println!(" ∟ Elapsed time: {:.2?}", p2.elapsed());
}
