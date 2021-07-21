// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 3 "Perfectly Spherical Houses in a Vacuum"
//!
//! **--- Day 3: Perfectly Spherical Houses in a Vacuum ---**
//!
//! **--- Part 1 ---**
//!
//! Santa is delivering presents to an infinite two-dimensional grid of houses.
//!
//! He begins by delivering a present to the house at his starting location,
//! and then an elf at the North Pole calls him via radio and tells him where
//! to move next. Moves are always exactly one house to the north (^), south (v),
//! east (>), or west (<). After each move, he delivers another present to the
//! house at his new location.
//!
//! However, the elf back at the north pole has had a little too much eggnog,
//! and so his directions are a little off, and Santa ends up visiting some houses
//! more than once. How many houses receive at least one present?
//!
//! For example:
//!
//! * `>` delivers presents to `2` houses: one at the starting location, and one to the east.
//! * `^>v<` delivers presents to `4` houses in a square, including twice to the
//! house at his starting/ending location.
//! * `^v^v^v^v^v` delivers a bunch of presents to some very lucky children at only 2 houses.
//!
//! **--- Part Two ---**
//!
//! The next year, to speed up the process, Santa creates a robot version of himself,
//! Robo-Santa, to deliver presents with him.
//!
//! Santa and Robo-Santa start at the same location (delivering two presents to the
//! same starting house), then take turns moving based on instructions from the elf,
//! who is eggnoggedly reading from the same script as the previous year.
//!
//! This year, how many houses receive at least one present?
//!
//! For example:
//!
//! * `^v` delivers presents to `3` houses, because Santa goes north, and then Robo-Santa goes south.
//! * `^>v<` now delivers presents to 3 houses, and Santa and Robo-Santa end up back where they started.
//! * `^v^v^v^v^v` now delivers presents to 11 houses, with Santa going one direction and Robo-Santa going the other.

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use std::{
    collections::HashMap,
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
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD03, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader).unwrap_or_default()
}

fn find_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let mut house_map = HashMap::new();

    for line in valid_lines(reader) {
        start_delivery(&mut house_map, line.chars())?;
    }

    Ok(house_map.values().filter(|x| **x > 0).count())
}

fn start_delivery(
    house_map: &mut HashMap<(isize, isize), usize>,
    iter: impl Iterator<Item = char>,
) -> Result<()> {
    let mut x: isize = 0;
    let mut y: isize = 0;
    *house_map.entry((x, y)).or_insert(0_usize) += 1;
    for ch in iter {
        deliver(house_map, &mut x, &mut y, ch)?;
    }
    Ok(())
}

fn deliver(
    house_map: &mut HashMap<(isize, isize), usize>,
    x: &mut isize,
    y: &mut isize,
    ch: char,
) -> Result<()> {
    match ch {
        '^' => *y += 1,
        '>' => *x += 1,
        'v' => *y -= 1,
        '<' => *x -= 1,
        _ => return Err(anyhow!("invalid movement")),
    }

    *house_map.entry((*x, *y)).or_insert(0) += 1;
    Ok(())
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD03, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader).unwrap_or_default()
}

fn find2_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let mut house_map = HashMap::new();

    for line in valid_lines(reader) {
        let santa_iter = line.chars().step_by(2);
        let robo_iter = line.chars().skip(1).step_by(2);

        start_delivery(&mut house_map, santa_iter)?;
        start_delivery(&mut house_map, robo_iter)?;
    }

    Ok(house_map.values().filter(|x| **x > 0).count())
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r">";
    const TEST_2: &str = r"^>v<";
    const TEST_3: &str = r"^v^v^v^v^v";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1))?, 2);
        assert_eq!(find_br(Cursor::new(TEST_2))?, 4);
        assert_eq!(find_br(Cursor::new(TEST_3))?, 2);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"^v";
    const TEST_2: &str = r"^>v<";
    const TEST_3: &str = r"^v^v^v^v^v";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find2_br(Cursor::new(TEST_1))?, 3);
        assert_eq!(find2_br(Cursor::new(TEST_2))?, 3);
        assert_eq!(find2_br(Cursor::new(TEST_3))?, 11);
        Ok(())
    }
}
