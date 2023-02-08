// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 10 "Elves Look, Elves Say"
//!
//! **--- Day 10: Elves Look, Elves Say ---**
//!
//! **--- Part 1 ---**
//!
//! Today, the Elves are playing a game called look-and-say. They take turns making
//! sequences by reading aloud the previous sequence and using that reading as the next sequence.
//! For example, 211 is read as "one two, two ones", which becomes 1221 (1 2, 2 1s).
//!
//! Look-and-say sequences are generated iteratively, using the previous value as input for the next step.
//! For each step, take the previous value, and replace each run of digits (like 111)
//! with the number of digits (3) followed by the digit itself (1).
//!
//! For example:
//!
//! ```text
//! 1 becomes 11 (1 copy of digit 1).
//! 11 becomes 21 (2 copies of digit 1).
//! 21 becomes 1211 (one 2 followed by one 1).
//! 1211 becomes 111221 (one 1, one 2, and two 1s).
//! 111221 becomes 312211 (three 1s, two 2s, and one 1).
//! ```
//!
//! Starting with the digits in your puzzle input, apply this process 40 times. What is the length of the result?
//!
//! **--- Part Two ---**
//!
//! Neat, right? You might also enjoy hearing `John Conway` talking about this sequence
//! (that's Conway of Conway's Game of Life fame).
//!
//! Now, starting again with the digits in your puzzle input, apply this process 50 times.
//! What is the length of the new result?

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
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD10, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader, 40)
}

fn find_br<T>(reader: T, count: usize) -> usize
where
    T: BufRead,
{
    let mut in_buf = String::new();

    for line in valid_lines(reader) {
        in_buf = line;
        let mut out_buf = String::new();

        for _ in 0..count {
            let mut count = 1;
            let mut iter = in_buf.chars().peekable();

            loop {
                let curr = iter.next();
                let next = iter.peek();

                if curr.is_none() {
                    break;
                } else if let (Some(next), Some(curr)) = (next, curr) {
                    if curr == *next {
                        count += 1;
                        continue;
                    }
                    flush(&mut out_buf, &mut count, curr);
                } else if let (None, Some(curr)) = (next, curr) {
                    flush(&mut out_buf, &mut count, curr);
                }
            }
            in_buf = out_buf.clone();
            out_buf.clear();
        }
    }
    in_buf.len()
}

fn flush(buf: &mut String, count: &mut usize, curr: char) {
    buf.push_str(&count.to_string());
    buf.push(curr);
    *count = 1;
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD10, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find_br(reader, 50)
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use std::io::Cursor;

    const TEST_1: &str = r"1";
    const TEST_2: &str = r"11";
    const TEST_3: &str = r"21";
    const TEST_4: &str = r"1211";
    const TEST_5: &str = r"111221";

    #[test]
    fn solution() {
        assert_eq!(find_br(Cursor::new(TEST_1), 1), 2);
        assert_eq!(find_br(Cursor::new(TEST_2), 1), 2);
        assert_eq!(find_br(Cursor::new(TEST_3), 1), 4);
        assert_eq!(find_br(Cursor::new(TEST_4), 1), 6);
        assert_eq!(find_br(Cursor::new(TEST_5), 1), 6);
    }
}

#[cfg(test)]
mod two_star {
    use super::find_br;
    use std::io::Cursor;

    const TEST_1: &str = r"1";
    const TEST_2: &str = r"11";
    const TEST_3: &str = r"21";
    const TEST_4: &str = r"1211";
    const TEST_5: &str = r"111221";

    #[test]
    fn solution() {
        assert_eq!(find_br(Cursor::new(TEST_1), 1), 2);
        assert_eq!(find_br(Cursor::new(TEST_2), 1), 2);
        assert_eq!(find_br(Cursor::new(TEST_3), 1), 4);
        assert_eq!(find_br(Cursor::new(TEST_4), 1), 6);
        assert_eq!(find_br(Cursor::new(TEST_5), 1), 6);
    }
}
