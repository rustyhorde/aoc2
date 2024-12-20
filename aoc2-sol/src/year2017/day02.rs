// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Corruption Checksum
//!
//! **--- Day 2: Corruption Checksum ---**
//!
//! As you walk through the door, a glowing humanoid shape yells in your
//! direction. "You there! Your state appears to be idle. Come help us repair
//! the corruption in this spreadsheet - if we take another millisecond,
//! we'll have to display an hourglass cursor!"
//!
//! The spreadsheet consists of rows of apparently-random numbers. To make
//! sure the recovery process is on the right track, they need you to calculate
//! the spreadsheet's checksum. For each row, determine the difference between
//! the largest value and the smallest value; the checksum is the sum of all
//! of these differences.
//!
//! For example, given the following spreadsheet:
//!
//! ```text
//! 5 1 9 5
//! 7 5 3
//! 2 4 6 8
//! ```
//!
//! ```text
//! The first row's largest and smallest values are 9 and 1, and their difference is 8.
//! The second row's largest and smallest values are 7 and 3, and their difference is 4.
//! The third row's difference is 6.
//! ```
//!
//! In this example, the spreadsheet's checksum would be `8 + 4 + 6 = 18`.
//!
//! What is the checksum for the spreadsheet in your puzzle input?
//!
//! **--- Part Two ---**
//!
//! "Great work; looks like we're on the right track after all. Here's a star for your
//! effort." However, the program seems a little worried. Can programs be worried?
//!
//! "Based on what we're seeing, it looks like all the User wanted is some information
//! about the evenly divisible values in the spreadsheet. Unfortunately, none of us
//! are equipped for that kind of calculation - most of us specialize in bitwise
//! operations."
//!
//! It sounds like the goal is to find the only two numbers in each row where one evenly
//! divides the other - that is, where the result of the division operation is a whole
//! number. They would like you to find those numbers on each line, divide them, and
//! add up each line's result.
//!
//! For example, given the following spreadsheet:
//!
//! ```text
//! 5 9 2 8
//! 9 4 7 3
//! 3 8 6 5
//! ```
//!
//! ```text
//! In the first row, the only two numbers that evenly divide are 8 and 2; the result of this division is 4.
//! In the second row, the two numbers are 9 and 3; the result is 3.
//! In the third row, the result is 2.
//! ```
//!
//! In this example, the sum of the results would be `4 + 3 + 2 = 9`.
//!
//! What is the sum of each row's result in your puzzle input?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{print_err, run_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use itertools::Itertools;
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
    run_solution::<usize>(AoCYear::AOC2017, AoCDay::AOCD02, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader).map_err(print_err).unwrap_or_default()
}

fn find_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    checksum(reader, false)
}

fn checksum<T>(reader: T, part2: bool) -> Result<usize>
where
    T: BufRead,
{
    let mut checksum = 0;
    for line in valid_lines(reader) {
        if part2 {
            checksum += row_evenly_divisible_value(&line)?;
        } else {
            checksum += row_min_max_diff(&line)?;
        }
    }
    Ok(checksum)
}

/// Find the difference between the max and min of a whitespace separated string
fn row_min_max_diff(line: &str) -> Result<usize> {
    let mut min = usize::MAX;
    let mut max = 0;

    for val_res in line.split_whitespace().map(str::parse) {
        let val = val_res?;
        if val < min {
            min = val;
        }

        if val > max {
            max = val;
        }
    }

    Ok(max - min)
}

/// Find the only two evenly divisible values in a whitespace separated string
fn row_evenly_divisible_value(line: &str) -> Result<usize> {
    let vals_vec = line
        .split_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect::<Vec<usize>>();
    for duo in vals_vec.iter().permutations(2) {
        if duo[0] % duo[1] == 0 {
            return Ok(duo[0] / duo[1]);
        }
    }

    Err(anyhow!("no evenly divisible values"))
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2017, AoCDay::AOCD02, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader).map_err(print_err).unwrap_or_default()
}

fn find2_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    checksum(reader, true)
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"5 1 9 5
7 5 3
2 4 6 8";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1))?, 18);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"5 9 2 8
9 4 7 3
3 8 6 5";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find2_br(Cursor::new(TEST_1))?, 9);
        Ok(())
    }
}
