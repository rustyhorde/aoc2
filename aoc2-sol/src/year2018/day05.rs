// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code ---**
//!
//! **--- Day 5: Alchemical Reduction ---**
//!
//! You've managed to sneak in to the prototype suit manufacturing lab. The Elves are making decent progress, but are still struggling with the suit's size reduction capabilities.
//!
//! While the very latest in 1518 alchemical technology might have solved their problem eventually, you can do better. You scan the chemical composition of the suit's material and discover that it is formed by extremely long polymers (one of which is available as your puzzle input).
//!
//! The polymer is formed by smaller units which, when triggered, react with each other such that two adjacent units of the same type and opposite polarity are destroyed. Units' types are represented by letters; units' polarity is represented by capitalization. For instance, r and R are units with the same type but opposite polarity, whereas r and s are entirely different types and do not react.
//!
//! For example:
//!
//! ```text
//!     In aA, a and A react, leaving nothing behind.
//!     In abBA, bB destroys itself, leaving aA. As above, this then destroys itself, leaving nothing.
//!     In abAB, no two adjacent units are of the same type, and so nothing happens.
//!     In aabAAB, even though aa and AA are of the same type, their polarities match, and so nothing happens.
//! ```
//!
//! Now, consider a larger example, dabAcCaCBAcCcaDA:
//!
//! ```text
//! dabAcCaCBAcCcaDA  The first 'cC' is removed.
//! dabAaCBAcCcaDA    This creates 'Aa', which is removed.
//! dabCBAcCcaDA      Either 'cC' or 'Cc' are removed (the result is the same).
//! dabCBAcaDA        No further actions can be taken.
//! ```
//!
//! After all possible reactions, the resulting polymer contains 10 units.
//!
//! How many units remain after fully reacting the polymer you scanned? (Note: in this puzzle and others, the input is large; if you copy/paste your input, make sure you get the whole thing.)
//!
//! **--- Part Two ---**
//!
//! Time to improve the polymer.
//!
//! One of the unit types is causing problems; it's preventing the polymer from collapsing as much as it should. Your goal is to figure out which unit type is causing the most problems, remove all instances of it (regardless of polarity), fully react the remaining polymer, and measure its length.
//!
//! For example, again using the polymer dabAcCaCBAcCcaDA from above:
//!
//! ```text
//!     Removing all A/a units produces dbcCCBcCcD. Fully reacting this polymer produces dbCBcD, which has length 6.
//!     Removing all B/b units produces daAcCaCAcCcaDA. Fully reacting this polymer produces daCAcaDA, which has length 8.
//!     Removing all C/c units produces dabAaBAaDA. Fully reacting this polymer produces daDA, which has length 4.
//!     Removing all D/d units produces abAcCaCBAcCcaA. Fully reacting this polymer produces abCBAc, which has length 6.
//! ```
//!
//! In this example, removing all C/c units was best, producing the answer 4.
//!
//! What is the length of the shortest polymer you can produce by removing all units of exactly one type and fully reacting the result?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use std::{
    cmp::{max, min},
    collections::HashMap,
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
    run_setup_solution::<Vec<String>, usize>(AoCYear::AOC2018, AoCDay::AOCD05, setup, find)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<String>, usize>(bench, AoCYear::AOC2018, AoCDay::AOCD05, setup, find)
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
    find_res(&data[0], false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(line: &str, second_star: bool) -> Result<usize> {
    if second_star {
        let mut results = HashMap::new();
        for lower in 97..=122 {
            let upper = lower - 32;
            let bytes_vec = line.as_bytes().to_vec();
            let mut filtered: Vec<u8> = bytes_vec
                .iter()
                .filter(|x| **x != lower && **x != upper)
                .copied()
                .collect();

            let _ = results.insert(lower, collapse_polymer(&mut filtered));
        }

        if let Some(min) = results.values().min() {
            Ok(*min)
        } else {
            Err(anyhow!("unable to find minimum"))
        }
    } else {
        Ok(collapse_polymer(&mut line.as_bytes().to_vec()))
    }
}

fn collapse_polymer(bytes: &mut Vec<u8>) -> usize {
    'outer: loop {
        let cloned = bytes.clone();
        for i in 0..bytes.len() {
            if let Some(first) = cloned.get(i) {
                if let Some(second) = cloned.get(i + 1) {
                    let max = max(first, second);
                    let min = min(first, second);

                    if max - min == 32 {
                        let _ = bytes.remove(i);
                        let _ = bytes.remove(i);
                        break;
                    }
                } else {
                    break 'outer;
                }
            } else {
                break 'outer;
            }
        }
    }

    bytes.len()
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<Vec<String>, usize>(AoCYear::AOC2018, AoCDay::AOCD05, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<String>, usize>(bench, AoCYear::AOC2018, AoCDay::AOCD05, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: Vec<String>) -> usize {
    find_res(&data[0], true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"dabAcCaCBAcCcaDA";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find(data), 10);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"dabAcCaCBAcCcaDA";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find2(data), 4);
        Ok(())
    }
}
