// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 3 "Squares With Three Sides"
//!
//! **--- Day 3: Squares With Three Sides ---**
//!
//! **--- Part 1 --**
//!
//! Now that you can think clearly, you move deeper into the labyrinth of
//! hallways and office furniture that makes up this part of Easter Bunny HQ.
//! This must be a graphic design department; the walls are covered in
//! specifications for triangles.
//!
//! Or are they?
//!
//! The design document gives the side lengths of each triangle it describes,
//! but... `5 10 25`? Some of these aren't triangles. You can't help but mark
//! the impossible ones.
//!
//! In a valid triangle, the sum of any two sides must be larger than the
//! remaining side. For example, the "triangle" given above is impossible,
//! because `5 + 10` is not larger than `25`.
//!
//! In your puzzle input, how many of the listed triangles are possible?
//!
//! **--- Part Two ---**
//!
//! Now that you've helpfully marked up their design documents, it occurs to
//! you that triangles are specified in groups of three vertically. Each set of
//! three numbers in a column specifies a triangle. Rows are unrelated.
//!
//! For example, given the following specification, numbers with the same hundreds
//! digit would be part of the same triangle:
//!
//! ```text
//! 101 301 501
//! 102 302 502
//! 103 303 503
//! 201 401 601
//! 202 402 602
//! 203 403 603
//! ```
//!
//! In your puzzle input, and instead reading by columns, how many of the listed
//! triangles are possible?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{get_cap_x, run_solution, valid_lines},
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
    run_solution::<usize>(AoCYear::AOC2016, AoCDay::AOCD03, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader).unwrap_or_default()
}

fn find_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let line_re = Regex::new(r"(\d+) +(\d+) +(\d+)")?;
    let mut triangles = vec![];

    for line in valid_lines(reader) {
        for caps in line_re.captures_iter(&line) {
            let mut tri = vec![];
            let s1 = get_cap_x::<usize>(1, &caps)?;
            let s2 = get_cap_x::<usize>(2, &caps)?;
            let s3 = get_cap_x::<usize>(3, &caps)?;
            tri.push(s1);
            tri.push(s2);
            tri.push(s3);
            tri.sort_unstable();
            triangles.push(tri);
        }
    }

    let mut valid = 0;

    for triangle in triangles {
        if triangle[0] + triangle[1] > triangle[2] {
            valid += 1;
        }
    }
    Ok(valid)
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2016, AoCDay::AOCD03, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader).unwrap_or_default()
}

fn find2_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let line_re = Regex::new(r"(\d+) +(\d+) +(\d+)")?;
    let mut triangles = vec![];

    for line in valid_lines(reader) {
        for caps in line_re.captures_iter(&line) {
            let mut tri = vec![];
            let s1 = get_cap_x::<usize>(1, &caps)?;
            let s2 = get_cap_x::<usize>(2, &caps)?;
            let s3 = get_cap_x::<usize>(3, &caps)?;
            tri.push(s1);
            tri.push(s2);
            tri.push(s3);
            triangles.push(tri);
        }
    }

    let mut valid = 0;
    for s in triangles.chunks(3) {
        let mut tri1 = [s[0][0], s[1][0], s[2][0]];
        tri1.sort_unstable();
        let mut tri2 = [s[0][1], s[1][1], s[2][1]];
        tri2.sort_unstable();
        let mut tri3 = [s[0][2], s[1][2], s[2][2]];
        tri3.sort_unstable();

        if tri1[0] + tri1[1] > tri1[2] {
            valid += 1;
        }
        if tri2[0] + tri2[1] > tri2[2] {
            valid += 1;
        }
        if tri3[0] + tri3[1] > tri3[2] {
            valid += 1;
        }
    }
    Ok(valid)
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"5 10 25
5 10 14";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1))?, 1);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"101 301 501
102 302 502
103 303 503
201 401 601
202 402 602
203 403 603";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find2_br(Cursor::new(TEST_1))?, 6);
        Ok(())
    }
}
