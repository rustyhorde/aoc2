// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code - Day 3 ---**
//!
//! **--- Day 3: Mull It Over ---**
//!
//! "Our computers are having issues, so I have no idea if we have any Chief Historians in stock! You're welcome to check the warehouse, though," says the mildly flustered shopkeeper at the North Pole Toboggan Rental Shop. The Historians head out to take a look.
//!
//! The shopkeeper turns to you. "Any chance you can see why our computers are having issues again?"
//!
//! The computer appears to be trying to run a program, but its memory (your puzzle input) is corrupted. All of the instructions have been jumbled up!
//!
//! It seems like the goal of the program is just to multiply some numbers. It does that with instructions like mul(X,Y), where X and Y are each 1-3 digit numbers. For instance, mul(44,46) multiplies 44 by 46 to get a result of 2024. Similarly, mul(123,4) would multiply 123 by 4.
//!
//! However, because the program's memory has been corrupted, there are also many invalid characters that should be ignored, even if they look like part of a mul instruction. Sequences like mul(4*, mul(6,9!, ?(12,34), or mul ( 2 , 4 ) do nothing.
//!
//! For example, consider the following section of corrupted memory:
//!
//! `xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))`
//!
//! Only the four highlighted sections are real mul instructions. Adding up the result of each instruction produces 161 (2*4 + 5*5 + 11*8 + 8*5).
//!
//! Scan the corrupted memory for uncorrupted mul instructions. What do you get if you add up all of the results of the multiplications?
//!
//! **--- Part Two ---**
//!
//! As you scan through the corrupted memory, you notice that some of the conditional statements are also still intact. If you handle some of the uncorrupted conditional statements in the program, you might be able to get an even more accurate result.
//!
//! There are two new instructions you'll need to handle:
//!
//! ```text
//!     The do() instruction enables future mul instructions.
//!     The don't() instruction disables future mul instructions.
//! ```
//!
//! Only the most recent `do()` or `don't()` instruction applies. At the beginning of the program, mul instructions are enabled.
//!
//! For example:
//!
//! `xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))`
//!
//! This corrupted memory is similar to the example from before, but this time the mul(5,5) and mul(11,8) instructions are disabled because there is a `don't()` instruction before them. The other mul instructions function normally, including the one at the end that gets re-enabled by a `do()` instruction.
//!
//! This time, the sum of the results is 48 (2*4 + 8*5).
//!
//! Handle the new instructions; what do you get if you add up all of the results of just the enabled multiplications?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_solution, valid_lines},
};
use anyhow::Result;
use regex::Regex;
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
    run_solution::<usize>(AoCYear::AOC2024, AoCDay::AOCD03, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader).unwrap_or_default()
}

fn find_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let mut total = 0;

    for line in valid_lines(reader) {
        total += run_mults(&line)?;
    }
    Ok(total)
}

fn run_mults(input: &str) -> Result<usize> {
    let mut total = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;
    for (_full, [l, r]) in re.captures_iter(input).map(|c| c.extract()) {
        let l_val = l.parse::<usize>()?;
        let r_val = r.parse::<usize>()?;
        total += l_val * r_val;
    }
    Ok(total)
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2024, AoCDay::AOCD03, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader).unwrap_or_default()
}

fn find2_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let do_re = Regex::new(r".*?do\(\)(.*)")?;
    let mut total = 0;
    for line in valid_lines(reader) {
        let parts = line.split("don't()").collect::<Vec<&str>>();
        for (idx, ins) in parts.iter().enumerate() {
            if idx == 0 {
                total += run_mults(ins)?;
            } else {
                for (_full, [after_do]) in do_re.captures_iter(ins).map(|c| c.extract()) {
                    total += run_mults(after_do)?;
                }
            }
        }
    }
    Ok(total)
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST_2: &str =
        r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8001,5))";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1))?, 161);
        assert_eq!(find_br(Cursor::new(TEST_2))?, 121);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str =
        r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    const TEST_2: &str = r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8000,5))jfjeka;jkjkfedon't()do()don't()do()do()mul(101,2)";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find2_br(Cursor::new(TEST_1))?, 48);
        assert_eq!(find2_br(Cursor::new(TEST_2))?, 210);
        Ok(())
    }
}
