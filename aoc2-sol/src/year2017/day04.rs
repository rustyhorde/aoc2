// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! High-Entropy Passphrases
//!
//! **--- Day 4: High-Entropy Passphrases ---**
//!
//! **--- Part 1 ---**
//!
//! A new system policy has been put in place that requires all accounts to
//! use a passphrase instead of simply a password. A passphrase consists
//! of a series of words (lowercase letters) separated by spaces.
//!
//! To ensure security, a valid passphrase must contain no duplicate words.
//!
//! For example:
//!
//! ```text
//! aa bb cc dd ee is valid.
//! aa bb cc dd aa is not valid - the word aa appears more than once.
//! aa bb cc dd aaa is valid - aa and aaa count as different words.
//! ```
//!
//! The system's full passphrase list is available as your puzzle input.
//! How many passphrases are valid?
//!
//! **--- Part Two ---**
//!
//! For added security, yet another system policy has been put in place. Now,
//! a valid passphrase must contain no two words that are anagrams of each other
//! - that is, a passphrase is invalid if any word's letters can be rearranged
//!   to form any other word in the passphrase.
//!
//! For example:
//!
//! ```text
//! abcde fghij is a valid passphrase.
//! abcde xyz ecdab is not valid - the letters from the third word can be rearranged to form the first word.
//! a ab abc abd abf abj is a valid passphrase, because all letters need to be used when forming another word.
//! iiii oiii ooii oooi oooo is valid.
//! oiii ioii iioi iiio is not valid - any of these words can be rearranged to form any other word.
//! ```
//!
//! Under this new system policy, how many passphrases are valid?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_solution, valid_lines},
};
use anyhow::Result;
use itertools::Itertools;
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
    run_solution::<usize>(AoCYear::AOC2017, AoCDay::AOCD04, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader)
}

fn find_br<T>(reader: T) -> usize
where
    T: BufRead,
{
    check_password(reader, false)
}

fn check_password<T>(reader: T, part2: bool) -> usize
where
    T: BufRead,
{
    let mut count = 0;
    for line in valid_lines(reader) {
        if (part2 && check_for_anagrams(&line)) || (!part2 && check_for_duplicates(&line)) {
            count += 1;
        }
    }
    count
}

/// Check each passphrase for the same word an toss out any that violate.
fn check_for_duplicates(line: &str) -> bool {
    let mut word_set = HashSet::new();

    for word in line.split_whitespace() {
        if !word_set.insert(word) {
            return false;
        }
    }
    true
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2017, AoCDay::AOCD04, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader)
}

fn find2_br<T>(reader: T) -> usize
where
    T: BufRead,
{
    check_password(reader, true)
}

fn check_for_anagrams(line: &str) -> bool {
    let mut word_set = HashSet::new();

    for word in line.split_whitespace() {
        let s = word.chars().sorted_by(|a, b| b.cmp(a)).collect::<String>();
        if !word_set.insert(s) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use std::io::Cursor;

    const TEST_1: &str = r"aa bb cc dd ee
aa bb cc dd aa
aa bb cc dd aaa";

    #[test]
    fn solution() {
        assert_eq!(find_br(Cursor::new(TEST_1)), 2);
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
