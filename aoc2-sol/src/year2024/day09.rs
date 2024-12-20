// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 1
//! --- Day 9: Disk Fragmenter ---
//!
//! Another push of the button leaves you in the familiar hallways of some friendly amphipods! Good thing you each somehow got your own personal mini submarine. The Historians jet away in search of the Chief, mostly by driving directly into walls.
//!
//! While The Historians quickly figure out how to pilot these things, you notice an amphipod in the corner struggling with his computer. He's trying to make more contiguous free space by compacting all of the files, but his program isn't working; you offer to help.
//!
//! He shows you the disk map (your puzzle input) he's already generated. For example:
//!
//! ```text
//! 2333133121414131402
//! ```
//!
//! The disk map uses a dense format to represent the layout of files and free space on the disk. The digits alternate between indicating the length of a file and the length of free space.
//!
//! So, a disk map like 12345 would represent a one-block file, two blocks of free space, a three-block file, four blocks of free space, and then a five-block file. A disk map like 90909 would represent three nine-block files in a row (with no free space between them).
//!
//! Each file on disk also has an ID number based on the order of the files as they appear before they are rearranged, starting with ID 0. So, the disk map 12345 has three files: a one-block file with ID 0, a three-block file with ID 1, and a five-block file with ID 2. Using one character for each block where digits are the file ID and . is free space, the disk map 12345 represents these individual blocks:
//!
//! ```text
//! 0..111....22222
//! ```
//!
//! The first example above, 2333133121414131402, represents these individual blocks:
//!
//! ```text
//! 00...111...2...333.44.5555.6666.777.888899
//! ```
//!
//! The amphipod would like to move file blocks one at a time from the end of the disk to the leftmost free space block (until there are no gaps remaining between file blocks). For the disk map 12345, the process looks like this:
//!
//! ```text
//! 0..111....22222
//! 02.111....2222.
//! 022111....222..
//! 0221112...22...
//! 02211122..2....
//! 022111222......
//! ```
//!
//! The first example requires a few more steps:
//!
//! ```text
//! 00...111...2...333.44.5555.6666.777.888899
//! 009..111...2...333.44.5555.6666.777.88889.
//! 0099.111...2...333.44.5555.6666.777.8888..
//! 00998111...2...333.44.5555.6666.777.888...
//! 009981118..2...333.44.5555.6666.777.88....
//! 0099811188.2...333.44.5555.6666.777.8.....
//! 009981118882...333.44.5555.6666.777.......
//! 0099811188827..333.44.5555.6666.77........
//! 00998111888277.333.44.5555.6666.7.........
//! 009981118882777333.44.5555.6666...........
//! 009981118882777333644.5555.666............
//! 00998111888277733364465555.66.............
//! 0099811188827773336446555566..............
//! ```
//!
//! The final step of this file-compacting process is to update the filesystem checksum. To calculate the checksum, add up the result of multiplying each of these blocks' position with the file ID number it contains. The leftmost block is in position 0. If a block contains free space, skip it instead.
//!
//! Continuing the first example, the first few blocks' position multiplied by its file ID number are 0 * 0 = 0, 1 * 0 = 0, 2 * 9 = 18, 3 * 9 = 27, 4 * 8 = 32, and so on. In this example, the checksum is the sum of these, 1928.
//!
//! Compact the amphipod's hard drive using the process he requested. What is the resulting filesystem checksum? (Be careful copy/pasting the input for this puzzle; it is a single, very long line.)
//!
//! --- Part Two ---
//!
//! Upon completion, two things immediately become clear. First, the disk definitely has a lot more contiguous free space, just like the amphipod hoped. Second, the computer is running much more slowly! Maybe introducing all of that file system fragmentation was a bad idea?
//!
//! The eager amphipod already has a new plan: rather than move individual blocks, he'd like to try compacting the files on his disk by moving whole files instead.
//!
//! This time, attempt to move whole files to the leftmost span of free space blocks that could fit the file. Attempt to move each file exactly once in order of decreasing file ID number starting with the file with the highest file ID number. If there is no span of free space to the left of a file that is large enough to fit the file, the file does not move.
//!
//! The first example from above now proceeds differently:
//!
//! ```text
//! 00...111...2...333.44.5555.6666.777.888899
//! 0099.111...2...333.44.5555.6666.777.8888..
//! 0099.1117772...333.44.5555.6666.....8888..
//! 0099.111777244.333....5555.6666.....8888..
//! 00992111777.44.333....5555.6666.....8888..
//! ```
//!
//! The process of updating the filesystem checksum is the same; now, this example's checksum would be 2858.
//!
//! Start over, now compacting the amphipod's hard drive using this new method instead. What is the resulting filesystem checksum?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<Vec<usize>, usize>(AoCYear::AOC2024, AoCDay::AOCD09, setup, find)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<usize>, usize>(bench, AoCYear::AOC2024, AoCDay::AOCD09, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> Vec<usize> {
    setup_br(reader).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T) -> Result<Vec<usize>>
