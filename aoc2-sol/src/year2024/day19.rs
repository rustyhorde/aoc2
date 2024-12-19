// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2024 ---**
//!
//! **--- Day 19: Linen Layout ---**
//!
//! Today, The Historians take you up to the hot springs on Gear Island! Very suspiciously, absolutely nothing goes wrong as they begin their careful search of the vast field of helixes.
//!
//! Could this finally be your chance to visit the onsen next door? Only one way to find out.
//!
//! After a brief conversation with the reception staff at the onsen front desk, you discover that you don't have the right kind of money to pay the admission fee. However, before you can leave, the staff get your attention. Apparently, they've heard about how you helped at the hot springs, and they're willing to make a deal: if you can simply help them arrange their towels, they'll let you in for free!
//!
//! Every towel at this onsen is marked with a pattern of colored stripes. There are only a few patterns, but for any particular pattern, the staff can get you as many towels with that pattern as you need. Each stripe can be white (w), blue (u), black (b), red (r), or green (g). So, a towel with the pattern ggr would have a green stripe, a green stripe, and then a red stripe, in that order. (You can't reverse a pattern by flipping a towel upside-down, as that would cause the onsen logo to face the wrong way.)
//!
//! The Official Onsen Branding Expert has produced a list of designs - each a long sequence of stripe colors - that they would like to be able to display. You can use any towels you want, but all of the towels' stripes must exactly match the desired design. So, to display the design rgrgr, you could use two rg towels and then an r towel, an rgr towel and then a gr towel, or even a single massive rgrgr towel (assuming such towel patterns were actually available).
//!
//! To start, collect together all of the available towel patterns and the list of desired designs (your puzzle input). For example:
//!
//! ```text
//! r, wr, b, g, bwu, rb, gb, br
//!
//! brwrr
//! bggr
//! gbbr
//! rrbgbr
//! ubwu
//! bwurrg
//! brgr
//! bbrgwb
//! ```
//!
//! The first line indicates the available towel patterns; in this example, the onsen has unlimited towels with a single red stripe (r), unlimited towels with a white stripe and then a red stripe (wr), and so on.
//!
//! After the blank line, the remaining lines each describe a design the onsen would like to be able to display. In this example, the first design (brwrr) indicates that the onsen would like to be able to display a black stripe, a red stripe, a white stripe, and then two red stripes, in that order.
//!
//! Not all designs will be possible with the available towels. In the above example, the designs are possible or impossible as follows:
//!
//! ```text
//!     brwrr can be made with a br towel, then a wr towel, and then finally an r towel.
//!     bggr can be made with a b towel, two g towels, and then an r towel.
//!     gbbr can be made with a gb towel and then a br towel.
//!     rrbgbr can be made with r, rb, g, and br.
//!     ubwu is impossible.
//!     bwurrg can be made with bwu, r, r, and g.
//!     brgr can be made with br, g, and r.
//!     bbrgwb is impossible.
//! ```
//!
//! In this example, 6 of the eight designs are possible with the available towel patterns.
//!
//! To get into the onsen as soon as possible, consult your list of towel patterns and desired designs carefully. How many designs are possible?
//!
//! **--- Part Two ---**
//!
//! The staff don't really like some of the towel arrangements you came up with. To avoid an endless cycle of towel rearrangement, maybe you should just give them every possible option.
//!
//! Here are all of the different ways the above example's designs can be made:
//!
//! brwrr can be made in two different ways: b, r, wr, r or br, wr, r.
//!
//! bggr can only be made with b, g, g, and r.
//!
//! gbbr can be made 4 different ways:
//!
//! ```text
//!     g, b, b, r
//!     g, b, br
//!     gb, b, r
//!     gb, br
//! ```
//!
//! rrbgbr can be made 6 different ways:
//!
//! ```text
//!     r, r, b, g, b, r
//!     r, r, b, g, br
//!     r, r, b, gb, r
//!     r, rb, g, b, r
//!     r, rb, g, br
//!     r, rb, gb, r
//! ```
//!
//! bwurrg can only be made with bwu, r, r, and g.
//!
//! brgr can be made in two different ways: b, r, g, r or br, g, r.
//!
//! ubwu and bbrgwb are still impossible.
//!
//! Adding up all of the ways the towels in this example could be arranged into the desired designs yields 16 (2 + 1 + 4 + 6 + 1 + 2).
//!
//! They'll let you into the onsen as soon as you have the list. What do you get if you add up the number of different ways you could make each design?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::Result;
use memoize::memoize;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

type TowelData = (Vec<String>, Vec<String>);

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<TowelData, usize>(AoCYear::AOC2024, AoCDay::AOCD19, setup, find).map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<TowelData, usize>(bench, AoCYear::AOC2024, AoCDay::AOCD19, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> TowelData {
    setup_br(reader).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T) -> Result<TowelData>
where
    T: BufRead,
{
    let mut towel_parts = vec![];
    let mut patterns = vec![];
    for line in valid_lines(reader) {
        if line.contains(',') {
            towel_parts = line
                .split(", ")
                .map(str::to_string)
                .collect::<Vec<String>>();
        } else if !line.is_empty() {
            patterns.push(line);
        }
    }
    Ok((towel_parts, patterns))
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: TowelData) -> usize {
    find_res(data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(data: TowelData, second_star: bool) -> Result<usize> {
    let (towel_parts, towel_patterns) = data;
    if second_star {
        Ok(towel_patterns
            .iter()
            .cloned()
            .map(|x| is_pattern_possible_2(x, &towel_parts))
            .sum())
    } else {
        Ok(towel_patterns
            .iter()
            .cloned()
            .map(|x| is_pattern_possible(x, &towel_parts))
            .filter(|x| *x)
            .count())
    }
}

#[memoize(Ignore: parts)]
#[allow(clippy::needless_pass_by_value)]
fn is_pattern_possible(pattern: String, parts: &[String]) -> bool {
    if pattern.is_empty() {
        true
    } else {
        let mut possibles = vec![];
        for part in parts {
            if pattern.starts_with(part) {
                possibles.push(is_pattern_possible(
                    pattern.strip_prefix(part).unwrap_or_default().to_string(),
                    parts,
                ));
            }
        }
        possibles.iter().any(|x| *x)
    }
}

#[memoize(Ignore: parts)]
#[allow(clippy::needless_pass_by_value)]
fn is_pattern_possible_2(pattern: String, parts: &[String]) -> usize {
    if pattern.is_empty() {
        1
    } else {
        let mut possibles = vec![];
        for part in parts {
            if pattern.starts_with(part) {
                possibles.push(is_pattern_possible_2(
                    pattern.strip_prefix(part).unwrap_or_default().to_string(),
                    parts,
                ));
            }
        }
        possibles.iter().sum()
    }
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<TowelData, usize>(AoCYear::AOC2024, AoCDay::AOCD19, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<TowelData, usize>(bench, AoCYear::AOC2024, AoCDay::AOCD19, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: TowelData) -> usize {
    find_res(data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find(data), 6);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find2(data), 16);
        Ok(())
    }
}
