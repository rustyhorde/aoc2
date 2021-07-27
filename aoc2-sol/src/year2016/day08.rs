// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 8 "Two-Factor Authentication"
//!
//! --- Day 8: Two-Factor Authentication ---
//!
//! You come across a door implementing what you can only assume is an implementation
//! of two-factor authentication after a long game of requirements telephone.
//!
//! To get past the door, you first swipe a keycard (no problem; there was one on
//! a nearby desk). Then, it displays a code on a little screen, and you type that
//! code on a keypad. Then, presumably, the door unlocks.
//!
//! Unfortunately, the screen has been smashed. After a few minutes, you've taken
//! everything apart and figured out how it works. Now you just have to work out what
//! the screen would have displayed.
//!
//! The magnetic strip on the card you swiped encodes a series of instructions for
//! the screen; these instructions are your puzzle input. The screen is 50 pixels wide and
//! 6 pixels tall, all of which start off, and is capable of three somewhat peculiar operations:
//!
//! ```text
//! rect AxB turns on all of the pixels in a rectangle at the top-left of the screen which is A wide and B tall.
//! rotate row y=A by B shifts all of the pixels in row A (0 is the top row) right by B pixels. Pixels that would fall off the right end appear at the left end of the row.
//! rotate column x=A by B shifts all of the pixels in column A (0 is the left column) down by B pixels. Pixels that would fall off the bottom appear at the top of the column.
//! ```
//!
//! For example, here is a simple sequence on a smaller screen:
//!
//! ```text
//! rect 3x2 creates a small rectangle in the top-left corner:
//!
//! ###....
//! ###....
//! .......
//! ```
//! ```text
//! rotate column x=1 by 1 rotates the second column down by one pixel:
//!
//! #.#....
//! ###....
//! .#.....
//! ```
//! ```text
//! rotate row y=0 by 4 rotates the top row right by four pixels:
//!
//! ....#.#
//! ###....
//! .#.....
//! ```
//! ```text
//! rotate column x=1 by 1 again rotates the second column down by one pixel, causing the bottom pixel to wrap back to the top:
//!
//! .#..#.#
//! #.#....
//! .#.....
//! ```
//!
//! As you can see, this display technology is extremely powerful, and will soon
//! dominate the tiny-code-displaying-screen market. That's what the advertisement on
//! the back of the display tries to convince you, anyway.
//!
//! There seems to be an intermediate check of the voltage used by the display: after
//! you swipe your card, if the screen did work, how many pixels should be lit?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{get_cap_x, run_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use ndarray::Array2;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Instruction {
    Rect(usize, usize),
    RotRow(usize, usize),
    RotCol(usize, usize),
}

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2016, AoCDay::AOCD08, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader, 50, 6).unwrap_or_default()
}

fn find_br<T>(reader: T, max_x: usize, max_y: usize) -> Result<usize>
where
    T: BufRead,
{
    let (lit, _) = process(reader, max_x, max_y)?;
    Ok(lit)
}

fn process<T>(reader: T, max_x: usize, max_y: usize) -> Result<(usize, Array2<char>)>
where
    T: BufRead,
{
    let rect_re = Regex::new(r"^rect (\d+)x(\d+)$")?;
    let rot_row_re = Regex::new(r"^rotate row y=(\d+) by (\d+)$")?;
    let rot_col_re = Regex::new(r"^rotate column x=(\d+) by (\d+)$")?;

    let mut arr: Array2<char> = Array2::default((max_y, max_x));
    arr.fill('.');

    let mut instructions = vec![];

    for line in valid_lines(reader) {
        if rect_re.is_match(&line) {
            for caps in rect_re.captures_iter(&line) {
                let x = get_cap_x::<usize>(1, &caps)?;
                let y = get_cap_x::<usize>(2, &caps)?;
                instructions.push(Instruction::Rect(x, y));
            }
        } else if rot_row_re.is_match(&line) {
            for caps in rot_row_re.captures_iter(&line) {
                let row = get_cap_x::<usize>(1, &caps)?;
                let amount = get_cap_x::<usize>(2, &caps)?;
                instructions.push(Instruction::RotRow(row, amount));
            }
        } else if rot_col_re.is_match(&line) {
            for caps in rot_col_re.captures_iter(&line) {
                let col = get_cap_x::<usize>(1, &caps)?;
                let amount = get_cap_x::<usize>(2, &caps)?;
                instructions.push(Instruction::RotCol(col, amount));
            }
        } else {
            return Err(anyhow!(format!("invalid instruction: {}", line)));
        }
    }

    for inst in instructions {
        match inst {
            Instruction::Rect(x, y) => {
                for x in 0..x {
                    for y in 0..y {
                        arr[[y, x]] = '#';
                    }
                }
            }
            Instruction::RotCol(col, amount) => {
                let mut column = arr.column_mut(col);
                let mut col_vec = column.iter().copied().collect::<Vec<char>>();
                col_vec.rotate_right(amount);

                for (val, old) in col_vec.iter().zip(column.iter_mut()) {
                    *old = *val;
                }
            }
            Instruction::RotRow(row, amount) => {
                let mut row = arr.row_mut(row);
                let mut row_vec = row.iter().copied().collect::<Vec<char>>();
                row_vec.rotate_right(amount);

                for (val, old) in row_vec.iter().zip(row.iter_mut()) {
                    *old = *val;
                }
            }
        }
    }

    Ok((arr.iter().filter(|x| **x == '#').count(), arr))
}

fn to_string(arr: &Array2<char>) -> String {
    let mut arr_str = String::new();
    arr_str.push('\n');
    for row in arr.outer_iter() {
        for point in row.iter() {
            arr_str.push(*point);
        }
        arr_str.push('\n');
    }
    arr_str
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<String>(AoCYear::AOC2016, AoCDay::AOCD08, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> String {
    find2_br(reader, 50, 6).unwrap_or_default()
}

fn find2_br<T>(reader: T, max_x: usize, max_y: usize) -> Result<String>
where
    T: BufRead,
{
    let (_, arr) = process(reader, max_x, max_y)?;
    Ok(to_string(&arr))
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1), 7, 3)?, 6);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(
            find2_br(Cursor::new(TEST_1), 7, 3)?,
            "\n.#..#.#\n#.#....\n.#.....\n"
        );
        Ok(())
    }
}
