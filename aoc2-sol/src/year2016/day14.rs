// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 14 "One-Time Pad"
//!
//! **--- Day 14: One-Time Pad ---**
//!
//! **--- Part 1 ---**
//!
//! In order to communicate securely with Santa while you're on this mission,
//! you've been using a one-time pad that you generate using a pre-agreed algorithm.
//! Unfortunately, you've run out of keys in your one-time pad, and so you
//! need to generate some more.
//!
//! To generate keys, you first get a stream of random data by taking the MD5
//! of a pre-arranged salt (your puzzle input) and an increasing integer index
//! (starting with 0, and represented in decimal); the resulting MD5 hash should be
//! represented as a string of lowercase hexadecimal digits.
//!
//! However, not all of these MD5 hashes are keys, and you need 64 new keys for
//! your one-time pad. A hash is a key only if:
//!
//! ```text
//! It contains three of the same character in a row, like 777. Only consider the first such triplet in a hash.
//! One of the next 1000 hashes in the stream contains that same character five times in a row, like 77777.
//!```
//!
//! Considering future hashes for five-of-a-kind sequences does not cause those hashes
//! to be skipped; instead, regardless of whether the current hash is a key, always
//! resume testing for keys starting with the very next hash.
//!
//! For example, if the pre-arranged salt is `abc`:
//!
//! ```text
//! The first index which produces a triple is 18, because the MD5 hash of abc18 contains ...cc38887a5.... However, index 18 does not count as a key for your one-time pad, because none of the next thousand hashes (index 19 through index 1018) contain 88888.
//! The next index which produces a triple is 39; the hash of abc39 contains eee. It is also the first key: one of the next thousand hashes (the one at index 816) contains eeeee.
//! None of the next six triples are keys, but the one after that, at index 92, is: it contains 999 and index 200 contains 99999.
//! Eventually, index 22728 meets all of the criteria to generate the 64th key.
//! ```
//!
//! So, using our example salt of `abc`, index `22728` produces the 64th key.
//!
//! Given the actual salt in your puzzle input, what index produces your 64th one-time pad key?
//!
//! **--- Part Two ---**
//!
//! Of course, in order to make this process even more secure, you've also implemented key stretching.
//!
//! Key stretching forces attackers to spend more time generating hashes. Unfortunately,
//! it forces everyone else to spend more time, too.
//!
//! To implement key stretching, whenever you generate a hash, before you use it, you first
//! find the MD5 hash of that hash, then the MD5 hash of that hash, and so on, a total of
//! `2016` additional hashings. Always use lowercase hexadecimal representations of hashes.
//!
//! For example, to find the stretched hash for index `0` and salt `abc`:
//!
//! ```text
//! Find the MD5 hash of abc0: 577571be4de9dcce85a041ba0410f29f.
//! Then, find the MD5 hash of that hash: eec80a0c92dc8a0777c619d9bb51e910.
//! Then, find the MD5 hash of that hash: 16062ce768787384c81fe17a7a60c7e3.
//! ...repeat many times...
//! Then, find the MD5 hash of that hash: a107ff634856bb300138cac6568c0f24.
//! ```
//!
//! So, the stretched hash for index `0` in this situation is `a107ff....`
//! In the end, you find the original hash (one use of MD5), then find the hash-of-the-previous-hash
//~ 2016 times, for a total of 2017 uses of MD5.
//!
//! The rest of the process remains the same, but now the keys are entirely different.
//! Again for salt `abc`:
//!
//! ```text
//! The first triple (222, at index 5) has no matching 22222 in the next thousand hashes.
//! The second triple (eee, at index 10) hash a matching eeeee at index 89, and so it is the first key.
//! Eventually, index 22551 produces the 64th key (triple fff with matching fffff at index 22859.
//! ```
//!
//! Given the actual salt in your puzzle input and using 2016 extra MD5
//! calls of key stretching, what index now produces your 64th one-time pad key?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{print_err, run_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use md5::{Digest, Md5};
use std::{
    collections::VecDeque,
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
    run_solution::<usize>(AoCYear::AOC2016, AoCDay::AOCD14, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader).map_err(print_err).unwrap_or_default()
}

fn find_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let mut md5 = Md5::new();
    let (salt, mut hashes) = setup(reader, &mut md5, false);
    Ok(find_index(&salt, &mut hashes, &mut md5, false)? - 1000)
}

fn find_index(
    salt: &str,
    hashes: &mut VecDeque<String>,
    md5: &mut Md5,
    part2: bool,
) -> Result<usize> {
    let mut count = 0;
    let mut next_idx = 1000;

    while count != 64 {
        let curr = hashes.pop_front().ok_or_else(|| anyhow!("invalid md5"))?;
        let mut triple = 0;
        for win in curr.as_bytes().windows(3) {
            if win[0] == win[1] && win[0] == win[2] {
                triple = win[0];
                break;
            }
        }

        if triple != 0 {
            'outer: for hash in hashes.iter() {
                for win in hash.as_bytes().windows(5) {
                    if win.iter().all(|x| *x == triple) {
                        count += 1;
                        break 'outer;
                    }
                }
            }
        }

        if count < 64 {
            hashes.push_back(generate_md5(next_idx, salt, md5, part2));
            next_idx += 1;
        }
    }
    Ok(next_idx)
}

fn setup<T>(reader: T, md5: &mut Md5, part2: bool) -> (String, VecDeque<String>)
where
    T: BufRead,
{
    let mut salt = String::new();
    for line in valid_lines(reader) {
        salt = line;
    }
    let hashes = generate_1000(&salt, md5, part2);
    (salt, hashes)
}

fn generate_1000(salt: &str, md5: &mut Md5, part2: bool) -> VecDeque<String> {
    let mut queue = VecDeque::new();
    for i in 0..1000 {
        queue.push_back(generate_md5(i, salt, md5, part2));
    }
    queue
}

fn generate_md5(idx: usize, salt: &str, md5: &mut Md5, part2: bool) -> String {
    let md5_in = format!("{salt}{idx}");
    md5.update(md5_in);
    let md5_hash = md5.finalize_reset();
    let mut hex = format!("{md5_hash:x}");

    if part2 {
        for _ in 0..2016 {
            md5.update(hex);
            let md5_hash = md5.finalize_reset();
            hex = format!("{md5_hash:x}");
        }
    }

    hex
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2016, AoCDay::AOCD14, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader).map_err(print_err).unwrap_or_default()
}

fn find2_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let mut md5 = Md5::new();
    let (salt, mut hashes) = setup(reader, &mut md5, true);
    Ok(find_index(&salt, &mut hashes, &mut md5, true)? - 1000)
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"abc";

    #[test]
    #[ignore]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1))?, 22728);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"abc";

    #[test]
    #[ignore]
    fn solution() -> Result<()> {
        assert_eq!(find2_br(Cursor::new(TEST_1))?, 22551);
        Ok(())
    }
}
