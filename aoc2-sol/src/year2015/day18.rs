// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 18 "Like a GIF For Your Yard"
//!
//! **--- Day 18: Like a GIF For Your Yard ---**
//!
//! **--- Part 1 ---**
//!
//! After the million lights incident, the fire code has gotten stricter:
//! now, at most ten thousand lights are allowed. You arrange them in a 100x100 grid.
//!
//! Never one to let you down, Santa again mails you instructions on the ideal
//! lighting configuration. With so few lights, he says, you'll have to resort to animation.
//!
//! Start by setting your lights to the included initial configuration
//! (your puzzle input). A # means "on", and a . means "off".
//!
//! Then, animate your grid in steps, where each step decides the next configuration
//! based on the current one. Each light's next state (either on or off) depends on
//! its current state and the current states of the eight lights adjacent to it (including diagonals).
//! Lights on the edge of the grid might have fewer than eight neighbors;
//! the missing ones always count as "off".
//!
//! For example, in a simplified 6x6 grid, the light marked A has the neighbors
//! numbered 1 through 8, and the light marked B, which is on an edge, only has
//! the neighbors marked 1 through 5:
//!
//! ```text
//! 1B5...
//! 234...
//! ......
//! ..123.
//! ..8A4.
//! ..765.
//! ```
//!
//! The state a light should have next is based on its current state (on or off) plus the
//! number of neighbors that are on:
//!
//! * A light which is on stays on when 2 or 3 neighbors are on, and turns off otherwise.
//! * A light which is off turns on if exactly 3 neighbors are on, and stays off otherwise.
//!
//! All of the lights update simultaneously; they all consider the same current state before
//! moving to the next.
//!
//! Here's a few steps from an example configuration of another 6x6 grid:
//!
//! ```text
//! Initial state:
//! .#.#.#
//! ...##.
//! #....#
//! ..#...
//! #.#..#
//! ####..
//!
//! After 1 step:
//! ..##..
//! ..##.#
//! ...##.
//! ......
//! #.....
//! #.##..
//!
//! After 2 steps:
//! ..###.
//! ......
//! ..###.
//! ......
//! .#....
//! .#....
//!
//! After 3 steps:
//! ...#..
//! ......
//! ...#..
//! ..##..
//! ......
//! ......
//!
//! After 4 steps:
//! ......
//! ......
//! ..##..
//! ..##..
//! ......
//! ......
//! ```
//!
//! After 4 steps, this example has four lights on.
//!
//! In your grid of 100x100 lights, given your initial configuration,
//! how many lights are on after 100 steps?
//!
//! **--- Part Two ---**
//!
//! You flip the instructions over; Santa goes on to point out that this is all
//! just an implementation of Conway's Game of Life. At least, it was, until you
//! notice that something's wrong with the grid of lights you bought: four lights,
//! one in each corner, are stuck on and can't be turned off. The example above will
//! actually run like this:
//!
//! ```text
//! Initial state:
//! ##.#.#
//! ...##.
//! #....#
//! ..#...
//! #.#..#
//! ####.#
//!
//! After 1 step:
//! #.##.#
//! ####.#
//! ...##.
//! ......
//! #...#.
//! #.####
//!
//! After 2 steps:
//! #..#.#
//! #....#
//! .#.##.
//! ...##.
//! .#..##
//! ##.###
//!
//! After 3 steps:
//! #...##
//! ####.#
//! ..##.#
//! ......
//! ##....
//! ####.#
//!
//! After 4 steps:
//! #.####
//! #....#
//! ...#..
//! .##...
//! #.....
//! #.#..#
//!
//! After 5 steps:
//! ##.###
//! .##..#
//! .##...
//! .##...
//! #.#...
//! ##...#
//! ```
//!
//! After 5 steps, this example now has 17 lights on.
//!
//! In your grid of 100x100 lights, given your initial configuration, but with the
//! four corners always in the on state, how many lights are on after 100 steps?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_solution, valid_lines},
};
use anyhow::Result;
use ndarray::Array2;
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
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD18, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader, 100, 100)
}

