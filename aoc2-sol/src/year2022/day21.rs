// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 21

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_solution, valid_lines},
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
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2022, AoCDay::AOCD21, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader)
}

fn find_br<T>(reader: T) -> usize
where
    T: BufRead,
{
    for _line in valid_lines(reader) {}
    0
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2022, AoCDay::AOCD21, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader)
}

fn find2_br<T>(reader: T) -> usize
where
    T: BufRead,
{
    for _line in valid_lines(reader) {}
    0
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use std::io::Cursor;

    const TEST_1: &str = r">";

    #[test]
    fn solution() {
        assert_eq!(find_br(Cursor::new(TEST_1)), 0);
    }
}

#[cfg(test)]
mod two_star {
    // use super::find2_br;
    // use std::io::Cursor;

    // const TEST_1: &str = r"^v";
    // const TEST_2: &str = r"^>v<";
    // const TEST_3: &str = r"^v^v^v^v^v";

    #[test]
    fn solution() {
        // assert_eq!(find2_br(Cursor::new(TEST_1))?, 3);
    }
}