where
    T: BufRead,
{
    let mut data = vec![];
    for row in valid_lines(reader) {
        data = row
            .chars()
            .map(|x| x.to_string().parse::<usize>())
            .filter_map(Result::ok)
            .collect::<Vec<usize>>();
    }
    Ok(data)
}

fn setup_disk(data: &[usize]) -> Vec<isize> {
    let mut disk = vec![];
    for (curr_id, size_free) in (0_isize..).zip(data.chunks(2)) {
        let file_len = size_free[0];
        let free_len = size_free.get(1);
        for _ in 0..file_len {
            disk.push(curr_id);
        }
        if let Some(free_len) = free_len {
            for _ in 0..*free_len {
                disk.push(-1);
            }
        }
    }
    disk
}

fn calc_total(disk: &[isize], day1: bool) -> usize {
    let mut total = 0;
    for (idx, val) in disk.iter().enumerate() {
        if *val >= 0 {
            if let Ok(v) = usize::try_from(*val) {
                total += idx * v;
            }
        } else if *val == -1 && day1 {
            break;
        }
    }
    total
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: Vec<usize>) -> usize {
    let mut disk = setup_disk(&data);
    loop {
        let mut val = -1;
        for i in (0..disk.len()).rev() {
            val = disk[i];
            if disk[i] >= 0 {
                disk[i] = -1;
                break;
            }
        }

        for empty in &mut disk {
            if *empty == -1 {
                *empty = val;
                break;
            }
        }

        let first_idx_empty = disk.iter().position(|x| *x == -1);
        let mut last_idx_file = 0;
        for i in (0..disk.len()).rev() {
            if disk[i] != -1 {
                last_idx_file = i;
                break;
            }
        }
        if let Some(first) = first_idx_empty {
            if first > last_idx_file {
                break;
            }
        } else {
            disk.clear();
            break;
        }
    }

    calc_total(&disk, true)
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<Vec<usize>, usize>(AoCYear::AOC2024, AoCDay::AOCD09, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<usize>, usize>(bench, AoCYear::AOC2024, AoCDay::AOCD09, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: Vec<usize>) -> usize {
    let mut disk = setup_disk(&data);
    let mut id_to_move = -2;

    loop {
        let mut indicies = vec![];

        if id_to_move == -2 {
            for i in (0..disk.len()).rev() {
                if disk[i] != -1 {
                    id_to_move = disk[i];
                    break;
                }
            }
        }
        for (idx, val) in disk.iter().enumerate() {
            if *val == id_to_move {
                indicies.push(idx);
            }
        }

        let empty_len = indicies.len();
        let mut empty_indicies = vec![];
        for i in 0..disk.len() {
            if disk[i] == -1 {
                empty_indicies.push(i);
                for j in (i + 1)..(i + empty_len) {
                    if let Some(val) = disk.get(j) {
                        if *val == -1 {
                            empty_indicies.push(j);
                        }
                    }
                }
                if empty_indicies.len() == indicies.len() {
                    break;
                }
            }
            empty_indicies.clear();
        }

        if empty_indicies.is_empty() && indicies.is_empty() {
            break;
        }

        if empty_indicies.len() == indicies.len() && indicies[0] > empty_indicies[0] {
            for i in indicies {
                disk[i] = -1;
            }

            for i in empty_indicies {
                disk[i] = id_to_move;
            }
        }

        id_to_move -= 1;
        if id_to_move == 0 {
            break;
        }
    }
    calc_total(&disk, false)
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"12345";
    const TEST_2: &str = r"2333133121414131402";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find(data), 60);
        let data2 = setup_br(Cursor::new(TEST_2))?;
        assert_eq!(find(data2), 1928);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"2333133121414131402";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find2(data), 2858);
        Ok(())
    }
}
