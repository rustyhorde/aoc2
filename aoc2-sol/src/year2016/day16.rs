// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 16 "Dragon Checksum"
//!
//! **--- Day 16: Dragon Checksum ---**
//!
//! **--- Part 1 ---**
//!
//! You're done scanning this part of the network, but you've left traces of your
//! presence. You need to overwrite some disks with random-looking data to cover
//! your tracks and update the local security system with a new checksum for those
//! disks.
//!
//! For the data to not be suspicious, it needs to have certain properties; purely
//! random data will be detected as tampering. To generate appropriate random data,
//! you'll need to use a modified dragon curve.
//!
//! Start with an appropriate initial state (your puzzle input). Then, so long as
//! you don't have enough data yet to fill the disk, repeat the following steps:
//!
//! ```text
//! Call the data you have at this point "a".
//! Make a copy of "a"; call this copy "b".
//! Reverse the order of the characters in "b".
//! In "b", replace all instances of 0 with 1 and all 1s with 0.
//! The resulting data is "a", then a single 0, then "b".
//! ```
//!
//! For example, after a single step of this process,
//!
//! ```text
//! 1 becomes 100.
//! 0 becomes 001.
//! 11111 becomes 11111000000.
//! 111100001010 becomes 1111000010100101011110000.
//! ```
//!
//! Repeat these steps until you have enough data to fill the desired disk.
//!
//! Once the data has been generated, you also need to create a checksum of that data.
//! Calculate the checksum only for the data that fits on the disk, even if you generated
//! more data than that in the previous step.
//!
//! The checksum for some given data is created by considering each non-overlapping pair
//! of characters in the input data. If the two characters match (00 or 11), the next checksum
//! character is a 1. If the characters do not match (01 or 10), the next checksum character
//! is a 0. This should produce a new string which is exactly half as long as the original.
//! If the length of the checksum is even, repeat the process until you end up with a checksum
//! with an odd length.
//!
//! For example, suppose we want to fill a disk of length 12, and when we finally generate a
//! string of at least length 12, the first 12 characters are `110010110100`.
//! To generate its checksum:
//!
//! ```text
//! Consider each pair: 11, 00, 10, 11, 01, 00.
//! These are same, same, different, same, different, same, producing 110101.
//! The resulting string has length 6, which is even, so we repeat the process.
//! The pairs are 11 (same), 01 (different), 01 (different).
//! This produces the checksum 100, which has an odd length, so we stop.
//! ```
//!
//! Therefore, the checksum for `110010110100` is `100`.
//!
//! Combining all of these steps together, suppose you want to fill a disk of length 20
//! using an initial state of `10000`:
//!
//! ```text
//! Because 10000 is too short, we first use the modified dragon curve to make it longer.
//! After one round, it becomes 10000011110 (11 characters), still too short.
//! After two rounds, it becomes 10000011110010000111110 (23 characters), which is enough.
//! Since we only need 20, but we have 23, we get rid of all but the first 20 characters: 10000011110010000111.
//! Next, we start calculating the checksum; after one round, we have 0111110101, which 10 characters long (even), so we continue.
//! After two rounds, we have 01100, which is 5 characters long (odd), so we are done.
//! ```
//!
//! In this example, the correct checksum would therefore be `01100`.
//!
//! The first disk you have to fill has length `272`. Using the initial state in your puzzle input,
//! what is the correct checksum?
//!
//! **--- Part Two ---**
//!
//! The second disk you have to fill has length `35651584`. Again using the initial state in your
//! puzzle input, what is the correct checksum for this disk?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{print_err, run_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use bitvec::prelude::*;
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
    run_solution::<String>(AoCYear::AOC2016, AoCDay::AOCD16, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> String {
    find_br(reader, 272).map_err(print_err).unwrap_or_default()
}

fn find_br<T>(reader: T, disk_len: usize) -> Result<String>
where
    T: BufRead,
{
    let mut result = String::new();
    for line in valid_lines(reader) {
        let mut bv = BitVec::with_capacity(line.len());
        for ch in line.chars() {
            match ch {
                '1' => bv.push(true),
                '0' => bv.push(false),
                _ => return Err(anyhow!("invalid input")),
            }
        }
        while bv.len() < disk_len {
            dragon(&mut bv);
        }

        let slice = &bv[0..disk_len];
        let mut bv = slice.to_bitvec();
        loop {
            let mut chk_sum = BitVec::new();
            for chunk in bv.chunks(2) {
                if chunk[0] == chunk[1] {
                    chk_sum.push(true);
                } else {
                    chk_sum.push(false);
                }
            }

            if chk_sum.len() % 2 == 0 {
                bv = chk_sum;
                continue;
            }
            bv = chk_sum;
            break;
        }
        result = format!("{bv}");
    }
    result.retain(|c| c != '[' && c != ']' && c != ',' && c != ' ');
    Ok(result)
}

fn dragon(data: &mut BitVec<usize, Msb0>) {
    let mut copy = data.to_bitvec();
    copy = copy.iter().rev().map(|x| !x).collect();
    data.push(false);
    data.extend_from_bitslice(&copy);
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<String>(AoCYear::AOC2016, AoCDay::AOCD16, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> String {
    find2_br(reader).map_err(print_err).unwrap_or_default()
}

fn find2_br<T>(reader: T) -> Result<String>
where
    T: BufRead,
{
    find_br(reader, 35_651_584)
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"10000";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1), 20)?, "01100");
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    // use super::find2_br;
    // use std::io::Cursor;

    // const TEST_1: &str = r"^v";
    // const TEST_2: &str = r"^>v<";
    // const TEST_3: &str = r"^v^v^v^v^v";

    #[test]
    fn solution() {
        // assert_eq!(find2_br(Cursor::new(TEST_1))?, 3);
    }
}
