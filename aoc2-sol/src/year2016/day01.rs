// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 1 "No Time for a Taxicab"
//!
//! **--- Day 1: No Time for a Taxicab ---**
//!
//! **--- Part 1 ---**
//!
//! Santa's sleigh uses a very high-precision clock to guide its movements, and
//! the clock's oscillator is regulated by stars. Unfortunately, the stars have
//! been stolen... by the Easter Bunny. To save Christmas, Santa needs you to
//! retrieve all fifty stars by December 25th.
//!
//! Collect stars by solving puzzles. Two puzzles will be made available on each
//! day in the Advent calendar; the second puzzle is unlocked when you complete
//! the first. Each puzzle grants one star. Good luck!
//!
//! You're airdropped near Easter Bunny Headquarters in a city somewhere.
//! "Near", unfortunately, is as close as you can get - the instructions on the
//! Easter Bunny Recruiting Document the Elves intercepted start here, and nobody
//! had time to work them out further.
//!
//! The Document indicates that you should start at the given coordinates
//! (where you just landed) and face North. Then, follow the provided sequence:
//! either turn left (L) or right (R) 90 degrees, then walk forward the given number of blocks,
//! ending at a new intersection.
//!
//! There's no time to follow such ridiculous instructions on foot, though, so you
//! take a moment and work out the destination. Given that you can only walk on the
//! street grid of the city, how far is the shortest path to the destination?
//!
//! For example:
//!
//! ```text
//! Following R2, L3 leaves you 2 blocks East and 3 blocks North, or 5 blocks away.
//! R2, R2, R2 leaves you 2 blocks due South of your starting position, which is 2 blocks away.
//! R5, L5, R5, R3 leaves you 12 blocks away.
//! ```
//!
//! How many blocks away is Easter Bunny HQ?
//!
//! **--- Part Two ---**
//!
//! Then, you notice the instructions continue on the back of the Recruiting Document.
//! Easter Bunny HQ is actually at the first location you visit twice.
//!
//! For example, if your instructions are `R8, R4, R4, R8`, the first location you visit
//! twice is 4 blocks away, due East.
//!
//! How many blocks away is the first location you visit twice?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{get_cap_x, run_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use regex::Regex;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

enum Vector {
    Right(isize),
    Left(isize),
}

