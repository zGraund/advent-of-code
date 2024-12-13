use std::{fs::read_to_string, isize, time::Instant, u64, usize};

fn part1(data: &str) -> u64 {
    let mut disk_map: Vec<isize> = vec![];
    let mut empty = false;
    let mut id = 0;
    for num in data.trim().chars() {
        let digit = num.to_digit(10).unwrap();
        if !empty {
            disk_map.extend_from_slice(&vec![id; digit as usize]);
            id += 1;
        } else {
            disk_map.extend_from_slice(&vec![-1; digit as usize]);
        }
        empty = !empty
    }

    let (mut l, mut r) = (0, disk_map.len() - 1);
    while l < r {
        if disk_map[l] != -1 {
            l += 1;
            continue;
        };
        if disk_map[r] == -1 {
            r -= 1;
            continue;
        };
        disk_map.swap(l, r);
        l += 1;
        r -= 1;
    }

    return disk_map
        .iter()
        .enumerate()
        .filter_map(|(i, n)| {
            if *n == -1 {
                None
            } else {
                Some((*n * i as isize) as u64)
            }
        })
        .sum();
}

fn part2(data: &str) -> isize {
    // [start, length]
    let mut empty_spaces: Vec<[isize; 2]> = vec![];
    // [start, length, id]
    let mut files: Vec<[isize; 3]> = vec![];

    let mut empty = false;
    let mut id = 0;
    let mut start = 0;
    for num in data.trim().chars() {
        let digit = num.to_digit(10).unwrap() as isize;
        if !empty {
            files.push([start, digit, id]);
            id += 1;
        } else {
            empty_spaces.push([start, digit]);
        }
        start += digit;
        empty = !empty
    }

    let mut res = 0;
    // Iter in reverse over all files
    for f_ind in (0..files.len()).rev() {
        let file_block = files[f_ind];
        let file_len = file_block[1];
        // Iter over all spaces
        for e_ind in 0..empty_spaces.len() {
            let empty_block = empty_spaces[e_ind];
            let empty_len = empty_block[1];
            // If the space is on the right of the file we can
            // break early
            if empty_block[0] >= file_block[0] {
                break;
            }
            // If the empty space is equal or bigger than the file...
            if empty_len >= file_len {
                // Modify the start position of the file...
                files[f_ind][0] = empty_block[0];
                // Reduce the dimension of the empty space...
                empty_spaces[e_ind][0] += file_len;
                empty_spaces[e_ind][1] -= file_len;
                // Break the loop because the file can only move once and
                // to the left
                break;
            }
        }
        // Calculate file checksum
        res += (files[f_ind][0]..files[f_ind][0] + file_len)
            .map(move |i| i * file_block[2])
            .sum::<isize>()
    }

    res
}

fn main() {
    let data = read_to_string("rsc/input09.txt").expect("Input file not found!");

    let p1 = Instant::now();
    println!("Part 1 solution: {}", part1(&data));
    println!(" ∟ Elapsed time: {:.2?}", p1.elapsed());

    let p2 = Instant::now();
    println!("Part 2 solution: {}", part2(&data));
    println!(" ∟ Elapsed time: {:.2?}", p2.elapsed());
}
