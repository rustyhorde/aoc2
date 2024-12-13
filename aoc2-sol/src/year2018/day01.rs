// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2018 ---**
//!
//! **--- Day 1: Chronal Calibration ---**
//!
//! "We've detected some temporal anomalies," one of Santa's Elves at the Temporal Anomaly Research and Detection Instrument Station tells you. She sounded pretty worried when she called you down here. "At 500-year intervals into the past, someone has been changing Santa's history!"
//!
//! "The good news is that the changes won't propagate to our time stream for another 25 days, and we have a device" - she attaches something to your wrist - "that will let you fix the changes with no such propagation delay. It's configured to send you 500 years further into the past every few days; that was the best we could do on such short notice."
//!
//! "The bad news is that we are detecting roughly fifty anomalies throughout time; the device will indicate fixed anomalies with stars. The other bad news is that we only have one device and you're the best person for the job! Good lu--" She taps a button on the device and you suddenly feel like you're falling. To save Christmas, you need to get all fifty stars by December 25th.
//!
//! Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
//!
//! After feeling like you've been falling for a few minutes, you look at the device's tiny screen. "Error: Device must be calibrated before first use. Frequency drift detected. Cannot maintain destination lock." Below the message, the device shows a sequence of changes in frequency (your puzzle input). A value like +6 means the current frequency increases by 6; a value like -3 means the current frequency decreases by 3.
//!
//! For example, if the device displays frequency changes of +1, -2, +3, +1, then starting from a frequency of zero, the following changes would occur:
//!
//! ```text
//!     Current frequency  0, change of +1; resulting frequency  1.
//!     Current frequency  1, change of -2; resulting frequency -1.
//!     Current frequency -1, change of +3; resulting frequency  2.
//!     Current frequency  2, change of +1; resulting frequency  3.
//! ```
//!
//! In this example, the resulting frequency is 3.
//!
//! Here are other example situations:
//!
//! ```text
//!     +1, +1, +1 results in  3
//!     +1, +1, -2 results in  0
//!     -1, -2, -3 results in -6
//! ```
//!
//! Starting with a frequency of zero, what is the resulting frequency after all of the changes in frequency have been applied?
//!
//! **--- Part Two ---**
//!
//! You notice that the device repeats the same frequency change list over and over. To calibrate the device, you need to find the first frequency it reaches twice.
//!
//! For example, using the same list of changes above, the device would loop as follows:
//!
//! ```text
//!     Current frequency  0, change of +1; resulting frequency  1.
//!     Current frequency  1, change of -2; resulting frequency -1.
//!     Current frequency -1, change of +3; resulting frequency  2.
//!     Current frequency  2, change of +1; resulting frequency  3.
//!     (At this point, the device continues from the start of the list.)
//!     Current frequency  3, change of +1; resulting frequency  4.
//!     Current frequency  4, change of -2; resulting frequency  2, which has already been seen.
//! ```
//!
//! In this example, the first frequency reached twice is 2. Note that your device might need to repeat its list of frequency changes many times before a duplicate frequency is found, and that duplicates might be found while in the middle of processing the list.
//!
//! Here are other examples:
//!
//! ```text
//!     +1, -1 first reaches 0 twice.
//!     +3, +3, +4, -2, -4 first reaches 10 twice.
//!     -6, +3, +8, +5, -6 first reaches 5 twice.
//!     +7, +7, -2, -7, -4 first reaches 14 twice.
//! ```
//!
//! What is the first frequency your device reaches twice?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::Result;
use std::{
    collections::HashSet,
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
    run_setup_solution::<Vec<String>, i32>(AoCYear::AOC2018, AoCDay::AOCD01, setup, find).map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<String>, i32>(bench, AoCYear::AOC2018, AoCDay::AOCD01, setup, find)
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
fn find(data: Vec<String>) -> i32 {
    find_res(data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(data: Vec<String>, second_star: bool) -> Result<i32> {
    let mut acc_vec = Vec::new();

    for line in data {
        let chars: Vec<char> = line.chars().collect();
        let num_str: String = chars[1..].iter().collect();
        let num_to_add = num_str.parse::<i32>()?;
        let tuple = (chars[0] == '+', num_to_add);
        acc_vec.push(tuple);
    }
    val(&acc_vec, second_star)
}

/// Calculate the 'inverse captcha' value for a byte array.
fn val(acc_vec: &[(bool, i32)], second_star: bool) -> Result<i32> {
    if second_star {
        let mut result_set = HashSet::new();
        let _res = result_set.insert(0);
        let mut acc = 0;
        loop {
            for (add, val) in acc_vec {
                if *add {
                    acc += val;
                } else {
                    acc -= val;
                }

                if !result_set.insert(acc) {
                    return Ok(acc);
                }
            }
        }
    } else {
        Ok(acc_vec
            .iter()
            .fold(0, |acc, x| if x.0 { acc + x.1 } else { acc - x.1 }))
    }
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<Vec<String>, i32>(AoCYear::AOC2018, AoCDay::AOCD01, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<String>, i32>(bench, AoCYear::AOC2018, AoCDay::AOCD01, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: Vec<String>) -> i32 {
    find_res(data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"+1
-2
+3
+1";
    const TEST_2: &str = r"+1
+1
+1";
    const TEST_3: &str = r"+1
+1
-2";
    const TEST_4: &str = r"-1
-2
-3";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find(data), 3);
        let data = setup_br(Cursor::new(TEST_2))?;
        assert_eq!(find(data), 3);
        let data = setup_br(Cursor::new(TEST_3))?;
        assert_eq!(find(data), 0);
        let data = setup_br(Cursor::new(TEST_4))?;
        assert_eq!(find(data), -6);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"+1
-1";
    const TEST_2: &str = r"+3
+3
+4
-2
-4";
    const TEST_3: &str = r"-6
+3
+8
+5
-6";
    const TEST_4: &str = r"+7
+7
-2
-7
-4";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find2(data), 0);
        let data = setup_br(Cursor::new(TEST_2))?;
        assert_eq!(find2(data), 10);
        let data = setup_br(Cursor::new(TEST_3))?;
        assert_eq!(find2(data), 5);
        let data = setup_br(Cursor::new(TEST_4))?;
        assert_eq!(find2(data), 14);
        Ok(())
    }
}
