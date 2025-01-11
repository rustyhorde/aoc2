// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2017 ---**
//!
//! **--- Day 9: Stream Processing ---**
//!
//! A large stream blocks your path. According to the locals, it's not safe to cross the stream at the moment because it's full of garbage. You look down at the stream; rather than water, you discover that it's a stream of characters.
//!
//! You sit for a while and record part of the stream (your puzzle input). The characters represent groups - sequences that begin with { and end with }. Within a group, there are zero or more other things, separated by commas: either another group or garbage. Since groups can contain other groups, a } only closes the most-recently-opened unclosed group - that is, they are nestable. Your puzzle input represents a single, large group which itself contains many smaller ones.
//!
//! Sometimes, instead of a group, you will find garbage. Garbage begins with < and ends with >. Between those angle brackets, almost any character can appear, including { and }. Within garbage, < has no special meaning.
//!
//! In a futile attempt to clean up the garbage, some program has canceled some of the characters within it using !: inside garbage, any character that comes after ! should be ignored, including <, >, and even another !.
//!
//! You don't see any characters that deviate from these rules. Outside garbage, you only find well-formed groups, and garbage always terminates according to the rules above.
//!
//! Here are some self-contained pieces of garbage:
//!
//! ```text
//!     <>, empty garbage.
//!     <random characters>, garbage containing random characters.
//!     <<<<>, because the extra < are ignored.
//!     <{!>}>, because the first > is canceled.
//!     <!!>, because the second ! is canceled, allowing the > to terminate the garbage.
//!     <!!!>>, because the second ! and the first > are canceled.
//!     <{o"i!a,<{i<a>, which ends at the first >.
//! ```
//!
//! Here are some examples of whole streams and the number of groups they contain:
//!
//! ```text
//!     {}, 1 group.
//!     {{{}}}, 3 groups.
//!     {{},{}}, also 3 groups.
//!     {{{},{},{{}}}}, 6 groups.
//!     {<{},{},{{}}>}, 1 group (which itself contains garbage).
//!     {<a>,<a>,<a>,<a>}, 1 group.
//!     {{<a>},{<a>},{<a>},{<a>}}, 5 groups.
//!     {{<!>},{<!>},{<!>},{<a>}}, 2 groups (since all but the last > are canceled).
//! ```
//!
//! Your goal is to find the total score for all groups in your input. Each group is assigned a score which is one more than the score of the group that immediately contains it. (The outermost group gets a score of 1.)
//!
//! ```text
//!     {}, score of 1.
//!     {{{}}}, score of 1 + 2 + 3 = 6.
//!     {{},{}}, score of 1 + 2 + 2 = 5.
//!     {{{},{},{{}}}}, score of 1 + 2 + 3 + 3 + 3 + 4 = 16.
//!     {<a>,<a>,<a>,<a>}, score of 1.
//!     {{<ab>},{<ab>},{<ab>},{<ab>}}, score of 1 + 2 + 2 + 2 + 2 = 9.
//!     {{<!!>},{<!!>},{<!!>},{<!!>}}, score of 1 + 2 + 2 + 2 + 2 = 9.
//!     {{<a!>},{<a!>},{<a!>},{<ab>}}, score of 1 + 2 = 3.
//! ```
//!
//! What is the total score for all groups in your input?
//!
//! --- Part Two ---
//!
//! Now, you're ready to remove the garbage.
//!
//! To prove you've removed it, you need to count all of the characters within the garbage. The leading and trailing < and > don't count, nor do any canceled characters or the ! doing the canceling.
//!
//! ```text
//!     <>, 0 characters.
//!     <random characters>, 17 characters.
//!     <<<<>, 3 characters.
//!     <{!>}>, 2 characters.
//!     <!!>, 0 characters.
//!     <!!!>>, 0 characters.
//!     <{o"i!a,<{i<a>, 10 characters.
//! ```
//!
//! How many non-canceled characters are within the garbage in your puzzle input?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::{anyhow, Result};
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
    run_setup_solution::<Vec<String>, usize>(AoCYear::AOC2017, AoCDay::AOCD09, setup, find)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<String>, usize>(bench, AoCYear::AOC2017, AoCDay::AOCD09, setup, find)
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

