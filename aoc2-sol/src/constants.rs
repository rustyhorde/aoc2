// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `aoc2` Year and Day enums
//!
//! # Example
//! ```
//! # use anyhow::Result;
//! # use aoc2_sol::constants::{AoCDay, AoCYear};
//! # use std::convert::TryFrom;
//! # fn main() -> Result<()> {
//! assert_eq!("day01", <&str>::from(AoCDay::AOCD01));
//! let year = AoCYear::try_from("2015")?;
//! assert_eq!("2015", <&str>::from(year));
//! #   Ok(())
//! # }
//! ```
//!

use anyhow::{anyhow, Error, Result};
use std::convert::TryFrom;

/// Advent of Code Year
///
/// # Example
/// ```
/// # use anyhow::Result;
/// # use aoc2_sol::constants::AoCYear;
/// # use std::convert::TryFrom;
/// # fn main() -> Result<()> {
/// let year = AoCYear::try_from("2015")?;
/// assert_eq!("2015", <&str>::from(year));
/// #   Ok(())
/// # }
/// ```
///
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AoCYear {
    /// Advent of Code 2015
    AOC2015,
    /// Advent of Code 2016
    AOC2016,
    /// Advent of Code 2017
    AOC2017,
    /// Advent of Code 2018
    AOC2018,
    /// Advent of Code 2019
    AOC2019,
    /// Advent of Code 2020
    AOC2020,
    /// Advent of Code 2021
    AOC2021,
}

impl From<AoCYear> for &'_ str {
    fn from(year: AoCYear) -> Self {
        match year {
            AoCYear::AOC2015 => "2015",
            AoCYear::AOC2016 => "2016",
            AoCYear::AOC2017 => "2017",
            AoCYear::AOC2018 => "2018",
            AoCYear::AOC2019 => "2019",
            AoCYear::AOC2020 => "2020",
            AoCYear::AOC2021 => "2021",
        }
    }
}

impl TryFrom<&'_ str> for AoCYear {
    type Error = Error;

    fn try_from(year: &str) -> Result<Self> {
        match year {
            "2015" => Ok(AoCYear::AOC2015),
            "2016" => Ok(AoCYear::AOC2016),
            "2017" => Ok(AoCYear::AOC2017),
            "2018" => Ok(AoCYear::AOC2018),
            "2019" => Ok(AoCYear::AOC2019),
            "2020" => Ok(AoCYear::AOC2020),
            "2021" => Ok(AoCYear::AOC2021),
            _ => Err(anyhow!("Unable to convert to year!")),
        }
    }
}

/// Advent of Code Days
///
/// # Example
/// ```
/// # use aoc2_sol::constants::AoCDay;
/// assert_eq!("day01", <&str>::from(AoCDay::AOCD01));
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AoCDay {
    /// Day 1
    AOCD01,
    /// Day 2
    AOCD02,
    /// Day 3
    AOCD03,
    /// Day 4
    AOCD04,
    /// Day 5
    AOCD05,
    /// Day 6
    AOCD06,
    /// Day 7
    AOCD07,
    /// Day 8
    AOCD08,
    /// Day 9
    AOCD09,
    /// Day 10
    AOCD10,
    /// Day 11
    AOCD11,
    /// Day 12
    AOCD12,
    /// Day 13
    AOCD13,
    /// Day 14
    AOCD14,
    /// Day 15
    AOCD15,
    /// Day 16
    AOCD16,
    /// Day 17
    AOCD17,
    /// Day 18
    AOCD18,
    /// Day 19
    AOCD19,
    /// Day 20
    AOCD20,
    /// Day 21
    AOCD21,
    /// Day 22
    AOCD22,
    /// Day 23
    AOCD23,
    /// Day 24
    AOCD24,
    /// Day 25
    AOCD25,
}

