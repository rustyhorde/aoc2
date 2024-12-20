// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Like a Rogue
//!
//! **--- Day 18: Like a Rogue ---**
//!
//! **--- Part 1 ---**
//!
//! As you enter this room, you hear a loud click! Some of the tiles in the floor
//! here seem to be pressure plates for traps, and the trap you just triggered
//! has run out of... whatever it tried to do to you. You doubt you'll be so
//! lucky next time.
//!
//! Upon closer examination, the traps and safe tiles in this room seem to
//! follow a pattern. The tiles are arranged into rows that are all the same
//! width; you take note of the safe tiles (.) and traps (^) in the first row
//! (your puzzle input).
//!
//! The type of tile (trapped or safe) in each row is based on the types of
//! the tiles in the same position, and to either side of that position, in
//! the previous row. (If either side is off either end of the row, it counts
//! as "safe" because there isn't a trap embedded in the wall.)
//!
//! For example, suppose you know the first row (with tiles marked by letters)
//! and want to determine the next row (with tiles marked by numbers):
//!
//! ```text
//! ABCDE
//! 12345
//! ```
//!
//! The type of tile `2` is based on the types of tiles `A`, `B`, and `C`; the
//! type of tile `5` is based on tiles `D`, `E`, and an imaginary "safe" tile.
//! Let's call these three tiles from the previous row the `left`, `center`,
//! and `right` tiles, respectively. Then, a new tile is a trap only in one of
//! the following situations:
//!
//! ```text
//! Its left and center tiles are traps, but its right tile is not.
//! Its center and right tiles are traps, but its left tile is not.
//! Only its left tile is a trap.
//! Only its right tile is a trap.
//! ```
//!
//! In any other situation, the new tile is safe.
//!
//! Then, starting with the row `..^^.`, you can determine the next row by applying
//! those rules to each new tile:
//!
//! ```text
//! The leftmost character on the next row considers the left (nonexistent, so we assume "safe"), center (the first ., which means "safe"), and right (the second ., also "safe") tiles on the previous row. Because all of the trap rules require a trap in at least one of the previous three tiles, the first tile on this new row is also safe, ..
//! The second character on the next row considers its left (.), center (.), and right (^) tiles from the previous row. This matches the fourth rule: only the right tile is a trap. Therefore, the next tile in this new row is a trap, ^.
//! The third character considers .^^, which matches the second trap rule: its center and right tiles are traps, but its left tile is not. Therefore, this tile is also a trap, ^.
//! The last two characters in this new row match the first and third rules, respectively, and so they are both also traps, ^.
//! ```
//!
//! After these steps, we now know the next row of tiles in the room: `.^^^^.`
//! Then, we continue on to the next row, using the same rules, and get `^^..^.`
//! After determining two new rows, our map looks like this:
//!
//! ```text
//! ..^^.
//! .^^^^
//! ^^..^
//! ```
//!
//! Here's a larger example with ten tiles per row and ten rows:
//!
//! ```text
//! .^^.^.^^^^
//! ^^^...^..^
//! ^.^^.^.^^.
//! ..^^...^^^
//! .^^^^.^^.^
//! ^^..^.^^..
//! ^^^^..^^^.
//! ^..^^^^.^^
//! .^^^..^.^^
//! ^^.^^^..^^
//! ```
//!
//! In ten rows, this larger example has `38` safe tiles.
//!
//! Starting with the map in your puzzle input, in a total of `40` rows (including the
//! starting row), how many safe tiles are there?
//!
//! **--- Part Two ---**
//!
//! How many safe tiles are there in a total of `400000` rows?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{print_err, run_solution, valid_lines},
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
    run_solution::<usize>(AoCYear::AOC2016, AoCDay::AOCD18, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader, 40).map_err(print_err).unwrap_or_default()
}

fn find_br<T>(reader: T, rows: usize) -> Result<usize>
where
    T: BufRead,
{
    let mut start = String::new();
    for line in valid_lines(reader) {
        start = line;
    }

    let mut tiles: Vec<Vec<bool>> = Vec::with_capacity(rows);
    // safe tile to the left
    let row_capacity = start.len() + 2;
    let mut row = Vec::with_capacity(row_capacity);
    row.push(true);
    for ch in start.chars() {
        match ch {
            '.' => row.push(true),
            '^' => row.push(false),
            _ => return Err(anyhow!("invalid tile")),
        }
    }
    // safe tile to the right
    row.push(true);
    tiles.push(row);

    for i in 1..rows {
        let prev_row = tiles.get(i - 1).ok_or_else(|| anyhow!("invalid row"))?;
        let mut row = Vec::with_capacity(row_capacity);
        row.push(true);
        for win in prev_row.windows(3) {
            let left = win[0];
            let center = win[1];
            let right = win[2];

            if !center && (!left && right || left && !right) {
                row.push(center);
            } else if center && (!left && right || left && !right) {
                row.push(!center);
            } else {
                row.push(true);
            }
        }
        row.push(true);
        tiles.push(row);
    }

    // count the true tiles, removing the extras added at the end of each row
    Ok(tiles.iter().flatten().filter(|x| **x).count() - (2 * rows))
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2016, AoCDay::AOCD18, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader).map_err(print_err).unwrap_or_default()
}

fn find2_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    find_br(reader, 400_000)
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"..^^.";
    const TEST_2: &str = r".^^.^.^^^^";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1), 3)?, 6);
        assert_eq!(find_br(Cursor::new(TEST_2), 10)?, 38);
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
