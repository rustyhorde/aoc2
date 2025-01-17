// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 17 "No Such Thing as Too Much"
//!
//! **--- Day 17: No Such Thing as Too Much ---**
//!
//! **--- Part 1 ---**
//!
//! The elves bought too much eggnog again - 150 liters this time. To fit it all
//! into your refrigerator, you'll need to move it into smaller containers.
//! You take an inventory of the capacities of the available containers.
//!
//! For example, suppose you have containers of size `20`, `15`, `10`, `5`,
//! and `5` liters. If you need to store `25` liters, there are four ways to do it:
//!
//! ```text
//! 15 and 10
//! 20 and 5 (the first 5)
//! 20 and 5 (the second 5)
//! 15, 5, and 5
//! ```
//!
//! Filling all containers entirely, how many different combinations of
//! containers can exactly fit all 150 liters of eggnog?
//!
//! **--- Part Two ---**
//!
//! While playing with all the containers in the kitchen, another load of
//! eggnog arrives! The shipping and receiving department is requesting as many
//! containers as you can spare.
//!
//! Find the minimum number of containers that can exactly fit all `150` liters of
//! eggnog. How many different ways can you fill that number of containers and still
//! hold exactly 150 litres?
//!
//! In the example above, the minimum number of containers was two. There were
//! three ways to use that many containers, and so the answer there would be 3.
//!

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_solution, valid_lines},
};
use anyhow::Result;
use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD17, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader, 150).unwrap_or_default()
}

fn find_br<T>(reader: T, amount: usize) -> Result<usize>
where
    T: BufRead,
{
    let mut containers = vec![];

    for line in valid_lines(reader) {
        let val = line.parse::<usize>()?;
        containers.push(val);
    }

    let mut results = vec![];
    for ps in containers.into_iter().powerset() {
        if ps.iter().sum::<usize>() == amount {
            results.push(ps);
        }
    }
    Ok(results.len())
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD17, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader, 150).unwrap_or_default()
}

fn find2_br<T>(reader: T, amount: usize) -> Result<usize>
where
    T: BufRead,
{
    let mut containers = vec![];

    for line in valid_lines(reader) {
        let val = line.parse::<usize>()?;
        containers.push(val);
    }

    let mut results = vec![];
    for ps in containers.into_iter().powerset() {
        if ps.iter().sum::<usize>() == amount {
            results.push(ps);
        }
    }
    let mut min = usize::MAX;
    for blah in &results {
        if blah.len() < min {
            min = blah.len();
        }
    }
    let num_min = results.iter().filter(|a| a.len() == min).count();
    Ok(num_min)
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"20
15
10
5
5";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1), 25)?, 4);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"20
15
10
5
5";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find2_br(Cursor::new(TEST_1), 25)?, 3);
        Ok(())
    }
}
