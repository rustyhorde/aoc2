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
//!
//! **--- Part Two ---**
//!
//! Now that you have a better understanding of the grid, it's time to get to work.
//!
//! Your goal is to gain access to the data which begins in the node with `y=0` and
//! the highest `x` (that is, the node in the top-right corner).
//!
//! For example, suppose you have the following grid:
//!
//! ```text
//! Filesystem            Size  Used  Avail  Use%
//! /dev/grid/node-x0-y0   10T    8T     2T   80%
//! /dev/grid/node-x0-y1   11T    6T     5T   54%
//! /dev/grid/node-x0-y2   32T   28T     4T   87%
//! /dev/grid/node-x1-y0    9T    7T     2T   77%
//! /dev/grid/node-x1-y1    8T    0T     8T    0%
//! /dev/grid/node-x1-y2   11T    7T     4T   63%
//! /dev/grid/node-x2-y0   10T    6T     4T   60%
//! /dev/grid/node-x2-y1    9T    8T     1T   88%
//! /dev/grid/node-x2-y2    9T    6T     3T   66%
//! ```
//!
//! In this example, you have a storage grid 3 nodes wide and 3 nodes tall. The node
//! you can access directly, `node-x0-y0`, is almost full. The node containing the
//! data you want to access, `node-x2-y0` (because it has `y=0` and the highest x value),
//! contains 6 terabytes of data - enough to fit on your node, if only you could make
//! enough space to move it there.
//!
//! Fortunately, `node-x1-y1` looks like it has enough free space to enable you to
//! move some of this data around. In fact, it seems like all of the nodes have enough
//! space to hold any node's data (except `node-x0-y2`, which is much larger, very full,
//! and not moving any time soon). So, initially, the grid's capacities and connections
//! look like this:
//!
//! ```text
//! ( 8T/10T) --  7T/ 9T -- [ 6T/10T]
//!     |           |           |
//!   6T/11T  --  0T/ 8T --   8T/ 9T
//!     |           |           |
//!  28T/32T  --  7T/11T --   6T/ 9T
//! ```
//!
//! The node you can access directly is in parentheses; the data you want starts in
//! the node marked by square brackets.
//!
//! In this example, most of the nodes are interchangable: they're full enough that no
//! other node's data would fit, but small enough that their data could be moved around.
//! Let's draw these nodes as `.`. The exceptions are the empty node, which we'll draw
//! as `_`, and the very large, very full node, which we'll draw as `#`. Let's also
//! draw the goal data as `G`. Then, it looks like this:
//!
//! ```text
//! (.) .  G
//!  .  _  .
//!  #  .  .
//! ```
//!
//! The goal is to move the data in the top right, `G`, to the node in parentheses.
//! To do this, we can issue some commands to the grid and rearrange the data:
//!
//! Move data from `node-y0-x1` to `node-y1-x1`, leaving node `node-y0-x1` empty:
//!
//! ```text
//! (.) _  G
//!  .  .  .
//!  #  .  .
//! ```
//!
//! Move the goal data from `node-y0-x2` to `node-y0-x1`:
//!
//! ```text
//! (.) G  _
//!  .  .  .
//!  #  .  .
//! ```
//!
//! At this point, we're quite close. However, we have no deletion command, so we
//! have to move some more data around. So, next, we move the data from `node-y1-x2`
//! to `node-y0-x2`:
//!
//! ```text
//! (.) G  .
//!  .  .  _
//!  #  .  .
//! ```
//!
//! Move the data from `node-y1-x1` to `node-y1-x2`:
//!
//! ```text
//! (.) G  .
//!  .  _  .
//!  #  .  .
//! ```
//!
//! Move the data from `node-y1-x0` to `node-y1-x1`:
//!
//! ```text
//! (.) G  .
//!  _  .  .
//!  #  .  .
//! ```
//!
//! Next, we can free up space on our node by moving the data from `node-y0-x0`
//! to `node-y1-x0`:
//!
//! ```text
//! (_) G  .
//!  .  .  .
//!  #  .  .
//! ```
//!
//! Finally, we can access the goal data by moving the it from `node-y0-x1`
//! to `node-y0-x0`:
//!
//! ```text
//! (G) _  .
//!  .  .  .
//!  #  .  .
//! ```
//!
//! So, after 7 steps, we've accessed the data we want. Unfortunately, each
//! of these moves takes time, and we need to be efficient:
//!
//! What is the fewest number of steps required to move your goal data to `node-x0-y0`?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{get_cap_x, print_err, run_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use itertools::Itertools;
use ndarray::Array2;
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
    let nodes = load(reader)?;
    let mut valid = 0;
    for keys in nodes.keys().permutations(2) {
        let (_first_s, first_u, _first_a) =
            nodes.get(keys[0]).ok_or_else(|| anyhow!("invalid node"))?;
        let (_second_s, _second_u, second_a) =
            nodes.get(keys[1]).ok_or_else(|| anyhow!("invalid node"))?;

        if *first_u != 0 && first_u <= second_a {
            valid += 1;
        }
    }
    Ok(valid)
}

type NodeMap = HashMap<(usize, usize), (usize, usize, usize)>;

fn load<T>(reader: T) -> Result<NodeMap>
where
    T: BufRead,
{
    let ds_re = Regex::new(r"^/dev/grid/node-x(\d+)-y(\d+) +(\d+)T +(\d+)T +(\d+)T")?;
    let mut nodes = HashMap::new();
    for line in valid_lines(reader) {
        if ds_re.is_match(&line) {
            for caps in ds_re.captures_iter(&line) {
                let x = get_cap_x::<usize>(1, &caps)?;
                let y = get_cap_x::<usize>(2, &caps)?;
                let size = get_cap_x::<usize>(3, &caps)?;
                let used = get_cap_x::<usize>(4, &caps)?;
                let avail = get_cap_x::<usize>(5, &caps)?;
                _ = nodes.insert((x, y), (size, used, avail));
            }
        }
    }
    Ok(nodes)
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
    find2_br(reader).map_err(print_err).unwrap_or_default()
}

fn find2_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let nodes = load(reader)?;
    let max_x = nodes
        .keys()
        .max_by(|a, b| (a.0).cmp(&b.0))
        .ok_or_else(|| anyhow!("bad x nodes"))?;
    let max_y = nodes
        .keys()
        .max_by(|a, b| (a.1).cmp(&b.1))
        .ok_or_else(|| anyhow!("bad y nodes"))?;
    println!("Max X: {}, Max Y: {}", max_x.0, max_y.1);

    let mut arr = Array2::<NodeKind>::default((max_y.1 + 1, max_x.0 + 1));
    for ((cols, rows), (size, used, _avail)) in nodes {
        arr[[rows, cols]] = if size > 100 {
            NodeKind::Wall
        } else if used == 0 {
            NodeKind::Empty
        } else {
            NodeKind::Normal
        };
    }
    // println!("Initial State");
    // for rows in arr.axis_iter(Axis(0)) {
    //     for col in rows.iter() {
    //         print!("{}", match *col {
    //             NodeKind::Normal => ". ",
    //             NodeKind::Wall => "| ",
    //             NodeKind::Empty => "_ ",
    //         });
    //     }
    //     println!();
    // }
    // println!();
    Ok(0)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum NodeKind {
    Normal,
    Wall,
    Empty,
}

impl Default for NodeKind {
    fn default() -> Self {
        Self::Normal
    }
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
    use super::find2_br;
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
        assert_eq!(find2_br(Cursor::new(TEST_1))?, 0);
        Ok(())
    }
}
