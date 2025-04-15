// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 16 "Aunt Sue"
//!
//! **--- Day 16: Aunt Sue ---**
//!
//! **--- Part 1 ---**
//!
//! Your Aunt Sue has given you a wonderful gift, and you'd like to send her
//! a thank you card. However, there's a small problem: she signed it "From, Aunt Sue".
//!
//! You have 500 Aunts named "Sue".
//!
//! So, to avoid sending the card to the wrong person, you need to figure out
//! which Aunt Sue (which you conveniently number 1 to 500, for sanity) gave you the gift.
//! You open the present and, as luck would have it, good ol' Aunt Sue got you a
//! My First Crime Scene Analysis Machine! Just what you wanted. Or needed, as the case may be.
//!
//! The My First Crime Scene Analysis Machine (MFCSAM for short) can detect a few specific
//! compounds in a given sample, as well as how many distinct kinds of
//! those compounds there are. According to the instructions, these are what the MFCSAM can detect:
//!
//! ```text
//! children, by human DNA age analysis.
//! cats. It doesn't differentiate individual breeds.
//! Several seemingly random breeds of dog: samoyeds, pomeranians, akitas, and vizslas.
//! goldfish. No other kinds of fish.
//! trees, all in one group.
//! cars, presumably by exhaust or gasoline or something.
//! perfumes, which is handy, since many of your Aunts Sue wear a few kinds.
//! ```
//!
//! In fact, many of your Aunts Sue have many of these. You put the wrapping from the
//! gift into the MFCSAM. It beeps inquisitively at you a few times and then prints out
//! a message on ticker tape:
//!
//! ```text
//! children: 3
//! cats: 7
//! samoyeds: 2
//! pomeranians: 3
//! akitas: 0
//! vizslas: 0
//! goldfish: 5
//! trees: 3
//! cars: 2
//! perfumes: 1
//! ```
//!
//! You make a list of the things you can remember about each Aunt Sue. Things missing
//! from your list aren't zero - you simply don't remember the value.
//!
//! What is the number of the Sue that got you the gift?
//!
//! **--- Part Two ---**
//!
//! As you're about to send the thank you note, something in the MFCSAM's instructions
//! catches your eye. Apparently, it has an outdated retroencabulator, and so the output
//! from the machine isn't exact values - some of them indicate ranges.
//!
//! In particular, the `cats` and `trees` readings indicates that there are greater than
//! that many (due to the unpredictable nuclear decay of cat dander and tree pollen), while
//! the `pomeranians` and `goldfish` readings indicate that there are fewer than that many
//! (due to the modial interaction of magnetoreluctance).
//!
//! What is the number of the real Aunt Sue?
//!

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{get_cap, get_cap_x, run_solution, valid_lines},
};
use anyhow::Result;
use getset::MutGetters;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy, Debug, Hash, Eq, MutGetters, PartialEq)]
#[getset(get_mut)]
struct Sue {
    num: usize,
    children: usize,
    cats: usize,
    samoyeds: usize,
    pomeranians: usize,
    akitas: usize,
    vizslas: usize,
    goldfish: usize,
    trees: usize,
    cars: usize,
    perfumes: usize,
}

impl Default for Sue {
    fn default() -> Self {
        Self {
            num: usize::default(),
            children: usize::MAX,
            cats: usize::MAX,
            samoyeds: usize::MAX,
            pomeranians: usize::MAX,
            akitas: usize::MAX,
            vizslas: usize::MAX,
            goldfish: usize::MAX,
            trees: usize::MAX,
            cars: usize::MAX,
            perfumes: usize::MAX,
        }
    }
}
/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD16, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader).unwrap_or_default()
}

