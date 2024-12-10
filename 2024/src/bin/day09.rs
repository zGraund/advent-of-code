use std::{fs::read_to_string, isize, u64, usize};

fn part1(data: &str) -> u64 {
    let mut disk_map: Vec<isize> = vec![];
    let mut empty = false;
    let mut id = 0;
    for num in data.chars() {
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
    // [start, finish, id]
    let mut empty_spaces: Vec<[isize; 3]> = vec![];
    let mut files: Vec<[isize; 3]> = vec![];

    let mut empty = false;
    let mut id = 0;
    let mut start = 0;
    for num in data.chars() {
        let digit = num.to_digit(10).unwrap() as isize;
        let end = start + digit;
        if !empty {
            files.push([start, end, id]);
            id += 1;
        } else {
            empty_spaces.push([start, end, -1]);
        }
        start += digit;
        empty = !empty
    }

    // Iter in reverse over all files
    for f_ind in (0..files.len()).rev() {
        let file_block = files[f_ind];
        let file_len = file_block[1] - file_block[0];
        // Iter over all spaces
        for e_ind in 0..empty_spaces.len() {
            let empty_block = empty_spaces[e_ind];
            let empty_len = empty_block[1] - empty_block[0];
            // If the space is on the right of the file we can
            // break early
            if empty_block[0] >= file_block[0] {
                break;
            }
            // If the empty space is equal or bigger than the file
            if empty_len >= file_len {
                // Modify the start and end position of the file
                files[f_ind] = [empty_block[0], empty_block[0] + file_len, file_block[2]];
                // Reduce the dimension of the empty space
                empty_spaces[e_ind][0] += file_len;
                // Break the loop since we don't need to add the new space back in the empty_spaces
                // vec because files can't be moved right
                break;
            }
        }
    }

    // Maybe not the easiest to read, but it's **ELEGANT**
    return files
        .iter()
        .flat_map(|range| (range[0]..range[1]).map(move |i| i * range[2]))
        .sum::<isize>();
}

fn main() {
    let data = read_to_string("rsc/input09.txt").expect("Input file not found!");

    println!("Part 1 solution: {}", part1(&data));
    println!("Part 2 solution: {}", part2(&data));
}
