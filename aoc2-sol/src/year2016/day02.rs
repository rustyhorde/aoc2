// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 2 "Bathroom Security"
//!
//! --- Day 2: Bathroom Security ---
//!
//! You arrive at Easter Bunny Headquarters under cover of darkness. However,
//! you left in such a rush that you forgot to use the bathroom! Fancy office
//! buildings like this one usually have keypad locks on their bathrooms, so
//! you search the front desk for the code.
//!
//! "In order to improve security," the document you find says, "bathroom codes
//! will no longer be written down. Instead, please memorize and follow the
//! procedure below to access the bathrooms."
//!
//! The document goes on to explain that each button to be pressed can be
//! found by starting on the previous button and moving to adjacent buttons
//! on the keypad: U moves up, D moves down, L moves left, and R moves right.
//! Each line of instructions corresponds to one button, starting at the previous
//! button (or, for the first line, the "5" button); press whatever button you're
//! on at the end of each line. If a move doesn't lead to a button, ignore it.
//!
//! You can't hold it much longer, so you decide to figure out the code as you
//! walk to the bathroom. You picture a keypad like this:
//!
//! ```text
//! 1 2 3
//! 4 5 6
//! 7 8 9
//! ```
//!
//! Suppose your instructions are:
//!
//! ```text
//! ULL
//! RRDDD
//! LURDL
//! UUUUD
//! ```
//!
//! ```text
//! You start at "5" and move up (to "2"), left (to "1"), and left (you can't, and stay on "1"), so the first button is 1.
//! Starting from the previous button ("1"), you move right twice (to "3") and then down three times (stopping at "9" after two moves and ignoring the third), ending up with 9.
//! Continuing from "9", you move left, up, right, down, and left, ending with 8.
//! Finally, you move up four times (stopping at "2"), then down once, ending with 5.
//! ```
//!
//! So, in this example, the bathroom code is `1985`.
//!
//! Your puzzle input is the instructions from the document you found at the
//! front desk. What is the bathroom code?
//!
//! **--- Part Two ---**
//!
//! You finally arrive at the bathroom (it's a several minute walk from the lobby so visitors
//! can behold the many fancy conference rooms and water coolers on this floor) and go to
//! punch in the code. Much to your bladder's dismay, the keypad is not at all like you
//! imagined it. Instead, you are confronted with the result of hundreds of man-hours of
//! bathroom-keypad-design meetings:
//!
//! ```text
//!     1
//!   2 3 4
//! 5 6 7 8 9
//!   A B C
//!     D
//! ```
//!
//! You still start at "5" and stop when you're at an edge, but given the same instructions
//! as above, the outcome is very different:
//!
//! ```text
//! You start at "5" and don't move at all (up and left are both edges), ending at 5.
//! Continuing from "5", you move right twice and down three times (through "6", "7", "B", "D", "D"), ending at D.
//! Then, from "D", you move five more times (through "D", "B", "C", "C", "B"), ending at B.
//! Finally, after five more moves, you end at 3.
//! ```
//!
//! So, given the actual keypad layout, the code would be `5DB3`.
//!
//! Using the same instructions in your puzzle input, what is the correct bathroom code?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_solution, valid_lines},
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
    run_solution::<usize>(AoCYear::AOC2016, AoCDay::AOCD02, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader).unwrap_or_default()
}

fn find_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let mut buttons_pressed = String::new();
    let mut x = 1;
    let mut y = 1;
    let max_x = 2;
    let max_y = 2;

    for line in valid_lines(reader) {
        for ch in line.chars() {
            match ch {
                'R' => {
                    if x < max_x {
                        x += 1;
                    }
                }
                'L' => {
                    if x > 0 {
                        x -= 1;
                    }
                }
                'U' => {
                    if y < max_y {
                        y += 1;
                    }
                }
                'D' => {
                    if y > 0 {
                        y -= 1;
                    }
                }
                _ => return Err(anyhow!("invalid direction")),
            }
        }

        buttons_pressed.push_str(
            &match (x, y) {
                (0, 0) => 7,
                (1, 0) => 8,
                (2, 0) => 9,
                (0, 1) => 4,
                (1, 1) => 5,
                (2, 1) => 6,
                (0, 2) => 1,
                (1, 2) => 2,
                (2, 2) => 3,
                _ => return Err(anyhow!("invalid button")),
            }
            .to_string(),
        );
    }
    Ok(buttons_pressed.parse::<usize>()?)
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<String>(AoCYear::AOC2016, AoCDay::AOCD02, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> String {
    find2_br(reader).unwrap_or_default()
}

fn find2_br<T>(reader: T) -> Result<String>
where
    T: BufRead,
{
    let valid_coords = [
        (2, 0),
        (1, 1),
        (2, 1),
        (3, 1),
        (0, 2),
        (1, 2),
        (2, 2),
        (3, 2),
        (4, 2),
        (1, 3),
        (2, 3),
        (3, 3),
        (2, 4),
    ];
    let mut buttons_pressed = String::new();
    let mut x = 0;
    let mut y = 2;

    for line in valid_lines(reader) {
        for ch in line.chars() {
            match ch {
                'R' => {
                    x += 1;
                    if !valid_coords.contains(&(x, y)) {
                        x -= 1;
                    }
                }
                'L' => {
                    if x > 0 {
                        x -= 1;
                        if !valid_coords.contains(&(x, y)) {
                            x += 1;
                        }
                    }
                }
                'U' => {
                    y += 1;
                    if !valid_coords.contains(&(x, y)) {
                        y -= 1;
                    }
                }
                'D' => {
                    if y > 0 {
                        y -= 1;
                        if !valid_coords.contains(&(x, y)) {
                            y += 1;
                        }
                    }
                }
                _ => return Err(anyhow!("invalid direction")),
            }
        }
        buttons_pressed.push(match (x, y) {
            (2, 0) => 'D',
            (1, 1) => 'A',
            (2, 1) => 'B',
            (3, 1) => 'C',
            (0, 2) => '5',
            (1, 2) => '6',
            (2, 2) => '7',
            (3, 2) => '8',
            (4, 2) => '9',
            (1, 3) => '2',
            (2, 3) => '3',
            (3, 3) => '4',
            (2, 4) => '1',
            _ => return Err(anyhow!("invalid button")),
        });
    }
    Ok(buttons_pressed)
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"ULL
RRDDD
LURDL
UUUUD";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1))?, 1985);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"ULL
RRDDD
LURDL
UUUUD";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find2_br(Cursor::new(TEST_1))?, "5DB3");
        Ok(())
    }
}
