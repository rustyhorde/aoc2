// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2017 ---**
//!
//! **--- Day 12: Digital Plumber ---**
//!
//! Walking along the memory banks of the stream, you find a small village that is experiencing a little confusion: some programs can't communicate with each other.
//!
//! Programs in this village communicate using a fixed system of pipes. Messages are passed between programs using these pipes, but most programs aren't connected to each other directly. Instead, programs pass messages between each other until the message reaches the intended recipient.
//!
//! For some reason, though, some of these messages aren't ever reaching their intended recipient, and the programs suspect that some pipes are missing. They would like you to investigate.
//!
//! You walk through the village and record the ID of each program and the IDs with which it can communicate directly (your puzzle input). Each program has one or more programs with which it can communicate, and these pipes are bidirectional; if 8 says it can communicate with 11, then 11 will say it can communicate with 8.
//!
//! You need to figure out how many programs are in the group that contains program ID 0.
//!
//! For example, suppose you go door-to-door like a travelling salesman and record the following list:
//!
//! ```text
//! 0 <-> 2
//! 1 <-> 1
//! 2 <-> 0, 3, 4
//! 3 <-> 2, 4
//! 4 <-> 2, 3, 6
//! 5 <-> 6
//! 6 <-> 4, 5
//! ```
//!
//! In this example, the following programs are in the group that contains program ID 0:
//!
//! ```text
//!     Program 0 by definition.
//!     Program 2, directly connected to program 0.
//!     Program 3 via program 2.
//!     Program 4 via program 2.
//!     Program 5 via programs 6, then 4, then 2.
//!     Program 6 via programs 4, then 2.
//! ```
//!
//! Therefore, a total of 6 programs are in this group; all but program 1, which has a pipe that connects it to itself.
//!
//! How many programs are in the group that contains program ID 0?
//!
//! --- Part Two ---
//!
//! There are more programs than just the ones in the group containing program ID 0. The rest of them have no way of reaching that group, and still might have no way of reaching each other.
//!
//! A group is a collection of programs that can all communicate via pipes either directly or indirectly. The programs you identified just a moment ago are all part of the same group. Now, they would like you to determine the total number of groups.
//!
//! In the example above, there were 2 groups: one consisting of programs 0,2,3,4,5,6, and the other consisting solely of program 1.
//!
//! How many groups are there in total?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use std::{
    collections::{HashMap, HashSet},
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
    run_setup_solution::<Vec<String>, u32>(AoCYear::AOC2017, AoCDay::AOCD12, setup, find).map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<String>, u32>(bench, AoCYear::AOC2017, AoCDay::AOCD12, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> Vec<String> {
    setup_br(reader).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T) -> Result<Vec<String>>
where
    T: BufRead,
{
    let mut data = vec![];
    for line in valid_lines(reader) {
        data.push(line);
    }
    Ok(data)
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: Vec<String>) -> u32 {
    find_res(data, false).unwrap_or_default()
}

fn find_res(data: Vec<String>, second_star: bool) -> Result<u32> {
    let mut group_map = HashMap::new();

    for line in data {
        parse_and_add(&line, &mut group_map)?;
    }

    if second_star {
        let mut groups: Vec<HashSet<u32>> = Vec::new();
        let group_map_clone = group_map.clone();

        for k in group_map.keys() {
            add_to_groups(*k, &group_map_clone, &mut groups)?;
        }

        Ok(u32::try_from(groups.len())?)
    } else {
        let mut group_zero = HashSet::new();
        get_and_add(0, &group_map, &mut group_zero)?;
        Ok(u32::try_from(group_zero.len())?)
    }
}

/// Parse the line and add to group map.
fn parse_and_add(line: &str, group_map: &mut HashMap<u32, Vec<u32>>) -> Result<()> {
    let piped: Vec<&str> = line.split(" <-> ").collect();
    let group_str = piped.first().ok_or(anyhow!("Invalid group"))?;
    let group = group_str.parse::<u32>()?;
    let piped_to_strs: Vec<&str> = piped
        .get(1)
        .ok_or(anyhow!("Invalid pipes"))?
        .split(", ")
        .collect();
    let mut piped_to_vec = Vec::new();

    for piped_to_str in piped_to_strs {
        piped_to_vec.push(piped_to_str.parse::<u32>()?);
    }

    let _res = group_map.insert(group, piped_to_vec);

    Ok(())
}

/// Get and add
fn get_and_add(
    group: u32,
    group_map: &HashMap<u32, Vec<u32>>,
    group_set: &mut HashSet<u32>,
) -> Result<()> {
    let piped_tos = group_map.get(&group).ok_or(anyhow!("Group not found"))?;
    let _res = group_set.insert(group);

    for piped_to in piped_tos {
        if !group_set.contains(piped_to) {
            get_and_add(*piped_to, group_map, group_set)?;
        }
    }

    Ok(())
}

/// Add to groups
fn add_to_groups(
    group: u32,
    group_map: &HashMap<u32, Vec<u32>>,
    group_sets: &mut Vec<HashSet<u32>>,
) -> Result<()> {
    let mut found = false;

    for group_set in group_sets.iter() {
        if group_set.contains(&group) {
            found = true;
            break;
        }
    }

    if !found {
        let mut new_set = HashSet::new();
        get_and_add(group, group_map, &mut new_set)?;
        group_sets.push(new_set);
    }
    Ok(())
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<Vec<String>, u32>(AoCYear::AOC2017, AoCDay::AOCD12, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<String>, u32>(bench, AoCYear::AOC2017, AoCDay::AOCD12, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: Vec<String>) -> u32 {
    find_res(data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find(data), 6);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find2(data), 2);
        Ok(())
    }
}
