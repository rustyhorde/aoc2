// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Inverse Captcha
//!
//! **--- Day 1: Inverse Captcha ---**
//!
//! **--- Part 1 ---**
//!
//! The night before Christmas, one of Santa's Elves calls you in a panic.
//! "The printer's broken! We can't print the Naughty or Nice List!" By the time
//! you make it to sub-basement 17, there are only a few minutes until midnight.
//! "We have a big problem," she says; "there must be almost fifty bugs in this
//! system, but nothing else can print The List. Stand in this square, quick!
//! There's no time to explain; if you can convince them to pay you in stars,
//! you'll be able to--" She pulls a lever and the world goes blurry.
//!
//! When your eyes can focus again, everything seems a lot more pixelated than
//! before. She must have sent you inside the computer! You check the system clock:
//! 25 milliseconds until midnight. With that much time, you should be able
//! to collect all fifty stars by December 25th.
//!
//! Collect stars by solving puzzles. Two puzzles will be made available on each
//! ~~day~~ millisecond in the Advent calendar; the second puzzle is unlocked
//! when you complete the first. Each puzzle grants one star. Good luck!
//!
//! You're standing in a room with "digitization quarantine" written in LEDs
//! along one wall. The only door is locked, but it includes a small interface.
//! "Restricted Area - Strictly No Digitized Users Allowed."
//!
//! It goes on to explain that you may only leave by solving a captcha to prove
//! you're not a human. Apparently, you only get one millisecond to solve the
//! captcha: too fast for a normal human, but it feels like hours to you.
//!
//! The captcha requires you to review a sequence of digits (your puzzle input)
//! and find the sum of all digits that match the next digit in the list. The list
//! is circular, so the digit after the last digit is the first digit in the list.
//!
//! For example:
//!
//! ```text
//! 1122 produces a sum of 3 (1 + 2) because the first digit (1) matches the second digit and the third digit (2) matches the fourth digit.
//! 1111 produces 4 because each digit (all 1) matches the next.
//! 1234 produces 0 because no digit matches the next.
//! 91212129 produces 9 because the only digit that matches the next one is the last digit, 9.
//! ```
//!
//! What is the solution to your captcha?
//!
//! **--- Part Two ---**
//!
//! You notice a progress bar that jumps to 50% completion. Apparently, the door
//! isn't yet satisfied, but it did emit a star as encouragement. The instructions change:
//!
//! Now, instead of considering the next digit, it wants you to consider the digit
//! halfway around the circular list. That is, if your list contains `10` items, only
//! include a digit in your sum if the digit `10/2 = 5` steps forward matches it. Fortunately,
//! your list has an even number of elements.
//!
//! For example:
//!
//! ```text
//! 1212 produces 6: the list contains 4 items, and all four digits match the digit 2 items ahead.
//! 1221 produces 0, because every comparison is between a 1 and a 2.
//! 123425 produces 4, because both 2s match each other, but no other digit has a match.
//! 123123 produces 12.
//! 12131415 produces 4.
//! ```
//!
//! What is the solution to your new captcha?

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
    run_solution::<u32>(AoCYear::AOC2017, AoCDay::AOCD01, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> u32 {
    find_br(reader)
}

fn find_br<T>(reader: T) -> u32
where
    T: BufRead,
{
    inverse_captcha(reader, false)
}

fn inverse_captcha<T>(reader: T, part2: bool) -> u32
where
    T: BufRead,
{
    let mut digits = vec![];
    for line in valid_lines(reader) {
        digits = line
            .chars()
            .filter_map(|x| x.to_digit(10))
            .collect::<Vec<u32>>();
    }

    let mut sum = 0;
    let len = digits.len();
    let la_idx = if part2 { len / 2 } else { 1 };

    for (idx, curr) in digits.iter().enumerate() {
        let next_idx = (idx + la_idx) % len;

        if *curr == digits[next_idx] {
            sum += digits[idx];
        }
    }
    sum
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<u32>(AoCYear::AOC2017, AoCDay::AOCD01, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> u32 {
    find2_br(reader)
}

fn find2_br<T>(reader: T) -> u32
where
    T: BufRead,
{
    inverse_captcha(reader, true)
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"1122";
    const TEST_2: &str = r"1111";
    const TEST_3: &str = r"1234";
    const TEST_4: &str = r"91212129";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1)), 3);
        assert_eq!(find_br(Cursor::new(TEST_2)), 4);
        assert_eq!(find_br(Cursor::new(TEST_3)), 0);
        assert_eq!(find_br(Cursor::new(TEST_4)), 9);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"1212";
    const TEST_2: &str = r"1221";
    const TEST_3: &str = r"123425";
    const TEST_4: &str = r"123123";
    const TEST_5: &str = r"12131415";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find2_br(Cursor::new(TEST_1)), 6);
        assert_eq!(find2_br(Cursor::new(TEST_2)), 0);
        assert_eq!(find2_br(Cursor::new(TEST_3)), 4);
        assert_eq!(find2_br(Cursor::new(TEST_4)), 12);
        assert_eq!(find2_br(Cursor::new(TEST_5)), 4);
        Ok(())
    }
}
