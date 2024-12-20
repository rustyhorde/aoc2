// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code - Day 2 ---**
//!
//! **--- Day 2: Red-Nosed Reports ---**
//!
//! Fortunately, the first location The Historians want to search isn't a long walk from the Chief Historian's office.
//!
//! While the Red-Nosed Reindeer nuclear fusion/fission plant appears to contain no sign of the Chief Historian, the engineers there run up to you as soon as they see you. Apparently, they still talk about the time Rudolph was saved through molecular synthesis from a single electron.
//!
//! They're quick to add that - since you're already here - they'd really appreciate your help analyzing some unusual data from the Red-Nosed reactor. You turn to check if The Historians are waiting for you, but they seem to have already divided into groups that are currently searching every corner of the facility. You offer to help with the unusual data.
//!
//! The unusual data (your puzzle input) consists of many reports, one report per line. Each report is a list of numbers called levels that are separated by spaces. For example:
//!
//! ```text
//! 7 6 4 2 1
//! 1 2 7 8 9
//! 9 7 6 2 1
//! 1 3 2 4 5
//! 8 6 4 4 1
//! 1 3 6 7 9
//! ```
//!
//! This example data contains six reports each containing five levels.
//!
//! The engineers are trying to figure out which reports are safe. The Red-Nosed reactor safety systems can only tolerate levels that are either gradually increasing or gradually decreasing. So, a report only counts as safe if both of the following are true:
//!
//! ```text
//!     The levels are either all increasing or all decreasing.
//!     Any two adjacent levels differ by at least one and at most three.
//! ```
//!
//! In the example above, the reports can be found safe or unsafe by checking those rules:
//!
//! ```text
//!     7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
//!     1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
//!     9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
//!     1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
//!     8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
//!     1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.
//! ```
//!
//! So, in this example, 2 reports are safe.
//!
//! Analyze the unusual data from the engineers. How many reports are safe?
//!
//! **--- Part Two ---**
//!
//! The engineers are surprised by the low number of safe reports until they realize they forgot to tell you about the Problem Dampener.
//!
//! The Problem Dampener is a reactor-mounted module that lets the reactor safety systems tolerate a single bad level in what would otherwise be a safe report. It's like the bad level never happened!
//!
//! Now, the same rules apply as before, except if removing a single level from an unsafe report would make it safe, the report instead counts as safe.
//!
//! More of the above example's reports are now safe:
//!
//! ```text
//!     7 6 4 2 1: Safe without removing any level.
//!     1 2 7 8 9: Unsafe regardless of which level is removed.
//!     9 7 6 2 1: Unsafe regardless of which level is removed.
//!     1 3 2 4 5: Safe by removing the second level, 3.
//!     8 6 4 4 1: Safe by removing the third level, 4.
//!     1 3 6 7 9: Safe without removing any level.
//! ```
//!
//! Thanks to the Problem Dampener, 4 reports are actually safe!
//!
//! Update your analysis by handling situations where the Problem Dampener can remove a single level from unsafe reports. How many reports are now safe?

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
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2024, AoCDay::AOCD02, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader)
}

fn find_br<T>(reader: T) -> usize
where
    T: BufRead,
{
    let mut safe = 0;
    for line in valid_lines(reader) {
        let levels = line
            .split(' ')
            .map(str::parse::<isize>)
            .filter_map(Result::ok)
            .collect::<Vec<isize>>();
        if check_safety(&levels) {
            safe += 1;
        }
    }
    safe
}

fn check_safety(levels: &[isize]) -> bool {
    let mut levels_iter = levels.iter().peekable();
    let mut curr_opt = levels_iter.next();
    let mut is_safe = true;
    let mut mode = Mode::Unk;
    while let Some(curr) = curr_opt {
        if let Some(next) = levels_iter.peek() {
            if **next < *curr && mode == Mode::Unk {
                mode = Mode::Dec;
            } else if **next > *curr && mode == Mode::Unk {
                mode = Mode::Inc;
            } else if mode == Mode::Unk {
                is_safe = false;
                break;
            }

            if mode == Mode::Dec {
                if **next >= *curr || **next < *curr - 3 {
                    is_safe = false;
                    break;
                }
            } else if mode == Mode::Inc {
                if **next <= *curr || **next > *curr + 3 {
                    is_safe = false;
                    break;
                }
            } else {
                break;
            }
        }
        curr_opt = levels_iter.next();
    }
    is_safe
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Mode {
    Inc,
    Dec,
    Unk,
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2024, AoCDay::AOCD02, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader)
}

fn find2_br<T>(reader: T) -> usize
where
    T: BufRead,
{
    let mut safe = 0;
    for line in valid_lines(reader) {
        let levels = line
            .split(' ')
            .map(str::parse::<isize>)
            .filter_map(Result::ok)
            .collect::<Vec<isize>>();
        if check_safety(&levels) {
            safe += 1;
        } else {
            for i in 0..levels.len() {
                let mut blah = levels.clone();
                let _smaller = blah.remove(i);
                if check_safety(&blah) {
                    safe += 1;
                    break;
                }
            }
        }
    }
    safe
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use std::io::Cursor;

    const TEST_1: &str = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn solution() {
        assert_eq!(find_br(Cursor::new(TEST_1)), 2);
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use std::io::Cursor;

    const TEST_1: &str = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    const TEST_2: &str = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
7 7 4 2 1
1 2 4 7 7";

    #[test]
    fn solution() {
        assert_eq!(find2_br(Cursor::new(TEST_1)), 4);
        assert_eq!(find2_br(Cursor::new(TEST_2)), 6);
    }
}
