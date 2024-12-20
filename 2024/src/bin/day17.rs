use core::panic;
use std::{fs::read_to_string, time::Instant, usize};

fn part1(data: &str) -> String {
    let (r, rin) = data.trim().split_once("\n\n").unwrap();

    // A: 0, B: 1, C: 2
    let mut registers: Vec<usize> = r
        .lines()
        .map(|line| line.split_once(": ").unwrap().1.parse().unwrap())
        .collect();

    let instructions: Vec<usize> = rin
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|c| c.parse().unwrap())
        .collect();

    let mut out = vec![];
    let mut i_pointer = 0;
    while i_pointer < instructions.len() {
        let i_operand = i_pointer + 1;
        let literal = instructions[i_operand];
        let opt = instructions[i_pointer];
        let combo = match literal {
            0 | 1 | 2 | 3 => instructions[i_operand],
            4 => registers[0],
            5 => registers[1],
            6 => registers[2],
            _ => panic!("Operand not recognized!"),
        };
        match opt {
            0 => registers[0] /= 2usize.pow(combo as u32),
            1 => registers[1] ^= literal,
            2 => registers[1] = combo % 8,
            3 => {
                if registers[0] != 0 {
                    i_pointer = literal;
                    continue;
                }
            }
            4 => registers[1] ^= registers[2],
            5 => out.push(combo % 8),
            6 => registers[1] = registers[0] / 2usize.pow(combo as u32),
            7 => registers[2] = registers[0] / 2usize.pow(combo as u32),
            _ => panic!("Operation not recognized!"),
        }
        i_pointer += 2;
    }

    out.iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn part2(data: &str) -> usize {
    let rin = data.trim().split_once("\n\n").unwrap().1;

    let instructions: Vec<usize> = rin
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|c| c.parse().unwrap())
        .collect();

    // https://www.youtube.com/watch?v=y-UPxMAh2N8
    assert_eq!(instructions[instructions.len() - 2..], [3, 0]);
    fn solve(target: Vec<usize>, instructions: &Vec<usize>, res: usize) -> Option<usize> {
        if target.len() == 0 {
            return Some(res);
        }
        for i in 0..8 {
            let a = res << 3 | i;
            let mut b = 0;
            let mut c = 0;
            let mut adv3 = false;
            let mut out: Option<usize> = None;

            for p in (0..instructions.len() - 2).step_by(2) {
                let inst = instructions[p];
                let operand = instructions[p + 1];
                let combo = match operand {
                    0 | 1 | 2 | 3 => operand,
                    4 => a,
                    5 => b,
                    6 => c,
                    _ => panic!("Operand not recognized!"),
                };
                match inst {
                    0 => {
                        assert!(!adv3, "More than 1 adv in instructions");
                        assert_eq!(operand, 3, "ADV with non 3 operand");
                        adv3 = true;
                    }
                    1 => b ^= operand,
                    2 => b = combo % 8,
                    3 => panic!("Instruction 3 inside instructions [0..-2]"),
                    4 => b ^= c,
                    5 => {
                        assert_eq!(out, None, "Multiple out in instructions");
                        out = Some(combo % 8);
                    }
                    6 => b = a >> combo,
                    7 => c = a >> combo,
                    _ => panic!("Operation not recognized!"),
                }
                if out.is_some() && out.unwrap() == *target.last().unwrap() {
                    let s = solve(target[..target.len() - 1].to_vec(), instructions, a);
                    if s.is_none() {
                        continue;
                    }
                    return s;
                }
            }
        }
        None
    }

    solve(instructions.clone(), &instructions, 0).unwrap()
}

fn main() {
    let data = read_to_string("rsc/input17.txt").expect("Input file not found!");

    let p1 = Instant::now();
    println!("Part 1 solution: {}", part1(&data));
    println!(" ∟ Elapsed time: {:.2?}", p1.elapsed());

    let p2 = Instant::now();
    println!("Part 2 solution: {}", part2(&data));
    println!(" ∟ Elapsed time: {:.2?}", p2.elapsed());
}