#[allow(clippy::needless_collect)]
fn find_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let sue_re = Regex::new(r"^Sue (\d+): (.*): (\d+), (.*): (\d+), (.*): (\d+)$")?;
    let mut sues = vec![];
    for line in valid_lines(reader) {
        for caps in sue_re.captures_iter(&line) {
            let num = get_cap_x::<usize>(1, &caps)?;
            let item1 = get_cap(2, &caps)?;
            let item1_count = get_cap_x::<usize>(3, &caps)?;
            let item2 = get_cap(4, &caps)?;
            let item2_count = get_cap_x::<usize>(5, &caps)?;
            let item3 = get_cap(6, &caps)?;
            let item3_count = get_cap_x::<usize>(7, &caps)?;

            let mut sue = Sue::default();
            *sue.num_mut() = num;
            add_item(&mut sue, &item1, item1_count);
            add_item(&mut sue, &item2, item2_count);
            add_item(&mut sue, &item3, item3_count);
            sues.push(sue);
        }
    }

    let matching_sues = sues
        .iter()
        .filter(|a| {
            (a.children == 3 || a.children == usize::MAX)
                && (a.cats == 7 || a.cats == usize::MAX)
                && (a.samoyeds == 2 || a.samoyeds == usize::MAX)
                && (a.pomeranians == 3 || a.pomeranians == usize::MAX)
                && (a.akitas == 0 || a.akitas == usize::MAX)
                && (a.vizslas == 0 || a.vizslas == usize::MAX)
                && (a.goldfish == 5 || a.goldfish == usize::MAX)
                && (a.trees == 3 || a.trees == usize::MAX)
                && (a.cars == 2 || a.cars == usize::MAX)
                && (a.perfumes == 1 || a.perfumes == usize::MAX)
        })
        .collect::<Vec<&Sue>>();

    Ok(matching_sues[0].num)
}

fn add_item(sue: &mut Sue, item: &str, count: usize) {
    match item {
        "children" => *sue.children_mut() = count,
        "cats" => *sue.cats_mut() = count,
        "samoyeds" => *sue.samoyeds_mut() = count,
        "pomeranians" => *sue.pomeranians_mut() = count,
        "akitas" => *sue.akitas_mut() = count,
        "vizslas" => *sue.vizslas_mut() = count,
        "goldfish" => *sue.goldfish_mut() = count,
        "trees" => *sue.trees_mut() = count,
        "cars" => *sue.cars_mut() = count,
        "perfumes" => *sue.perfumes_mut() = count,
        _ => {}
    }
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD16, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader).unwrap_or_default()
}

fn find2_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let sue_re = Regex::new(r"^Sue (\d+): (.*): (\d+), (.*): (\d+), (.*): (\d+)$")?;
    let mut sues = vec![];
    for line in valid_lines(reader) {
        for caps in sue_re.captures_iter(&line) {
            let num = get_cap_x::<usize>(1, &caps)?;
            let item1 = get_cap(2, &caps)?;
            let item1_count = get_cap_x::<usize>(3, &caps)?;
            let item2 = get_cap(4, &caps)?;
            let item2_count = get_cap_x::<usize>(5, &caps)?;
            let item3 = get_cap(6, &caps)?;
            let item3_count = get_cap_x::<usize>(7, &caps)?;

            let mut sue = Sue::default();
            *sue.num_mut() = num;
            add_item(&mut sue, &item1, item1_count);
            add_item(&mut sue, &item2, item2_count);
            add_item(&mut sue, &item3, item3_count);
            sues.push(sue);
        }
    }

    let matching_sues = sues
        .iter()
        .filter(|a| {
            (a.children == 3 || a.children == usize::MAX)
                && (a.cats > 7)
                && (a.samoyeds == 2 || a.samoyeds == usize::MAX)
                && (a.pomeranians < 3 || a.pomeranians == usize::MAX)
                && (a.akitas == 0 || a.akitas == usize::MAX)
                && (a.vizslas == 0 || a.vizslas == usize::MAX)
                && (a.goldfish < 5 || a.goldfish == usize::MAX)
                && (a.trees > 3)
                && (a.cars == 2 || a.cars == usize::MAX)
                && (a.perfumes == 1 || a.perfumes == usize::MAX)
        })
        .collect::<Vec<&Sue>>();

    Ok(matching_sues[0].num)
}

#[cfg(test)]
mod one_star {
    // use super::find_br;
    // use std::io::Cursor;

    // const TEST_1: &str = r"";

    #[test]
    fn solution() {
        // assert_eq!(find_br(Cursor::new(TEST_1))?, 1_000_000);
    }
}

#[cfg(test)]
mod two_star {
    // use super::find2_br;
    // use std::io::Cursor;

    // const TEST_1: &str = r"turn on 0,0 through 0,0";
    // const TEST_2: &str = r"toggle 0,0 through 999,999";

    #[test]
    fn solution() {
        // assert_eq!(find2_br(Cursor::new(TEST_1))?, 1);
        // assert_eq!(find2_br(Cursor::new(TEST_2))?, 2_000_000);
    }
}
