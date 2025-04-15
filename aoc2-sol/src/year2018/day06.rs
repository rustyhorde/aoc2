// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2018 ---**
//!
//! **--- Day 6: Chronal Coordinates ---**
//!
//! The device on your wrist beeps several times, and once again you feel like you're falling.
//!
//! "Situation critical," the device announces. "Destination indeterminate. Chronal interference detected. Please specify new target coordinates."
//!
//! The device then produces a list of coordinates (your puzzle input). Are they places it thinks are safe or dangerous? It recommends you check manual page 729. The Elves did not give you a manual.
//!
//! If they're dangerous, maybe you can minimize the danger by finding the coordinate that gives the largest distance from the other points.
//!
//! Using only the Manhattan distance, determine the area around each coordinate by counting the number of integer X,Y locations that are closest to that coordinate (and aren't tied in distance to any other coordinate).
//!
//! Your goal is to find the size of the largest area that isn't infinite. For example, consider the following list of coordinates:
//!
//! ```text
//! 1, 1
//! 1, 6
//! 8, 3
//! 3, 4
//! 5, 5
//! 8, 9
//! ```
//!
//! If we name these coordinates A through F, we can draw them on a grid, putting 0,0 at the top left:
//!
//! ```text
//! ..........
//! .A........
//! ..........
//! ........C.
//! ...D......
//! .....E....
//! .B........
//! ..........
//! ..........
//! ........F.
//! ```
//!
//! This view is partial - the actual grid extends infinitely in all directions. Using the Manhattan distance, each location's closest coordinate can be determined, shown here in lowercase:
//!
//! ```text
//! aaaaa.cccc
//! aAaaa.cccc
//! aaaddecccc
//! aadddeccCc
//! ..dDdeeccc
//! bb.deEeecc
//! bBb.eeee..
//! bbb.eeefff
//! bbb.eeffff
//! bbb.ffffFf
//! ```
//!
//! Locations shown as . are equally far from two or more coordinates, and so they don't count as being closest to any.
//!
//! In this example, the areas of coordinates A, B, C, and F are infinite - while not shown here, their areas extend forever outside the visible grid. However, the areas of coordinates D and E are finite: D is closest to 9 locations, and E is closest to 17 (both including the coordinate's location itself). Therefore, in this example, the size of the largest area is 17.
//!
//! What is the size of the largest area that isn't infinite?
//!
//! **--- Part Two ---**
//!
//! On the other hand, if the coordinates are safe, maybe the best you can do is try to find a region near as many coordinates as possible.
//!
//! For example, suppose you want the sum of the Manhattan distance to all of the coordinates to be less than 32. For each location, add up the distances to all of the given coordinates; if the total of those distances is less than 32, that location is within the desired region. Using the same coordinates as above, the resulting region looks like this:
//!
//! ```text
//! ..........
//! .A........
//! ..........
//! ...###..C.
//! ..#D###...
//! ..###E#...
//! .B.###....
//! ..........
//! ..........
//! ........F.
//! ```
//!
//! In particular, consider the highlighted location 4,3 located at the top middle of the region. Its calculation is as follows, where `abs()` is the absolute value function:
//!
//! ```text
//!     Distance to coordinate A: abs(4-1) + abs(3-1) =  5
//!     Distance to coordinate B: abs(4-1) + abs(3-6) =  6
//!     Distance to coordinate C: abs(4-8) + abs(3-3) =  4
//!     Distance to coordinate D: abs(4-3) + abs(3-4) =  2
//!     Distance to coordinate E: abs(4-5) + abs(3-5) =  3
//!     Distance to coordinate F: abs(4-8) + abs(3-9) = 10
//!     Total distance: 5 + 6 + 4 + 2 + 3 + 10 = 30
//! ```
//!
//! Because the total distance to all coordinates (30) is less than 32, the location is within the region.
//!
//! This region, which also includes coordinates D and E, has a total size of 16.
//!
//! Your actual region will need to be much larger than this example, though, instead including all locations with a total distance of less than 10000.
//!
//! What is the size of the region containing all locations which have a total distance to all given coordinates of less than 10000?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use indexmap::IndexSet;
use regex::Regex;
use std::{
    collections::{BTreeMap, HashMap},
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
    run_setup_solution::<Vec<(i32, i32)>, u32>(AoCYear::AOC2018, AoCDay::AOCD06, setup, find)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<(i32, i32)>, u32>(bench, AoCYear::AOC2018, AoCDay::AOCD06, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> Vec<(i32, i32)> {
    setup_br(reader).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T) -> Result<Vec<(i32, i32)>>
where
    T: BufRead,
{
    let line_re = Regex::new(r"(\d+), (\d+)")?;
    let mut coords: Vec<(i32, i32)> = Vec::new();

    for line in valid_lines(reader) {
        for cap in line_re.captures_iter(&line) {
            let x = &cap[1].parse::<i32>()?;
            let y = &cap[2].parse::<i32>()?;
            coords.push((*x, *y));
        }
    }
    Ok(coords)
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: Vec<(i32, i32)>) -> u32 {
    find_res(&data, false).unwrap_or_default()
}

fn find_res(coords: &[(i32, i32)], second_star: bool) -> Result<u32> {
    let (max_x, max_y) = max_coords(coords);

    if second_star {
        let d_to_check = if coords.len() == 6 { 32 } else { 10000 };
        let mut less_than_d = 0;
        for y in 0..=max_y {
            for x in 0..=max_x {
                let total_of_mds = total_of_mds((x, y), coords)?;

                if total_of_mds < d_to_check {
                    less_than_d += 1;
                }
            }
        }
        Ok(less_than_d)
    } else {
        let mut md_map = BTreeMap::new();
        let mut on_boundary = IndexSet::new();

        for y in 0..=max_y {
            for x in 0..=max_x {
                let closest = find_closest((x, y), coords);

                if closest.len() == 1 {
                    let _ = md_map.insert((x, y), closest[0]);

                    if x == 0 || y == 0 || x == max_x || y == max_y {
                        let _ = on_boundary.insert(closest[0]);
                    }
                }
            }
        }

        let rest: BTreeMap<(i32, i32), (i32, i32)> = md_map
            .iter()
            .filter(|(_, closest)| !on_boundary.contains(*closest))
            .map(|(x, v)| (*x, *v))
            .collect();

        let mut frequency: HashMap<(i32, i32), u32> = HashMap::new();
        for (_, bounded_closest) in rest {
            *frequency.entry(bounded_closest).or_insert(0) += 1;
        }

        let max = frequency
            .iter()
            .max_by_key(|(_, x)| *x)
            .map(|(_, x)| *x)
            .ok_or(anyhow!("no maximum"))?;
        Ok(max)
    }
}

fn find_closest(point: (i32, i32), coords: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let distances: HashMap<usize, i32> = coords
        .iter()
        .enumerate()
        .map(|(idx, coord)| (idx, manhattan_distance(point, *coord)))
        .collect();
    let mut min = i32::MAX;
    let mut result = Vec::new();
    for (idx, distance) in distances {
        #[allow(clippy::comparison_chain)]
        if distance < min {
            result.clear();
            result.push(coords[idx]);
            min = distance;
        } else if distance == min {
            result.push(coords[idx]);
        }
    }
    result
}

fn max_coords(coords: &[(i32, i32)]) -> (i32, i32) {
    let max_x = coords.iter().max_by_key(|(x, _)| x).unwrap_or(&(0, 0)).0;
    let max_y = coords.iter().max_by_key(|(_, y)| y).unwrap_or(&(0, 0)).1;
    (max_x, max_y)
}

fn total_of_mds(point: (i32, i32), coords: &[(i32, i32)]) -> Result<u32> {
    let sum: i32 = coords
        .iter()
        .map(|coord| manhattan_distance(point, *coord))
        .sum();
    u32::try_from(sum).map_err(|e| anyhow!("{}", e))
}

/// Calculate the manhattan distance between two (x,y) tuples.
fn manhattan_distance(from: (i32, i32), to: (i32, i32)) -> i32 {
    (from.0 - to.0).abs() + (from.1 - to.1).abs()
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<Vec<(i32, i32)>, u32>(AoCYear::AOC2018, AoCDay::AOCD06, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<(i32, i32)>, u32>(
        bench,
        AoCYear::AOC2018,
        AoCDay::AOCD06,
        setup,
        find2,
    )
    .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: Vec<(i32, i32)>) -> u32 {
    find_res(&data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find(data), 17);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find2(data), 16);
        Ok(())
    }
}
