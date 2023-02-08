// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 5 "How About a Nice Game of Chess?"
//!
//! --- Day 5: How About a Nice Game of Chess? ---
//!
//! You are faced with a security door designed by Easter Bunny engineers that
//! seem to have acquired most of their security knowledge by watching
//! hacking movies.
//!
//! The eight-character password for the door is generated one character at a
//! time by finding the MD5 hash of some Door ID (your puzzle input) and an
//! increasing integer index (starting with 0).
//!
//! A hash indicates the next character in the password if its hexadecimal
//! representation starts with five zeroes. If it does, the sixth character
//! in the hash is the next character of the password.
//!
//! For example, if the Door ID is `abc`:
//!
//! ```text
//! The first index which produces a hash that starts with five zeroes is 3231929, which we find by hashing abc3231929; the sixth character of the hash, and thus the first character of the password, is 1.
//! 5017308 produces the next interesting hash, which starts with 000008f82..., so the second character of the password is 8.
//! The third time a hash starts with five zeroes is for abc5278568, discovering the character f.
//! ```
//!
//! In this example, after continuing this search a total of eight times, the password is `18f47a30`.
//!
//! Given the actual Door ID, what is the password?
//!
//! **--- Part Two ---**
//!
//! As the door slides open, you are presented with a second door that uses a slightly more inspired security
//! mechanism. Clearly unimpressed by the last version (in what movie is the password
//! decrypted in order?!), the Easter Bunny engineers have worked out a better solution.
//!
//! Instead of simply filling in the password from left to right, the hash now also indicates the
//! position within the password to fill. You still look for hashes that begin with five zeroes;
//! however, now, the sixth character represents the position (0-7), and the seventh character is the
//! character to put in that position.
//!
//! A hash result of `000001f` means that `f` is the second character in the password. Use only the
//! first result for each position, and ignore invalid positions.
//!
//! For example, if the Door ID is `abc`:
//!
//! ```text
//! The first interesting hash is from abc3231929, which produces 0000015...; so, 5 goes in position 1: _5______.
//! In the previous method, 5017308 produced an interesting hash; however, it is ignored, because it specifies an invalid position (8).
//! The second interesting hash is at index 5357525, which produces 000004e...; so, e goes in position 4: _5__e___.
//! ```
//!
//! You almost choke on your popcorn as the final character falls into place, producing the password `05ace8e3`.
//!
//! Given the actual Door ID and this new method, what is the password? Be extra proud of your solution
//! if it uses a cinematic "decrypting" animation.
//!

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_solution, valid_lines},
};
use anyhow::Result;
use md5::{Digest, Md5};
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
    run_solution::<String>(AoCYear::AOC2016, AoCDay::AOCD05, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> String {
    find_br(reader)
}

fn find_br<T>(reader: T) -> String
where
    T: BufRead,
{
    let mut password = String::new();

    for line in valid_lines(reader) {
        let hasher = Md5::new();
        let mut idx: usize = 0;
        loop {
            let mut my_hasher = hasher.clone();
            let mut to_hash = String::from(&line);
            to_hash.push_str(&idx.to_string());
            my_hasher.update(to_hash.as_bytes());

            let hash = my_hasher.finalize();
            let hash_str = format!("{hash:x}");
            if hash_str.starts_with("00000") {
                if let Some(v) = hash_str.get(5..6) {
                    password.push_str(v);
                }

                if password.len() == 8 {
                    break;
                }
            }
            idx += 1;
        }
    }
    password
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<String>(AoCYear::AOC2016, AoCDay::AOCD05, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> String {
    find2_br(reader)
}

fn find2_br<T>(reader: T) -> String
where
    T: BufRead,
{
    let mut spots = Vec::with_capacity(8);
    for _i in 0..8 {
        spots.push(String::new());
    }

    for line in valid_lines(reader) {
        let hasher = Md5::new();
        let mut idx: usize = 0;
        loop {
            let mut my_hasher = hasher.clone();
            let mut to_hash = String::from(&line);
            to_hash.push_str(&idx.to_string());
            my_hasher.update(to_hash.as_bytes());

            let hash = my_hasher.finalize();
            let hash_str = format!("{hash:x}");
            if hash_str.starts_with("00000") {
                let idx_h_str = hash_str.clone();
                let v_h_str = hash_str.clone();
                if let Some(idx_str) = idx_h_str.get(5..6) {
                    if let Some(v) = v_h_str.get(6..7) {
                        if let Ok(idx) = idx_str.parse::<usize>() {
                            if let Some(at) = spots.get(idx) {
                                if at.is_empty() {
                                    spots[idx] = v.to_string();
                                }
                            }
                        }
                    }
                }

                if spots.len() == 8 && spots.iter().all(|x| !x.is_empty()) {
                    break;
                }
            }
            idx += 1;
        }
    }
    spots.join("")
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use std::io::Cursor;

    const TEST_1: &str = r"abc";

    #[test]
    #[ignore]
    fn solution() {
        assert_eq!(find_br(Cursor::new(TEST_1)), "18f47a30");
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use std::io::Cursor;

    const TEST_1: &str = r"abc";

    #[test]
    #[ignore]
    fn solution() {
        assert_eq!(find2_br(Cursor::new(TEST_1)), "05ace8e3");
    }
}
