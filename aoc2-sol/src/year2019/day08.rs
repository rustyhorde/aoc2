// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2019 ---**
//!
//! **--- Day 8: Space Image Format ---**
//!
//! The Elves' spirits are lifted when they realize you have an opportunity to reboot one of their Mars rovers, and so they are curious if you would spend a brief sojourn on Mars. You land your ship near the rover.
//!
//! When you reach the rover, you discover that it's already in the process of rebooting! It's just waiting for someone to enter a BIOS password. The Elf responsible for the rover takes a picture of the password (your puzzle input) and sends it to you via the Digital Sending Network.
//!
//! Unfortunately, images sent via the Digital Sending Network aren't encoded with any normal encoding; instead, they're encoded in a special Space Image Format. None of the Elves seem to remember why this is the case. They send you the instructions to decode it.
//!
//! Images are sent as a series of digits that each represent the color of a single pixel. The digits fill each row of the image left-to-right, then move downward to the next row, filling rows top-to-bottom until every pixel of the image is filled.
//!
//! Each image actually consists of a series of identically-sized layers that are filled in this way. So, the first digit corresponds to the top-left pixel of the first layer, the second digit corresponds to the pixel to the right of that on the same layer, and so on until the last digit, which corresponds to the bottom-right pixel of the last layer.
//!
//! For example, given an image 3 pixels wide and 2 pixels tall, the image data 123456789012 corresponds to the following image layers:
//!
//! ```text
//! Layer 1: 123
//!          456
//!
//! Layer 2: 789
//!          012
//! ```
//!
//! The image you received is 25 pixels wide and 6 pixels tall.
//!
//! To make sure the image wasn't corrupted during transmission, the Elves would like you to find the layer that contains the fewest 0 digits. On that layer, what is the number of 1 digits multiplied by the number of 2 digits?
//!
//! **--- Part Two ---**
//!
//! Now you're ready to decode the image. The image is rendered by stacking the layers and aligning the pixels with the same positions in each layer. The digits indicate the color of the corresponding pixel: 0 is black, 1 is white, and 2 is transparent.
//!
//! The layers are rendered with the first layer in front and the last layer in back. So, if a given position has a transparent pixel in the first and second layers, a black pixel in the third layer, and a white pixel in the fourth layer, the final image would have a black pixel at that position.
//!
//! For example, given an image 2 pixels wide and 2 pixels tall, the image data 0222112222120000 corresponds to the following image layers:
//!
//! ```text
//! Layer 1: 02
//!          22
//!
//! Layer 2: 11
//!          22
//!
//! Layer 3: 22
//!          12
//!
//! Layer 4: 00
//!          00
//! ```
//!
//! Then, the full image can be found by determining the top visible pixel in each position:
//!
//! ```text
//!     The top-left pixel is black because the top layer is 0.
//!     The top-right pixel is white because the top layer is 2 (transparent), but the second layer is 1.
//!     The bottom-left pixel is white because the top two layers are 2, but the third layer is 1.
//!     The bottom-right pixel is black because the only visible pixel in that position is 0 (from layer 4).
//! ```
//!
//! So, the final image looks like this:
//!
//! ```text
//! 01
//! 10
//! ```
//!
//! What message is produced after decoding your image?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::Result;
// use crossterm::{cursor::{Hide, MoveRight, MoveToNextLine, RestorePosition, SavePosition, Show}, style::Print, ExecutableCommand, QueueableCommand};
use itertools::Itertools;
use ndarray::{Array3, Axis};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

type SifData = (Vec<Vec<u32>>, usize, usize);

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<SifData, usize>(AoCYear::AOC2019, AoCDay::AOCD08, setup, find).map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<SifData, usize>(bench, AoCYear::AOC2019, AoCDay::AOCD08, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> SifData {
    setup_br(reader, 25, 6).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T, width: usize, height: usize) -> Result<SifData>
where
    T: BufRead,
{
    let mut data = vec![];
    for line in valid_lines(reader) {
        for all_layer in &line.chars().chunks(width * height) {
            data.push(all_layer.filter_map(|x| x.to_digit(10)).collect());
        }
    }
    Ok((data, width, height))
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: SifData) -> usize {
    find_res(data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(sif_data: SifData, second_star: bool) -> Result<usize> {
    let (sif_data, width, _height) = sif_data;
    let mut min_zeroes = usize::MAX;
    let mut max_score = usize::MIN;

    if second_star {
        let mut layer_cube: Vec<Vec<Vec<u32>>> = vec![];
        for layer in &sif_data {
            let mut layer_square = vec![];
            for layer_chunk in &layer.iter().chunks(width) {
                let mut layer_row = vec![];
                for digit in layer_chunk {
                    layer_row.push(*digit);
                }
                layer_square.push(layer_row);
            }
            layer_cube.push(layer_square);
        }
        let max_z = layer_cube.len();
        let max_y = layer_cube[0].len();
        let max_x = layer_cube[0][0].len();
        let mut arr = Array3::<u32>::zeros((max_x, max_y, max_z));

        for z in 0..max_z {
            for y in 0..max_y {
                for x in 0..max_x {
                    arr[[x, y, z]] = layer_cube[z][y][x];
                }
            }
        }

        let mut curr_x = 0;
        let mut curr_y = 0;
        let mut image = HashMap::new();
        for axis in arr.axis_iter(Axis(1)) {
            for chunk in &axis.iter().chunks(max_z) {
                for val in chunk {
                    if *val == 2 {
                        continue;
                    }
                    if *val == 0 {
                        let _ = image.entry((curr_x, curr_y)).or_insert(' ');
                        break;
                    } else if *val == 1 {
                        let _ = image.entry((curr_x, curr_y)).or_insert('#');
                        break;
                    }
                }
                curr_x += 1;
            }
            curr_x = 0;
            curr_y += 1;
        }

        for y in 0..max_y {
            for x in 0..max_x {
                eprint!("{}", image.get(&(x, y)).unwrap_or(&' '));
            }
            eprintln!();
        }
    } else {
        for layer in &sif_data {
            let zeroes = layer.iter().filter(|x| **x == 0).count();
            if zeroes < min_zeroes {
                min_zeroes = zeroes;
                let ones = layer.iter().filter(|x| **x == 1).count();
                let twos = layer.iter().filter(|x| **x == 2).count();
                max_score = ones * twos;
            }
        }
    }
    Ok(max_score)
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<SifData, usize>(AoCYear::AOC2019, AoCDay::AOCD08, setup, find2).map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<SifData, usize>(bench, AoCYear::AOC2019, AoCDay::AOCD08, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: SifData) -> usize {
    find_res(data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"123456789012";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1), 3, 2)?;
        assert_eq!(find(data), 1);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    // const TEST_2: &str = r"123456789012";
    const TEST_1: &str = r"0222112222120000";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1), 2, 2)?;
        assert_eq!(find2(data), 0);
        Ok(())
    }
}
