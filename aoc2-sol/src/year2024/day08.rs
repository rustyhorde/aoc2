// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 1
//! --- Day 8: Resonant Collinearity ---
//!
//! You find yourselves on the roof of a top-secret Easter Bunny installation.
//!
//! While The Historians do their thing, you take a look at the familiar huge antenna. Much to your surprise, it seems to have been reconfigured to emit a signal that makes people 0.1% more likely to buy Easter Bunny brand Imitation Mediocre Chocolate as a Christmas gift! Unthinkable!
//!
//! Scanning across the city, you find that there are actually many such antennas. Each antenna is tuned to a specific frequency indicated by a single lowercase letter, uppercase letter, or digit. You create a map (your puzzle input) of these antennas. For example:
//!
//! ```text
//! ............
//! ........0...
//! .....0......
//! .......0....
//! ....0.......
//! ......A.....
//! ............
//! ............
//! ........A...
//! .........A..
//! ............
//! ............
//! ```
//!
//! The signal only applies its nefarious effect at specific antinodes based on the resonant frequencies of the antennas. In particular, an antinode occurs at any point that is perfectly in line with two antennas of the same frequency - but only when one of the antennas is twice as far away as the other. This means that for any pair of antennas with the same frequency, there are two antinodes, one on either side of them.
//!
//! So, for these two antennas with frequency a, they create the two antinodes marked with #:
//!
//! ```text
//! ..........
//! ...#......
//! ..........
//! ....a.....
//! ..........
//! .....a....
//! ..........
//! ......#...
//! ..........
//! ..........
//! ```
//!
//! Adding a third antenna with the same frequency creates several more antinodes. It would ideally add four antinodes, but two are off the right side of the map, so instead it adds only two:
//!
//! ```text
//! ..........
//! ...#......
//! #.........
//! ....a.....
//! ........a.
//! .....a....
//! ..#.......
//! ......#...
//! ..........
//! ..........
//! ```
//!
//! Antennas with different frequencies don't create antinodes; A and a count as different frequencies. However, antinodes can occur at locations that contain antennas. In this diagram, the lone antenna with frequency capital A creates no antinodes but has a lowercase-a-frequency antinode at its location:
//!
//! ```text
//! ..........
//! ...#......
//! #.........
//! ....a.....
//! ........a.
//! .....a....
//! ..#.......
//! ......A...
//! ..........
//! ..........
//! ```
//!
//! The first example has antennas with two different frequencies, so the antinodes they create look like this, plus an antinode overlapping the topmost A-frequency antenna:
//!
//! ```text
//! ......#....#
//! ...#....0...
//! ....#0....#.
//! ..#....0....
//! ....0....#..
//! .#....A.....
//! ...#........
//! #......#....
//! ........A...
//! .........A..
//! ..........#.
//! ..........#.
//! ```
//!
//! Because the topmost A-frequency antenna overlaps with a 0-frequency antinode, there are 14 total unique locations that contain an antinode within the bounds of the map.
//!
//! Calculate the impact of the signal. How many unique locations within the bounds of the map contain an antinode?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::Result;
use itertools::Itertools;
use regex::Regex;
use std::{
    collections::{BTreeMap, HashSet},
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
    run_setup_solution::<Vec<Vec<char>>, usize>(AoCYear::AOC2024, AoCDay::AOCD08, setup, find)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<Vec<char>>, usize>(
        bench,
        AoCYear::AOC2024,
        AoCDay::AOCD08,
        setup,
        find,
    )
    .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> Vec<Vec<char>> {
    setup_br(reader).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T) -> Result<Vec<Vec<char>>>
where
    T: BufRead,
{
    let mut matrix = vec![];
    for row in valid_lines(reader) {
        let cols = row.chars().collect::<Vec<char>>();
        matrix.push(cols);
    }
    Ok(matrix)
}

#[allow(clippy::needless_pass_by_value)]
fn find(matrix: Vec<Vec<char>>) -> usize {
    find_res(&matrix).unwrap_or_default()
}

fn find_res(matrix: &[Vec<char>]) -> Result<usize> {
    let mut freq_map: BTreeMap<char, Vec<(usize, usize)>> = BTreeMap::new();
    let tower_re = Regex::new("[A-Za-z0-9]")?;
    for (row, cols) in matrix.iter().enumerate() {
        for (col, next) in cols.iter().enumerate() {
            if tower_re.is_match(&next.to_string()) {
                let _ = freq_map
                    .entry(*next)
                    .and_modify(|locs| locs.push((row, col)))
                    .or_insert(vec![(row, col)]);
            }
        }
    }
    let max_x = isize::try_from(matrix.len())?;
    let max_y = isize::try_from(matrix[0].len())?;
    let mut antinodes = HashSet::new();
    for v in freq_map.values() {
        for pair in v.iter().permutations(2) {
            let p1 = pair[0];
            let p2 = pair[1];
            let (dist_x, dist_y) = dist_x_y(p1, p2)?;
            let antinode = (
                isize::try_from(p1.0)? + dist_x,
                isize::try_from(p1.1)? + dist_y,
            );
            if antinode.0 >= 0 && antinode.1 >= 0 && antinode.0 < max_x && antinode.1 < max_y {
                let _ = antinodes.insert(antinode);
            }
        }
    }
    Ok(antinodes.len())
}

fn dist_x_y(p1: &(usize, usize), p2: &(usize, usize)) -> Result<(isize, isize)> {
    let dist_x = isize::try_from(p1.0)? - isize::try_from(p2.0)?;
    let dist_y = isize::try_from(p1.1)? - isize::try_from(p2.1)?;
    Ok((dist_x, dist_y))
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<Vec<Vec<char>>, usize>(AoCYear::AOC2024, AoCDay::AOCD08, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<Vec<char>>, usize>(
        bench,
        AoCYear::AOC2024,
        AoCDay::AOCD08,
        setup,
        find2,
    )
    .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: Vec<Vec<char>>) -> usize {
    find2_res(&data).unwrap_or_default()
}

fn find2_res(matrix: &[Vec<char>]) -> Result<usize> {
    let mut freq_map: BTreeMap<char, Vec<(usize, usize)>> = BTreeMap::new();
    let tower_re = Regex::new("[A-Za-z0-9]")?;
    for (row, cols) in matrix.iter().enumerate() {
        for (col, next) in cols.iter().enumerate() {
            if tower_re.is_match(&next.to_string()) {
                let _ = freq_map
                    .entry(*next)
                    .and_modify(|locs| locs.push((row, col)))
                    .or_insert(vec![(row, col)]);
            }
        }
    }
    let max_x = isize::try_from(matrix.len())?;
    let max_y = isize::try_from(matrix[0].len())?;
    let mut antinodes = HashSet::new();
    for v in freq_map.values() {
        for pair in v.iter().permutations(2) {
            let p1 = pair[0];
            let p2 = pair[1];
            let (dist_x, dist_y) = dist_x_y(p1, p2)?;

            let _ = antinodes.insert((isize::try_from(p1.0)?, isize::try_from(p1.1)?));
            let mut antinode = (
                isize::try_from(p1.0)? + dist_x,
                isize::try_from(p1.1)? + dist_y,
            );
            while antinode.0 >= 0 && antinode.1 >= 0 && antinode.0 < max_x && antinode.1 < max_y {
                let _ = antinodes.insert(antinode);
                antinode = (antinode.0 + dist_x, antinode.1 + dist_y);
            }
        }
    }
    Ok(antinodes.len())
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find(data), 14);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
..........";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find2(data), 9);
        Ok(())
    }
}
