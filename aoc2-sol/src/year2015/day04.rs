// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 4 "The Ideal Stocking Stuffer"
//!
//! **--- Day 4: The Ideal Stocking Stuffer ---**
//!
//! **--- Part 1 ---**
//!
//! Santa needs help mining some `AdventCoins` (very similar to bitcoins)
//! to use as gifts for all the economically forward-thinking little girls and boys.
//!
//! To do this, he needs to find MD5 hashes which, in hexadecimal, start with at least five zeroes.
//! The input to the MD5 hash is some secret key (your puzzle input, given below) followed by a
//! number in decimal. To mine `AdventCoins`, you must find Santa the lowest positive number
//! (no leading zeroes: 1, 2, 3, ...) that produces such a hash.
//!
//! For example:
//!
//! * If your secret key is `abcdef`, the answer is `609043`, because the MD5 hash of `abcdef609043`
//!   starts with five zeroes (000001dbbfa...), and it is the lowest such number to do so.
//! * If your secret key is `pqrstuv`, the lowest number it combines with to make an MD5 hash
//!   starting with five zeroes is `1048970`; that is, the MD5 hash of `pqrstuv1048970` looks like `000006136ef`....

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use bitvec::{order::Msb0, view::BitView};
use md5::Digest;
use rayon::iter::{repeat, IntoParallelIterator, ParallelIterator};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::Range,
};
const TEN_MIL: Range<usize> = 0..10_000_000;
const HUN_MIL: Range<usize> = 10_000_000..100_000_000;
const BIL: Range<usize> = 100_000_000..1_000_000_000;
const REST: Range<usize> = 1_000_000_000..usize::MAX;
const PART_1_LZ: usize = 20;
const PART_2_LZ: usize = 24;

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD04, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader).unwrap_or_default()
}

fn find_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let mut result = 0;

    for line in valid_lines(reader) {
        result = check_all_ranges(&line, PART_1_LZ)?.1;
    }

    Ok(result)
}

fn check_all_ranges(line: &str, lz: usize) -> Result<((&str, usize), usize)> {
    check_range(line, lz, TEN_MIL)
        .or_else(|| check_range(line, lz, HUN_MIL))
        .or_else(|| check_range(line, lz, BIL))
        .or_else(|| check_range(line, lz, REST))
        .ok_or_else(|| anyhow!("Could not find match"))
}

fn check_range(line: &str, lz: usize, range: Range<usize>) -> Option<((&str, usize), usize)> {
    repeat((line, lz))
        .zip(range)
        .into_par_iter()
        .find_first(has_enough_leading_zeros)
}

fn has_enough_leading_zeros(tuple: &((&str, usize), usize)) -> bool {
    let digest = md5::Md5::digest(format!("{}{}", (tuple.0).0, tuple.1).as_bytes());
    digest.view_bits::<Msb0>().leading_zeros() >= (tuple.0).1
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD04, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader).unwrap_or_default()
}

fn find2_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let mut result = 0;

    for line in valid_lines(reader) {
        result = check_all_ranges(&line, PART_2_LZ)?.1;
    }

    Ok(result)
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"abcdef";
    const TEST_2: &str = r"pqrstuv";

    #[test]
    #[ignore = "slow"]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1))?, 609_043);
        assert_eq!(find_br(Cursor::new(TEST_2))?, 1_048_970);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"abcdef";
    const TEST_2: &str = r"pqrstuv";

    #[test]
    #[ignore = "slow"]
    fn solution() -> Result<()> {
        assert_eq!(find2_br(Cursor::new(TEST_1))?, 6_742_839);
        assert_eq!(find2_br(Cursor::new(TEST_2))?, 5_714_438);
        Ok(())
    }
}
