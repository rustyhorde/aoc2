// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 8 "Matchsticks"
//!
//! **--- Day 8: Matchsticks ---**
//!
//! **--- Part 1 ---**
//!
//! Space on the sleigh is limited this year, and so Santa will be bringing his
//! list as a digital copy. He needs to know how much space it will take up when stored.
//!
//! It is common in many programming languages to provide a way to escape
//! special characters in strings. For example, C, JavaScript, Perl, Python, and
//! even PHP handle special characters in very similar ways.
//!
//! However, it is important to realize the difference between the number
//! of characters in the code representation of the string literal and the number
//! of characters in the in-memory string itself.
//!
//! For example:
//!
//! * `""` is 2 characters of code (the two double quotes), but the string contains zero characters.
//! * `"abc"` is 5 characters of code, but 3 characters in the string data.
//! * `"aaa\"aaa"` is 10 characters of code, but the string itself contains six "a" characters
//! and a single, escaped quote character, for a total of 7 characters in the string data.
//! * `"\x27"` is 6 characters of code, but the string itself contains just one
//! - an apostrophe ('), escaped using hexadecimal notation.
//!
//! Santa's list is a file that contains many double-quoted string literals, one on each line.
//! The only escape sequences used are \\ (which represents a single backslash),
//! \" (which represents a lone double-quote character), and \x plus two hexadecimal characters
//! (which represents a single character with that ASCII code).
//!
//! Disregarding the whitespace in the file, what is the number of characters of code for string
//! literals minus the number of characters in memory for the values of the strings in total for the entire file?
//!
//! For example, given the four strings above, the total number of characters of string
//! code `(2 + 5 + 10 + 6 = 23)` minus the total number of characters in memory for string values
//! `(0 + 3 + 7 + 1 = 11)` is `23 - 11 = 12`.
//!
//! **--- Part Two ---**
//!
//! Now, let's go the other way. In addition to finding the number of characters of code,
//! you should now encode each code representation as a new string and find the number of
//! characters of the new encoded representation, including the surrounding double quotes.
//!
//! For example:
//!
//! * `""` encodes to `"\"\""`, an increase from 2 characters to 6.
//! * `"abc"` encodes to `"\"abc\""`, an increase from 5 characters to 9.
//! * `"aaa\"aaa"` encodes to `"\"aaa\\\"aaa\""`, an increase from 10 characters to 16.
//! * `"\x27"` encodes to `"\"\\x27\""`, an increase from 6 characters to 11.
//!
//! Your task is to find the total number of characters to represent the newly encoded strings
//! minus the number of characters of code in each original string literal. For example,
//! for the strings above, the total encoded length `(6 + 9 + 16 + 11 = 42)` minus
//! the characters in the original code representation (23, just like in the first
//! part of this puzzle) is 42 - 23 = 19.

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
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD08, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader).unwrap_or_default()
}

fn find_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let mut total_count = 0;
    let mut actual_count = 0;
    let bs_q_re = Regex::new(r#"\\""#)?;
    let bs_bs_re = Regex::new(r#"\\\\"#)?;
    let ascii_re = Regex::new(r#"\\x[0-9a-f]{2}"#)?;

    for line in valid_lines(reader) {
        total_count += line.chars().count();
        let trimmed = line.trim_matches('"');
        actual_count += trimmed.chars().count();
        let escaped_quote_count = bs_q_re.find_iter(trimmed).count();
        actual_count -= escaped_quote_count;
        let escaped_bs_count = bs_bs_re.find_iter(trimmed).count();
        actual_count -= escaped_bs_count;
        let ascii_count = ascii_re.find_iter(trimmed).count();
        actual_count -= 3 * ascii_count;
    }
    Ok(total_count - actual_count)
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD08, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader)
}

fn find2_br<T>(reader: T) -> usize
where
    T: BufRead,
{
    let mut total_count = 0;
    let mut encoded_count = 0;

    for line in valid_lines(reader) {
        total_count += line.chars().count();

        if let Some(buf) = line
            .chars()
            .scan(vec![], |acc, ch| {
                if ch == '"' {
                    acc.push('\\');
                    acc.push('"');
                } else if ch == '\\' {
                    acc.push('\\');
                    acc.push('\\');
                } else {
                    acc.push(ch);
                }
                Some(acc.clone())
            })
            .last()
        {
            encoded_count += buf.len() + 2;
        }
    }
    encoded_count - total_count
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r#"""
"abc"
"aaa\"aaa"
"\x27""#;

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1))?, 12);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use std::io::Cursor;

    const TEST_1: &str = r#"""
"abc"
"aaa\"aaa"
"\x27""#;

    #[test]
    fn solution() {
        assert_eq!(find2_br(Cursor::new(TEST_1)), 19);
    }
}
