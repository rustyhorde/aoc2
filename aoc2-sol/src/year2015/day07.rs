// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 7 "Some Assembly Required"
//!
//! **--- Day 7: Some Assembly Required ---**
//!
//! **--- Part 1 ---**
//!
//! This year, Santa brought little Bobby Tables a set of wires and bitwise logic gates!
//! Unfortunately, little Bobby is a little under the recommended age range, and he
//! needs help assembling the circuit.
//!
//! Each wire has an identifier (some lowercase letters) and can carry a 16-bit signal
//! (a number from 0 to 65535). A signal is provided to each wire by a gate, another wire,
//! or some specific value. Each wire can only get a signal from one source, but can
//! provide its signal to multiple destinations. A gate provides no signal until all of its
//! inputs have a signal.
//!
//! The included instructions booklet describes how to connect the parts together:
//! `x AND y -> z` means to connect wires `x` and `y` to an `AND` gate, and then connect
//! its output to wire `z`.
//!
//! For example:
//!
//! * `123 -> x` means that the signal `123` is provided to wire `x`.
//! * `x AND y -> z` means that the bitwise `AND` of wire `x` and wire `y` is provided to wire `z`.
//! * `p LSHIFT 2 -> q` means that the value from wire `p` is left-shifted by 2 and then provided to wire `q`.
//! * `NOT e -> f` means that the bitwise complement of the value from wire `e` is provided to wire `f`.
//!
//! Other possible gates include `OR` (bitwise OR) and `RSHIFT` (right-shift).
//! If, for some reason, you'd like to emulate the circuit instead, almost all programming languages
//! (for example, C, JavaScript, or Python) provide operators for these gates.
//!
//! For example, here is a simple circuit:
//!
//! ```text
//! 123 -> x
//! 456 -> y
//! x AND y -> d
//! x OR y -> e
//! x LSHIFT 2 -> f
//! y RSHIFT 2 -> g
//! NOT x -> h
//! NOT y -> i
//! ```
//!
//! After it is run, these are the signals on the wires:
//!
//! ```text
//! d: 72
//! e: 507
//! f: 492
//! g: 114
//! h: 65412
//! i: 65079
//! x: 123
//! y: 456
//! ```
//!
//! In little Bobby's kit's instructions booklet (provided as your puzzle input), what signal is ultimately provided to wire a?

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
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD06, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader)
}

fn find_br<T>(reader: T) -> usize
where
    T: BufRead,
{
    for _line in valid_lines(reader) {}
    0
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
    for _line in valid_lines(reader) {}
    0
}

#[cfg(test)]
mod one_star {
    // use super::find_br;
    use anyhow::Result;
    // use std::io::Cursor;

    // const TEST_1: &str = r"turn on 0,0 through 999,999";
    // const TEST_2: &str = r"toggle 0,0 through 999,0";
    // const TEST_3: &str = r"turn on 0,0 through 999,999\nturn off 499,499 through 500,500";

    #[test]
    fn solution() -> Result<()> {
        // assert_eq!(find_br(Cursor::new(TEST_1))?, 1_000_000);
        // assert_eq!(find_br(Cursor::new(TEST_2))?, 1_000);
        // assert_eq!(find_br(Cursor::new(TEST_3))?, 999_996);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    // use super::find2_br;
    use anyhow::Result;
    // use std::io::Cursor;

    // const TEST_1: &str = r"turn on 0,0 through 0,0";
    // const TEST_2: &str = r"toggle 0,0 through 999,999";

    #[test]
    fn solution() -> Result<()> {
        // assert_eq!(find2_br(Cursor::new(TEST_1))?, 1);
        // assert_eq!(find2_br(Cursor::new(TEST_2))?, 2_000_000);
        Ok(())
    }
}
