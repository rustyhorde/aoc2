// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 6 "Probably a Fire Hazard"
//!
//! **--- Day 6: Probably a Fire Hazard ---**
//!
//! **--- Part 1 ---**
//!
//! Because your neighbors keep defeating you in the holiday house decorating
//~ contest year after year, you've decided to deploy one million lights in a 1000x1000 grid.
//!
//! Furthermore, because you've been especially nice this year, Santa has mailed
//! you instructions on how to display the ideal lighting configuration.
//!
//! Lights in your grid are numbered from 0 to 999 in each direction; the lights at
//! each corner are at `0,0`, `0,999`, `999,999`, and `999,0`. The instructions include
//! whether to turn on, turn off, or toggle various inclusive ranges given as coordinate pairs.
//! Each coordinate pair represents opposite corners of a rectangle, inclusive;
//! a coordinate pair like `0,0` through `2,2` therefore refers to `9` lights in a `3x3` square.
//! The lights all start turned off.
//!
//! To defeat your neighbors this year, all you have to do is set up your lights by doing
//! the instructions Santa sent you in order.
//!
//! For example:
//!
//! * `turn on 0,0 through 999,999` would turn on (or leave on) every light.
//! * `toggle 0,0 through 999,0` would toggle the first line of `1000` lights, turning off the ones that were on, and turning on the ones that were off.
//! * `turn off 499,499 through 500,500` would turn off (or leave off) the middle four lights.
//!
//! After following the instructions, how many lights are lit?
//!
//! **--- Part Two ---**
//!
//! You just finish implementing your winning light pattern when you realize you
//! mistranslated Santa's message from Ancient Nordic Elvish.
//!
//! The light grid you bought actually has individual brightness controls;
//! each light can have a brightness of zero or more. The lights all start at zero.
//!
//! The phrase turn on actually means that you should increase the brightness of those lights by 1.
//!
//! The phrase turn off actually means that you should decrease the brightness of those lights by 1, to a minimum of zero.
//!
//! The phrase toggle actually means that you should increase the brightness of those lights by 2.
//!
//! What is the total brightness of all lights combined after following Santa's instructions?
//!
//! For example:
//!
//! * `turn on 0,0 through 0,0` would increase the total brightness by `1`.
//! * `toggle 0,0 through 999,999` would increase the total brightness by `2000000`.

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use ndarray::Array2;
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
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD06, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader).unwrap_or_default()
}

fn find_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let mut lights: Array2<bool> = Array2::default((1000, 1000));
    let line_re = Regex::new(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)")?;

    for line in valid_lines(reader) {
        for cap in line_re.captures_iter(&line) {
            let action = &cap[1];
            let x1 = (&cap[2]).parse::<usize>()?;
            let y1 = (&cap[3]).parse::<usize>()?;
            let x2 = (&cap[4]).parse::<usize>()?;
            let y2 = (&cap[5]).parse::<usize>()?;

            match action {
                "turn on" => {
                    turn_on(&mut lights, x1, y1, x2, y2);
                }
                "turn off" => {
                    turn_off(&mut lights, x1, y1, x2, y2);
                }
                "toggle" => {
                    toggle(&mut lights, x1, y1, x2, y2);
                }
                _ => return Err(anyhow!("invalid command")),
            }
        }
    }
    Ok(lights.iter().filter(|x| **x).count())
}

fn turn_on(lights: &mut Array2<bool>, x1: usize, y1: usize, x2: usize, y2: usize) {
    for x in x1..=x2 {
        for y in y1..=y2 {
            lights[[x, y]] = true;
        }
    }
}

fn toggle(lights: &mut Array2<bool>, x1: usize, y1: usize, x2: usize, y2: usize) {
    for x in x1..=x2 {
        for y in y1..=y2 {
            lights[[x, y]] = !lights[[x, y]];
        }
    }
}

fn turn_off(lights: &mut Array2<bool>, x1: usize, y1: usize, x2: usize, y2: usize) {
    for x in x1..=x2 {
        for y in y1..=y2 {
            lights[[x, y]] = false;
        }
    }
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD06, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader).unwrap_or_default()
}

fn find2_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let mut lights: Array2<usize> = Array2::zeros((1000, 1000));
    let line_re = Regex::new(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)")?;

    for line in valid_lines(reader) {
        for cap in line_re.captures_iter(&line) {
            let action = &cap[1];
            let x1 = (&cap[2]).parse::<usize>()?;
            let y1 = (&cap[3]).parse::<usize>()?;
            let x2 = (&cap[4]).parse::<usize>()?;
            let y2 = (&cap[5]).parse::<usize>()?;

            match action {
                "turn on" => {
                    increase_brightness(&mut lights, x1, y1, x2, y2);
                }
                "turn off" => {
                    decrease_brightness(&mut lights, x1, y1, x2, y2);
                }
                "toggle" => {
                    really_brighten(&mut lights, x1, y1, x2, y2);
                }
                _ => return Err(anyhow!("invalid command")),
            }
        }
    }

    Ok(lights.iter().sum())
}

fn increase_brightness(lights: &mut Array2<usize>, x1: usize, y1: usize, x2: usize, y2: usize) {
    for x in x1..=x2 {
        for y in y1..=y2 {
            lights[[x, y]] += 1;
        }
    }
}

fn decrease_brightness(lights: &mut Array2<usize>, x1: usize, y1: usize, x2: usize, y2: usize) {
    for x in x1..=x2 {
        for y in y1..=y2 {
            if lights[[x, y]] >= 1 {
                lights[[x, y]] -= 1;
            }
        }
    }
}

fn really_brighten(lights: &mut Array2<usize>, x1: usize, y1: usize, x2: usize, y2: usize) {
    for x in x1..=x2 {
        for y in y1..=y2 {
            lights[[x, y]] += 2;
        }
    }
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"turn on 0,0 through 999,999";
    const TEST_2: &str = r"toggle 0,0 through 999,0";
    const TEST_3: &str = r"turn on 0,0 through 999,999\nturn off 499,499 through 500,500";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1))?, 1_000_000);
        assert_eq!(find_br(Cursor::new(TEST_2))?, 1_000);
        assert_eq!(find_br(Cursor::new(TEST_3))?, 999_996);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"turn on 0,0 through 0,0";
    const TEST_2: &str = r"toggle 0,0 through 999,999";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find2_br(Cursor::new(TEST_1))?, 1);
        assert_eq!(find2_br(Cursor::new(TEST_2))?, 2_000_000);
        Ok(())
    }
}
