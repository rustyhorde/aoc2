// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2018 ---**
//!
//! **--- Day 8: Memory Maneuver ---**
//!
//! The sleigh is much easier to pull than you'd expect for something its weight. Unfortunately, neither you nor the Elves know which way the North Pole is from here.
//!
//! You check your wrist device for anything that might help. It seems to have some kind of navigation system! Activating the navigation system produces more bad news: "Failed to start navigation system. Could not read software license file."
//!
//! The navigation system's license file consists of a list of numbers (your puzzle input). The numbers define a data structure which, when processed, produces some kind of tree that can be used to calculate the license number.
//!
//! The tree is made up of nodes; a single, outermost node forms the tree's root, and it contains all other nodes in the tree (or contains nodes that contain nodes, and so on).
//!
//! Specifically, a node consists of:
//!
//! ```text
//!     A header, which is always exactly two numbers:
//!         The quantity of child nodes.
//!         The quantity of metadata entries.
//!     Zero or more child nodes (as specified in the header).
//!     One or more metadata entries (as specified in the header).
//! ```
//!
//! Each child node is itself a node that has its own header, child nodes, and metadata. For example:
//!
//! ```text
//! 2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2
//! A----------------------------------
//!     B----------- C-----------
//!                      D-----
//! ```
//!
//! In this example, each node of the tree is also marked with an underline starting with a letter for easier identification. In it, there are four nodes:
//!
//! ```text
//!     A, which has 2 child nodes (B, C) and 3 metadata entries (1, 1, 2).
//!     B, which has 0 child nodes and 3 metadata entries (10, 11, 12).
//!     C, which has 1 child node (D) and 1 metadata entry (2).
//!     D, which has 0 child nodes and 1 metadata entry (99).
//! ```
//!
//! The first check done on the license file is to simply add up all of the metadata entries. In this example, that sum is 1+1+2+10+11+12+2+99=138.
//!
//! What is the sum of all metadata entries?
//!
//! **--- Part Two ---**
//!
//! The second check is slightly more complicated: you need to find the value of the root node (A in the example above).
//!
//! The value of a node depends on whether it has child nodes.
//!
//! If a node has no child nodes, its value is the sum of its metadata entries. So, the value of node B is 10+11+12=33, and the value of node D is 99.
//!
//! However, if a node does have child nodes, the metadata entries become indexes which refer to those child nodes. A metadata entry of 1 refers to the first child node, 2 to the second, 3 to the third, and so on. The value of this node is the sum of the values of the child nodes referenced by the metadata entries. If a referenced child node does not exist, that reference is skipped. A child node can be referenced multiple time and counts each time it is referenced. A metadata entry of 0 does not refer to any child node.
//!
//! For example, again using the above nodes:
//!
//! ```text
//!     Node C has one metadata entry, 2. Because node C has only one child node, 2 references a child node which does not exist, and so the value of node C is 0.
//!     Node A has three metadata entries: 1, 1, and 2. The 1 references node A's first child node, B, and the 2 references node A's second child node, C. Because node B has a value of 33 and node C has a value of 0, the value of node A is 33+33+0=66.
//! ```
//!
//! So, in this example, the value of the root node is 66.
//!
//! What is the value of the root node?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
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
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<Vec<u32>, u32>(AoCYear::AOC2018, AoCDay::AOCD08, setup, find).map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<u32>, u32>(bench, AoCYear::AOC2018, AoCDay::AOCD08, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> Vec<u32> {
    setup_br(reader).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T) -> Result<Vec<u32>>
where
    T: BufRead,
{
    let mut data = vec![];
    for line in valid_lines(reader) {
        for tok in line.split(' ').map(str::parse::<u32>).map_while(Result::ok) {
            data.push(tok);
        }
    }
    data.reverse();
    Ok(data)
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: Vec<u32>) -> u32 {
    find_res(data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(data: Vec<u32>, second_star: bool) -> Result<u32> {
    let mut data = data;
    recurse(&mut data, second_star)
}

fn recurse(license_vec: &mut Vec<u32>, second_star: bool) -> Result<u32> {
    let children_count = license_vec.pop().ok_or(anyhow!(""))?;
    let metadata_count = license_vec.pop().ok_or(anyhow!(""))?;
    let mut result = 0;

    if second_star {
        let mut children_values = Vec::new();

        for _ in 0..children_count {
            children_values.push(recurse(license_vec, second_star)?);
        }

        for _ in 0..metadata_count {
            let metadata = license_vec.pop().ok_or(anyhow!(""))?;
            if children_count == 0 {
                result += metadata;
            } else {
                result += children_values.get(metadata as usize - 1).unwrap_or(&0);
            }
        }
    } else {
        for _ in 0..children_count {
            result += recurse(license_vec, second_star)?;
        }

        for _ in 0..metadata_count {
            result += license_vec.pop().ok_or(anyhow!(""))?;
        }
    }

    Ok(result)
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<Vec<u32>, u32>(AoCYear::AOC2018, AoCDay::AOCD08, setup, find2).map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<u32>, u32>(bench, AoCYear::AOC2018, AoCDay::AOCD08, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: Vec<u32>) -> u32 {
    find_res(data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find(data), 138);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find2(data), 66);
        Ok(())
    }
}
