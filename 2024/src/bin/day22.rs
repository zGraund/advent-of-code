use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
    time::Instant,
};

fn part1(data: &str) -> usize {
    data.trim()
        .lines()
        .map(|line| {
            let mut sn = line.parse::<usize>().unwrap();
            for _ in 0..2000 {
                sn = (sn ^ (sn * 64)) % 16777216;
                sn = (sn ^ (sn / 32)) % 16777216;
                sn = (sn ^ (sn * 2048)) % 16777216;
            }
            sn
        })
        .sum()
}

fn part2(data: &str) -> isize {
    // A map of all found sequences as keys and the number of bananas we
    // can buy with that sequence as the value
    let mut sequences: HashMap<[isize; 4], isize> = HashMap::new();

    for line in data.trim().lines() {
        let mut sn = line.parse::<isize>().unwrap();
        // All valid sequences for this secret number
        let mut sn_sequences: HashMap<[isize; 4], isize> = HashMap::new();
        // A de-queue to keep track of the last 4 price changes
        // this is basically a sliding window
        let mut sequence = VecDeque::new();
        let mut price = sn % 10;
        for _ in 0..2000 {
            // Calculate next secret number
            sn = (sn ^ (sn * 64)) % 16777216;
            sn = (sn ^ (sn / 32)) % 16777216;
            sn = (sn ^ (sn * 2048)) % 16777216;
            let nprice = sn % 10;
            // Push the price difference in the history
            sequence.push_back(nprice - price);
            price = nprice;
            // Ignore the sequences that are less than 4 since
            // they are invalid
            if sequence.len() < 4 {
                continue;
            }
            if sequence.len() > 4 {
                sequence.pop_front();
            }
            // Make an array from the de-queue to use as a key
            let mut k = [0; 4];
            k.copy_from_slice(&sequence.make_contiguous());
            let seq_val = sn_sequences.get(&k);
            // Only insert one time since the monkey will buy the sequence
            // as soon as it's encountered
            if seq_val.is_none() {
                sn_sequences.insert(k, nprice);
            }
        }
        // Merge the sequence for this secret number into the complete map
        for (k, v) in sn_sequences {
            *sequences.entry(k).or_insert(0) += v;
        }
    }

    // The max value in this map is the answer
    *sequences.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().1
}

fn main() {
    let data = read_to_string("rsc/input22.txt").expect("Input file not found!");

    let p1 = Instant::now();
    println!("Part 1 solution: {}", part1(&data));
    println!(" ∟ Elapsed time: {:.2?}", p1.elapsed());

    let p2 = Instant::now();
    println!("Part 2 solution: {}", part2(&data));
    println!(" ∟ Elapsed time: {:.2?}", p2.elapsed());
}