fn process_line_as_chars(line: &str) -> Result<(u32, u32)> {
    let mut scores: Vec<u32> = Vec::new();
    let mut current_nesting = 0;
    let mut garbage_count = 0;
    let mut in_garbage = false;
    let mut skip_next = false;

    for c in line.chars() {
        if skip_next {
            skip_next = false;
            continue;
        }
        match c {
            '{' if !in_garbage => {
                current_nesting += 1;
            }
            '}' if !in_garbage => {
                scores.push(current_nesting);
                current_nesting -= 1;
            }
            '<' if !in_garbage => {
                in_garbage = true;
            }
            '!' if in_garbage => {
                skip_next = true;
            }
            '>' if in_garbage => {
                in_garbage = false;
            }
            ',' if current_nesting > 0 && !in_garbage => {}
            _ if in_garbage => {
                garbage_count += 1;
            }
            _ => return Err(anyhow!("Unknown character encountered!")),
        }
    }
    Ok((scores.iter().sum(), garbage_count))
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: Vec<String>) -> usize {
    let mut score = (0, 0);
    for line in data {
        score = process_line_as_chars(&line).unwrap_or((0, 0));
    }
    usize::try_from(score.0).unwrap_or_default()
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<Vec<String>, usize>(AoCYear::AOC2017, AoCDay::AOCD09, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<String>, usize>(bench, AoCYear::AOC2017, AoCDay::AOCD09, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: Vec<String>) -> usize {
    let mut score = (0, 0);
    for line in data {
        score = process_line_as_chars(&line).unwrap_or((0, 0));
    }
    usize::try_from(score.1).unwrap_or_default()
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"{}";
    const TEST_2: &str = r"{{{}}}";
    const TEST_3: &str = r"{{},{}}";
    const TEST_4: &str = r"{{{},{},{{}}}}";
    const TEST_5: &str = r"{<a>,<a>,<a>,<a>}";
    const TEST_6: &str = r"{{<ab>},{<ab>},{<ab>},{<ab>}}";
    const TEST_7: &str = r"{{<!!>},{<!!>},{<!!>},{<!!>}}";
    const TEST_8: &str = r"{{<a!>},{<a!>},{<a!>},{<ab>}}";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find(data), 1);
        let data = setup_br(Cursor::new(TEST_2))?;
        assert_eq!(find(data), 6);
        let data = setup_br(Cursor::new(TEST_3))?;
        assert_eq!(find(data), 5);
        let data = setup_br(Cursor::new(TEST_4))?;
        assert_eq!(find(data), 16);
        let data = setup_br(Cursor::new(TEST_5))?;
        assert_eq!(find(data), 1);
        let data = setup_br(Cursor::new(TEST_6))?;
        assert_eq!(find(data), 9);
        let data = setup_br(Cursor::new(TEST_7))?;
        assert_eq!(find(data), 9);
        let data = setup_br(Cursor::new(TEST_8))?;
        assert_eq!(find(data), 3);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"<>";
    const TEST_2: &str = r"<random characters>";
    const TEST_3: &str = r"<<<<>";
    const TEST_4: &str = r"<{!>}>";
    const TEST_5: &str = r"<!!>";
    const TEST_6: &str = r"<!!!>>";
    const TEST_7: &str = r#"<{o"i!a,<{i<a>"#;

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find2(data), 0);
        let data = setup_br(Cursor::new(TEST_2))?;
        assert_eq!(find2(data), 17);
        let data = setup_br(Cursor::new(TEST_3))?;
        assert_eq!(find2(data), 3);
        let data = setup_br(Cursor::new(TEST_4))?;
        assert_eq!(find2(data), 2);
        let data = setup_br(Cursor::new(TEST_5))?;
        assert_eq!(find2(data), 0);
        let data = setup_br(Cursor::new(TEST_6))?;
        assert_eq!(find2(data), 0);
        let data = setup_br(Cursor::new(TEST_7))?;
        assert_eq!(find2(data), 10);
        Ok(())
    }
}