impl From<AoCDay> for &'_ str {
    fn from(year: AoCDay) -> Self {
        match year {
            AoCDay::AOCD01 => DAY_1,
            AoCDay::AOCD02 => DAY_2,
            AoCDay::AOCD03 => DAY_3,
            AoCDay::AOCD04 => DAY_4,
            AoCDay::AOCD05 => DAY_5,
            AoCDay::AOCD06 => DAY_6,
            AoCDay::AOCD07 => DAY_7,
            AoCDay::AOCD08 => DAY_8,
            AoCDay::AOCD09 => DAY_9,
            AoCDay::AOCD10 => DAY_10,
            AoCDay::AOCD11 => DAY_11,
            AoCDay::AOCD12 => DAY_12,
            AoCDay::AOCD13 => DAY_13,
            AoCDay::AOCD14 => DAY_14,
            AoCDay::AOCD15 => DAY_15,
            AoCDay::AOCD16 => DAY_16,
            AoCDay::AOCD17 => DAY_17,
            AoCDay::AOCD18 => DAY_18,
            AoCDay::AOCD19 => DAY_19,
            AoCDay::AOCD20 => DAY_20,
            AoCDay::AOCD21 => DAY_21,
            AoCDay::AOCD22 => DAY_22,
            AoCDay::AOCD23 => DAY_23,
            AoCDay::AOCD24 => DAY_24,
            AoCDay::AOCD25 => DAY_25,
        }
    }
}

/// Day 1
pub const DAY_1: &str = "day01";
/// Day 2
pub const DAY_2: &str = "day02";
/// Day 3
pub const DAY_3: &str = "day03";
/// Day 4
pub const DAY_4: &str = "day04";
/// Day 5
pub const DAY_5: &str = "day05";
/// Day 6
pub const DAY_6: &str = "day06";
/// Day 7
pub const DAY_7: &str = "day07";
/// Day 8
pub const DAY_8: &str = "day08";
/// Day 9
pub const DAY_9: &str = "day09";
/// Day 10
pub const DAY_10: &str = "day10";
/// Day 11
pub const DAY_11: &str = "day11";
/// Day 12
pub const DAY_12: &str = "day12";
/// Day 13
pub const DAY_13: &str = "day13";
/// Day 14
pub const DAY_14: &str = "day14";
/// Day 15
pub const DAY_15: &str = "day15";
/// Day 16
pub const DAY_16: &str = "day16";
/// Day 17
pub const DAY_17: &str = "day17";
/// Day 18
pub const DAY_18: &str = "day18";
/// Day 19
pub const DAY_19: &str = "day19";
/// Day 20
pub const DAY_20: &str = "day20";
/// Day 21
pub const DAY_21: &str = "day21";
/// Day 22
pub const DAY_22: &str = "day22";
/// Day 23
pub const DAY_23: &str = "day23";
/// Day 24
pub const DAY_24: &str = "day24";
/// Day 25
pub const DAY_25: &str = "day25";

