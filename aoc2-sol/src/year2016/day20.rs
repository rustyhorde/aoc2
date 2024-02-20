// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Firewall Rules
//!
//! **--- Day 20: Firewall Rules ---**
//!
//! **--- Part 1 ---**
//!
//! You'd like to set up a small hidden computer here so you can use
//! it to get back into the network later. However, the corporate firewall
//! only allows communication with certain external IP addresses.
//!
//! You've retrieved the list of blocked IPs from the firewall, but the list
//! seems to be messy and poorly maintained, and it's not clear which IPs
//! are allowed. Also, rather than being written in dot-decimal notation,
//! they are written as plain 32-bit integers, which can have any value from
//! `0` through `4294967295`, inclusive.
//!
//! For example, suppose only the values `0` through `9` were valid, and
//! that you retrieved the following blacklist:
//!
//! ```text
//! 5-8
//! 0-2
//! 4-7
//! ````
//!
//! The blacklist specifies ranges of IPs (inclusive of both the start and end
//! value) that are not allowed. Then, the only IPs that this firewall allows
//! are `3` and `9`, since those are the only numbers not in any range.
//!
//! Given the list of blocked IPs you retrieved from the firewall (your puzzle
//! input), what is the lowest-valued IP that is not blocked?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{get_cap_x, print_err, run_solution, valid_lines},
};
use anyhow::Result;
use regex::Regex;
use std::{
    collections::BTreeMap,
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
    run_solution::<usize>(AoCYear::AOC2016, AoCDay::AOCD20, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader, u32::MAX)
        .map_err(print_err)
        .unwrap_or_default()
}

fn find_br<T>(reader: T, max_value: u32) -> Result<usize>
where
    T: BufRead,
{
    run(reader, max_value, false)
}

fn run<T>(reader: T, max_value: u32, part2: bool) -> Result<usize>
where
    T: BufRead,
{
    let line_re = Regex::new(r"^(\d+)-(\d+)$")?;
    let mut blah = BTreeMap::new();

    for line in valid_lines(reader) {
        for caps in line_re.captures_iter(&line) {
            let from = get_cap_x::<u32>(1, &caps)?;
            let to = get_cap_x::<u32>(2, &caps)?;
            _ = blah.insert(from, to);
        }
    }

    let mut max = 0;
    let mut allowed = 0;
    for (from, to) in blah {
        if from <= max || from - 1 == max {
            if to > max {
                max = to;
            }
        } else if part2 {
            allowed += from - max - 1;
            if to > max {
                max = to;
            }
        } else {
            break;
        }
    }

    if part2 {
        allowed += max_value - max;
        Ok(usize::try_from(allowed)?)
    } else {
        Ok(usize::try_from(max + 1)?)
    }
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2016, AoCDay::AOCD20, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader, u32::MAX)
        .map_err(print_err)
        .unwrap_or_default()
}

fn find2_br<T>(reader: T, max_value: u32) -> Result<usize>
where
    T: BufRead,
{
    run(reader, max_value, true)
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"5-8
0-2
4-7";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1), 9)?, 3);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"5-8
0-2
4-7";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find2_br(Cursor::new(TEST_1), 9)?, 2);
        Ok(())
    }
}
