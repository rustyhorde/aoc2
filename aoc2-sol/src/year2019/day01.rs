// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 1
//!
//! --- Day 1: The Tyranny of the Rocket Equation ---
//!
//! Santa has become stranded at the edge of the Solar System while delivering presents to other planets! To accurately calculate his position in space, safely align his warp drive, and return to Earth in time to save Christmas, he needs you to bring him measurements from fifty stars.
//!
//! Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
//!
//! The Elves quickly load you into a spacecraft and prepare to launch.
//!
//! At the first Go / No Go poll, every Elf is Go until the Fuel Counter-Upper. They haven't determined the amount of fuel required yet.
//!
//! Fuel required to launch a given module is based on its mass. Specifically, to find the fuel required for a module, take its mass, divide by three, round down, and subtract 2.
//!
//! For example:
//!
//! - For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
//! - For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
//! - For a mass of 1969, the fuel required is 654.
//! - For a mass of 100756, the fuel required is 33583.
//!
//! The Fuel Counter-Upper needs to know the total fuel requirement. To find it, individually calculate the fuel needed for the mass of each module (your puzzle input), then add together all the fuel values.
//!
//! What is the sum of the fuel requirements for all of the modules on your spacecraft?
//!
//! --- Part Two ---
//!
//! During the second Go / No Go poll, the Elf in charge of the Rocket Equation Double-Checker stops the launch sequence. Apparently, you forgot to include additional fuel for the fuel you just added.
//!
//! Fuel itself requires fuel just like a module - take its mass, divide by three, round down, and subtract 2. However, that fuel also requires fuel, and that fuel requires fuel, and so on. Any mass that would require negative fuel should instead be treated as if it requires zero fuel; the remaining mass, if any, is instead handled by wishing really hard, which has no mass and is outside the scope of this calculation.
//!
//! So, for each module mass, calculate its fuel and add it to the total. Then, treat the fuel amount you just calculated as the input mass and repeat the process, continuing until a fuel requirement is zero or negative. For example:
//!
//! - A module of mass 14 requires 2 fuel. This fuel requires no further fuel (2 divided by 3 and rounded down is 0, which would call for a negative fuel), so the total fuel required is still just 2.
//! - At first, a module of mass 1969 requires 654 fuel. Then, this fuel requires 216 more fuel (654 / 3 - 2). 216 then requires 70 more fuel, which requires 21 fuel, which requires 5 fuel, which requires no further fuel. So, the total fuel required for a module of mass 1969 is 654 + 216 + 70 + 21 + 5 = 966.
//! - The fuel required by a module of mass 100756 and its fuel is: 33583 + 11192 + 3728 + 1240 + 411 + 135 + 43 + 12 + 2 = 50346.
//!
//! What is the sum of the fuel requirements for all of the modules on your spacecraft when also taking into account the mass of the added fuel? (Calculate the fuel requirements for each module separately, then add them all up at the end.)

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_solution, valid_lines},
};
use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
use tracing::error;

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_solution::<u32>(AoCYear::AOC2019, AoCDay::AOCD01, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> u32 {
    find_br(reader)
        .map_err(|e| {
            error!("{e}");
            e
        })
        .unwrap_or_default()
}

fn find_br<T>(reader: T) -> Result<u32>
where
    T: BufRead,
{
    let mut digits = vec![];

    for line in valid_lines(reader) {
        digits.push(line.parse::<u32>()?);
    }
    Ok(digits.into_iter().map(|x| (x / 3) - 2).sum())
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<u32>(AoCYear::AOC2019, AoCDay::AOCD01, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> u32 {
    find2_br(reader)
        .map_err(|e| {
            error!("{e}");
            e
        })
        .unwrap_or_default()
}

fn find2_br<T>(reader: T) -> Result<u32>
where
    T: BufRead,
{
    let mut digits = vec![];

    for line in valid_lines(reader) {
        digits.push(line.parse::<u32>()?);
    }
    Ok(digits.into_iter().map(acc).sum())
}

fn acc(x: u32) -> u32 {
    let fuel_by_three = x / 3;
    if fuel_by_three < 2 {
        0
    } else {
        let next = fuel_by_three - 2;
        next + acc(next)
    }
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"12";
    const TEST_2: &str = r"14";
    const TEST_3: &str = r"1969";
    const TEST_4: &str = r"100756";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1))?, 2);
        assert_eq!(find_br(Cursor::new(TEST_2))?, 2);
        assert_eq!(find_br(Cursor::new(TEST_3))?, 654);
        assert_eq!(find_br(Cursor::new(TEST_4))?, 33583);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"12";
    const TEST_2: &str = r"14";
    const TEST_3: &str = r"1969";
    const TEST_4: &str = r"100756";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find2_br(Cursor::new(TEST_1))?, 2);
        assert_eq!(find2_br(Cursor::new(TEST_2))?, 2);
        assert_eq!(find2_br(Cursor::new(TEST_3))?, 966);
        assert_eq!(find2_br(Cursor::new(TEST_4))?, 50346);
        Ok(())
    }
}