/// Advent of Code Day 1 about string
const DAY_1_ABOUT: &str = "Advent of Code - Day 01";
/// Advent of Code Day 2 about string
const DAY_2_ABOUT: &str = "Advent of Code - Day 02";
/// Advent of Code Day 3 about string
const DAY_3_ABOUT: &str = "Advent of Code - Day 03";
/// Advent of Code Day 4 about string
const DAY_4_ABOUT: &str = "Advent of Code - Day 04";
/// Advent of Code Day 5 about string
const DAY_5_ABOUT: &str = "Advent of Code - Day 05";
/// Advent of Code Day 6 about string
const DAY_6_ABOUT: &str = "Advent of Code - Day 06";
/// Advent of Code Day 7 about string
const DAY_7_ABOUT: &str = "Advent of Code - Day 07";
/// Advent of Code Day 8 about string
const DAY_8_ABOUT: &str = "Advent of Code - Day 08";
/// Advent of Code Day 9 about string
const DAY_9_ABOUT: &str = "Advent of Code - Day 09";
/// Advent of Code Day 10 about string
const DAY_10_ABOUT: &str = "Advent of Code - Day 10";
/// Advent of Code Day 11 about string
const DAY_11_ABOUT: &str = "Advent of Code - Day 11";
/// Advent of Code Day 12 about string
const DAY_12_ABOUT: &str = "Advent of Code - Day 12";
/// Advent of Code Day 13 about string
const DAY_13_ABOUT: &str = "Advent of Code - Day 13";
/// Advent of Code Day 14 about string
const DAY_14_ABOUT: &str = "Advent of Code - Day 14";
/// Advent of Code Day 15 about string
const DAY_15_ABOUT: &str = "Advent of Code - Day 15";
/// Advent of Code Day 16 about string
const DAY_16_ABOUT: &str = "Advent of Code - Day 16";
/// Advent of Code Day 17 about string
const DAY_17_ABOUT: &str = "Advent of Code - Day 17";
/// Advent of Code Day 18 about string
const DAY_18_ABOUT: &str = "Advent of Code - Day 18";
/// Advent of Code Day 19 about string
const DAY_19_ABOUT: &str = "Advent of Code - Day 19";
/// Advent of Code Day 20 about string
const DAY_20_ABOUT: &str = "Advent of Code - Day 20";
/// Advent of Code Day 21 about string
const DAY_21_ABOUT: &str = "Advent of Code - Day 21";
/// Advent of Code Day 22 about string
const DAY_22_ABOUT: &str = "Advent of Code - Day 22";
/// Advent of Code Day 23 about string
const DAY_23_ABOUT: &str = "Advent of Code - Day 23";
/// Advent of Code Day 24 about string
const DAY_24_ABOUT: &str = "Advent of Code - Day 24";
/// Advent of Code Day 25 about string
const DAY_25_ABOUT: &str = "Advent of Code - Day 25";

/// Get the about string for the command line for a given day
///
/// # Example
/// ```
/// # use aoc2_sol::constants::{AoCDay, get_day_about};
/// assert_eq!("Advent of Code - Day 25", get_day_about(AoCDay::AOCD25));
/// ```
#[must_use]
pub fn get_day_about<'a>(day: AoCDay) -> &'a str {
    match day {
        AoCDay::AOCD01 => DAY_1_ABOUT,
        AoCDay::AOCD02 => DAY_2_ABOUT,
        AoCDay::AOCD03 => DAY_3_ABOUT,
        AoCDay::AOCD04 => DAY_4_ABOUT,
        AoCDay::AOCD05 => DAY_5_ABOUT,
        AoCDay::AOCD06 => DAY_6_ABOUT,
        AoCDay::AOCD07 => DAY_7_ABOUT,
        AoCDay::AOCD08 => DAY_8_ABOUT,
        AoCDay::AOCD09 => DAY_9_ABOUT,
        AoCDay::AOCD10 => DAY_10_ABOUT,
        AoCDay::AOCD11 => DAY_11_ABOUT,
        AoCDay::AOCD12 => DAY_12_ABOUT,
        AoCDay::AOCD13 => DAY_13_ABOUT,
        AoCDay::AOCD14 => DAY_14_ABOUT,
        AoCDay::AOCD15 => DAY_15_ABOUT,
        AoCDay::AOCD16 => DAY_16_ABOUT,
        AoCDay::AOCD17 => DAY_17_ABOUT,
        AoCDay::AOCD18 => DAY_18_ABOUT,
        AoCDay::AOCD19 => DAY_19_ABOUT,
        AoCDay::AOCD20 => DAY_20_ABOUT,
        AoCDay::AOCD21 => DAY_21_ABOUT,
        AoCDay::AOCD22 => DAY_22_ABOUT,
        AoCDay::AOCD23 => DAY_23_ABOUT,
        AoCDay::AOCD24 => DAY_24_ABOUT,
        AoCDay::AOCD25 => DAY_25_ABOUT,
    }
}
