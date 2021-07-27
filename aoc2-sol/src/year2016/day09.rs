// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 9 "Explosives in Cyberspace"
//!
//! **--- Day 9: Explosives in Cyberspace ---**
//!
//! **--- Part 1 ---**
//!
//! Wandering around a secure area, you come across a datalink port to a new
//! part of the network. After briefly scanning it for interesting files, you
//! find one file in particular that catches your attention. It's compressed
//! with an experimental format, but fortunately, the documentation for the format is nearby.
//!
//! The format compresses a sequence of characters. Whitespace is ignored.
//! To indicate that some sequence should be repeated, a marker is added to the file,
//! like (10x2). To decompress this marker, take the subsequent 10 characters and
//! repeat them 2 times. Then, continue reading the file after the repeated data.
//! The marker itself is not included in the decompressed output.
//!
//! If parentheses or other characters appear within the data referenced by a marker,
//! that's okay - treat it like normal data, not a marker, and then resume looking
//! for markers after the decompressed section.
//!
//! For example:
//!
//! ```text
//! ADVENT contains no markers and decompresses to itself with no changes, resulting in a decompressed length of 6.
//! A(1x5)BC repeats only the B a total of 5 times, becoming ABBBBBC for a decompressed length of 7.
//! (3x3)XYZ becomes XYZXYZXYZ for a decompressed length of 9.
//! A(2x2)BCD(2x2)EFG doubles the BC and EF, becoming ABCBCDEFEFG for a decompressed length of 11.
//! (6x1)(1x3)A simply becomes (1x3)A - the (1x3) looks like a marker, but because it's within a data section of another marker, it is not treated any differently from the A that comes after it. It has a decompressed length of 6.
//! X(8x2)(3x3)ABCY becomes X(3x3)ABC(3x3)ABCY (for a decompressed length of 18), because the decompressed data from the (8x2) marker (the (3x3)ABC) is skipped and not processed further.
//! ```
//!
//! What is the decompressed length of the file (your puzzle input)? Don't count whitespace.
//!
//! **--- Part Two ---**
//!
//! Apparently, the file actually uses version two of the format.
//!
//! In version two, the only difference is that markers within decompressed data are
//! decompressed. This, the documentation explains, provides much more substantial
//! compression capabilities, allowing many-gigabyte files to be stored in only a few kilobytes.
//!
//! For example:
//!
//! ```text
//! (3x3)XYZ still becomes XYZXYZXYZ, as the decompressed section contains no markers.
//! X(8x2)(3x3)ABCY becomes XABCABCABCABCABCABCY, because the decompressed data from the (8x2) marker is then further decompressed, thus triggering the (3x3) marker twice for a total of six ABC sequences.
//! (27x12)(20x12)(13x14)(7x10)(1x12)A decompresses into a string of A repeated 241920 times.
//! (25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN becomes 445 characters long.
//! ```
//!
//! Unfortunately, the computer you brought probably doesn't have enough memory to
//! actually decompress the file; you'll have to come up with another way to get its
//! decompressed length.
//!
//! What is the decompressed length of the file using this improved format?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_solution, valid_lines},
};
use anyhow::Result;
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
    run_solution::<usize>(AoCYear::AOC2016, AoCDay::AOCD09, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader).unwrap_or_default()
}

fn find_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let mut uncompressed = String::new();
    for line in valid_lines(reader) {
        let mut determining_comp = false;
        let mut found_x = false;
        let mut count = vec![];
        let mut repeat = vec![];
        let mut repeat_str = vec![];
        let mut count_val = 0;
        let mut repeat_val = 0;
        for ch in line.chars() {
            match ch {
                '(' => {
                    if count_val > 0 {
                        count_val -= 1;
                        repeat_str.push(ch.to_string());
                        if count_val == 0 {
                            let r = repeat_str.join("");
                            for _i in 0..repeat_val {
                                uncompressed.push_str(&r);
                            }
                            repeat.clear();
                            repeat_str.clear();
                            repeat_val = 0;
                        }
                    } else {
                        determining_comp = true;
                    }
                }
                ')' => {
                    if determining_comp {
                        determining_comp = false;
                        found_x = false;
                        repeat_val = repeat.join("").parse::<usize>()?;
                    } else if count_val > 0 {
                        count_val -= 1;
                        repeat_str.push(ch.to_string());
                        if count_val == 0 {
                            let r = repeat_str.join("");
                            for _i in 0..repeat_val {
                                uncompressed.push_str(&r);
                            }
                            repeat.clear();
                            repeat_str.clear();
                            repeat_val = 0;
                        }
                    }
                }
                _ => {
                    if determining_comp && !found_x {
                        if ch == 'x' {
                            found_x = true;
                            count_val = count.join("").parse::<usize>()?;
                            count.clear();
                        } else {
                            count.push(ch.to_string());
                        }
                    } else if determining_comp && found_x {
                        repeat.push(ch.to_string());
                    } else if count_val > 0 {
                        count_val -= 1;
                        repeat_str.push(ch.to_string());
                        if count_val == 0 {
                            let r = repeat_str.join("");
                            for _i in 0..repeat_val {
                                uncompressed.push_str(&r);
                            }
                            repeat.clear();
                            repeat_str.clear();
                            repeat_val = 0;
                        }
                    } else {
                        uncompressed.push(ch);
                    }
                }
            }
        }
    }

    Ok(uncompressed.len())
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2016, AoCDay::AOCD09, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader)
}

fn find2_br<T>(reader: T) -> usize
where
    T: BufRead,
{
    for line in valid_lines(reader) {
        for _ch in line.chars() {}
    }
    0
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"ADVENT";
    const TEST_2: &str = r"A(1x5)BC";
    const TEST_3: &str = r"(3x3)XYZ";
    const TEST_4: &str = r"A(2x2)BCD(2x2)EFG";
    const TEST_5: &str = r"(6x1)(1x3)A";
    const TEST_6: &str = r"X(8x2)(3x3)ABCY";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1))?, 6);
        assert_eq!(find_br(Cursor::new(TEST_2))?, 7);
        assert_eq!(find_br(Cursor::new(TEST_3))?, 9);
        assert_eq!(find_br(Cursor::new(TEST_4))?, 11);
        assert_eq!(find_br(Cursor::new(TEST_5))?, 6);
        assert_eq!(find_br(Cursor::new(TEST_6))?, 18);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"X(8x2)(3x3)ABCY";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find2_br(Cursor::new(TEST_1)), 0);
        Ok(())
    }
}
