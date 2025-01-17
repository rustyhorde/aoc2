// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 6 "Signals and Noise"
//!
//! **--- Day 6: Signals and Noise ---**
//!
//! **--- Part 1 ---**
//!
//! Something is jamming your communications with Santa. Fortunately, your signal
//! is only partially jammed, and protocol in situations like this is to switch to a
//! simple repetition code to get the message through.
//!
//! In this model, the same message is sent repeatedly. You've recorded the
//! repeating message signal (your puzzle input), but the data seems quite corrupted -
//! almost too badly to recover. Almost.
//!
//! All you need to do is figure out which character is most frequent for each position.
//! For example, suppose you had recorded the following messages:
//!
//! ```text
//! eedadn
//! drvtee
//! eandsr
//! raavrd
//! atevrs
//! tsrnev
//! sdttsa
//! rasrtv
//! nssdts
//! ntnada
//! svetve
//! tesnvt
//! vntsnd
//! vrdear
//! dvrsen
//! enarar
//! ```
//!
//! The most common character in the first column is e; in the second, a; in the third, s,
//! and so on. Combining these characters returns the error-corrected message, `easter`.
//!
//! Given the recording in your puzzle input, what is the error-corrected version of the message being sent?
//!
//! **--- Part Two ---**
//!
//! Of course, that would be the message - if you hadn't agreed to use a modified repetition
//! code instead.
//!
//! In this modified code, the sender instead transmits what looks like random data, but
//! for each character, the character they actually want to send is slightly less likely
//! than the others. Even after signal-jamming noise, you can look at the letter distributions
//! in each column and choose the least common letter to reconstruct the original message.
//!
//! In the above example, the least common character in the first column is a; in the second,
//! d, and so on. Repeating this process for the remaining characters produces the original message,
//! `advent`.
//!
//! Given the recording in your puzzle input and this new decoding methodology, what is the
//! original message that Santa is trying to send?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_solution, valid_lines},
};
use anyhow::Result;
use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
    fs::File,
    io::{BufRead, BufReader},
};

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_solution::<String>(AoCYear::AOC2016, AoCDay::AOCD06, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> String {
    find_br(reader)
}

fn find_br<T>(reader: T) -> String
where
    T: BufRead,
{
    let mut freq_map = BTreeMap::new();
    for line in valid_lines(reader) {
        for (idx, ch) in line.chars().enumerate() {
            let idx_e = freq_map.entry(idx).or_insert_with(HashMap::new);
            *idx_e.entry(ch).or_insert(0) += 1;
        }
    }

    let tupled = freq_map
        .iter()
        .map(|(k, v)| {
            (
                *k,
                v.iter()
                    .map(|(k, v)| (*k, *v))
                    .sorted_by(|(ch1, c1), (ch2, c2)| {
                        if c2.cmp(c1) == Ordering::Equal {
                            ch1.cmp(ch2)
                        } else {
                            c2.cmp(c1)
                        }
                    })
                    .collect(),
            )
        })
        .collect::<BTreeMap<usize, Vec<(char, usize)>>>();

    let mut result = String::new();

    for (_k, v) in tupled {
        result.push(v[0].0);
    }
    result
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<String>(AoCYear::AOC2016, AoCDay::AOCD06, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> String {
    find2_br(reader)
}

fn find2_br<T>(reader: T) -> String
where
    T: BufRead,
{
    let mut freq_map = BTreeMap::new();
    for line in valid_lines(reader) {
        for (idx, ch) in line.chars().enumerate() {
            let idx_e = freq_map.entry(idx).or_insert_with(HashMap::new);
            *idx_e.entry(ch).or_insert(0) += 1;
        }
    }

    let tupled = freq_map
        .iter()
        .map(|(k, v)| {
            (
                *k,
                v.iter()
                    .map(|(k, v)| (*k, *v))
                    .sorted_by(|(ch1, c1), (ch2, c2)| {
                        if c1.cmp(c2) == Ordering::Equal {
                            ch1.cmp(ch2)
                        } else {
                            c1.cmp(c2)
                        }
                    })
                    .collect(),
            )
        })
        .collect::<BTreeMap<usize, Vec<(char, usize)>>>();

    let mut result = String::new();

    for (_k, v) in tupled {
        result.push(v[0].0);
    }
    result
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use std::io::Cursor;

    const TEST_1: &str = r"eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";

    #[test]
    fn solution() {
        assert_eq!(find_br(Cursor::new(TEST_1)), "easter");
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use std::io::Cursor;

    const TEST_1: &str = r"eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";

    #[test]
    fn solution() {
        assert_eq!(find2_br(Cursor::new(TEST_1)), "advent");
    }
}
