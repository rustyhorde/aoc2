// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2017 ---***
//!
//! **--- Day 16: Permutation Promenade ---**
//!
//! You come upon a very unusual sight; a group of programs here appear to be dancing.
//!
//! There are sixteen programs in total, named a through p. They start by standing in a line: a stands in position 0, b stands in position 1, and so on until p, which stands in position 15.
//!
//! The programs' dance consists of a sequence of dance moves:
//!
//! ```text
//!     Spin, written sX, makes X programs move from the end to the front, but maintain their order otherwise. (For example, s3 on abcde produces cdeab).
//!     Exchange, written xA/B, makes the programs at positions A and B swap places.
//!     Partner, written pA/B, makes the programs named A and B swap places.
//! ```
//!
//! For example, with only five programs standing in a line (abcde), they could do the following dance:
//!
//! ```text
//!     s1, a spin of size 1: eabcd.
//!     x3/4, swapping the last two programs: eabdc.
//!     pe/b, swapping programs e and b: baedc.
//! ```
//!
//! After finishing their dance, the programs end up in order baedc.
//!
//! You watch the dance for a while and record their dance moves (your puzzle input). In what order are the programs standing after their dance?
//!
//! **--- Part Two ---**
//!
//! Now that you're starting to get a feel for the dance moves, you turn your attention to the dance as a whole.
//!
//! Keeping the positions they ended up in from their previous dance, the programs perform it again and again: including the first dance, a total of one billion (1000000000) times.
//!
//! In the example above, their second dance would begin with the order baedc, and use the same dance moves:
//!
//! ```text
//!     s1, a spin of size 1: cbaed.
//!     x3/4, swapping the last two programs: cbade.
//!     pe/b, swapping programs e and b: ceadb.
//! ```
//!
//! In what order are the programs standing after their billion dances?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// Various Dance Moves
#[derive(Clone, Copy, Debug)]
enum Move {
    /// Exchange pos1 with pos2
    Exchange(u8, u8),
    /// Rotate x from end to beginning maintaining order
    Spin(u32),
    /// Swap name1 with name2
    Partner(char, char),
}

type Moves = (Vec<Move>, Vec<char>);

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<Moves, String>(AoCYear::AOC2017, AoCDay::AOCD16, setup, find).map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Moves, String>(bench, AoCYear::AOC2017, AoCDay::AOCD16, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> Moves {
    setup_br(
        reader,
        &[
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
        ],
    )
    .unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T, dancers: &[char]) -> Result<Moves>
where
    T: BufRead,
{
    let mut moves = vec![];
    let spin_re = Regex::new(r"s(\d{1,2})")?;
    let exchange_re = Regex::new(r"x(\d{1,2})/(\d{1,2})")?;
    let partner_re = Regex::new(r"p([a-p])/([a-p])")?;

    for line in valid_lines(reader) {
        let tokens: Vec<&str> = line.split(',').collect();
        for token in tokens {
            if spin_re.is_match(token) {
                let caps = spin_re
                    .captures(token)
                    .ok_or(anyhow!("invalid spin captures"))?;
                let val_str = caps.get(1).ok_or(anyhow!("invalid spin value"))?.as_str();
                let val = val_str.parse::<u32>()?;
                moves.push(Move::Spin(val));
            } else if exchange_re.is_match(token) {
                let caps = exchange_re
                    .captures(token)
                    .ok_or(anyhow!("invalid exchange captures"))?;
                let pos1_str = caps
                    .get(1)
                    .ok_or(anyhow!("invalid exchange pos1"))?
                    .as_str();
                let pos2_str = caps
                    .get(2)
                    .ok_or(anyhow!("invalid exchange pos2"))?
                    .as_str();
                let pos1 = pos1_str.parse::<u8>()?;
                let pos2 = pos2_str.parse::<u8>()?;
                moves.push(Move::Exchange(pos1, pos2));
            } else if partner_re.is_match(token) {
                let caps = partner_re
                    .captures(token)
                    .ok_or(anyhow!("invalid partner captures"))?;
                let name1_str = caps
                    .get(1)
                    .ok_or(anyhow!("invalid partner name1"))?
                    .as_str();
                let name2_str = caps
                    .get(2)
                    .ok_or(anyhow!("invalid partner name2"))?
                    .as_str();
                let name1 = name1_str
                    .chars()
                    .next()
                    .ok_or(anyhow!("name1 not a char"))?;
                let name2 = name2_str
                    .chars()
                    .next()
                    .ok_or(anyhow!("name2 not a char"))?;
                moves.push(Move::Partner(name1, name2));
            } else {
                return Err(anyhow!(format!("Invalid token: {token}")));
            }
        }
    }
    Ok((moves, dancers.to_vec()))
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: Moves) -> String {
    find_res(data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(data: Moves, second_star: bool) -> Result<String> {
    let (moves, mut dancers) = data;
    let orig = dancers.clone();
    if second_star {
        // The trick here is the patten repeats.  So we only need
        // to calculate 1_000_000_000 % repeat to figure this one out.
        let mut repeat = 0;
        for i in 0..1_000_000_000 {
            apply_moves(&moves, &mut dancers);
            if dancers == orig {
                repeat = i + 1;
                break;
            }
        }
        for _ in 0..(1_000_000_000 % repeat) {
            apply_moves(&moves, &mut dancers);
        }
    } else {
        apply_moves(&moves, &mut dancers);
    }
    Ok(dancers.iter().collect())
}

/// Apply moves
fn apply_moves(moves: &[Move], dancers: &mut [char]) {
    for mov in moves {
        match *mov {
            Move::Spin(ref x) => {
                let len = dancers.len();
                dancers.rotate_left(len - *x as usize);
            }
            Move::Exchange(ref x, ref y) => {
                let first = dancers[*x as usize];
                let second = dancers[*y as usize];
                dancers[*y as usize] = first;
                dancers[*x as usize] = second;
            }
            Move::Partner(ref x, ref y) => {
                let mut idx_x = 0;
                let mut idx_y = 0;
                let mut found = (false, false);

                for (i, val) in dancers.iter().enumerate() {
                    if found == (true, true) {
                        break;
                    }
                    if val == x {
                        idx_x = i;
                        found.0 = true;
                        continue;
                    }
                    if val == y {
                        idx_y = i;
                        found.1 = true;
                        continue;
                    }
                }

                dancers[idx_y] = *x;
                dancers[idx_x] = *y;
            }
        }
    }
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<Moves, String>(AoCYear::AOC2017, AoCDay::AOCD16, setup, find2).map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Moves, String>(bench, AoCYear::AOC2017, AoCDay::AOCD16, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: Moves) -> String {
    find_res(data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"s1,x3/4,pe/b";

    #[test]
    fn solution() -> Result<()> {
        let dancers = vec!['a', 'b', 'c', 'd', 'e'];
        let data = setup_br(Cursor::new(TEST_1), &dancers)?;
        assert_eq!(find(data), "baedc");
        Ok(())
    }
}

#[cfg(test)]
mod two_star {}
