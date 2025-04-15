// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2017 ---**
//!
//! **--- Day 24: Electromagnetic Moat ---**
//!
//! The CPU itself is a large, black building surrounded by a bottomless pit. Enormous metal tubes extend outward from the side of the building at regular intervals and descend down into the void. There's no way to cross, but you need to get inside.
//!
//! No way, of course, other than building a bridge out of the magnetic components strewn about nearby.
//!
//! Each component has two ports, one on each end. The ports come in all different types, and only matching types can be connected. You take an inventory of the components by their port types (your puzzle input). Each port is identified by the number of pins it uses; more pins mean a stronger connection for your bridge. A 3/7 component, for example, has a type-3 port on one side, and a type-7 port on the other.
//!
//! Your side of the pit is metallic; a perfect surface to connect a magnetic, zero-pin port. Because of this, the first port you use must be of type 0. It doesn't matter what type of port you end with; your goal is just to make the bridge as strong as possible.
//!
//! The strength of a bridge is the sum of the port types in each component. For example, if your bridge is made of components 0/3, 3/7, and 7/4, your bridge has a strength of 0+3 + 3+7 + 7+4 = 24.
//!
//! For example, suppose you had the following components:
//!
//! ```text
//! 0/2
//! 2/2
//! 2/3
//! 3/4
//! 3/5
//! 0/1
//! 10/1
//! 9/10
//! ```
//!
//! With them, you could make the following valid bridges:
//!
//! ```text
//!     0/1
//!     0/1--10/1
//!     0/1--10/1--9/10
//!     0/2
//!     0/2--2/3
//!     0/2--2/3--3/4
//!     0/2--2/3--3/5
//!     0/2--2/2
//!     0/2--2/2--2/3
//!     0/2--2/2--2/3--3/4
//!     0/2--2/2--2/3--3/5
//! ```
//!
//! (Note how, as shown by 10/1, order of ports within a component doesn't matter. However, you may only use each port on a component once.)
//!
//! Of these bridges, the strongest one is 0/1--10/1--9/10; it has a strength of 0+1 + 1+10 + 10+9 = 31.
//!
//! What is the strength of the strongest bridge you can make with the components you have available?
//!
//! **--- Part Two ---**
//!
//! The bridge you've built isn't long enough; you can't jump the rest of the way.
//!
//! In the example above, there are two longest bridges:
//!
//! ```text
//!     0/2--2/2--2/3--3/4
//!     0/2--2/2--2/3--3/5
//! ```
//!
//! Of them, the one which uses the 3/5 component is stronger; its strength is 0+2 + 2+2 + 2+3 + 3+5 = 19.
//!
//! What is the strength of the longest bridge you can make? If you can make multiple bridges of the longest length, pick the strongest one.

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

/// Component
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Component {
    /// left port
    left: usize,
    /// right port
    right: usize,
}

type CompData = HashSet<Component>;

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<CompData, usize>(AoCYear::AOC2017, AoCDay::AOCD24, setup, find).map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<CompData, usize>(bench, AoCYear::AOC2017, AoCDay::AOCD24, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> CompData {
    setup_br(reader).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T) -> Result<CompData>
where
    T: BufRead,
{
    Ok(valid_lines(reader)
        .map(to_component)
        .collect::<HashSet<Component>>())
}

/// Convert a String to a Component
#[allow(clippy::needless_pass_by_value)]
fn to_component(line: String) -> Component {
    let parts = line
        .split('/')
        .map(|s| s.parse::<usize>().expect(""))
        .collect::<Vec<usize>>();

    Component {
        left: parts[0],
        right: parts[1],
    }
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: CompData) -> usize {
    find_res(&data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(all: &CompData, second_star: bool) -> Result<usize> {
    let mut scores = Vec::new();
    next(0, &[], all, &mut scores);

    if second_star {
        let mut max_length = 0;
        let mut ml_scores = Vec::new();
        for (s, l) in scores {
            #[allow(clippy::comparison_chain)]
            if l > max_length {
                ml_scores.clear();
                ml_scores.push((s, l));
                max_length = l;
            } else if l == max_length {
                ml_scores.push((s, l));
            }
        }

        let max = ml_scores
            .iter()
            .map(|&(s, _)| s)
            .max()
            .ok_or(anyhow!("no max"))?;
        Ok(max)
    } else {
        let max = scores
            .iter()
            .map(|&(s, _)| s)
            .max()
            .ok_or(anyhow!("no max"))?;
        Ok(max)
    }
}

/// Find the next component given a start, the current path, and the set of components.
fn next(
    start: usize,
    path: &[Component],
    components: &HashSet<Component>,
    scores: &mut Vec<(usize, usize)>,
) {
    let mut found = false;
    for c in components {
        if c.left == start || c.right == start {
            let mut new_components = components.clone();
            let _ = new_components.remove(c);
            let mut new_path = path.to_owned();
            new_path.push(*c);
            next(
                if c.left == start { c.right } else { c.left },
                &new_path,
                &new_components,
                scores,
            );
            found = true;
        }
    }
    if !found {
        let score = path.iter().map(|c| c.left + c.right).sum::<usize>();
        let length = path.len();
        scores.push((score, length));
    }
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<CompData, usize>(AoCYear::AOC2017, AoCDay::AOCD24, setup, find2).map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<CompData, usize>(bench, AoCYear::AOC2017, AoCDay::AOCD24, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: CompData) -> usize {
    find_res(&data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {}

#[cfg(test)]
mod two_star {}
