// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 13 "A Maze of Twisty Little Cubicles"
//!
//! **--- Day 13: A Maze of Twisty Little Cubicles ---**
//!
//! **--- Part 1 ---**
//!
//! You arrive at the first floor of this new building to discover a much less
//! welcoming environment than the shiny atrium of the last one. Instead, you
//! are in a maze of twisty little cubicles, all alike.
//!
//! Every location in this area is addressed by a pair of non-negative integers
//! `(x,y)`. Each such coordinate is either a wall or an open space. You can't
//! move diagonally. The cube maze starts at `0,0` and seems to extend infinitely
//! toward positive `x` and `y`; negative values are invalid, as they represent
//! a location outside the building. You are in a small waiting area at `1,1`.
//!
//! While it seems chaotic, a nearby morale-boosting poster explains, the layout
//! is actually quite logical. You can determine whether a given `x,y` coordinate
//! will be a wall or an open space using a simple system:
//!
//! ```text
//! Find x*x + 3*x + 2*x*y + y + y*y.
//! Add the office designer's favorite number (your puzzle input).
//! Find the binary representation of that sum; count the number of bits that are 1.
//!     If the number of bits that are 1 is even, it's an open space.
//!     If the number of bits that are 1 is odd, it's a wall.
//! ```
//!
//! For example, if the office designer's favorite number were 10, drawing walls as
//! # and open spaces as ., the corner of the building containing `0,0` would look like this:
//!
//! ```text
//!   0123456789
//! 0 .#.####.##
//! 1 ..#..#...#
//! 2 #....##...
//! 3 ###.#.###.
//! 4 .##..#..#.
//! 5 ..##....#.
//! 6 #...##.###
//! ```
//!
//! Now, suppose you wanted to reach `7,4`. The shortest route you could take is marked as O:
//!
//! ```text
//!   0123456789
//! 0 .#.####.##
//! 1 .O#..#...#
//! 2 #OOO.##...
//! 3 ###O#.###.
//! 4 .##OO#OO#.
//! 5 ..##OOO.#.
//! 6 #...##.###
//!
//! Thus, reaching `7,4` would take a minimum of `11` steps (starting from your current
//! location, `1,1`).
//!
//! What is the fewest number of steps required for you to reach `31,39`?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{print_err, run_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use bitvec::{order::Msb0, view::BitView};
use ndarray::{Array2, Axis};
use petgraph::{algo::dijkstra, graph::NodeIndex, Graph, Undirected};
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
    run_solution::<usize>(AoCYear::AOC2016, AoCDay::AOCD13, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader, 100, 100, 39, 31)
        .map_err(print_err)
        .unwrap_or_default()
}

fn find_br<T>(
    reader: T,
    max_row: usize,
    max_col: usize,
    end_row: usize,
    end_col: usize,
) -> Result<usize>
where
    T: BufRead,
{
    let graph = setup(reader, max_row, max_col)?;
    let end_ni = find_node(&graph, end_row, end_col)?;
    let res = pathing(&graph, Some(end_ni))?;

    if let Some(end) = res.get(&end_ni) {
        Ok(usize::try_from(*end)?)
    } else {
        Err(anyhow!("Unable to find solution"))
    }
}

fn pathing(
    graph: &Graph<(usize, usize), usize, Undirected>,
    end_ni: Option<NodeIndex>,
) -> Result<HashMap<NodeIndex, i32>> {
    let start_ni = find_node(graph, 1, 1)?;
    Ok(dijkstra(&graph, start_ni, end_ni, |_| 1))
}

fn setup<T>(
    reader: T,
    max_row: usize,
    max_col: usize,
) -> Result<Graph<(usize, usize), usize, Undirected>>
where
    T: BufRead,
{
    let mut fav_number = 0;
    for line in valid_lines(reader) {
        fav_number = line.parse::<usize>()?;
    }
    let mut arr = Array2::<usize>::zeros((max_col, max_row));
    for x in 0..max_row {
        for y in 0..max_col {
            let val = (x * x) + (3 * x) + (2 * x * y) + y + (y * y) + fav_number;
            let bv = val.view_bits::<Msb0>();
            let ones = bv.count_ones();

            if ones % 2 == 0 {
                arr[[y, x]] = 0;
            } else {
                arr[[y, x]] = 1;
            }
        }
    }

    let mut graph = Graph::new_undirected();

    for (row, row_arr) in arr.axis_iter(Axis(0)).enumerate() {
        for (col, _val) in row_arr.iter().enumerate() {
            _ = graph.add_node((row, col));
        }
    }
    add_edges(&arr, &mut graph, max_row, max_col)?;
    Ok(graph)
}

fn cross(row: usize, col: usize, max_row: usize, max_col: usize) -> Vec<(usize, usize)> {
    let mut udrl = vec![];
    if let Some(previous_row) = row.checked_sub(1) {
        udrl.push((previous_row, col));
    }

    if let Some(previous_col) = col.checked_sub(1) {
        udrl.push((row, previous_col));
    }

    if row + 1 < max_row {
        udrl.push((row + 1, col));
    }

    if col + 1 < max_col {
        udrl.push((row, col + 1));
    }
    udrl
}

fn add_edges(
    arr: &Array2<usize>,
    graph: &mut Graph<(usize, usize), usize, Undirected>,
    max_row: usize,
    max_col: usize,
) -> Result<()> {
    for (row, row_arr) in arr.axis_iter(Axis(0)).enumerate() {
        for (col, val) in row_arr.iter().enumerate() {
            if *val == 0 {
                let ni = find_node(graph, row, col)?;
                let cross = cross(row, col, max_row, max_col);

                for (c_row, c_col) in cross {
                    let child_ni = find_node(graph, c_row, c_col)?;
                    let row_col: [usize; 2] = (c_row, c_col).into();
                    let child_val = arr[row_col];
                    if child_val == 0 && !(c_row == row && c_col == col) {
                        _ = graph.add_edge(ni, child_ni, 1);
                    }
                }
            }
        }
    }
    Ok(())
}

fn find_node(
    graph: &Graph<(usize, usize), usize, Undirected>,
    row: usize,
    col: usize,
) -> Result<NodeIndex> {
    graph
        .node_indices()
        .find(|ni| graph[*ni] == (row, col))
        .ok_or_else(|| anyhow!("no node"))
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2016, AoCDay::AOCD13, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader, 100, 100)
        .map_err(print_err)
        .unwrap_or_default()
}

fn find2_br<T>(reader: T, max_row: usize, max_col: usize) -> Result<usize>
where
    T: BufRead,
{
    let graph = setup(reader, max_row, max_col)?;
    Ok(pathing(&graph, None)?
        .into_iter()
        .filter(|(_ni, steps)| *steps <= 50)
        .count())
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"10";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1), 10, 10, 4, 7)?, 11);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    // use super::find2_br;
    // use std::io::Cursor;

    // const TEST_1: &str = r"^v";
    // const TEST_2: &str = r"^>v<";
    // const TEST_3: &str = r"^v^v^v^v^v";

    #[test]
    fn solution() {
        // assert_eq!(find2_br(Cursor::new(TEST_1))?, 3);
    }
}
