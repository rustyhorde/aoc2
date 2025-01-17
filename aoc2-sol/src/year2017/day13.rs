// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2017 ---**
//!
//! **--- Day 13: Packet Scanners ---**
//!
//! You need to cross a vast firewall. The firewall consists of several layers, each with a security scanner that moves back and forth across the layer. To succeed, you must not be detected by a scanner.
//!
//! By studying the firewall briefly, you are able to record (in your puzzle input) the depth of each layer and the range of the scanning area for the scanner within it, written as depth: range. Each layer has a thickness of exactly 1. A layer at depth 0 begins immediately inside the firewall; a layer at depth 1 would start immediately after that.
//!
//! For example, suppose you've recorded the following:
//!
//! ```text
//! 0: 3
//! 1: 2
//! 4: 4
//! 6: 4
//! ```
//!
//! This means that there is a layer immediately inside the firewall (with range 3), a second layer immediately after that (with range 2), a third layer which begins at depth 4 (with range 4), and a fourth layer which begins at depth 6 (also with range 4). Visually, it might look like this:
//!
//! ```text
//!  0   1   2   3   4   5   6
//! [ ] [ ] ... ... [ ] ... [ ]
//! [ ] [ ]         [ ]     [ ]
//! [ ]             [ ]     [ ]
//!                 [ ]     [ ]
//! ```
//!
//! Within each layer, a security scanner moves back and forth within its range. Each security scanner starts at the top and moves down until it reaches the bottom, then moves up until it reaches the top, and repeats. A security scanner takes one picosecond to move one step. Drawing scanners as S, the first few picoseconds look like this:
//!
//! ```text
//! Picosecond 0:
//!  0   1   2   3   4   5   6
//! [S] [S] ... ... [S] ... [S]
//! [ ] [ ]         [ ]     [ ]
//! [ ]             [ ]     [ ]
//!                 [ ]     [ ]
//!
//! Picosecond 1:
//!  0   1   2   3   4   5   6
//! [ ] [ ] ... ... [ ] ... [ ]
//! [S] [S]         [S]     [S]
//! [ ]             [ ]     [ ]
//!                 [ ]     [ ]
//!
//! Picosecond 2:
//!  0   1   2   3   4   5   6
//! [ ] [S] ... ... [ ] ... [ ]
//! [ ] [ ]         [ ]     [ ]
//! [S]             [S]     [S]
//!                 [ ]     [ ]
//!
//! Picosecond 3:
//!  0   1   2   3   4   5   6
//! [ ] [ ] ... ... [ ] ... [ ]
//! [S] [S]         [ ]     [ ]
//! [ ]             [ ]     [ ]
//!                 [S]     [S]
//! ```
//!
//! Your plan is to hitch a ride on a packet about to move through the firewall. The packet will travel along the top of each layer, and it moves at one layer per picosecond. Each picosecond, the packet moves one layer forward (its first move takes it into layer 0), and then the scanners move one step. If there is a scanner at the top of the layer as your packet enters it, you are caught. (If a scanner moves into the top of its layer while you are there, you are not caught: it doesn't have time to notice you before you leave.) If you were to do this in the configuration above, marking your current position with parentheses, your passage through the firewall would look like this:
//!
//! ```text
//! Initial state:
//!  0   1   2   3   4   5   6
//! [S] [S] ... ... [S] ... [S]
//! [ ] [ ]         [ ]     [ ]
//! [ ]             [ ]     [ ]
//!                 [ ]     [ ]
//!
//! Picosecond 0:
//!  0   1   2   3   4   5   6
//! (S) [S] ... ... [S] ... [S]
//! [ ] [ ]         [ ]     [ ]
//! [ ]             [ ]     [ ]
//!                 [ ]     [ ]
//!
//!  0   1   2   3   4   5   6
//! ( ) [ ] ... ... [ ] ... [ ]
//! [S] [S]         [S]     [S]
//! [ ]             [ ]     [ ]
//!                 [ ]     [ ]
//!
//!
//! Picosecond 1:
//!  0   1   2   3   4   5   6
//! [ ] ( ) ... ... [ ] ... [ ]
//! [S] [S]         [S]     [S]
//! [ ]             [ ]     [ ]
//!                 [ ]     [ ]
//!
//!  0   1   2   3   4   5   6
//! [ ] (S) ... ... [ ] ... [ ]
//! [ ] [ ]         [ ]     [ ]
//! [S]             [S]     [S]
//!                 [ ]     [ ]
//!
//!
//! Picosecond 2:
//!  0   1   2   3   4   5   6
//! [ ] [S] (.) ... [ ] ... [ ]
//! [ ] [ ]         [ ]     [ ]
//! [S]             [S]     [S]
//!                 [ ]     [ ]
//!
//!  0   1   2   3   4   5   6
//! [ ] [ ] (.) ... [ ] ... [ ]
//! [S] [S]         [ ]     [ ]
//! [ ]             [ ]     [ ]
//!                 [S]     [S]
//!
//!
//! Picosecond 3:
//!  0   1   2   3   4   5   6
//! [ ] [ ] ... (.) [ ] ... [ ]
//! [S] [S]         [ ]     [ ]
//! [ ]             [ ]     [ ]
//!                 [S]     [S]
//!
//!  0   1   2   3   4   5   6
//! [S] [S] ... (.) [ ] ... [ ]
//! [ ] [ ]         [ ]     [ ]
//! [ ]             [S]     [S]
//!                 [ ]     [ ]
//!
//!
//! Picosecond 4:
//!  0   1   2   3   4   5   6
//! [S] [S] ... ... ( ) ... [ ]
//! [ ] [ ]         [ ]     [ ]
//! [ ]             [S]     [S]
//!                 [ ]     [ ]
//!
//!  0   1   2   3   4   5   6
//! [ ] [ ] ... ... ( ) ... [ ]
//! [S] [S]         [S]     [S]
//! [ ]             [ ]     [ ]
//!                 [ ]     [ ]
//!
//!
//! Picosecond 5:
//!  0   1   2   3   4   5   6
//! [ ] [ ] ... ... [ ] (.) [ ]
//! [S] [S]         [S]     [S]
//! [ ]             [ ]     [ ]
//!                 [ ]     [ ]
//!
//!  0   1   2   3   4   5   6
//! [ ] [S] ... ... [S] (.) [S]
//! [ ] [ ]         [ ]     [ ]
//! [S]             [ ]     [ ]
//!                 [ ]     [ ]
//!
//!
//! Picosecond 6:
//!  0   1   2   3   4   5   6
//! [ ] [S] ... ... [S] ... (S)
//! [ ] [ ]         [ ]     [ ]
//! [S]             [ ]     [ ]
//!                 [ ]     [ ]
//!
//!  0   1   2   3   4   5   6
//! [ ] [ ] ... ... [ ] ... ( )
//! [S] [S]         [S]     [S]
//! [ ]             [ ]     [ ]
//!                 [ ]     [ ]
//! ```
//!
//! In this situation, you are caught in layers 0 and 6, because your packet entered the layer when its scanner was at the top when you entered it. You are not caught in layer 1, since the scanner moved into the top of the layer once you were already there.
//!
//! The severity of getting caught on a layer is equal to its depth multiplied by its range. (Ignore layers in which you do not get caught.) The severity of the whole trip is the sum of these values. In the example above, the trip severity is 0*3 + 6*4 = 24.
//!
//! Given the details of the firewall you've recorded, if you leave immediately, what is the severity of your whole trip?
//!
//! **--- Part Two ---**
//!
//! Now, you need to pass through the firewall without being caught - easier said than done.
//!
//! You can't control the speed of the packet, but you can delay it any number of picoseconds. For each picosecond you delay the packet before beginning your trip, all security scanners move one step. You're not in the firewall during this time; you don't enter layer 0 until you stop delaying the packet.
//!
//! In the example above, if you delay 10 picoseconds (picoseconds 0 - 9), you won't get caught:
//!
//! ```text
//! State after delaying:
//!  0   1   2   3   4   5   6
//! [ ] [S] ... ... [ ] ... [ ]
//! [ ] [ ]         [ ]     [ ]
//! [S]             [S]     [S]
//!                 [ ]     [ ]
//!
//! Picosecond 10:
//!  0   1   2   3   4   5   6
//! ( ) [S] ... ... [ ] ... [ ]
//! [ ] [ ]         [ ]     [ ]
//! [S]             [S]     [S]
//!                 [ ]     [ ]
//!
//!  0   1   2   3   4   5   6
//! ( ) [ ] ... ... [ ] ... [ ]
//! [S] [S]         [S]     [S]
//! [ ]             [ ]     [ ]
//!                 [ ]     [ ]
//!
//!
//! Picosecond 11:
//!  0   1   2   3   4   5   6
//! [ ] ( ) ... ... [ ] ... [ ]
//! [S] [S]         [S]     [S]
//! [ ]             [ ]     [ ]
//!                 [ ]     [ ]
//!
//!  0   1   2   3   4   5   6
//! [S] (S) ... ... [S] ... [S]
//! [ ] [ ]         [ ]     [ ]
//! [ ]             [ ]     [ ]
//!                 [ ]     [ ]
//!
//!
//! Picosecond 12:
//!  0   1   2   3   4   5   6
//! [S] [S] (.) ... [S] ... [S]
//! [ ] [ ]         [ ]     [ ]
//! [ ]             [ ]     [ ]
//!                 [ ]     [ ]
//!
//!  0   1   2   3   4   5   6
//! [ ] [ ] (.) ... [ ] ... [ ]
//! [S] [S]         [S]     [S]
//! [ ]             [ ]     [ ]
//!                 [ ]     [ ]
//!
//!
//! Picosecond 13:
//!  0   1   2   3   4   5   6
//! [ ] [ ] ... (.) [ ] ... [ ]
//! [S] [S]         [S]     [S]
//! [ ]             [ ]     [ ]
//!                 [ ]     [ ]
//!
//!  0   1   2   3   4   5   6
//! [ ] [S] ... (.) [ ] ... [ ]
//! [ ] [ ]         [ ]     [ ]
//! [S]             [S]     [S]
//!                 [ ]     [ ]
//!
//!
//! Picosecond 14:
//!  0   1   2   3   4   5   6
//! [ ] [S] ... ... ( ) ... [ ]
//! [ ] [ ]         [ ]     [ ]
//! [S]             [S]     [S]
//!                 [ ]     [ ]
//!
//!  0   1   2   3   4   5   6
//! [ ] [ ] ... ... ( ) ... [ ]
//! [S] [S]         [ ]     [ ]
//! [ ]             [ ]     [ ]
//!                 [S]     [S]
//!
//!
//! Picosecond 15:
//!  0   1   2   3   4   5   6
//! [ ] [ ] ... ... [ ] (.) [ ]
//! [S] [S]         [ ]     [ ]
//! [ ]             [ ]     [ ]
//!                 [S]     [S]
//!
//!  0   1   2   3   4   5   6
//! [S] [S] ... ... [ ] (.) [ ]
//! [ ] [ ]         [ ]     [ ]
//! [ ]             [S]     [S]
//!                 [ ]     [ ]
//!
//!
//! Picosecond 16:
//!  0   1   2   3   4   5   6
//! [S] [S] ... ... [ ] ... ( )
//! [ ] [ ]         [ ]     [ ]
//! [ ]             [S]     [S]
//!                 [ ]     [ ]
//!
//!  0   1   2   3   4   5   6
//! [ ] [ ] ... ... [ ] ... ( )
//! [S] [S]         [S]     [S]
//! [ ]             [ ]     [ ]
//!                 [ ]     [ ]
//! ```
//!
//! Because all smaller delays would get you caught, the fewest number of picoseconds you would need to delay to get through safely is 10.
//!
//! What is the fewest number of picoseconds that you need to delay the packet to pass through the firewall without being caught?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
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
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<Vec<String>, u32>(AoCYear::AOC2017, AoCDay::AOCD13, setup, find).map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<String>, u32>(bench, AoCYear::AOC2017, AoCDay::AOCD13, setup, find)
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
fn find(data: Vec<String>) -> u32 {
    find_res(data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(data: Vec<String>, second_star: bool) -> Result<u32> {
    let mut layer_map = HashMap::new();
    for line in data {
        add_layer_to_map(&line, &mut layer_map)?;
    }

    let mut layers: HashMap<usize, Option<u32>> = HashMap::new();
    let maximum_layer = find_maximum_layer(&layer_map)?;
    setup_initial_state(maximum_layer, &layer_map, &mut layers);
    let mut result = 0;

    if second_star {
        for i in 0.. {
            let layers_to_check = layers.clone();
            if traverse_firewall(&layers_to_check, i, true).is_err() {
                continue;
            }
            result = i;
            break;
        }
    } else {
        // Traverse the firewall
        result = traverse_firewall(&layers, 0, false)?;
    }

    Ok(result)
}

/// Add a layer to the layer map.
fn add_layer_to_map(line: &str, layer_map: &mut HashMap<usize, u32>) -> Result<()> {
    let layer_desc_vec: Vec<&str> = line.split(": ").collect();
    let layer = layer_desc_vec
        .first()
        .ok_or(anyhow!("Invalid layer number"))?
        .parse::<usize>()?;
    let depth = layer_desc_vec
        .get(1)
        .ok_or(anyhow!("Invalid depty number"))?
        .parse::<u32>()?;

    let _res = layer_map.insert(layer, depth);

    Ok(())
}

/// Find the maximum layer number.
fn find_maximum_layer(layer_map: &HashMap<usize, u32>) -> Result<usize> {
    let max_layer = layer_map
        .keys()
        .max()
        .ok_or(anyhow!("Unable to find maximum layer"))?;
    Ok(*max_layer)
}

/// Setup the initial state of the layers.
fn setup_initial_state(
    maximum_layer: usize,
    layer_map: &HashMap<usize, u32>,
    layers: &mut HashMap<usize, Option<u32>>,
) {
    for i in 0..=maximum_layer {
        if let Some(depth) = layer_map.get(&i) {
            let _res = layers.insert(i, Some(*depth));
        } else {
            let _res = layers.insert(i, None);
        }
    }
}

/// Traverse the firewall
fn traverse_firewall(
    layers: &HashMap<usize, Option<u32>>,
    delay: u32,
    second_star: bool,
) -> Result<u32> {
    let mut severity = 0;

    // Loop over layers
    for i in 0..layers.len() {
        let curr_layer = layers.get(&i).ok_or(anyhow!("invalid layer"))?;

        // Get the max depth for this layer. It may be `None`, in which case we will never be
        // caught at this level, so skip.
        if let Some(max_depth) = *curr_layer {
            let current_picosecond = u32::try_from(i)?;
            let scan_length = (max_depth - 1) * 2;

            // Each scanner loops back to the 0 level at `((max_depth - 1) * 2)` picoseconds.
            // This means that if `current_picosecond % ((max_depth - 1) * 2) == 0`, then the packet
            // and scanner have met, and the packet is caught.
            if second_star && (current_picosecond + delay) % scan_length == 0 {
                // Uh oh, we got caught.
                return Err(anyhow!("We got caught"));
            } else if !second_star && current_picosecond % scan_length == 0 {
                // Uh oh, we got caught, bump up severity
                severity += current_picosecond * max_depth;
            }
        }
    }

    Ok(severity)
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<Vec<String>, u32>(AoCYear::AOC2017, AoCDay::AOCD13, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<String>, u32>(bench, AoCYear::AOC2017, AoCDay::AOCD13, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: Vec<String>) -> u32 {
    find_res(data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"0: 3
1: 2
4: 4
6: 4";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find(data), 24);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"0: 3
1: 2
4: 4
6: 4";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find(data), 24);
        Ok(())
    }
}
