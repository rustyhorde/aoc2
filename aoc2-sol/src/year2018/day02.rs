// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2018 ---**
//!
//! **--- Day 2: Inventory Management System ---**
//!
//! You stop falling through time, catch your breath, and check the screen on the device. "Destination reached. Current Year: 1518. Current Location: North Pole Utility Closet 83N10." You made it! Now, to find those anomalies.
//!
//! Outside the utility closet, you hear footsteps and a voice. "...I'm not sure either. But now that so many people have chimneys, maybe he could sneak in that way?" Another voice responds, "Actually, we've been working on a new kind of suit that would let him fit through tight spaces like that. But, I heard that a few days ago, they lost the prototype fabric, the design plans, everything! Nobody on the team can even seem to remember important details of the project!"
//!
//! "Wouldn't they have had enough fabric to fill several boxes in the warehouse? They'd be stored together, so the box IDs should be similar. Too bad it would take forever to search the warehouse for two similar box IDs..." They walk too far away to hear any more.
//!
//! Late at night, you sneak to the warehouse - who knows what kinds of paradoxes you could cause if you were discovered - and use your fancy wrist device to quickly scan every box and produce a list of the likely candidates (your puzzle input).
//!
//! To make sure you didn't miss any, you scan the likely candidate boxes again, counting the number that have an ID containing exactly two of any letter and then separately counting those with exactly three of any letter. You can multiply those two counts together to get a rudimentary checksum and compare it to what your device predicts.
//!
//! For example, if you see the following box IDs:
//!
//! ```text
//!     abcdef contains no letters that appear exactly two or three times.
//!     bababc contains two a and three b, so it counts for both.
//!     abbcde contains two b, but no letter appears exactly three times.
//!     abcccd contains three c, but no letter appears exactly two times.
//!     aabcdd contains two a and two d, but it only counts once.
//!     abcdee contains two e.
//!     ababab contains three a and three b, but it only counts once.
//! ```
//!
//! Of these box IDs, four of them contain a letter which appears exactly twice, and three of them contain a letter which appears exactly three times. Multiplying these together produces a checksum of 4 * 3 = 12.
//!
//! What is the checksum for your list of box IDs?
//!
//! **--- Part Two ---**
//!
//! Confident that your list of box IDs is complete, you're ready to find the boxes full of prototype fabric.
//!
//! The boxes will have IDs which differ by exactly one character at the same position in both strings. For example, given the following box IDs:
//!
//! ```text
//! abcde
//! fghij
//! klmno
//! pqrst
//! fguij
//! axcye
//! wvxyz
//! ```
//!
//! The IDs abcde and axcye are close, but they differ by two characters (the second and fourth). However, the IDs fghij and fguij differ by exactly one character, the third (h and u). Those must be the correct boxes.
//!
//! What letters are common between the two correct box IDs? (In the example above, this is found by removing the differing character from either ID, producing fgij.)

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::Result;
use std::{
    collections::HashMap,
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
    run_setup_solution::<Vec<String>, usize>(AoCYear::AOC2018, AoCDay::AOCD02, setup, find)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<String>, usize>(bench, AoCYear::AOC2018, AoCDay::AOCD02, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> Vec<String> {
    setup_br(reader).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T) -> Result<Vec<String>>
where
    T: BufRead,
{
    let mut data = vec![];
    for line in valid_lines(reader) {
        data.push(line);
    }
    Ok(data)
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: Vec<String>) -> usize {
    find_res(data).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(data: Vec<String>) -> Result<usize> {
    let mut twos = 0;
    let mut threes = 0;

    for line in data {
        let (has_two, has_three) = has_two_or_three(&line);
        if has_two {
            twos += 1;
        };
        if has_three {
            threes += 1;
        };
    }

    Ok(twos * threes)
}

fn has_two_or_three(line: &str) -> (bool, bool) {
    let mut char_freq = HashMap::new();
    let mut result = (false, false);
    let chars: Vec<char> = line.chars().collect();
    for ch in chars {
        let freq = char_freq.entry(ch).or_insert(0);
        *freq += 1;
    }

    for val in char_freq.values() {
        if *val == 2 {
            result.0 = true;
            break;
        }
    }

    for val in char_freq.values() {
        if *val == 3 {
            result.1 = true;
            break;
        }
    }
    result
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<Vec<String>, String>(AoCYear::AOC2018, AoCDay::AOCD02, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<String>, String>(bench, AoCYear::AOC2018, AoCDay::AOCD02, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: Vec<String>) -> String {
    find2_res(data).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find2_res(data: Vec<String>) -> Result<String> {
    let mut all_ids = Vec::new();

    for line in data {
        all_ids.push(line);
    }

    Ok(find_closest(&mut all_ids))
}

fn find_closest(all_ids: &mut Vec<String>) -> String {
    let mut matches = Vec::new();

    while !all_ids.is_empty() {
        find_match(all_ids, &mut matches);
    }

    if let Some(longest) = matches.iter().max_by_key(|x| x.len()) {
        longest.clone()
    } else {
        String::new()
    }
}

fn find_match(all_ids: &mut Vec<String>, matches: &mut Vec<String>) {
    let current = all_ids.remove(0);
    let curr_ch: Vec<char> = current.chars().collect();

    for id in all_ids {
        matches.push(
            curr_ch
                .iter()
                .zip(id.chars())
                .filter(|(a, b)| *a == b)
                .map(|(_, b)| b)
                .collect(),
        );
    }
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"abcedf
bababc
abbcde
abcccd
aabcdd
abcdee
ababab";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find(data), 12);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find2(data), "fgij");
        Ok(())
    }
}
