// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 4
//! --- Day 4: Secure Container ---
//!
//! You arrive at the Venus fuel depot only to discover it's protected by a password. The Elves had written the password on a sticky note, but someone threw it out.
//!
//! However, they do remember a few key facts about the password:
//!
//! > It is a six-digit number.
//! > The value is within the range given in your puzzle input.
//! > Two adjacent digits are the same (like 22 in 122345).
//! > Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
//!
//! Other than the range rule, the following are true:
//!
//! > 111111 meets these criteria (double 11, never decreases).
//! > 223450 does not meet these criteria (decreasing pair of digits 50).
//! > 123789 does not meet these criteria (no double).
//!
//! How many different passwords within the range given in your puzzle input meet these criteria?
//!
//! --- Part Two ---
//!
//! An Elf just remembered one more important detail: the two adjacent matching digits are not part of a larger group of matching digits.
//!
//! Given this additional criterion, but still ignoring the range rule, the following are now true:
//!
//! > 112233 meets these criteria because the digits never decrease and all repeated digits are exactly two digits long.
//! > 123444 no longer meets the criteria (the repeated 44 is part of a larger group of 444).
//! > 111122 meets the criteria (even though 1 is repeated more than twice, it still contains a double 22).
//!
//! How many different passwords within the range given in your puzzle input meet all of the criteria?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_solution, valid_lines},
};
use anyhow::Result;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};
use tracing::error;

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2019, AoCDay::AOCD04, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader)
        .map_err(|e| {
            error!("{e}");
            e
        })
        .unwrap_or_default()
}

fn find_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let mut lower: usize = 0;
    let mut upper: usize = 0;
    for line in valid_lines(reader) {
        let split = line.split('-').collect::<Vec<&str>>();
        lower = split[0].parse::<usize>()?;
        upper = split[1].parse::<usize>()?;
    }
    Ok((lower..=upper)
        .map(to_digits)
        .filter_map(Result::ok)
        .filter(doubles)
        .filter(ascending)
        .collect::<Vec<Vec<u8>>>()
        .len())
}

fn to_digits(mut v: usize) -> Result<Vec<u8>> {
    let mut digits: Vec<u8> = Vec::with_capacity(20);

    while v > 0 {
        let n = u8::try_from(v % 10)?;
        v /= 10;
        digits.push(n);
    }
    digits.reverse();
    Ok(digits)
}

#[allow(clippy::ptr_arg)]
fn doubles(digits: &Vec<u8>) -> bool {
    digits.windows(2).any(|x| x[0] == x[1])
}

#[allow(clippy::ptr_arg)]
fn ascending(digits: &Vec<u8>) -> bool {
    digits.windows(2).all(|x| x[0] <= x[1])
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2019, AoCDay::AOCD04, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader)
        .map_err(|e| {
            error!("{e}");
            e
        })
        .unwrap_or_default()
}

fn find2_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let mut lower: usize = 0;
    let mut upper: usize = 0;
    for line in valid_lines(reader) {
        let split = line.split('-').collect::<Vec<&str>>();
        lower = split[0].parse::<usize>()?;
        upper = split[1].parse::<usize>()?;
    }
    Ok((lower..=upper)
        .map(to_digits)
        .filter_map(Result::ok)
        .filter(doubles)
        .filter(unique_double)
        .filter(ascending)
        .collect::<Vec<Vec<u8>>>()
        .len())
}

#[allow(clippy::ptr_arg)]
fn unique_double(digits: &Vec<u8>) -> bool {
    let mut doubles_count: HashMap<(u8, u8), usize> = HashMap::new();
    for pair in digits.windows(2) {
        if pair[0] == pair[1] {
            *doubles_count.entry((pair[0], pair[1])).or_insert_with(|| 0) += 1;
        }
    }
    doubles_count.values().any(|x| *x == 1)
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"111111-111111";
    const TEST_2: &str = r"223450-223450";
    const TEST_3: &str = r"123789-123789";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1))?, 1);
        assert_eq!(find_br(Cursor::new(TEST_2))?, 0);
        assert_eq!(find_br(Cursor::new(TEST_3))?, 0);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"112233-112233";
    const TEST_2: &str = r"123444-123444";
    const TEST_3: &str = r"111122-111122";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find2_br(Cursor::new(TEST_1))?, 1);
        assert_eq!(find2_br(Cursor::new(TEST_2))?, 0);
        assert_eq!(find2_br(Cursor::new(TEST_3))?, 1);
        Ok(())
    }
}
