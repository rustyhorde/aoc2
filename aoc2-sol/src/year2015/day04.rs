// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
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
//! starts with five zeroes (000001dbbfa...), and it is the lowest such number to do so.
//! * If your secret key is `pqrstuv`, the lowest number it combines with to make an MD5 hash
//! starting with five zeroes is `1048970`; that is, the MD5 hash of `pqrstuv1048970` looks like `000006136ef`....

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use bitvec::{order::Msb0, view::BitView};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
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
        result += (0..10_000_000)
            .into_par_iter()
            .find_first(|x| {
                let mashed = format!("{}{}", line, x);
                let md5 = md5::compute(mashed.as_bytes());
                let md5_b: [u8; 16] = md5.0;
                let blah = md5_b.view_bits::<Msb0>();
                let leading = blah.leading_zeros();
                leading >= 20
            })
            .ok_or_else(|| anyhow!("No value found"))?;
        if result == 0 {
            result += (10_000_000..100_000_000)
                .into_par_iter()
                .find_first(|x| {
                    let mashed = format!("{}{}", line, x);
                    let md5 = md5::compute(mashed.as_bytes());
                    let md5_b: [u8; 16] = md5.0;
                    let blah = md5_b.view_bits::<Msb0>();
                    let leading = blah.leading_zeros();
                    leading >= 20
                })
                .ok_or_else(|| anyhow!("No value found"))?;
        } else {
            result += 0;
        }
    }

    Ok(result)
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
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
        result += (0..usize::MAX)
            .into_par_iter()
            .find_first(|x| {
                let mashed = format!("{}{}", line, x);
                let md5 = md5::compute(mashed.as_bytes());
                let md5_str = format!("{:x}", md5);
                md5_str.starts_with("000000")
            })
            .ok_or_else(|| anyhow!("No value found"))?;
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
    #[ignore]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1))?, 609043);
        assert_eq!(find_br(Cursor::new(TEST_2))?, 1048970);
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
    #[ignore]
    fn solution() -> Result<()> {
        assert_eq!(find2_br(Cursor::new(TEST_1))?, 6742839);
        assert_eq!(find2_br(Cursor::new(TEST_2))?, 5714438);
        Ok(())
    }
}
