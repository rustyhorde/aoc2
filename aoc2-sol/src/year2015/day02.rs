// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 2 "I Was Told There Would Be No Math"
//!
//!
//! **--- Day 2: I Was Told There Would Be No Math ---**
//!
//! **--- Part 1 ---**
//!
//! The elves are running low on wrapping paper, and so they need to submit
//! an order for more. They have a list of the dimensions (length l, width w,
//! and height h) of each present, and only want to order exactly as much as they need.
//!
//! Fortunately, every present is a box (a perfect right rectangular prism),
//! which makes calculating the required wrapping paper for each gift a little easier:
//! find the surface area of the box, which is `2*l*w + 2*w*h + 2*h*l`.
//! The elves also need a little extra paper for each present: the area of the smallest side.
//!
//! For example:
//!
//! * A present with dimensions `2x3x4` requires `2*6 + 2*12 + 2*8 = 52` square
//! feet of wrapping paper plus `6` square feet of slack, for a total of `58` square feet.
//! * A present with dimensions `1x1x10` requires `2*1 + 2*10 + 2*10 = 42` square feet
//! of wrapping paper plus `1` square foot of slack, for a total of `43` square feet.
//!
//! All numbers in the elves' list are in feet. How many total square feet of wrapping
//! paper should they order?
//!
//! **--- Part Two ---**
//!
//! The elves are also running low on ribbon. Ribbon is all the same width, so they
//! only have to worry about the length they need to order, which they would again like
//! to be exact.
//!
//! The ribbon required to wrap a present is the shortest distance around its sides,
//! or the smallest perimeter of any one face. Each present also requires a bow made out
//! of ribbon as well; the feet of ribbon required for the perfect bow is equal to the
//! cubic feet of volume of the present. Don't ask how they tie the bow, though; they'll never tell.
//!
//! For example:
//!
//! * A present with dimensions `2x3x4` requires `2+2+3+3 = 10` feet of ribbon to wrap the
//! present plus `2*3*4 = 24` feet of ribbon for the bow, for a total of `34` feet.
//! * A present with dimensions `1x1x10` requires `1+1+1+1 = 4` feet of ribbon to wrap the
//! present plus `1*1*10 = 10` feet of ribbon for the bow, for a total of `14` feet.
//!
//! How many total feet of ribbon should they order?
//!
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
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD02, find_area).map(|_| 0)
}

fn find_area(reader: BufReader<File>) -> usize {
    find_area_br(reader)
}

fn find_area_br<T>(reader: T) -> usize
where
    T: BufRead,
{
    valid_lines(reader)
        .scan(0, handle_line_p1)
        .last()
        .unwrap_or_default()
}

#[allow(clippy::needless_pass_by_value)]
#[inline]
fn handle_line_p1(acc: &mut usize, line: String) -> Option<usize> {
    let parts = line.split('x').collect::<Vec<&str>>();
    if parts.len() == 3 {
        let l_res = str::parse::<usize>(parts[0]);
        let w_res = str::parse::<usize>(parts[1]);
        let h_res = str::parse::<usize>(parts[2]);

        if let (Ok(l), Ok(w), Ok(h)) = (l_res, w_res, h_res) {
            let mut areas = vec![];
            areas.push(l * w);
            areas.push(w * h);
            areas.push(h * l);

            areas.iter().min().map(|min| {
                let total_area: usize = areas.iter().map(|x| 2 * x).sum();
                let total = total_area + min;
                *acc += total;
                *acc
            })
        } else {
            None
        }
    } else {
        None
    }
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD02, find_length).map(|_| 0)
}

fn find_length(reader: BufReader<File>) -> usize {
    find_length_br(reader)
}

fn find_length_br<T>(reader: T) -> usize
where
    T: BufRead,
{
    valid_lines(reader)
        .scan(0, handle_line_p2)
        .last()
        .unwrap_or_default()
}

#[allow(clippy::needless_pass_by_value)]
#[inline]
fn handle_line_p2(acc: &mut usize, line: String) -> Option<usize> {
    let parts = line.split('x').collect::<Vec<&str>>();
    if parts.len() == 3 {
        let l_res = str::parse::<usize>(parts[0]);
        let w_res = str::parse::<usize>(parts[1]);
        let h_res = str::parse::<usize>(parts[2]);

        if let (Ok(l), Ok(w), Ok(h)) = (l_res, w_res, h_res) {
            let mut perims = vec![];
            perims.push(2 * (l + w));
            perims.push(2 * (w + h));
            perims.push(2 * (h + l));

            perims.iter().min().map(|min| {
                let total = (l * w * h) + min;
                *acc += total;
                *acc
            })
        } else {
            None
        }
    } else {
        None
    }
}
#[cfg(test)]
mod one_star {
    use super::find_area_br;
    use std::io::Cursor;

    const TEST_1: &str = r"2x3x4";
    const TEST_2: &str = r"1x1x10";

    #[test]
    fn solution() {
        assert_eq!(find_area_br(Cursor::new(TEST_1)), 58);
        assert_eq!(find_area_br(Cursor::new(TEST_2)), 43);
    }
}

#[cfg(test)]
mod two_star {
    use super::find_length_br;
    use std::io::Cursor;

    const TEST_1: &str = r"2x3x4";
    const TEST_2: &str = r"1x1x10";

    #[test]
    fn solution() {
        assert_eq!(find_length_br(Cursor::new(TEST_1)), 34);
        assert_eq!(find_length_br(Cursor::new(TEST_2)), 14);
    }
}
