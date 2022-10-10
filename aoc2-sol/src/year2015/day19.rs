// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 19 "Medicine for Rudolph"
//!
//! **--- Day 19: Medicine for Rudolph ---**
//!
//! **--- Part 1 ---**
//!
//! Rudolph the Red-Nosed Reindeer is sick! His nose isn't shining very brightly, and he needs medicine.
//!
//! Red-Nosed Reindeer biology isn't similar to regular reindeer biology; Rudolph
//! is going to need custom-made medicine. Unfortunately, Red-Nosed Reindeer chemistry
//! isn't similar to regular reindeer chemistry, either.
//!
//! The North Pole is equipped with a Red-Nosed Reindeer nuclear fusion/fission plant,
//! capable of constructing any Red-Nosed Reindeer molecule you need. It works by starting
//! with some input molecule and then doing a series of replacements, one per step,
//! until it has the right molecule.
//!
//! However, the machine has to be calibrated before it can be used. Calibration involves
//! determining the number of molecules that can be generated in one step from a given starting point.
//!
//! For example, imagine a simpler machine that supports only the following replacements:
//!
//! ```text
//! H => HO
//! H => OH
//! O => HH
//! ```
//!
//! Given the replacements above and starting with `HOH`, the following molecules could be generated:
//!
//! ```text
//! HOOH (via H => HO on the first H).
//! HOHO (via H => HO on the second H).
//! OHOH (via H => OH on the first H).
//! HOOH (via H => OH on the second H).
//! HHHH (via O => HH).
//! ```
//!
//! So, in the example above, there are 4 distinct molecules (not five, because `HOOH` appears twice)
//! after one replacement from `HOH`. Santa's favorite molecule, `HOHOHO`, can become 7 distinct molecules
//! (over nine replacements: six from H, and three from O).
//!
//! The machine replaces without regard for the surrounding characters. For example, given the string H2O,
//! the transition H => OO would result in OO2O.
//!
//! Your puzzle input describes all of the possible replacements and, at the bottom, the medicine molecule
//! for which you need to calibrate the machine. How many distinct molecules can be created after all the
//! different ways you can do one replacement on the medicine molecule?
//!
//! **--- Part Two ---**
//!
//! Now that the machine is calibrated, you're ready to begin molecule fabrication.
//!
//! Molecule fabrication always begins with just a single electron, `e`, and applying replacements
//! one at a time, just like the ones during calibration.
//!
//! For example, suppose you have the following replacements:
//!
//! ```text
//! e => H
//! e => O
//! H => HO
//! H => OH
//! O => HH
//! ```
//!
//! If you'd like to make `HOH`, you start with `e`, and then make the following replacements:
//!
//! ```text
//! e => O to get O
//! O => HH to get HH
//! H => OH (on the second H) to get HOH
//! ```
//!
//! So, you could make `HOH` after 3 steps. Santa's favorite molecule, `HOHOHO`, can be made in 6 steps.
//!
//! How long will it take to make the medicine? Given the available replacements and
//! the medicine molecule in your puzzle input, what is the fewest number of steps to go from `e` to
//! the medicine molecule?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{get_cap, run_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
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
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD19, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader, None)
        .map_err(|e| {
            eprintln!("{e}");
            e
        })
        .unwrap_or_default()
}

