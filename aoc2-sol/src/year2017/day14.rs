// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2017 ---**
//!
//! **--- Day 14: Disk Defragmentation ---**
//!
//! Suddenly, a scheduled job activates the system's disk defragmenter. Were the situation different, you might sit and watch it for a while, but today, you just don't have that kind of time. It's soaking up valuable system resources that are needed elsewhere, and so the only option is to help it finish its task as soon as possible.
//!
//! The disk in question consists of a 128x128 grid; each square of the grid is either free or used. On this disk, the state of the grid is tracked by the bits in a sequence of knot hashes.
//!
//! A total of 128 knot hashes are calculated, each corresponding to a single row in the grid; each hash contains 128 bits which correspond to individual grid squares. Each bit of a hash indicates whether that square is free (0) or used (1).
//!
//! The hash inputs are a key string (your puzzle input), a dash, and a number from 0 to 127 corresponding to the row. For example, if your key string were flqrgnkx, then the first row would be given by the bits of the knot hash of flqrgnkx-0, the second row from the bits of the knot hash of flqrgnkx-1, and so on until the last row, flqrgnkx-127.
//!
//! The output of a knot hash is traditionally represented by 32 hexadecimal digits; each of these digits correspond to 4 bits, for a total of 4 * 32 = 128 bits. To convert to bits, turn each hexadecimal digit to its equivalent binary value, high-bit first: 0 becomes 0000, 1 becomes 0001, e becomes 1110, f becomes 1111, and so on; a hash that begins with a0c2017... in hexadecimal would begin with 10100000110000100000000101110000... in binary.
//!
//! Continuing this process, the first 8 rows and columns for key flqrgnkx appear as follows, using # to denote used squares, and . to denote free ones:
//!
//! ```text
//! ##.#.#..-->
//! .#.#.#.#   
//! ....#.#.   
//! #.#.##.#   
//! .##.#...   
//! ##..#..#   
//! .#...#..   
//! ##.#.##.-->
//! |      |   
//! V      V   
//! ```
//!
//! In this example, 8108 squares are used across the entire 128x128 grid.
//!
//! Given your actual key string, how many squares are used?
//!
//! **--- Part Two ---**
//!
//! Now, all the defragmenter needs to know is the number of regions. A region is a group of used squares that are all adjacent, not including diagonals. Every used square is in exactly one region: lone used squares form their own isolated regions, while several adjacent squares all count as a single region.
//!
//! In the example above, the following nine regions are visible, each marked with a distinct digit:
//!
//! ```text
//! 11.2.3..-->
//! .1.2.3.4   
//! ....5.6.   
//! 7.8.55.9   
//! .88.5...   
//! 88..5..8   
//! .8...8..   
//! 88.8.88.-->
//! |      |   
//! V      V   
//! ```
//!
//! Of particular interest is the region marked 8; while it does not appear contiguous in this small view, all of the squares marked 8 are connected when considering the whole 128x128 grid. In total, in this example, 1242 regions are present.
//!
//! How many regions are present given your key string?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use ndarray::Array2;
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
    run_setup_solution::<Vec<String>, usize>(AoCYear::AOC2017, AoCDay::AOCD14, setup, find)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<String>, usize>(bench, AoCYear::AOC2017, AoCDay::AOCD14, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> Vec<String> {
    setup_br(reader).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T) -> Result<Vec<String>>
where
    T: BufRead,
{
    let mut data = vec![];
    for line in valid_lines(reader) {
        data.push(line);
    }
    Ok(data)
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: Vec<String>) -> usize {
    find_res(data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(data: Vec<String>, second_star: bool) -> Result<usize> {
    let mut count = 0;

    for line in data {
        let mut disk_arr: Array2<u32> = Array2::zeros((128, 128));
        let mut visited: Array2<bool> = Array2::from_elem((128, 128), false);

        for i in 0..128 {
            let mut hash = Vec::new();
            let mut input = line.clone();
            input.push('-');
            input.push_str(&i.to_string());
            parse_list_and_hash(&mut hash, &input, 256, true)?;
            let hex = &squash_and_hex(&hash);

            if second_star {
                fill_row(i, hex, &mut disk_arr)?;
            } else {
                let binary_str = to_binary_string(hex)?;
                let ones_str: String = binary_str.chars().filter(|c| *c == '1').collect();
                count += ones_str.len();
            }
        }

        if second_star {
            for i in 0..128 {
                for j in 0..128 {
                    if disk_arr[[i, j]] == 1 && !visited[[i, j]] {
                        // Do some DFS
                        depth_first_search(i, j, &mut disk_arr, &mut visited)?;
                        count += 1;
                    }
                }
            }
        }
    }
    Ok(count)
}

/// Parse the list of lengths and calculate the hash.
fn parse_list_and_hash(
    hash: &mut Vec<u32>,
    line: &str,
    num_elements: u32,
    second_star: bool,
) -> Result<()> {
    let lengths = generate_lengths(line, second_star)?;

    for i in 0..num_elements {
        hash.push(i);
    }

    let rounds = if second_star { 64 } else { 1 };
    let mut curr_pos: u32 = 0;
    let mut skip_size = 0;

    for _ in 0..rounds {
        for length in &lengths {
            let mut indices = Vec::new();
            let mut slice = Vec::new();

            for j in curr_pos..u32::from(*length) + curr_pos {
                let actual_idx = j % num_elements;
                indices.push(actual_idx);
                slice.push(*hash.get(actual_idx as usize).ok_or(anyhow!("invalid"))?);
            }

            slice.reverse();
            for (idx, val) in indices.iter().zip(slice.iter()) {
                *hash.get_mut(*idx as usize).ok_or(anyhow!("invalid"))? = *val;
            }

            curr_pos = (curr_pos + u32::from(*length) + skip_size) % num_elements;
            skip_size += 1;
        }
    }

    Ok(())
}

/// Generate the list of lengths.
fn generate_lengths(line: &str, second_star: bool) -> Result<Vec<u8>> {
    let mut lengths = Vec::new();

    if second_star {
        if !line.is_empty() {
            lengths.extend(line.as_bytes());
        }
        lengths.extend(vec![17, 31, 73, 47, 23]);
    } else {
        let length_strs: Vec<&str> = line.split(',').collect();
        for length_str in length_strs {
            lengths.push(length_str.parse::<u8>()?);
        }
    }

    Ok(lengths)
}

/// Create dense hash and hexify
fn squash_and_hex(hash: &[u32]) -> String {
    use std::fmt::Write;

    let chunks = hash.chunks(16);
    let mut byte_vec = Vec::new();

    for chunk in chunks {
        let val = chunk.iter().fold(0, |acc, x| x ^ acc);
        byte_vec.push(val);
    }

    let mut result = String::new();
    for byte in byte_vec {
        write!(result, "{byte:02x}").expect("Unable to write string");
    }
    result
}

/// Convert to a binary string
fn to_binary_string(hex: &str) -> Result<String> {
    let mut binary_str = String::new();
    for c in hex.chars() {
        let binary = match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'a' => "1010",
            'b' => "1011",
            'c' => "1100",
            'd' => "1101",
            'e' => "1110",
            'f' => "1111",
            _ => return Err(anyhow!("Invalid hex digit")),
        };
        binary_str.push_str(binary);
    }
    Ok(binary_str)
}

/// Fill the given row in the array
fn fill_row(row: usize, hex: &str, arr: &mut Array2<u32>) -> Result<()> {
    let binary_str = to_binary_string(hex)?;
    for (idx, c) in binary_str.chars().enumerate() {
        let bit = c.to_string().parse::<u32>()?;
        arr[[row, idx]] = bit;
    }
    Ok(())
}

/// Depth first search for adjacent neighbors
fn depth_first_search(
    row: usize,
    col: usize,
    disk_arr: &mut Array2<u32>,
    visited: &mut Array2<bool>,
) -> Result<()> {
    visited[[row, col]] = true;

    let row_deltas: Vec<isize> = vec![-1, 0, 0, 1];
    let col_deltas: Vec<isize> = vec![0, -1, 1, 0];
    let row_i: isize = TryFrom::try_from(row)?;
    let col_i: isize = TryFrom::try_from(col)?;

    // Check the four adjacent neighbors (left, down, up, right)
    for k in 0..4 {
        if let Ok(adj_row) = TryFrom::try_from(row_i + row_deltas[k]) {
            if let Ok(adj_col) = TryFrom::try_from(col_i + col_deltas[k]) {
                if adj_row < 128
                    && adj_col < 128
                    && disk_arr[[adj_row, adj_col]] == 1
                    && !visited[[adj_row, adj_col]]
                {
                    depth_first_search(adj_row, adj_col, disk_arr, visited)?;
                }
            }
        }
    }

    Ok(())
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<Vec<String>, usize>(AoCYear::AOC2017, AoCDay::AOCD14, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<String>, usize>(bench, AoCYear::AOC2017, AoCDay::AOCD14, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: Vec<String>) -> usize {
    find_res(data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"flqrgnkx";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find(data), 8108);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"flqrgnkx";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find2(data), 1242);
        Ok(())
    }
}
