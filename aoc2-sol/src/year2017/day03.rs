// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Spiral Memory
//!
//! **--- Day 3: Spiral Memory ---**
//!
//! **--- Part 1 ---**
//!
//! You come across an experimental new kind of memory stored on an infinite
//! two-dimensional grid.
//!
//! Each square on the grid is allocated in a spiral pattern starting at a
//! location marked `1` and then counting up while spiraling outward. For example,
//! the first few squares are allocated like this:
//!
//! ```text
//! 17  16  15  14  13
//! 18   5   4   3  12
//! 19   6   1   2  11
//! 20   7   8   9  10
//! 21  22  23---> ...
//! ```
//!
//! While this is very space-efficient (no squares are skipped), requested data must
//! be carried back to square `1` (the location of the only access port for this
//! memory system) by programs that can only move up, down, left, or right. They
//! always take the shortest path: the Manhattan Distance between the location of the
//! data and square `1`.
//!
//! For example:
//!
//! ```text
//! Data from square 1 is carried 0 steps, since it's at the access port.
//! Data from square 12 is carried 3 steps, such as: down, left, left.
//! Data from square 23 is carried only 2 steps: up twice.
//! Data from square 1024 must be carried 31 steps.
//! ```
//!
//! How many steps are required to carry the data from the square identified in your
//! puzzle input all the way to the access port?
//!
//! **--- Part Two ---**
//!
//! As a stress test on the system, the programs here clear the grid and then
//! store the value `1` in square `1`. Then, in the same allocation order as
//! shown above, they store the sum of the values in all adjacent squares,
//! including diagonals.
//!
//! So, the first few squares' values are chosen as follows:
//!
//! ```text
//! Square 1 starts with the value 1.
//! Square 2 has only one adjacent filled square (with value 1), so it also stores 1.
//! Square 3 has both of the above squares as neighbors and stores the sum of their values, 2.
//! Square 4 has all three of the aforementioned squares as neighbors and stores the sum of their values, 4.
//! Square 5 only has the first and fourth squares as neighbors, so it gets the value 5.
//! ```
//!
//! Once a square is written, its value does not change. Therefore, the first few squares
//! would receive the following values:
//!
//! ```text
//! 147  142  133  122   59
//! 304    5    4    2   57
//! 330   10    1    1   54
//! 351   11   23   25   26
//! 362  747  806--->   ...
//! ```
//!
//! What is the first value written that is larger than your puzzle input?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{print_err, run_solution, valid_lines},
};
use anyhow::{anyhow, Result};
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
    run_solution::<isize>(AoCYear::AOC2017, AoCDay::AOCD03, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> isize {
    find_br(reader).map_err(print_err).unwrap_or_default()
}

fn find_br<T>(reader: T) -> Result<isize>
where
    T: BufRead,
{
    let mut value = 0;
    for line in valid_lines(reader) {
        value = line.parse::<isize>()?;
    }
    let final_tuple = calculate_tuple(value);
    Ok((final_tuple.0.abs_diff(0) + final_tuple.1.abs_diff(0)).try_into()?)
}

// Start a (0,0) and rapidly move out "shells" until
// the shell with the value.  Then generate tuples
// one at a time until we get to it.
fn calculate_tuple(value: isize) -> (isize, isize) {
    let mut current_tuple = (0, 0);
    let mut generated = 1;

    for shell in 0.. {
        let upper_limit = generated + (8 * shell);
        if value <= upper_limit {
            let needed = value - generated;
            if needed > 0 {
                generate_next_n_tuples(&mut current_tuple, shell, value - generated);
            }
            break;
        }
        // We know the last tuple in any shell `x` is `(x, -x)`, so we just set it.
        current_tuple = (shell, -shell);
        // Bump the upper limit
        generated = upper_limit;
    }
    current_tuple
}

/// Calculate the last tuple in the given shell
fn generate_next_n_tuples(tuple: &mut (isize, isize), shell: isize, count: isize) {
    tuple.0 += 1;
    let side_length = (8 * shell) / 4;
    let max_y = shell;
    let min_x = -shell;
    let min_y = -shell;

    for idx in 1..count {
        if tuple.1 < max_y && idx <= side_length {
            tuple.1 += 1;
        } else if tuple.0 > min_x && idx <= side_length * 2 {
            tuple.0 -= 1;
        } else if tuple.1 > min_y && idx <= side_length * 3 {
            tuple.1 -= 1;
        } else {
            tuple.0 += 1;
        }
    }
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<isize>(AoCYear::AOC2017, AoCDay::AOCD03, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> isize {
    find2_br(reader).map_err(print_err).unwrap_or_default()
}

fn find2_br<T>(reader: T) -> Result<isize>
where
    T: BufRead,
{
    let mut value = 0;
    for line in valid_lines(reader) {
        value = line.parse::<isize>()?;
    }
    next_biggest(value)
}

// Find the next biggest value after the given maximum value.
fn next_biggest(max_value: isize) -> Result<isize> {
    let mut tuple_map: HashMap<(isize, isize), isize> = HashMap::new();
    let mut current_tuple = (0, 0);
    _ = tuple_map.insert(current_tuple, 1);

    for shell in 1.. {
        let shell_len = 8 * shell;
        let side_len = shell_len / 4;
        let max_y = shell;
        let min_x = -shell;
        let min_y = -shell;

        for idx in 0..shell_len {
            if idx == 0 {
                current_tuple.0 += 1;
            } else if current_tuple.1 < max_y && idx <= side_len {
                current_tuple.1 += 1;
            } else if current_tuple.0 > min_x && idx <= side_len * 2 {
                current_tuple.0 -= 1;
            } else if current_tuple.1 > min_y && idx <= side_len * 3 {
                current_tuple.1 -= 1;
            } else {
                current_tuple.0 += 1;
            }

            let value = calculate_tuple_val(current_tuple, &tuple_map);
            if value <= max_value {
                _ = tuple_map.insert(current_tuple, value);
            } else {
                return Ok(value);
            }
        }
    }

    Err(anyhow!("Unable to find next biggest value"))
}

// Calculate the value for the given tuple given the tuple map
fn calculate_tuple_val(tuple: (isize, isize), tuple_map: &HashMap<(isize, isize), isize>) -> isize {
    let x = tuple.0;
    let y = tuple.1;

    // Add 8 nearest neighbors.  Only previously populated neighbors will have values (`Some(x)`).
    // The rest will return `None` on get.
    let results = [
        // Add current column (not including self)
        tuple_map.get(&(x, y + 1)),
        tuple_map.get(&(x, y - 1)),
        // Add one column to right
        tuple_map.get(&(x + 1, y)),
        tuple_map.get(&(x + 1, y + 1)),
        tuple_map.get(&(x + 1, y - 1)),
        // Add one column to left
        tuple_map.get(&(x - 1, y)),
        tuple_map.get(&(x - 1, y + 1)),
        tuple_map.get(&(x - 1, y - 1)),
    ];

    results.iter().filter_map(|x| *x).sum()
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"1";
    const TEST_2: &str = r"12";
    const TEST_3: &str = r"23";
    const TEST_4: &str = r"1024";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1))?, 0);
        assert_eq!(find_br(Cursor::new(TEST_2))?, 3);
        assert_eq!(find_br(Cursor::new(TEST_3))?, 2);
        assert_eq!(find_br(Cursor::new(TEST_4))?, 31);
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
