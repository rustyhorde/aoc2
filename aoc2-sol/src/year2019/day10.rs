// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2019 ---**
//!
//! --- Day 10: Monitoring Station ---
//!  
//!  You fly into the asteroid belt and reach the Ceres monitoring station. The Elves here have an emergency: they're having trouble tracking all of the asteroids and can't be sure they're safe.
//!  
//!  The Elves would like to build a new monitoring station in a nearby area of space; they hand you a map of all of the asteroids in that region (your puzzle input).
//!  
//!  The map indicates whether each position is empty (.) or contains an asteroid (#). The asteroids are much smaller than they appear on the map, and every asteroid is exactly in the center of its marked position. The asteroids can be described with X,Y coordinates where X is the distance from the left edge and Y is the distance from the top edge (so the top-left corner is 0,0 and the position immediately to its right is 1,0).
//!  
//!  Your job is to figure out which asteroid would be the best place to build a new monitoring station. A monitoring station can detect any asteroid to which it has direct line of sight - that is, there cannot be another asteroid exactly between them. This line of sight can be at any angle, not just lines aligned to the grid or diagonally. The best location is the asteroid that can detect the largest number of other asteroids.
//!  
//!  For example, consider the following map:
//!  
//!  ```text
//!  .#..#
//!  .....
//!  #####
//!  ....#
//!  ...##
//!  ```
//!
//!  The best location for a new monitoring station on this map is the highlighted asteroid at 3,4 because it can detect 8 asteroids, more than any other location. (The only asteroid it cannot detect is the one at 1,0; its view of this asteroid is blocked by the asteroid at 2,2.) All other asteroids are worse locations; they can detect 7 or fewer other asteroids. Here is the number of other asteroids a monitoring station on each asteroid could detect:
//!
//!  ```text
//!  .7..7
//!  .....
//!  67775
//!  ....7
//!  ...87
//!  ```
//!
//!  Here is an asteroid (#) and some examples of the ways its line of sight might be blocked. If there were another asteroid at the location of a capital letter, the locations marked with the corresponding lowercase letter would be blocked and could not be detected:
//!
//!  ```text
//!  #.........
//!  ...A......
//!  ...B..a...
//!  .EDCG....a
//!  ..F.c.b...
//!  .....c....
//!  ..efd.c.gb
//!  .......c..
//!  ....f...c.
//!  ...e..d..c
//!  ```
//!
//!  Here are some larger examples:
//!
//!  ```text
//!      Best is 5,8 with 33 other asteroids detected:
//!  
//!      ......#.#.
//!      #..#.#....
//!      ..#######.
//!      .#.#.###..
//!      .#..#.....
//!      ..#....#.#
//!      #..#....#.
//!      .##.#..###
//!      ##...#..#.
//!      .#....####
//!  
//!      Best is 1,2 with 35 other asteroids detected:
//!  
//!      #.#...#.#.
//!      .###....#.
//!      .#....#...
//!      ##.#.#.#.#
//!      ....#.#.#.
//!      .##..###.#
//!      ..#...##..
//!      ..##....##
//!      ......#...
//!      .####.###.
//!  
//!      Best is 6,3 with 41 other asteroids detected:
//!  
//!      .#..#..###
//!      ####.###.#
//!      ....###.#.
//!      ..###.##.#
//!      ##.##.#.#.
//!      ....###..#
//!      ..#.#..#.#
//!      #..#.#.###
//!      .##...##.#
//!      .....#.#..
//!  
//!      Best is 11,13 with 210 other asteroids detected:
//!  
//!      .#..##.###...#######
//!      ##.############..##.
//!      .#.######.########.#
//!      .###.#######.####.#.
//!      #####.##.#.##.###.##
//!      ..#####..#.#########
//!      ####################
//!      #.####....###.#.#.##
//!      ##.#################
//!      #####.##.###..####..
//!      ..######..##.#######
//!      ####.##.####...##..#
//!      .#####..#.######.###
//!      ##...#.##########...
//!      #.##########.#######
//!      .####.#.###.###.#.##
//!      ....##.##.###..#####
//!      .#.#.###########.###
//!      #.#.#.#####.####.###
//!      ###.##.####.##.#..##
//!  ```
//!
//!  Find the best location for a new monitoring station. How many other asteroids can be detected from that location?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

type AsteroidData = Vec<Vec<char>>;

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](AoCYear) and
///   [`AoCDay`](AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<AsteroidData, usize>(AoCYear::AOC2019, AoCDay::AOCD10, setup, find)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<AsteroidData, usize>(bench, AoCYear::AOC2019, AoCDay::AOCD10, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> AsteroidData {
    setup_br(reader).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T) -> Result<AsteroidData>
where
    T: BufRead,
{
    let mut data = vec![];
    for line in valid_lines(reader) {
        let mut row = vec![];
        for ch in line.chars() {
            row.push(ch);
        }
        data.push(row);
    }
    Ok(data)
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: AsteroidData) -> usize {
    find_res(data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(_data: AsteroidData, _second_star: bool) -> Result<usize> {
    Ok(0)
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](AoCYear) and
///   [`AoCDay`](AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<AsteroidData, usize>(AoCYear::AOC2019, AoCDay::AOCD10, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<AsteroidData, usize>(bench, AoCYear::AOC2019, AoCDay::AOCD10, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: AsteroidData) -> usize {
    find_res(data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r".#..#
.....
#####
....#
...##";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find(data), 0);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r">";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find2(data), 0);
        Ok(())
    }
}
