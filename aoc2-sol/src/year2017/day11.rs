// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2017 ---**
//!
//! **--- Day 11: Hex Ed ---**
//!
//! Crossing the bridge, you've barely reached the other side of the stream when a program comes up to you, clearly in distress. "It's my child process," she says, "he's gotten lost in an infinite grid!"
//!
//! Fortunately for her, you have plenty of experience with infinite grids.
//!
//! Unfortunately for you, it's a hex grid.
//!
//! The hexagons ("hexes") in this grid are aligned such that adjacent hexes can be found to the north, northeast, southeast, south, southwest, and northwest:
//!
//! ```text
//!   \ n  /
//! nw +--+ ne
//!   /    \
//! -+      +-
//!   \    /
//! sw +--+ se
//!   / s  \
//! ```
//!
//! You have the path the child process took. Starting where he started, you need to determine the fewest number of steps required to reach him. (A "step" means to move from the hex you are in to any adjacent hex.)
//!
//! For example:
//!
//! ```text
//!     ne,ne,ne is 3 steps away.
//!     ne,ne,sw,sw is 0 steps away (back where you started).
//!     ne,ne,s,s is 2 steps away (se,se).
//!     se,sw,se,sw,sw is 3 steps away (s,s,sw).
//! ```
//!
//! --- Part Two ---
//!
//! How many steps away is the furthest he ever got from his starting position?

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
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<Vec<String>, u32>(AoCYear::AOC2017, AoCDay::AOCD11, setup, find).map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<String>, u32>(bench, AoCYear::AOC2017, AoCDay::AOCD11, setup, find)
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

#[allow(clippy::needless_pass_by_value)]
fn find(data: Vec<String>) -> u32 {
    let mut result = 0;
    for line in data {
        result = parse_and_go(&line, false).unwrap();
    }

    result
}

/// Parse the input and go.
fn parse_and_go(line: &str, second_star: bool) -> Result<u32> {
    let steps: Vec<&str> = line.split(',').collect();
    let mut coords = (0, 0, 0);
    let mut max_distance = 0;

    for step in steps {
        move_in_direction(step, &mut coords)?;
        let curr_distance = manhattan_distance_from_origin(coords)?;

        if curr_distance > max_distance {
            max_distance = curr_distance;
        }
    }

    if second_star {
        Ok(max_distance)
    } else {
        Ok(manhattan_distance_from_origin(coords)?)
    }
}

/// Calculate manhattan distance
fn manhattan_distance_from_origin(coords: (i32, i32, i32)) -> Result<u32> {
    let distance = (coords.0.abs() + coords.1.abs() + coords.2.abs()) / 2;
    Ok(TryFrom::try_from(distance)?)
}

/// Adjust the coordinates given a movement command.
fn move_in_direction(direction: &str, coords: &mut (i32, i32, i32)) -> Result<()> {
    match direction {
        "n" => {
            coords.1 += 1;
            coords.2 -= 1;
        }
        "ne" => {
            coords.0 += 1;
            coords.2 -= 1;
        }
        "se" => {
            coords.0 += 1;
            coords.1 -= 1;
        }
        "s" => {
            coords.1 -= 1;
            coords.2 += 1;
        }
        "sw" => {
            coords.0 -= 1;
            coords.2 += 1;
        }
        "nw" => {
            coords.0 -= 1;
            coords.1 += 1;
        }
        _ => return Err(anyhow!("Invalid movement direction")),
    }
    Ok(())
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<Vec<String>, u32>(AoCYear::AOC2017, AoCDay::AOCD11, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<String>, u32>(bench, AoCYear::AOC2017, AoCDay::AOCD11, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: Vec<String>) -> u32 {
    let mut result = 0;
    for line in data {
        result = parse_and_go(&line, true).unwrap();
    }

    result
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"ne,ne,ne";
    const TEST_2: &str = r"ne,ne,sw,sw";
    const TEST_3: &str = r"ne,ne,s,s";
    const TEST_4: &str = r"se,sw,se,sw,sw";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find(data), 3);
        let data = setup_br(Cursor::new(TEST_2))?;
        assert_eq!(find(data), 0);
        let data = setup_br(Cursor::new(TEST_3))?;
        assert_eq!(find(data), 2);
        let data = setup_br(Cursor::new(TEST_4))?;
        assert_eq!(find(data), 3);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"ne,ne,ne";
    const TEST_2: &str = r"ne,ne,sw,sw";
    const TEST_3: &str = r"ne,ne,s,s";
    const TEST_4: &str = r"se,sw,se,sw,sw";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find2(data), 3);
        let data = setup_br(Cursor::new(TEST_2))?;
        assert_eq!(find2(data), 2);
        let data = setup_br(Cursor::new(TEST_3))?;
        assert_eq!(find2(data), 2);
        let data = setup_br(Cursor::new(TEST_4))?;
        assert_eq!(find2(data), 3);
        Ok(())
    }
}
