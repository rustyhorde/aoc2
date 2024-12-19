// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2017 ---**
//!
//! **--- Day 21: Fractal Art ---**
//!
//! You find a program trying to generate some art. It uses a strange process that involves repeatedly enhancing the detail of an image through a set of rules.
//!
//! The image consists of a two-dimensional square grid of pixels that are either on (#) or off (.). The program always begins with this pattern:
//!
//! > .#.
//! > ..#
//! > ###
//!
//! Because the pattern is both 3 pixels wide and 3 pixels tall, it is said to have a size of 3.
//!
//! Then, the program repeats the following process:
//!
//! >    If the size is evenly divisible by 2, break the pixels up into 2x2 squares, and convert each 2x2 square into a 3x3 square by following the corresponding enhancement rule.
//! >    Otherwise, the size is evenly divisible by 3; break the pixels up into 3x3 squares, and convert each 3x3 square into a 4x4 square by following the corresponding enhancement rule.
//!
//! Because each square of pixels is replaced by a larger one, the image gains pixels and so its size increases.
//!
//! The artist's book of enhancement rules is nearby (your puzzle input); however, it seems to be missing rules. The artist explains that sometimes, one must rotate or flip the input pattern to find a match. (Never rotate or flip the output pattern, though.) Each pattern is written concisely: rows are listed as single units, ordered top-down, and separated by slashes. For example, the following rules correspond to the adjacent patterns:
//!
//! ```text
//! ../.#  =  ..
//!           .#
//!
//!                 .#.
//! .#./..#/###  =  ..#
//!                 ###
//!
//!                         #..#
//! #..#/..../#..#/.##.  =  ....
//!                         #..#
//!                         .##.
//!```
//!
//! When searching for a rule to use, rotate and flip the pattern as necessary. For example, all of the following patterns match the same rule:
//!
//! ```text
//! .#.   .#.   #..   ###
//! ..#   #..   #.#   ..#
//! ###   ###   ##.   .#.
//! ```
//!
//! Suppose the book contained the following two rules:
//!
//! > ../.# => ##./#../...
//! > .#./..#/### => #..#/..../..../#..#
//!
//! As before, the program begins with this pattern:
//!
//! > .#.
//! > ..#
//! > ###
//!
//! The size of the grid (3) is not divisible by 2, but it is divisible by 3. It divides evenly into a single square; the square matches the second rule, which produces:
//!
//! > #..#
//! > ....
//! > ....
//! > #..#
//!
//! The size of this enhanced grid (4) is evenly divisible by 2, so that rule is used. It divides evenly into four squares:
//!
//! > #.|.#
//! > ..|..
//! > --+--
//! > ..|..
//! > #.|.#
//!
//! Each of these squares matches the same rule (../.# => ##./#../...), three of which require some flipping and rotation to line up with the rule. The output for the rule is the same in all four cases:
//!
//! > ##.|##.
//! > #..|#..
//! > ...|...
//! > ---+---
//! > ##.|##.
//! > #..|#..
//! > ...|...
//!
//! Finally, the squares are joined into a new grid:
//!
//! > ##.##.
//! > #..#..
//! > ......
//! > ##.##.
//! > #..#..
//! > ......
//!
//! Thus, after 2 iterations, the grid contains 12 pixels that are on.
//!
//! How many pixels stay on after 5 iterations?
//!
//! **--- Part Two ---**
//!
//! How many pixels stay on after 18 iterations?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use itertools::{iproduct, Itertools};
use pathfinding::matrix::Matrix;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

type FractalData = HashMap<Matrix<u8>, Matrix<u8>>;

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<FractalData, usize>(AoCYear::AOC2017, AoCDay::AOCD21, setup, find)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<FractalData, usize>(bench, AoCYear::AOC2017, AoCDay::AOCD21, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> FractalData {
    setup_br(reader).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T) -> Result<FractalData>
where
    T: BufRead,
{
    Ok(valid_lines(reader)
        .flat_map(|line| {
            let (k, v) = line
                .trim()
                .split(" => ")
                .filter_map(matrix)
                .next_tuple()
                .ok_or(anyhow!("blah"))
                .expect("tough cookies");
            iproduct!(vec![k.clone(), k.flipped_ud(), k.flipped_lr()], 0..4)
                .map(move |(m, i)| (m.rotated_cw(i), v.clone()))
        })
        .collect::<HashMap<_, _>>())
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: FractalData) -> usize {
    find_res(&data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(data: &FractalData, second_star: bool) -> Result<usize> {
    let mut sharps = (0..).scan(matrix(".#./..#/###"), |grid, _| {
        if let Some(grid) = grid {
            let pt = 2 + (grid.rows % 2);
            let b = grid.rows / pt;
            let mut new_grid = Matrix::new_square(grid.rows + b, b'?');
            for (c, l) in iproduct!(0..b, 0..b) {
                let new = &data[&grid.slice(l * pt..l * pt + pt, c * pt..c * pt + pt).ok()?];
                new_grid.set_slice((l * (pt + 1), c * (pt + 1)), new);
            }
            *grid = new_grid;
            Some(bytecount::count(grid.as_ref(), b'#'))
        } else {
            None
        }
    });

    if second_star {
        Ok(sharps.nth(17).unwrap_or(0))
    } else {
        Ok(sharps.nth(4).unwrap_or_default())
    }
}

/// Make a matrix of bytes for a rule.
fn matrix(i: &str) -> Option<Matrix<u8>> {
    Matrix::square_from_vec(i.bytes().filter(|&c| c != b'/').collect()).ok()
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<FractalData, usize>(AoCYear::AOC2017, AoCDay::AOCD21, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<FractalData, usize>(bench, AoCYear::AOC2017, AoCDay::AOCD21, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: FractalData) -> usize {
    find_res(&data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {}

#[cfg(test)]
mod two_star {}
