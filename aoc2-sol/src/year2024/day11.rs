// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 1
//! --- Day 11: Plutonian Pebbles ---
//!
//! The ancient civilization on Pluto was known for its ability to manipulate spacetime, and while the Historians explore their infinite corridors, you've noticed a strange set of physics-defying stones.
//!
//! At first glance, they seem like normal stones: they're arranged in a perfectly straight line, and each stone has a number engraved on it.
//!
//! The strange part is that every time you blink, the stones change.
//!
//! Sometimes, the number engraved on a stone changes. Other times, a stone might split in two, causing all the other stones to shift over a bit to make room in their perfectly straight line.
//!
//! As you observe them for a while, you find that the stones have a consistent behavior. Every time you blink, the stones each simultaneously change according to the first applicable rule in this list:
//!
//! ```text
//!     If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
//!     If the stone is engraved with a number that has an even number of digits, it is replaced by two stones. The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
//!     If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.
//! ```
//!
//! No matter how the stones change, their order is preserved, and they stay on their perfectly straight line.
//!
//! How will the stones evolve if you keep blinking at them? You take a note of the number engraved on each stone in the line (your puzzle input).
//!
//! If you have an arrangement of five stones engraved with the numbers 0 1 10 99 999 and you blink once, the stones transform as follows:
//!
//! ```text
//!     The first stone, 0, becomes a stone marked 1.
//!     The second stone, 1, is multiplied by 2024 to become 2024.
//!     The third stone, 10, is split into a stone marked 1 followed by a stone marked 0.
//!     The fourth stone, 99, is split into two stones marked 9.
//!     The fifth stone, 999, is replaced by a stone marked 2021976.
//! ```
//!
//! So, after blinking once, your five stones would become an arrangement of seven stones engraved with the numbers 1 2024 1 0 9 9 2021976.
//!
//! Here is a longer example:
//!
//! ```text
//! Initial arrangement:
//! 125 17
//!
//! After 1 blink:
//! 253000 1 7
//!
//! After 2 blinks:
//! 253 0 2024 14168
//!
//! After 3 blinks:
//! 512072 1 20 24 28676032
//!
//! After 4 blinks:
//! 512 72 2024 2 0 2 4 2867 6032
//!
//! After 5 blinks:
//! 1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32
//!
//! After 6 blinks:
//! 2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2
//! ```
//!
//! In this example, after blinking six times, you would have 22 stones. After blinking 25 times, you would have 55312 stones!
//!
//! Consider the arrangement of stones in front of you. How many stones will you have after blinking 25 times?
//! --- Part Two ---
//!
//! The Historians sure are taking a long time. To be fair, the infinite corridors are very large.
//!
//! How many stones would you have after blinking a total of 75 times?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
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
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<(usize, Vec<usize>), usize>(AoCYear::AOC2024, AoCDay::AOCD11, setup, find)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<(usize, Vec<usize>), usize>(
        bench,
        AoCYear::AOC2024,
        AoCDay::AOCD11,
        setup,
        find,
    )
    .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> (usize, Vec<usize>) {
    setup_br(25, reader).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(blinks: usize, reader: T) -> Result<(usize, Vec<usize>)>
where
    T: BufRead,
{
    let mut stones = vec![];
    for line in valid_lines(reader) {
        stones = line
            .split(' ')
            .map(str::parse::<usize>)
            .filter_map(Result::ok)
            .collect();
    }
    Ok((blinks, stones))
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: (usize, Vec<usize>)) -> usize {
    let (blinks, stones) = data;
    let mut curr_stones = stones;

    for _ in 0..blinks {
        let mut new_stones = vec![];
        for stone in curr_stones {
            if stone == 0 {
                new_stones.push(1);
            } else if stone.checked_ilog10().is_some_and(|x| (x + 1) % 2 == 0) {
                let mut chars = stone.to_string().chars().collect::<Vec<char>>();
                let last = chars.split_off(chars.len() / 2);
                if let Ok(first) = chars.into_iter().collect::<String>().parse::<usize>() {
                    new_stones.push(first);
                }
                if let Ok(last) = last.into_iter().collect::<String>().parse::<usize>() {
                    new_stones.push(last);
                }
            } else {
                new_stones.push(stone * 2024);
            }
        }
        curr_stones = new_stones;
    }
    curr_stones.len()
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<(usize, Vec<(usize, usize)>), usize>(
        AoCYear::AOC2024,
        AoCDay::AOCD11,
        setup2,
        find2,
    )
    .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<(usize, Vec<(usize, usize)>), usize>(
        bench,
        AoCYear::AOC2024,
        AoCDay::AOCD11,
        setup2,
        find2,
    )
    .map(|_| 0)
}

fn setup2(reader: BufReader<File>) -> (usize, Vec<(usize, usize)>) {
    setup_br2(75, reader).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br2<T>(blinks: usize, reader: T) -> Result<(usize, Vec<(usize, usize)>)>
where
    T: BufRead,
{
    let mut stones = vec![];
    for line in valid_lines(reader) {
        stones = line
            .split(' ')
            .map(str::parse::<usize>)
            .filter_map(Result::ok)
            .map(|x| (x, 1))
            .collect();
    }
    Ok((blinks, stones))
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: (usize, Vec<(usize, usize)>)) -> usize {
    let (blinks, stones) = data;
    let mut curr_stones = stones;

    for _blink in 0..blinks {
        curr_stones.sort_by(|l, r| r.0.cmp(&l.0));
        let mut curr_idx = curr_stones.len() - 1;

        while curr_idx > 0 {
            let prev_idx = curr_idx - 1;
            if curr_stones[curr_idx].0 == curr_stones[prev_idx].0 {
                curr_stones[prev_idx].1 += curr_stones[curr_idx].1;
                let _ = curr_stones.remove(curr_idx);
            }
            curr_idx -= 1;
        }

        let mut idx = 0;

        while idx < curr_stones.len() {
            if curr_stones[idx].0 == 0 {
                curr_stones[idx].0 = 1;
            } else if curr_stones[idx]
                .0
                .checked_ilog10()
                .is_some_and(|x| (x + 1) % 2 == 0)
            {
                let count = curr_stones[idx].1;
                let mut chars = curr_stones[idx]
                    .0
                    .to_string()
                    .chars()
                    .collect::<Vec<char>>();
                let last = chars.split_off(chars.len() / 2);
                if let Ok(first) = chars.into_iter().collect::<String>().parse::<usize>() {
                    curr_stones[idx].0 = first;
                }
                if let Ok(last) = last.into_iter().collect::<String>().parse::<usize>() {
                    curr_stones.insert(idx + 1, (last, count));
                }
                idx += 1;
            } else {
                curr_stones[idx].0 *= 2024;
            }
            idx += 1;
        }
    }

    curr_stones.iter().map(|x| x.1).sum()
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"0 1 10 99 999";
    const TEST_2: &str = r"125 17";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(1, Cursor::new(TEST_1))?;
        assert_eq!(find(data), 7);
        let data = setup_br(6, Cursor::new(TEST_2))?;
        assert_eq!(find(data), 22);
        let data = setup_br(25, Cursor::new(TEST_2))?;
        assert_eq!(find(data), 55312);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br2};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"0 1 10 99 999";
    const TEST_2: &str = r"125 17";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br2(1, Cursor::new(TEST_1))?;
        assert_eq!(find2(data), 7);
        let data = setup_br2(6, Cursor::new(TEST_2))?;
        assert_eq!(find2(data), 22);
        let data = setup_br2(25, Cursor::new(TEST_2))?;
        assert_eq!(find2(data), 55312);
        Ok(())
    }
}
