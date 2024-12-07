// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code - Day 1 ---**
//!
//! **--- Day 7: Bridge Repair ---**
//!
//! The Historians take you to a familiar rope bridge over a river in the middle of a jungle. The Chief isn't on this side of the bridge, though; maybe he's on the other side?
//!
//! When you go to cross the bridge, you notice a group of engineers trying to repair it. (Apparently, it breaks pretty frequently.) You won't be able to cross until it's fixed.
//!
//! You ask how long it'll take; the engineers tell you that it only needs final calibrations, but some young elephants were playing nearby and stole all the operators from their calibration equations! They could finish the calibrations if only someone could determine which test values could possibly be produced by placing any combination of operators into their calibration equations (your puzzle input).
//!
//! For example:
//!
//! ```text
//! 190: 10 19
//! 3267: 81 40 27
//! 83: 17 5
//! 156: 15 6
//! 7290: 6 8 6 15
//! 161011: 16 10 13
//! 192: 17 8 14
//! 21037: 9 7 18 13
//! 292: 11 6 16 20
//! ```
//!
//! Each line represents a single equation. The test value appears before the colon on each line; it is your job to determine whether the remaining numbers can be combined with operators to produce the test value.
//!
//! Operators are always evaluated left-to-right, not according to precedence rules. Furthermore, numbers in the equations cannot be rearranged. Glancing into the jungle, you can see elephants holding two different types of operators: add (+) and multiply (*).
//!
//! Only three of the above equations can be made true by inserting operators:
//!
//! ```text
//!     190: 10 19 has only one position that accepts an operator: between 10 and 19. Choosing + would give 29, but choosing * would give the test value (10 * 19 = 190).
//!     3267: 81 40 27 has two positions for operators. Of the four possible configurations of the operators, two cause the right side to match the test value: 81 + 40 * 27 and 81 * 40 + 27 both equal 3267 (when evaluated left-to-right)!
//!     292: 11 6 16 20 can be solved in exactly one way: 11 + 6 * 16 + 20.
//! ```
//!
//! The engineers just need the total calibration result, which is the sum of the test values from just the equations that could possibly be true. In the above example, the sum of the test values for the three equations listed above is 3749.
//!
//! Determine which equations could possibly be true. What is their total calibration result?
//!
//! **--- Part Two ---**
//!
//! The engineers seem concerned; the total calibration result you gave them is nowhere close to being within safety tolerances. Just then, you spot your mistake: some well-hidden elephants are holding a third type of operator.
//!
//! The concatenation operator (||) combines the digits from its left and right inputs into a single number. For example, 12 || 345 would become 12345. All operators are still evaluated left-to-right.
//!
//! Now, apart from the three equations that could be made true using only addition and multiplication, the above example has three more equations that can be made true by inserting operators:
//!
//! ```text
//!     156: 15 6 can be made true through a single concatenation: 15 || 6 = 156.
//!     7290: 6 8 6 15 can be made true using 6 * 8 || 6 * 15.
//!     192: 17 8 14 can be made true using 17 || 8 + 14.
//! ```
//!
//! Adding up all six test values (the three that could be made before using only + and * plus the new three that can now be made by also using ||) produces the new total calibration result of 11387.
//!
//! Using your new knowledge of elephant hiding spots, determine which equations could possibly be true. What is their total calibration result?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_setup_solution, valid_lines},
};
use anyhow::Result;
use std::{
    collections::VecDeque,
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
    run_setup_solution::<Vec<(usize, Vec<usize>)>, usize>(
        AoCYear::AOC2024,
        AoCDay::AOCD07,
        setup,
        find,
    )
    .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> Vec<(usize, Vec<usize>)> {
    setup_br(reader).unwrap_or_default()
}

fn setup_br<T>(reader: T) -> Result<Vec<(usize, Vec<usize>)>>
where
    T: BufRead,
{
    let mut data = vec![];
    for line in valid_lines(reader) {
        let parts = line.split(": ").collect::<Vec<&str>>();
        let total = parts[0].parse::<usize>()?;
        let vals = parts[1]
            .split(' ')
            .map(str::parse::<usize>)
            .filter_map(Result::ok)
            .collect::<Vec<usize>>();
        data.push((total, vals));
    }
    Ok(data)
}

fn find(data: Vec<(usize, Vec<usize>)>) -> usize {
    let mut valid_total = 0;
    for (total, vals) in data {
        let mut curr_totals = VecDeque::new();
        curr_totals.push_back(0);
        let mut after_first = false;
        for next_val in vals {
            let mut new_totals = vec![];
            while let Some(curr) = curr_totals.pop_front() {
                new_totals.push(curr + next_val);
                let mul = if !after_first && curr == 0 { 1 } else { curr };
                new_totals.push(mul * next_val);
            }
            for total in new_totals {
                curr_totals.push_back(total);
            }
            after_first = true;
        }
        if curr_totals.contains(&total) {
            valid_total += total;
        }
    }
    valid_total
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<Vec<(usize, Vec<usize>)>, usize>(
        AoCYear::AOC2024,
        AoCDay::AOCD07,
        setup,
        find2,
    )
    .map(|_| 0)
}

fn find2(data: Vec<(usize, Vec<usize>)>) -> usize {
    let mut valid_total = 0;
    for (total, vals) in data {
        let mut curr_totals = VecDeque::new();
        curr_totals.push_back(0);
        let mut after_first = false;
        for next_val in vals {
            let mut new_totals = vec![];
            while let Some(curr) = curr_totals.pop_front() {
                new_totals.push(curr + next_val);
                let mul = if !after_first && curr == 0 { 1 } else { curr };
                new_totals.push(mul * next_val);
                if after_first {
                    if let Ok(val) = format!("{curr}{next_val}").parse::<usize>() {
                        new_totals.push(val);
                    }
                }
            }
            for total in new_totals {
                curr_totals.push_back(total);
            }
            after_first = true;
        }
        if curr_totals.contains(&total) {
            valid_total += total;
        }
    }
    valid_total
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find(data), 3749);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find2(data), 11387);
        Ok(())
    }
}
