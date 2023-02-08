// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 5 "Doesn't He Have Intern-Elves For This?"
//!
//! **--- Day 5: Doesn't He Have Intern-Elves For This? ---**
//!
//! **--- Part 1 ---**
//!
//! Santa needs help figuring out which strings in his text file are naughty or nice.
//!
//! A nice string is one with all of the following properties:
//!
//! * It contains at least three vowels (`aeiou` only), like `aei`, `xazegov`, or `aeiouaeiouaeiou`.
//! * It contains at least one letter that appears twice in a row, like `xx`, `abcdde (dd)`, or `aabbccdd (aa, bb, cc, or dd)`.
//! * It does not contain the strings `ab`, `cd`, `pq`, or `xy`, even if they are part of one of the other requirements.
//!
//! For example:
//!
//! * `ugknbfddgicrmopn` is nice because it has at least three vowels (u...i...o...), a double letter (...dd...), and none of the disallowed substrings.
//! * `aaa` is nice because it has at least three vowels and a double letter, even though the letters used by different rules overlap.
//! * `jchzalrnumimnmhp` is naughty because it has no double letter.
//! * `haegwjzuvuyypxyu` is naughty because it contains the string `xy`.
//! * `dvszwmarrgswjxmb` is naughty because it contains only one vowel.
//!
//! How many strings are nice?
//!
//! **--- Part Two ---**
//!
//! Realizing the error of his ways, Santa has switched to a better model of determining whether
//! a string is naughty or nice. None of the old rules apply, as they are all clearly ridiculous.
//!
//! Now, a nice string is one with all of the following properties:
//!
//! * It contains a pair of any two letters that appears at least twice in the string without
//! overlapping, like `xyxy (xy)` or `aabcdefgaa (aa)`, but not like `aaa` (`aa`, but it overlaps).
//! * It contains at least one letter which repeats with exactly one letter between them, like `xyx`,
//! `abcdefeghi (efe)`, or even `aaa`.
//!
//! For example:
//!
//! * `qjhvhtzxzqqjkmpb` is nice because is has a pair that appears twice (`qj`) and a letter that repeats with exactly one letter between them (`zxz`).
//! * `xxyxx` is nice because it has a pair that appears twice and a letter that repeats with one between, even though the letters used by each rule overlap.
//! * `uurcxstgmygtbstg` is naughty because it has a pair (`tg`) but no repeat with a single letter between them.
//! * `ieodomkazucvgmuy` is naughty because it has a repeating letter with one between (`odo`), but no pair that appears twice.
//!
//! How many strings are nice under these new rules?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_solution, valid_lines},
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
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD05, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader)
}

fn find_br<T>(reader: T) -> usize
where
    T: BufRead,
{
    let mut nice = 0;

    for line in valid_lines(reader) {
        // Check that the vowel count is greater than 3
        let vowel_count = line
            .chars()
            .filter(|ch| *ch == 'a' || *ch == 'e' || *ch == 'i' || *ch == 'o' || *ch == 'u')
            .count();
        if vowel_count < 3 {
            continue;
        }

        // Check for a least one double letter
        let char_vec = line.chars().collect::<Vec<char>>();
        let double_count = char_vec.windows(2).filter(|s| s[0] == s[1]).count();
        if double_count == 0 {
            continue;
        }

        // Check that the restricted combos don't appear
        let char_vec = line.chars().collect::<Vec<char>>();
        let restricted_count = char_vec
            .windows(2)
            .filter(|s| {
                *s == ['a', 'b'] || *s == ['c', 'd'] || *s == ['p', 'q'] || *s == ['x', 'y']
            })
            .count();
        if restricted_count > 0 {
            continue;
        }
        nice += 1;
    }

    nice
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD05, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader)
}

fn find2_br<T>(reader: T) -> usize
where
    T: BufRead,
{
    let mut nice = 0;

    for line in valid_lines(reader) {
        // Check for a least one valid triple
        let char_vec = line.chars().collect::<Vec<char>>();
        let t_c = char_vec.windows(3).filter(|s| s[0] == s[2]).count();
        if t_c == 0 {
            continue;
        }

        // Check for non-overlapping pairs
        let mut chunk_map: HashMap<(char, char), usize> = HashMap::new();
        let char_vec = line.chars().collect::<Vec<char>>();
        let pairs = char_vec.windows(2);
        let mut has_non_overlapping_pair = false;
        for (idx, pair) in pairs.enumerate() {
            if pair.len() == 2 {
                // Store the index of the pair into the map or get the index value out if one exists.
                let first_idx = chunk_map.entry((pair[0], pair[1])).or_insert(idx);

                // If the current index is further than the first occurrence index + 2 we have a non-overlapping
                // pair.
                if idx >= *first_idx + 2 {
                    has_non_overlapping_pair = true;
                    break;
                }
            }
        }
        if !has_non_overlapping_pair {
            continue;
        }

        nice += 1;
    }

    nice
}
#[cfg(test)]
mod one_star {
    use super::find_br;
    use std::io::Cursor;

    const TEST_1: &str = r"ugknbfddgicrmopn";
    const TEST_2: &str = r"aaa";
    const TEST_3: &str = r"jchzalrnumimnmhp";
    const TEST_4: &str = r"haegwjzuvuyypxyu";
    const TEST_5: &str = r"dvszwmarrgswjxmb";

    #[test]
    fn solution() {
        assert_eq!(find_br(Cursor::new(TEST_1)), 1);
        assert_eq!(find_br(Cursor::new(TEST_2)), 1);
        assert_eq!(find_br(Cursor::new(TEST_3)), 0);
        assert_eq!(find_br(Cursor::new(TEST_4)), 0);
        assert_eq!(find_br(Cursor::new(TEST_5)), 0);
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use std::io::Cursor;

    const TEST_1: &str = r"qjhvhtzxzqqjkmpb";
    const TEST_2: &str = r"xxyxx";
    const TEST_3: &str = r"uurcxstgmygtbstg";
    const TEST_4: &str = r"ieodomkazucvgmuy";

    #[test]
    fn solution() {
        assert_eq!(find2_br(Cursor::new(TEST_1)), 1);
        assert_eq!(find2_br(Cursor::new(TEST_2)), 1);
        assert_eq!(find2_br(Cursor::new(TEST_3)), 0);
        assert_eq!(find2_br(Cursor::new(TEST_4)), 0);
    }
}
