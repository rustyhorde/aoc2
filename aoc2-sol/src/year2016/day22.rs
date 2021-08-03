// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Grid Computing
//!
//! **--- Day 22: Grid Computing ---**
//!
//! **--- Part 1 ---**
//!
//! You gain access to a massive storage cluster arranged in a grid; each storage
//! node is only connected to the four nodes directly adjacent to it (three if
//! the node is on an edge, two if it's in a corner).
//!
//! You can directly access data only on node `/dev/grid/node-x0-y0`, but you can
//! perform some limited actions on the other nodes:
//!
//! ```text
//! You can get the disk usage of all nodes (via df). The result of doing this is in your puzzle input.
//! You can instruct a node to move (not copy) all of its data to an adjacent node (if the destination node has enough space to receive the data). The sending node is left empty after this operation.
//! ```
//!
//! Nodes are named by their position: the node named `node-x10-y10` is adjacent to
//! nodes `node-x9-y10`, `node-x11-y10`, `node-x10-y9`, and `node-x10-y11`.
//!
//! Before you begin, you need to understand the arrangement of data on these nodes.
//! Even though you can only move data between directly connected nodes, you're going
//! to need to rearrange a lot of the data to get access to the data you need. Therefore,
//! you need to work out how you might be able to shift data around.
//!
//! To do this, you'd like to count the number of viable pairs of nodes. A viable pair is any
//! two nodes `(A,B)`, regardless of whether they are directly connected, such that:
//!
//! ```text
//! Node A is not empty (its Used is not zero).
//! Nodes A and B are not the same node.
//! The data on node A (its Used) would fit on node B (its Avail).
//! ```
//!
//! How many viable pairs of nodes are there?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{get_cap_x, print_err, run_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use itertools::Itertools;
use regex::Regex;
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
    run_solution::<usize>(AoCYear::AOC2016, AoCDay::AOCD22, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader).map_err(print_err).unwrap_or_default()
}

fn find_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let ds_re = Regex::new(r"^/dev/grid/node-x(\d+)-y(\d+) +(\d+)T +(\d+)T +(\d+)T")?;
    let mut blah = HashMap::new();
    for line in valid_lines(reader) {
        if ds_re.is_match(&line) {
            for caps in ds_re.captures_iter(&line) {
                let x = get_cap_x::<usize>(1, &caps)?;
                let y = get_cap_x::<usize>(2, &caps)?;
                let size = get_cap_x::<usize>(3, &caps)?;
                let used = get_cap_x::<usize>(4, &caps)?;
                let avail = get_cap_x::<usize>(5, &caps)?;
                let _ = blah.insert((x, y), (size, used, avail));
            }
        }
    }

    let mut valid = 0;
    for keys in blah.keys().permutations(2) {
        let (_first_s, first_u, _first_a) =
            blah.get(keys[0]).ok_or_else(|| anyhow!("invalid node"))?;
        let (_second_s, _second_u, second_a) =
            blah.get(keys[1]).ok_or_else(|| anyhow!("invalid node"))?;

        if *first_u != 0 && first_u <= second_a {
            valid += 1;
        }
    }
    Ok(valid)
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2016, AoCDay::AOCD22, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader)
}

fn find2_br<T>(reader: T) -> usize
where
    T: BufRead,
{
    for _line in valid_lines(reader) {}
    0
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"root@ebhq-gridcenter# df -h
Filesystem              Size  Used  Avail  Use%
/dev/grid/node-x0-y0     92T   73T    19T   79%
/dev/grid/node-x0-y1     91T   66T    25T   72%
/dev/grid/node-x0-y2     85T   73T    12T   85%
/dev/grid/node-x0-y3     85T   68T    17T   80%";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1))?, 0);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    // use super::find2_br;
    use anyhow::Result;
    // use std::io::Cursor;

    // const TEST_1: &str = r"^v";
    // const TEST_2: &str = r"^>v<";
    // const TEST_3: &str = r"^v^v^v^v^v";

    #[test]
    fn solution() -> Result<()> {
        // assert_eq!(find2_br(Cursor::new(TEST_1))?, 3);
        Ok(())
    }
}
