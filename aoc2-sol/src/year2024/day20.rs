// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 20

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
    run_setup_solution::<Vec<String>, usize>(AoCYear::AOC2024, AoCDay::AOCD20, setup, find)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<String>, usize>(bench, AoCYear::AOC2024, AoCDay::AOCD20, setup, find)
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
fn find_res(_data: Vec<String>, _second_star: bool) -> Result<usize> {
    Ok(0)
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<Vec<String>, usize>(AoCYear::AOC2024, AoCDay::AOCD20, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<String>, usize>(bench, AoCYear::AOC2024, AoCDay::AOCD20, setup, find2)
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

    const TEST_1: &str = r">";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find(data), 0);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r">";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find2(data), 0);
        Ok(())
    }
}
