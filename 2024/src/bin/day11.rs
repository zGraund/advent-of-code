use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
};

fn part1(data: &str) -> usize {
    let mut stones = VecDeque::from(
        data.trim()
            .split(" ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
    );

    for _ in 0..25 {
        for _ in 0..stones.len() {
            let stone = stones.pop_front().unwrap();
            if stone == "0" {
                stones.push_back("1".to_string());
            } else if stone.len() % 2 == 0 {
                let (l, r) = stone.split_at(stone.len() / 2);
                let r_new = r.parse::<u64>().unwrap().to_string();
                stones.push_back(l.to_string());
                stones.push_back(r_new);
            } else {
                stones.push_back((stone.parse::<u64>().unwrap() * 2024).to_string());
            };
        }
    }

    return stones.len();
}

fn part2(data: &str) -> usize {
    let stones: Vec<usize> = data
        .trim()
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let mut cache: HashMap<[usize; 2], usize> = HashMap::new();

    fn solve_stone(blink: usize, stone: usize, cache: &mut HashMap<[usize; 2], usize>) -> usize {
        // Essentially this is a DFS on every stone to cache the result
        // it's still a bruteforce but at least it can finish before the thermal
        // death of the universe and without using 900Tb of ram
        if blink == 0 {
            return 1;
        }

        let key = [stone, blink];
        let cached_stone = cache.get(&key);
        if cached_stone.is_some() {
            return *cached_stone.unwrap();
        }

        let res;
        if stone == 0 {
            res = solve_stone(blink - 1, 1, cache);
        } else if (stone.ilog10() + 1) % 2 == 0 {
            let p = 10usize.pow((stone.ilog10() + 1) / 2);
            res =
                solve_stone(blink - 1, stone / p, cache) + solve_stone(blink - 1, stone % p, cache)
        } else {
            res = solve_stone(blink - 1, stone * 2024, cache)
        }

        cache.insert(key, res);
        res
    }

    return stones
        .into_iter()
        .map(|stone| solve_stone(75, stone, &mut cache))
        .sum();
}

fn main() {
    let data = read_to_string("rsc/input11.txt").expect("Input file not found!");

    println!("Part 1 solution: {}", part1(&data));
    println!("Part 2 solution: {}", part2(&data));
}
