// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 11 "Corporate Policy"
//!
//! **--- Day 11: Corporate Policy ---**
//!
//! **--- Part 1 ---**
//!
//! Santa's previous password expired, and he needs help choosing a new one.
//!
//! To help him remember his new password after the old one expires,
//! Santa has devised a method of coming up with a password based on the previous one.
//! Corporate policy dictates that passwords must be exactly eight lowercase
//! letters (for security reasons), so he finds his new password by
//! incrementing his old password string repeatedly until it is valid.
//!
//! Incrementing is just like counting with numbers: `xx`, `xy`, `xz`, `ya`,
//! `yb`, and so on. Increase the rightmost letter one step; if it was `z`,
//! it wraps around to `a`, and repeat with the next letter to the left
//! until one doesn't wrap around.
//!
//! Unfortunately for Santa, a new Security-Elf recently started, and
//! he has imposed some additional password requirements:
//!
//! 1. Passwords must include one increasing straight of at least three letters,
//! like `abc`, `bcd`, `cde`, and so on, up to `xyz`. They cannot skip letters;
//! `abd` doesn't count.
//! 2. Passwords may not contain the letters `i`, `o`, or `l`, as these letters
//! can be mistaken for other characters and are therefore confusing.
//! 3. Passwords must contain at least two different, non-overlapping pairs
//! of letters, like `aa`, `bb`, or `zz`.
//!
//! For example:
//!
//! ```text
//! hijklmmn meets the first requirement (because it contains the straight hij) but fails the second requirement requirement (because it contains i and l).
//! abbceffg meets the third requirement (because it repeats bb and ff) but fails the first requirement.
//! abbcegjk fails the third requirement, because it only has one double letter (bb).
//! The next password after abcdefgh is abcdffaa.
//! The next password after ghijklmn is ghjaabcc, because you eventually skip all the passwords that start with ghi..., since i is not allowed.
//! ```
//!
//! Given Santa's current password (your puzzle input), what should his next password be?
//!
//! **--- Part Two ---**
//!
//! Santa's password expired again. What's the next one?
//!

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_solution, valid_lines},
};
use anyhow::{anyhow, Result};
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
    run_solution::<String>(AoCYear::AOC2015, AoCDay::AOCD11, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> String {
    find_br(reader).unwrap_or_default()
}

fn find_br<T>(reader: T) -> Result<String>
where
    T: BufRead,
{
    let mut result = String::new();

    for line in valid_lines(reader) {
        result = get_next(&line)?;
    }

    Ok(result)
}

fn get_next(initial: &str) -> Result<String> {
    let mut pass_vec = to_vec(initial)?;
    let mut valid = false;

    while !valid {
        increment(&mut pass_vec);
        valid = is_valid(&pass_vec);
    }

    to_str(&mut pass_vec)
}

fn is_valid(pass: &[u8]) -> bool {
    !contains_iol(pass) && has_run_of_3(pass) && enough_dups(pass)
}

fn contains_iol(pass: &[u8]) -> bool {
    // Cannot contain 'i', 'l', or 'o'
    pass.contains(&8) || pass.contains(&11) || pass.contains(&14)
}

fn has_run_of_3(pass: &[u8]) -> bool {
    // Needs to contain a run of 3
    let mut found_run = false;
    for sec in pass.windows(3) {
        let first = sec[0];
        let second = sec[1] + 1;
        let third = sec[2] + 2;
        if first == second && second == third {
            found_run = true;
            break;
        }
    }
    found_run
}

fn enough_dups(pass: &[u8]) -> bool {
    // Has to have at least 2 non-overlapping double chars
    let mut found_dups = false;
    let mut found_one = false;
    let mut skip_next = false;
    for sec in pass.windows(2) {
        if skip_next {
            skip_next = false;
            continue;
        }
        if sec[0] == sec[1] {
            if found_one {
                found_dups = true;
                break;
            }
            found_one = true;
            // Skip the next window to avoid overlaps
            skip_next = true;
        }
    }
    found_dups
}

fn increment(pass: &mut Vec<u8>) {
    let mut push_a = false;
    let len = pass.len();

    for (idx, val) in pass.iter_mut().enumerate() {
        *val = (*val + 1) % 26;
        if *val == 0 {
            if idx == len - 1 {
                push_a = true;
            }
            continue;
        }
        break;
    }

    if push_a {
        pass.push(0);
    }
}

fn to_str(vec: &mut [u8]) -> Result<String> {
    let mut pass = String::new();
    vec.reverse();

    for v in vec {
        match v {
            0 => pass.push('a'),
            1 => pass.push('b'),
            2 => pass.push('c'),
            3 => pass.push('d'),
            4 => pass.push('e'),
            5 => pass.push('f'),
            6 => pass.push('g'),
            7 => pass.push('h'),
            8 => pass.push('i'),
            9 => pass.push('j'),
            10 => pass.push('k'),
            11 => pass.push('l'),
            12 => pass.push('m'),
            13 => pass.push('n'),
            14 => pass.push('o'),
            15 => pass.push('p'),
            16 => pass.push('q'),
            17 => pass.push('r'),
            18 => pass.push('s'),
            19 => pass.push('t'),
            20 => pass.push('u'),
            21 => pass.push('v'),
            22 => pass.push('w'),
            23 => pass.push('x'),
            24 => pass.push('y'),
            25 => pass.push('z'),
            _ => return Err(anyhow!("invalid password value")),
        }
    }
    Ok(pass)
}

fn to_vec(line: &str) -> Result<Vec<u8>> {
    let mut pass = vec![];

    for ch in line.chars() {
        match ch {
            'a' => pass.push(0),
            'b' => pass.push(1),
            'c' => pass.push(2),
            'd' => pass.push(3),
            'e' => pass.push(4),
            'f' => pass.push(5),
            'g' => pass.push(6),
            'h' => pass.push(7),
            'i' => pass.push(8),
            'j' => pass.push(9),
            'k' => pass.push(10),
            'l' => pass.push(11),
            'm' => pass.push(12),
            'n' => pass.push(13),
            'o' => pass.push(14),
            'p' => pass.push(15),
            'q' => pass.push(16),
            'r' => pass.push(17),
            's' => pass.push(18),
            't' => pass.push(19),
            'u' => pass.push(20),
            'v' => pass.push(21),
            'w' => pass.push(22),
            'x' => pass.push(23),
            'y' => pass.push(24),
            'z' => pass.push(25),
            _ => return Err(anyhow!("invalid password character")),
        }
    }
    pass.reverse();
    Ok(pass)
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<String>(AoCYear::AOC2015, AoCDay::AOCD11, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> String {
    find2_br(reader).unwrap_or_default()
}

fn find2_br<T>(reader: T) -> Result<String>
where
    T: BufRead,
{
    let mut result = String::new();

    for line in valid_lines(reader) {
        let first_pass = get_next(&line)?;
        result = get_next(&first_pass)?;
    }

    Ok(result)
}
#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"abcdefgh";
    const TEST_2: &str = r"ghijklmn";
    const TEST_3: &str = r"hepxcrrq";

    #[test]
    #[ignore]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1))?, "abcdffaa");
        assert_eq!(find_br(Cursor::new(TEST_2))?, "ghjaabcc");
        assert_eq!(find_br(Cursor::new(TEST_3))?, "hepxxyzz");
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"abcdefgh";
    const TEST_2: &str = r"ghijklmn";
    const TEST_3: &str = r"hepxcrrq";

    #[test]
    #[ignore]
    fn solution() -> Result<()> {
        assert_eq!(find2_br(Cursor::new(TEST_1))?, "abcdffbb");
        assert_eq!(find2_br(Cursor::new(TEST_2))?, "ghjbbcdd");
        assert_eq!(find2_br(Cursor::new(TEST_3))?, "heqaabcc");
        Ok(())
    }
}
