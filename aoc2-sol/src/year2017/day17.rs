// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2017 ---**
//!
//! **--- Day 17: Spinlock ---**
//!
//! Suddenly, whirling in the distance, you notice what looks like a massive, pixelated hurricane: a deadly spinlock. This spinlock isn't just consuming computing power, but memory, too; vast, digital mountains are being ripped from the ground and consumed by the vortex.
//!
//! If you don't move quickly, fixing that printer will be the least of your problems.
//!
//! This spinlock's algorithm is simple but efficient, quickly consuming everything in its path. It starts with a circular buffer containing only the value 0, which it marks as the current position. It then steps forward through the circular buffer some number of steps (your puzzle input) before inserting the first new value, 1, after the value it stopped on. The inserted value becomes the current position. Then, it steps forward from there the same number of steps, and wherever it stops, inserts after it the second new value, 2, and uses that as the new current position again.
//!
//! It repeats this process of stepping forward, inserting a new value, and using the location of the inserted value as the new current position a total of 2017 times, inserting 2017 as its final operation, and ending with a total of 2018 values (including 0) in the circular buffer.
//!
//! For example, if the spinlock were to step 3 times per insert, the circular buffer would begin to evolve like this (using parentheses to mark the current position after each iteration of the algorithm):
//!
//! ```text
//!     (0), the initial state before any insertions.
//!     0 (1): the spinlock steps forward three times (0, 0, 0), and then inserts the first value, 1, after it. 1 becomes the current position.
//!     0 (2) 1: the spinlock steps forward three times (0, 1, 0), and then inserts the second value, 2, after it. 2 becomes the current position.
//!     0  2 (3) 1: the spinlock steps forward three times (1, 0, 2), and then inserts the third value, 3, after it. 3 becomes the current position.
//! ```
//!
//! And so on:
//!
//! ```text
//!     0  2 (4) 3  1
//!     0 (5) 2  4  3  1
//!     0  5  2  4  3 (6) 1
//!     0  5 (7) 2  4  3  6  1
//!     0  5  7  2  4  3 (8) 6  1
//!     0 (9) 5  7  2  4  3  8  6  1
//! ```
//!
//! Eventually, after 2017 insertions, the section of the circular buffer near the last insertion looks like this:
//!
//! ```text
//! 1512  1134  151 (2017) 638  1513  851
//! ```
//!
//! Perhaps, if you can identify the value that will ultimately be after the last value written (2017), you can short-circuit the spinlock. In this example, that would be 638.
//!
//! What is the value after 2017 in your completed circular buffer?
//!
//! **--- Part Two ---**
//!
//! The spinlock does not short-circuit. Instead, it gets more angry. At least, you assume that's what happened; it's spinning significantly faster than it was a moment ago.
//!
//! You have good news and bad news.
//!
//! The good news is that you have improved calculations for how to stop the spinlock. They indicate that you actually need to identify the value after 0 in the current state of the circular buffer.
//!
//! The bad news is that while you were determining this, the spinlock has just finished inserting its fifty millionth value (50000000).
//!
//! What is the value after 0 the moment 50000000 is inserted?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

type SpinlockData = (Vec<u64>, Option<u64>);

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<SpinlockData, u64>(AoCYear::AOC2017, AoCDay::AOCD17, setup, find)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<SpinlockData, u64>(bench, AoCYear::AOC2017, AoCDay::AOCD17, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> SpinlockData {
    setup_br(reader, None).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T, insertions: Option<u64>) -> Result<SpinlockData>
where
    T: BufRead,
{
    let mut data = vec![];
    for line in valid_lines(reader) {
        data.push(line.parse::<u64>()?);
    }
    Ok((data, insertions))
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: SpinlockData) -> u64 {
    find_res(data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(data: SpinlockData, second_star: bool) -> Result<u64> {
    let (start, insertions_opt) = data;
    let mut buf = vec![0, 0];
    let insertions = if let Some(ins) = insertions_opt {
        ins
    } else if second_star {
        50_000_000
    } else {
        2017
    };

    spinlock(&mut buf, start[0], insertions, second_star)
}

/// Run the spinlock.
fn spinlock(buf: &mut Vec<u64>, steps: u64, iterations: u64, second_star: bool) -> Result<u64> {
    let mut curr_index = 0;
    for i in 0..iterations {
        let next_index = next_index(curr_index, i, steps);

        if second_star && next_index + 1 == 1 {
            buf[1] = i + 1;
        } else if !second_star {
            let next_idx = usize::try_from(next_index + 1)?;
            buf.insert(next_idx, i + 1);
        }
        curr_index = next_index + 1;
    }

    if second_star {
        Ok(buf[1])
    } else {
        let curr_idx = usize::try_from(curr_index + 1)?;
        Ok(buf[curr_idx])
    }
}

/// Calculate the next index
fn next_index(curr_index: u64, max_index: u64, steps: u64) -> u64 {
    let mut idx = curr_index;

    for _ in 0..steps {
        #[allow(clippy::comparison_chain)]
        if idx < max_index {
            idx += 1;
        } else if idx == max_index {
            idx = 0;
        }
    }

    idx
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<SpinlockData, u64>(AoCYear::AOC2017, AoCDay::AOCD17, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<SpinlockData, u64>(bench, AoCYear::AOC2017, AoCDay::AOCD17, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: SpinlockData) -> u64 {
    find_res(data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"3";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1), Some(2017))?;
        assert_eq!(find(data), 638);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {}
