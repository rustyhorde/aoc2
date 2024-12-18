// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2017 ---**
//!
//! **--- Day 19: A Series of Tubes ---**
//!
//! Somehow, a network packet got lost and ended up here. It's trying to follow a routing diagram (your puzzle input), but it's confused about where to go.
//!
//! Its starting point is just off the top of the diagram. Lines (drawn with |, -, and +) show the path it needs to take, starting by going down onto the only line connected to the top of the diagram. It needs to follow this path until it reaches the end (located somewhere within the diagram) and stop there.
//!
//! Sometimes, the lines cross over each other; in these cases, it needs to continue going the same direction, and only turn left or right when there's no other option. In addition, someone has left letters on the line; these also don't change its direction, but it can use them to keep track of where it's been. For example:
//!
//! ```text
//!      |          
//!      |  +--+    
//!      A  |  C    
//!  F---|----E|--+
//!      |  |  |  D
//!      +B-+  +--+
//! ```
//!
//! Given this diagram, the packet needs to take the following path:
//!
//! ```text
//!     Starting at the only line touching the top of the diagram, it must go down, pass through A, and continue onward to the first +.
//!     Travel right, up, and right, passing through B in the process.
//!     Continue down (collecting C), right, and up (collecting D).
//!     Finally, go all the way left through E and stopping at F.
//! ```
//!
//! Following the path to the end, the letters it sees on its path are ABCDEF.
//!
//! The little packet looks up at you, hoping you can help it find the way. What letters will it see (in the order it would see them) if it follows the path? (The routing diagram is very wide; make sure you view it without line wrapping.)
//!
//! **--- Part Two ---**
//!
//! The packet is curious how many steps it needs to go.
//!
//! For example, using the same routing diagram from the example above...
//!
//! ```text
//!      |          
//!      |  +--+    
//!      A  |  C    
//!  F---|--|-E---+
//!      |  |  |  D
//!      +B-+  +--+
//! ```
//!
//! ...the packet would go:
//!
//! ```text
//!     6 steps down (including the first line at the top of the diagram).
//!     3 steps right.
//!     4 steps up.
//!     3 steps right.
//!     4 steps down.
//!     3 steps right.
//!     2 steps up.
//!     13 steps left (including the F it stops on).
//! ```
//!
//! This would result in a total of 38 steps.
//!
//! How many steps does the packet need to go?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use ndarray::Array2;
use std::{
    fmt,
    fs::File,
    io::{BufRead, BufReader},
};

