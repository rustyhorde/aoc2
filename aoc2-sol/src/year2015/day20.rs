// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 20 "Infinite Elves and Infinite Houses"
//!
//! **--- Day 20: Infinite Elves and Infinite Houses ---**
//!
//! To keep the Elves busy, Santa has them deliver some presents by hand, door-to-door.
//! He sends them down a street with infinite houses numbered sequentially: 1, 2, 3, 4, 5, and so on.
//!
//! Each Elf is assigned a number, too, and delivers presents to houses based on that number:
//!
//! The first Elf (number 1) delivers presents to every house: 1, 2, 3, 4, 5, ....
//! The second Elf (number 2) delivers presents to every second house: 2, 4, 6, 8, 10, ....
//! Elf number 3 delivers presents to every third house: 3, 6, 9, 12, 15, ....
//! There are infinitely many Elves, numbered starting with 1. Each Elf delivers presents
//! equal to ten times his or her number at each house.
//!
//! So, the first nine houses on the street end up like this:
//!
//! ```text
//! House 1 got 10 presents.
//! House 2 got 30 presents.
//! House 3 got 40 presents.
//! House 4 got 70 presents.
//! House 5 got 60 presents.
//! House 6 got 120 presents.
//! House 7 got 80 presents.
//! House 8 got 150 presents.
//! House 9 got 130 presents.
//! ```
//!
//! The first house gets `10` presents: it is visited only by Elf 1, which delivers
//! `1 * 10 = 10` presents. The fourth house gets `70` presents, because it is visited
//! by Elves 1, 2, and 4, for a total of `10 + 20 + 40 = 70` presents.
//!
//! What is the lowest house number of the house to get at least as many presents as the
//! number in your puzzle input?
//!
//! **--- Part Two ---**
//!
//! The Elves decide they don't want to visit an infinite number of houses.
//! Instead, each Elf will stop after delivering presents to 50 houses. To make up for it,
//! they decide to deliver presents equal to eleven times their number at each house.
//!
//! With these changes, what is the new lowest house number of the house to get at
//! least as many presents as the number in your puzzle input?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_solution, valid_lines},
};
use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter,
};

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD20, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader).unwrap_or_default()
}

#[allow(clippy::needless_range_loop)]
fn find_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let mut house_count = 0;
    for line in valid_lines(reader) {
        let total = line.parse::<usize>()?;
        let mut houses: Vec<usize> = iter::repeat(0).take(total).collect();

        for i in 1..(total / 10) {
            for j in (i..(total / 10)).step_by(i) {
                houses[j] += i * 10;
            }
        }
        for (idx, at_house) in houses.iter().enumerate() {
            if *at_house >= total {
                house_count = idx;
                break;
            }
        }
    }

    Ok(house_count)
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD20, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader).unwrap_or_default()
}

fn find2_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let mut house_count = 0;
    for line in valid_lines(reader) {
        let total = line.parse::<usize>()?;
        let mut houses: Vec<usize> = iter::repeat(0).take(total).collect();

        for i in 1..(total / 10) {
            let mut k = 0;
            for j in (i..(total / 10)).step_by(i) {
                houses[j] += i * 11;
                k += 1;
                if k >= 50 {
                    break;
                }
            }
        }
        for (idx, at_house) in houses.iter().enumerate() {
            if *at_house >= total {
                house_count = idx;
                break;
            }
        }
    }

    Ok(house_count)
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"150";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1))?, 8);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"150";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find2_br(Cursor::new(TEST_1))?, 8);
        Ok(())
    }
}