#[allow(clippy::comparison_chain)]
fn find_br<T>(reader: T, e_len: usize, steps: usize) -> usize
where
    T: BufRead,
{
    let mut lights: Array2<u8> = Array2::zeros((e_len + 2, e_len + 2));
    for (y, line) in valid_lines(reader).enumerate() {
        for (x, ch) in line.chars().enumerate() {
            lights[[x + 1, y + 1]] = u8::from(ch == '#');
        }
    }

    let mut output: Array2<u8> = Array2::zeros((e_len + 2, e_len + 2));
    let mut input = lights.clone();

    for _i in 0..steps {
        output.iter_mut().for_each(|x| *x = 0);
        let mut curr_x = 1;
        let mut curr_y = 1;

        for window in input.windows((3, 3)) {
            let curr_cell = window[(1, 1)];
            let total_alive =
                window.fold(0, |count, cell| if *cell == 1 { count + 1 } else { count });
            let neighbors_alive = total_alive - i32::from(curr_cell == 1);

            if curr_cell == 1 {
                if neighbors_alive == 2 || neighbors_alive == 3 {
                    output[[curr_x, curr_y]] = 1;
                }
            } else if neighbors_alive == 3 {
                output[[curr_x, curr_y]] = 1;
            }

            if curr_y < e_len {
                curr_y += 1;
            } else if curr_y == e_len {
                curr_y = 1;
                curr_x += 1;
            }
        }

        input.clone_from(&output);
    }

    let mut total = 0;
    output.iter().for_each(|x| {
        if *x == 1 {
            total += 1;
        }
    });
    total
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD18, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader, 100, 100)
}

#[allow(clippy::comparison_chain)]
fn find2_br<T>(reader: T, e_len: usize, steps: usize) -> usize
where
    T: BufRead,
{
    let mut lights: Array2<u8> = Array2::zeros((e_len + 2, e_len + 2));
    for (y, line) in valid_lines(reader).enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if (x + 1 == 1 && (y + 1 == 1 || y + 1 == e_len))
                || (y + 1 == e_len && (x + 1 == 1 || x + 1 == e_len))
            {
                lights[[x + 1, y + 1]] = 1;
            } else {
                lights[[x + 1, y + 1]] = u8::from(ch == '#');
            }
        }
    }

    let mut output: Array2<u8> = Array2::zeros((e_len + 2, e_len + 2));
    let mut input = lights.clone();

    for _i in 0..steps {
        output.iter_mut().for_each(|x| *x = 0);
        let mut curr_x = 1;
        let mut curr_y = 1;

        for window in input.windows((3, 3)) {
            let curr_cell = window[(1, 1)];
            let total_alive =
                window.fold(0, |count, cell| if *cell == 1 { count + 1 } else { count });
            let neighbors_alive = total_alive - i32::from(curr_cell == 1);

            if curr_cell == 1 {
                if neighbors_alive == 2 || neighbors_alive == 3 {
                    output[[curr_x, curr_y]] = 1;
                }
            } else if neighbors_alive == 3 {
                output[[curr_x, curr_y]] = 1;
            }

            if curr_y < e_len {
                curr_y += 1;
            } else if curr_y == e_len {
                curr_y = 1;
                curr_x += 1;
            }
        }
        output[[1, 1]] = 1;
        output[[1, e_len]] = 1;
        output[[e_len, 1]] = 1;
        output[[e_len, e_len]] = 1;

        input.clone_from(&output);
    }

    let mut total = 0;
    output.iter().for_each(|x| {
        if *x == 1 {
            total += 1;
        }
    });
    total
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use std::io::Cursor;

    const TEST_1: &str = r".#.#.#
...##.
#....#
..#...
#.#..#
####..";

    #[test]
    fn solution() {
        assert_eq!(find_br(Cursor::new(TEST_1), 6, 4), 4);
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use std::io::Cursor;

    const TEST_1: &str = r".#.#.#
...##.
#....#
..#...
#.#..#
####..";

    #[test]
    fn solution() {
        assert_eq!(find2_br(Cursor::new(TEST_1), 6, 5), 17);
    }
}