/// Direction of Movement
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
enum Direction {
    /// We are moving up.
    Up,
    /// We are moving down.
    Down,
    /// We are moving left.
    Left,
    /// We are moving right.
    Right,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dir_str = match *self {
            Direction::Up => "Up",
            Direction::Down => "Down",
            Direction::Left => "Left",
            Direction::Right => "Right",
        };
        write!(f, "{dir_str}")
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Output {
    letters: String,
    steps: usize,
}

impl Output {
    fn new(letters: String, steps: usize) -> Self {
        Self { letters, steps }
    }
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.letters, self.steps)
    }
}

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<Vec<String>, Output>(AoCYear::AOC2017, AoCDay::AOCD19, setup, find)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<String>, Output>(bench, AoCYear::AOC2017, AoCDay::AOCD19, setup, find)
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
fn find(data: Vec<String>) -> Output {
    find_res(&data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(data: &[String], _second_star: bool) -> Result<Output> {
    let max_y = data.len();
    let max_x = data
        .iter()
        .map(String::len)
        .max()
        .ok_or_else(|| anyhow!("no max x"))?;
    let mut network_map = Array2::<u8>::default((max_y + 1, max_x + 1));

    for (idx, line) in data.iter().enumerate() {
        fill_row(&mut network_map, idx, line);
    }

    let (letters, steps) = traverse_map(&network_map)?;
    Ok(Output::new(letters, steps))
}

/// Fill a row in the network map array.
fn fill_row(network_map: &mut Array2<u8>, row: usize, line: &str) {
    for (idx, bit) in line.as_bytes().iter().enumerate() {
        network_map[[row, idx]] = *bit;
    }
}

/// Traverse the map.
fn traverse_map(network_map: &Array2<u8>) -> Result<(String, usize)> {
    let mut bytes = Vec::new();
    let mut curr_row = 0;
    let mut curr_col = 0;
    let max_col = network_map.ncols();
    let max_row = network_map.nrows();
    let mut curr_direction = Direction::Down;
    let mut steps = 0;

    loop {
        if curr_row == max_row {
            return Err(anyhow!(format!("Invalid row value: {curr_row}")));
        }
        if curr_col == max_col {
            return Err(anyhow!(format!("Invalid col value: {curr_col}")));
        }

        let curr_byte = network_map[[curr_row, curr_col]];

        match curr_byte {
            32 => {
                if curr_row == 0 {
                    // We are still in the first row.  We need to find
                    // the down byte.
                    curr_col += 1;
                } else {
                    break;
                }
            }
            124 | 45 => {
                match curr_direction {
                    Direction::Down => curr_row += 1,
                    Direction::Up => curr_row -= 1,
                    Direction::Right => curr_col += 1,
                    Direction::Left => curr_col -= 1,
                }
                steps += 1;
            }
            43 => {
                let (next_row, next_col) = get_next_neighbor(
                    curr_row,
                    curr_col,
                    max_row,
                    max_col,
                    curr_direction,
                    network_map,
                )?;
                let next_direction =
                    get_next_dir(curr_row, curr_col, next_row, next_col, curr_direction);
                curr_row = next_row;
                curr_col = next_col;
                curr_direction = next_direction;
                steps += 1;
            }
            x => {
                bytes.push(x);

                match curr_direction {
                    Direction::Up => curr_row -= 1,
                    Direction::Down => curr_row += 1,
                    Direction::Left => curr_col -= 1,
                    Direction::Right => curr_col += 1,
                }
                steps += 1;
            }
        }
    }

    Ok((String::from_utf8_lossy(&bytes).into_owned(), steps))
}

/// Check the three nearest neighbors for the next valid direction.
fn get_next_neighbor(
    row: usize,
    col: usize,
    max_row: usize,
    max_col: usize,
    direction: Direction,
    network_map: &Array2<u8>,
) -> Result<(usize, usize)> {
    let row_i: isize = TryFrom::try_from(row)?;
    let col_i: isize = TryFrom::try_from(col)?;

    let (row_deltas, col_deltas) = match direction {
        Direction::Down | Direction::Up => (vec![0, 0], vec![-1, 1]),
        Direction::Right | Direction::Left => (vec![-1, 1], vec![0, 0]),
    };

    // Check the four adjacent neighbors (left, down, up, right)
    for k in 0..2 {
        if let Ok(adj_row) = TryFrom::try_from(row_i + row_deltas[k]) {
            if let Ok(adj_col) = TryFrom::try_from(col_i + col_deltas[k]) {
                if adj_row < max_row && adj_col < max_col && network_map[[adj_row, adj_col]] != 32 {
                    return Ok((adj_row, adj_col));
                }
            } else {
                continue;
            }
        }
    }

    Err(anyhow!("Unable to find valid next neighbor"))
}

/// Get the next direction
fn get_next_dir(
    curr_row: usize,
    curr_col: usize,
    next_row: usize,
    next_col: usize,
    curr_direction: Direction,
) -> Direction {
    match curr_direction {
        Direction::Down | Direction::Up => {
            if next_col < curr_col {
                Direction::Left
            } else {
                Direction::Right
            }
        }
        Direction::Right | Direction::Left => {
            if next_row < curr_row {
                Direction::Up
            } else {
                Direction::Down
            }
        }
    }
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<Vec<String>, Output>(AoCYear::AOC2017, AoCDay::AOCD19, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<String>, Output>(bench, AoCYear::AOC2017, AoCDay::AOCD19, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: Vec<String>) -> Output {
    find_res(&data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ ";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        let out = find(data);
        assert_eq!(out.letters, "ABCDEF");
        assert_eq!(out.steps, 38);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ ";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        let out = find2(data);
        assert_eq!(out.letters, "ABCDEF");
        assert_eq!(out.steps, 38);
        Ok(())
    }
}