enum Cardinal {
    North,
    South,
    East,
    West,
}

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_solution::<isize>(AoCYear::AOC2016, AoCDay::AOCD01, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> isize {
    find_br(reader)
        .map_err(|e| {
            eprintln!("{e}");
            e
        })
        .unwrap_or_default()
}

fn find_br<T>(reader: T) -> Result<isize>
where
    T: BufRead,
{
    let right_re = Regex::new(r"^R(\d+)$")?;
    let left_re = Regex::new(r"^L(\d+)$")?;
    let mut steps = vec![];

    for line in valid_lines(reader) {
        let split = line.split(", ").collect::<Vec<&str>>();
        for tok in split {
            if right_re.is_match(tok) {
                let val = get_cap_x::<isize>(
                    1,
                    &right_re.captures(tok).ok_or_else(|| anyhow!("bad cap"))?,
                )?;
                steps.push(Vector::Right(val));
            } else if left_re.is_match(tok) {
                let val = get_cap_x::<isize>(
                    1,
                    &left_re.captures(tok).ok_or_else(|| anyhow!("bad cap"))?,
                )?;
                steps.push(Vector::Left(val));
            } else {
                return Err(anyhow!(format!("Invalid direction! {tok}")));
            }
        }
    }

    let mut x = 0;
    let mut y = 0;
    let mut curr_face = Cardinal::North;

    for step in steps {
        match (step, curr_face) {
            (Vector::Right(magnitude), Cardinal::North)
            | (Vector::Left(magnitude), Cardinal::South) => {
                curr_face = Cardinal::East;
                x += magnitude;
            }
            (Vector::Right(magnitude), Cardinal::East)
            | (Vector::Left(magnitude), Cardinal::West) => {
                curr_face = Cardinal::South;
                y -= magnitude;
            }
            (Vector::Right(magnitude), Cardinal::South)
            | (Vector::Left(magnitude), Cardinal::North) => {
                curr_face = Cardinal::West;
                x -= magnitude;
            }
            (Vector::Right(magnitude), Cardinal::West)
            | (Vector::Left(magnitude), Cardinal::East) => {
                curr_face = Cardinal::North;
                y += magnitude;
            }
        }
    }
    Ok(x.abs() + y.abs())
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<isize>(AoCYear::AOC2016, AoCDay::AOCD01, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> isize {
    find2_br(reader)
        .map_err(|e| {
            eprintln!("{e}");
            e
        })
        .unwrap_or_default()
}

fn find2_br<T>(reader: T) -> Result<isize>
where
    T: BufRead,
{
    let right_re = Regex::new(r"^R(\d+)$")?;
    let left_re = Regex::new(r"^L(\d+)$")?;
    let mut steps = vec![];

    for line in valid_lines(reader) {
        let split = line.split(", ").collect::<Vec<&str>>();
        for tok in split {
            if right_re.is_match(tok) {
                let val = get_cap_x::<isize>(
                    1,
                    &right_re.captures(tok).ok_or_else(|| anyhow!("bad cap"))?,
                )?;
                steps.push(Vector::Right(val));
            } else if left_re.is_match(tok) {
                let val = get_cap_x::<isize>(
                    1,
                    &left_re.captures(tok).ok_or_else(|| anyhow!("bad cap"))?,
                )?;
                steps.push(Vector::Left(val));
            } else {
                return Err(anyhow!(format!("Invalid direction! {tok}")));
            }
        }
    }

    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut curr_face = Cardinal::North;
    let mut visited = HashSet::new();

    'outer: for step in steps {
        match (step, curr_face) {
            (Vector::Right(magnitude), Cardinal::North)
            | (Vector::Left(magnitude), Cardinal::South) => {
                curr_face = Cardinal::East;
                for _i in 0..magnitude {
                    let _ = visited.insert((x, y));
                    x += 1;
                    if visited.contains(&(x, y)) {
                        break 'outer;
                    }
                }
            }
            (Vector::Right(magnitude), Cardinal::East)
            | (Vector::Left(magnitude), Cardinal::West) => {
                curr_face = Cardinal::South;
                for _i in 0..magnitude {
                    let _ = visited.insert((x, y));
                    y -= 1;
                    if visited.contains(&(x, y)) {
                        break 'outer;
                    }
                }
            }
            (Vector::Right(magnitude), Cardinal::South)
            | (Vector::Left(magnitude), Cardinal::North) => {
                curr_face = Cardinal::West;
                for _i in 0..magnitude {
                    let _ = visited.insert((x, y));
                    x -= 1;
                    if visited.contains(&(x, y)) {
                        break 'outer;
                    }
                }
            }
            (Vector::Right(magnitude), Cardinal::West)
            | (Vector::Left(magnitude), Cardinal::East) => {
                curr_face = Cardinal::North;
                for _i in 0..magnitude {
                    let _ = visited.insert((x, y));
                    y += 1;
                    if visited.contains(&(x, y)) {
                        break 'outer;
                    }
                }
            }
        }
    }

    Ok(x.abs() + y.abs())
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"R2, L3";
    const TEST_2: &str = r"R2, R2, R2";
    const TEST_3: &str = r"R5, L5, R5, R3";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1))?, 5);
        assert_eq!(find_br(Cursor::new(TEST_2))?, 2);
        assert_eq!(find_br(Cursor::new(TEST_3))?, 12);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"R8, R4, R4, R8";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find2_br(Cursor::new(TEST_1))?, 4);
        Ok(())
    }
}