fn find_br<T>(reader: T, start_opt: Option<&str>) -> Result<usize>
where
    T: BufRead,
{
    let rep_re = Regex::new(r"^(.*) => (.*)$")?;
    let ini_re = Regex::new(r"^([a-zA-Z]+)$")?;
    let mut rep_map = HashMap::new();
    let mut start_set = false;
    let mut start = String::new();

    for line in valid_lines(reader) {
        if rep_re.is_match(&line) {
            for caps in rep_re.captures_iter(&line) {
                let val = get_cap(1, &caps)?;
                let rep = get_cap(2, &caps)?;

                let rep_vec = rep_map.entry(val).or_insert_with(Vec::new);
                rep_vec.push(rep);
            }
        } else if ini_re.is_match(&line) {
            for caps in ini_re.captures_iter(&line) {
                start = get_cap(1, &caps)?;
                start_set = true;
            }
        }
    }

    if !start_set {
        if let Some(start_str) = start_opt {
            start = start_str.to_string();
            start_set = true;
        }
    }

    if !start_set {
        return Err(anyhow!("I can't start"));
    }

    let mut molecules = HashSet::new();
    for (idx, ch) in start.chars().enumerate() {
        if let Some(rep_vec) = rep_map.get(&ch.to_string()) {
            for rep in rep_vec {
                let mut new_string = String::new();
                let str_to_splice = start.clone();
                let parts = str_to_splice.split_at(idx + 1);
                let sub_parts = parts.0.split_at(parts.0.len() - 1);

                new_string.push_str(sub_parts.0);
                new_string.push_str(rep);
                new_string.push_str(parts.1);

                let _ = molecules.insert(new_string.clone());
            }
        }
    }

    let bytes = start.as_bytes();
    for (idx, pair) in bytes.windows(2).enumerate() {
        let p_str = String::from_utf8_lossy(pair).to_string();
        if let Some(rep_vec) = rep_map.get(&p_str) {
            for rep in rep_vec {
                let mut new_string = String::new();
                let str_to_splice = start.clone();
                let parts = str_to_splice.split_at(idx + 2);
                let sub_parts = parts.0.split_at(parts.0.len() - 2);

                new_string.push_str(sub_parts.0);
                new_string.push_str(rep);
                new_string.push_str(parts.1);

                let _ = molecules.insert(new_string.clone());
            }
        }
    }

    Ok(molecules.len())
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD19, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader, None)
        .map_err(|e| {
            eprintln!("{e}");
            e
        })
        .unwrap_or_default()
}

fn find2_br<T>(reader: T, start_opt: Option<&str>) -> Result<usize>
where
    T: BufRead,
{
    let rep_re = Regex::new(r"^(.*) => (.*)$")?;
    let ini_re = Regex::new(r"^([a-zA-Z]+)$")?;
    let mut rep_map = HashMap::new();
    let mut start_set = false;
    let mut start = String::new();

    for line in valid_lines(reader) {
        if rep_re.is_match(&line) {
            for caps in rep_re.captures_iter(&line) {
                let val = get_cap(1, &caps)?;
                let rep = get_cap(2, &caps)?;

                let rep_vec = rep_map.entry(val).or_insert_with(Vec::new);
                rep_vec.push(rep);
            }
        } else if ini_re.is_match(&line) {
            for caps in ini_re.captures_iter(&line) {
                start = get_cap(1, &caps)?;
                start_set = true;
            }
        }
    }

    if !start_set {
        if let Some(start_str) = start_opt {
            start = start_str.to_string();
            start_set = true;
        }
    }

    if !start_set {
        return Err(anyhow!("I can't start"));
    }

    let toks = rep_map
        .keys()
        .map(|k| k.chars().rev().collect())
        .collect::<Vec<String>>();
    start = start.replace("Rn", "(");
    start = start.replace("Ar", ")");
    start = start.replace('Y', ",");
    start = start.chars().rev().collect();

    let mut curr = start;
    let mut t = 0;
    let mut p = 0;
    let mut c = 0;
    loop {
        if curr.is_empty() {
            break;
        }

        let (first, rest) = curr.split_at(1);
        match first {
            "(" | ")" => {
                t += 1;
                p += 1;
                curr = rest.to_string();
                continue;
            }
            "," => {
                t += 1;
                c += 1;
                curr = rest.to_string();
                continue;
            }
            _ => {}
        }
        if toks.contains(&first.to_string()) {
            t += 1;
            curr = rest.to_string();
            continue;
        } else if curr.len() > 1 {
            let (first_two, rest_two) = curr.split_at(2);
            if toks.contains(&first_two.to_string()) {
                t += 1;
                curr = rest_two.to_string();
                continue;
            }
            break;
        }

        t += 1;
        curr = rest.to_string();
    }

    Ok(t - p - (2 * c) - 1)
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"H => HO
H => OH
O => HH";
    const TEST_1_IN: &str = r"HOH";
    const TEST_2_IN: &str = r"HOHOHO";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1), Some(TEST_1_IN))?, 4);
        assert_eq!(find_br(Cursor::new(TEST_1), Some(TEST_2_IN))?, 7);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"H => HO
H => OH
O => HH";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find2_br(Cursor::new(TEST_1), Some(TEST_1))?, 1);
        Ok(())
    }
}
