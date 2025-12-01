use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
    time::Instant,
};

fn part1(data: &str) -> usize {
    let (w, g) = data.trim().split_once("\n\n").unwrap();

    let mut wires: HashMap<&str, bool> = w
        .lines()
        .map(|line| {
            let w = line.split_once(": ").unwrap();
            (w.0, w.1 == "1")
        })
        .collect();

    // [wire1, gate_type, wire2, destination]
    let mut gates: VecDeque<Vec<&str>> = g
        .lines()
        .map(|line| line.split(" ").filter(|s| *s != "->").collect())
        .collect();

    let mut res = vec![];
    while let Some(gate) = gates.pop_front() {
        let w1 = wires.get(gate[0]);
        let w2 = wires.get(gate[2]);
        if w1.is_none() || w2.is_none() {
            gates.push_back(gate);
            continue;
        }
        let w1v = *w1.unwrap();
        let w2v = *w2.unwrap();
        let r = match gate[1] {
            "AND" => w1v && w2v,
            "OR" => w1v || w2v,
            _ => w1v != w2v,
        };
        wires.insert(gate[3], r);
        if gate[3].starts_with("z") {
            res.push((gate[3], r));
        }
    }

    res.sort();
    usize::from_str_radix(
        &res.iter()
            .rev()
            .map(|v| (v.1 as u8).to_string())
            .collect::<Vec<String>>()
            .join(""),
        2,
    )
    .unwrap()
}

fn part2(data: &str) -> usize {
    let _ = data;
    return 0;
}

fn main() {
    let data = read_to_string("rsc/input24.txt").expect("Input file not found!");

    let p1 = Instant::now();
    println!("Part 1 solution: {}", part1(&data));
    println!(" ∟ Elapsed time: {:.2?}", p1.elapsed());

    let p2 = Instant::now();
    println!("Part 2 solution: {}", part2(&data));
    println!(" ∟ Elapsed time: {:.2?}", p2.elapsed());
}
