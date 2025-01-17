// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2024 ---**
//!
//! **--- Day 20: Race Condition ---**
//!
//! The Historians are quite pixelated again. This time, a massive, black building looms over you - you're right outside the CPU!
//!
//! While The Historians get to work, a nearby program sees that you're idle and challenges you to a race. Apparently, you've arrived just in time for the frequently-held race condition festival!
//!
//! The race takes place on a particularly long and twisting code path; programs compete to see who can finish in the fewest picoseconds. The winner even gets their very own mutex!
//!
//! They hand you a map of the racetrack (your puzzle input). For example:
//!
//! ```text
//! ###############
//! #...#...#.....#
//! #.#.#.#.#.###.#
//! #S#...#.#.#...#
//! #######.#.#.###
//! #######.#.#...#
//! #######.#.###.#
//! ###..E#...#...#
//! ###.#######.###
//! #...###...#...#
//! #.#####.#.###.#
//! #.#...#.#.#...#
//! #.#.#.#.#.#.###
//! #...#...#...###
//! ###############
//! ```
//!
//! The map consists of track (.) - including the start (S) and end (E) positions (both of which also count as track) - and walls (#).
//!
//! When a program runs through the racetrack, it starts at the start position. Then, it is allowed to move up, down, left, or right; each such move takes 1 picosecond. The goal is to reach the end position as quickly as possible. In this example racetrack, the fastest time is 84 picoseconds.
//!
//! Because there is only a single path from the start to the end and the programs all go the same speed, the races used to be pretty boring. To make things more interesting, they introduced a new rule to the races: programs are allowed to cheat.
//!
//! The rules for cheating are very strict. Exactly once during a race, a program may disable collision for up to 2 picoseconds. This allows the program to pass through walls as if they were regular track. At the end of the cheat, the program must be back on normal track again; otherwise, it will receive a segmentation fault and get disqualified.
//!
//! So, a program could complete the course in 72 picoseconds (saving 12 picoseconds) by cheating for the two moves marked 1 and 2:
//!
//! ```text
//! ###############
//! #...#...12....#
//! #.#.#.#.#.###.#
//! #S#...#.#.#...#
//! #######.#.#.###
//! #######.#.#...#
//! #######.#.###.#
//! ###..E#...#...#
//! ###.#######.###
//! #...###...#...#
//! #.#####.#.###.#
//! #.#...#.#.#...#
//! #.#.#.#.#.#.###
//! #...#...#...###
//! ###############
//! ```
//!
//! Or, a program could complete the course in 64 picoseconds (saving 20 picoseconds) by cheating for the two moves marked 1 and 2:
//!
//! ```text
//! ###############
//! #...#...#.....#
//! #.#.#.#.#.###.#
//! #S#...#.#.#...#
//! #######.#.#.###
//! #######.#.#...#
//! #######.#.###.#
//! ###..E#...12..#
//! ###.#######.###
//! #...###...#...#
//! #.#####.#.###.#
//! #.#...#.#.#...#
//! #.#.#.#.#.#.###
//! #...#...#...###
//! ###############
//! ```
//!
//! This cheat saves 38 picoseconds:
//!
//! ```text
//! ###############
//! #...#...#.....#
//! #.#.#.#.#.###.#
//! #S#...#.#.#...#
//! #######.#.#.###
//! #######.#.#...#
//! #######.#.###.#
//! ###..E#...#...#
//! ###.####1##.###
//! #...###.2.#...#
//! #.#####.#.###.#
//! #.#...#.#.#...#
//! #.#.#.#.#.#.###
//! #...#...#...###
//! ###############
//! ```
//!
//! This cheat saves 64 picoseconds and takes the program directly to the end:
//!
//! ```text
//! ###############
//! #...#...#.....#
//! #.#.#.#.#.###.#
//! #S#...#.#.#...#
//! #######.#.#.###
//! #######.#.#...#
//! #######.#.###.#
//! ###..21...#...#
//! ###.#######.###
//! #...###...#...#
//! #.#####.#.###.#
//! #.#...#.#.#...#
//! #.#.#.#.#.#.###
//! #...#...#...###
//! ###############
//! ```
//!
//! Each cheat has a distinct start position (the position where the cheat is activated, just before the first move that is allowed to go through walls) and end position; cheats are uniquely identified by their start position and end position.
//!
//! In this example, the total number of cheats (grouped by the amount of time they save) are as follows:
//!
//! ```text
//!     There are 14 cheats that save 2 picoseconds.
//!     There are 14 cheats that save 4 picoseconds.
//!     There are 2 cheats that save 6 picoseconds.
//!     There are 4 cheats that save 8 picoseconds.
//!     There are 2 cheats that save 10 picoseconds.
//!     There are 3 cheats that save 12 picoseconds.
//!     There is one cheat that saves 20 picoseconds.
//!     There is one cheat that saves 36 picoseconds.
//!     There is one cheat that saves 38 picoseconds.
//!     There is one cheat that saves 40 picoseconds.
//!     There is one cheat that saves 64 picoseconds.
//! ```
//!
//! You aren't sure what the conditions of the racetrack will be like, so to give yourself as many options as possible, you'll need a list of the best cheats. How many cheats would save you at least 100 picoseconds?
//!
//! --- Part Two ---
//!
//! The programs seem perplexed by your list of cheats. Apparently, the two-picosecond cheating rule was deprecated several milliseconds ago! The latest version of the cheating rule permits a single cheat that instead lasts at most 20 picoseconds.
//!
//! Now, in addition to all the cheats that were possible in just two picoseconds, many more cheats are possible. This six-picosecond cheat saves 76 picoseconds:
//!
//! ```text
//! ###############
//! #...#...#.....#
//! #.#.#.#.#.###.#
//! #S#...#.#.#...#
//! #1#####.#.#.###
//! #2#####.#.#...#
//! #3#####.#.###.#
//! #456.E#...#...#
//! ###.#######.###
//! #...###...#...#
//! #.#####.#.###.#
//! #.#...#.#.#...#
//! #.#.#.#.#.#.###
//! #...#...#...###
//! ###############
//! ```
//!
//! Because this cheat has the same start and end positions as the one above, it's the same cheat, even though the path taken during the cheat is different:
//!
//! ```text
//! ###############
//! #...#...#.....#
//! #.#.#.#.#.###.#
//! #S12..#.#.#...#
//! ###3###.#.#.###
//! ###4###.#.#...#
//! ###5###.#.###.#
//! ###6.E#...#...#
//! ###.#######.###
//! #...###...#...#
//! #.#####.#.###.#
//! #.#...#.#.#...#
//! #.#.#.#.#.#.###
//! #...#...#...###
//! ###############
//! ```
//!
//! Cheats don't need to use all 20 picoseconds; cheats can last any amount of time up to and including 20 picoseconds (but can still only end when the program is on normal track). Any cheat time not used is lost; it can't be saved for another cheat later. If cheat mode is active when the end position is reached, cheat mode ends automatically.
//!
//! You'll still need a list of the best cheats, but now there are even more to choose between. Here are the quantities of cheats in this example that save 50 picoseconds or more:
//!
//! ```text
//!     There are 32 cheats that save 50 picoseconds.
//!     There are 31 cheats that save 52 picoseconds.
//!     There are 29 cheats that save 54 picoseconds.
//!     There are 39 cheats that save 56 picoseconds.
//!     There are 25 cheats that save 58 picoseconds.
//!     There are 23 cheats that save 60 picoseconds.
//!     There are 20 cheats that save 62 picoseconds.
//!     There are 19 cheats that save 64 picoseconds.
//!     There are 12 cheats that save 66 picoseconds.
//!     There are 14 cheats that save 68 picoseconds.
//!     There are 12 cheats that save 70 picoseconds.
//!     There are 22 cheats that save 72 picoseconds.
//!     There are 4 cheats that save 74 picoseconds.
//!     There are 3 cheats that save 76 picoseconds.
//! ```
//!
//! Find the best cheats using the updated cheating rules. How many cheats would save you at least 100 picoseconds?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use ndarray::Array2;
use pathfinding::prelude::dfs;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

type MazeData = (Vec<Vec<char>>, usize);

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<MazeData, usize>(AoCYear::AOC2024, AoCDay::AOCD20, setup, find).map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<MazeData, usize>(bench, AoCYear::AOC2024, AoCDay::AOCD20, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> MazeData {
    setup_br(reader, 100).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T, least_save: usize) -> Result<MazeData>
where
    T: BufRead,
{
    let mut data = vec![];
    for line in valid_lines(reader) {
        data.push(line.chars().collect());
    }
    Ok((data, least_save))
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: MazeData) -> usize {
    match find_res(data, false) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("{e}");
            0
        }
    }
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(data: MazeData, second_star: bool) -> Result<usize> {
    let (data, least_save) = data;
    let max_x = data[0].len();
    let max_y = data.len();
    let mut maze = Array2::<bool>::default((max_x, max_y));
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (y, row) in data.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            match cell {
                '#' => {}
                '.' => {
                    maze[[x, y]] = true;
                }
                'S' => {
                    start = (x, y);
                    maze[[x, y]] = true;
                }
                'E' => {
                    end = (x, y);
                    maze[[x, y]] = true;
                }
                _ => return Err(anyhow!("Invalid cell")),
            }
        }
    }

    let dist = if second_star { 20 } else { 2 };
    let locs = dfs(start, |loc| succ(*loc, &maze), |node| *node == end).unwrap_or_default();

    let count = locs
        .iter()
        .enumerate()
        .filter_map(|(from_pos, from_loc)| {
            locs.iter()
                .enumerate()
                .try_fold(0, |acc, (to_pos, to_loc)| -> Result<usize> {
                    let mut n_acc = acc;
                    if to_pos > from_pos {
                        let md = manhattan_distance(*from_loc, *to_loc)?;
                        if md > 1
                            && md <= dist
                            && to_pos - from_pos > md
                            && to_pos - from_pos - md >= least_save
                        {
                            n_acc += 1;
                        }
                    }
                    Ok(n_acc)
                })
                .ok()
        })
        .sum();

    Ok(count)
}

fn succ(loc: (usize, usize), maze: &Array2<bool>) -> Vec<(usize, usize)> {
    let mut succ = vec![];
    let (x, y) = loc;
    if x > 0 && maze_at_loc((x - 1, y), maze) {
        succ.push((x - 1, y));
    }
    if y > 0 && maze_at_loc((x, y - 1), maze) {
        succ.push((x, y - 1));
    }
    if maze_at_loc((x + 1, y), maze) {
        succ.push((x + 1, y));
    }
    if maze_at_loc((x, y + 1), maze) {
        succ.push((x, y + 1));
    }
    succ
}

fn maze_at_loc(loc: (usize, usize), maze: &Array2<bool>) -> bool {
    maze.get(loc).copied().unwrap_or_default()
}

/// Calculate the manhattan distance between two (x,y) tuples.
fn manhattan_distance(from: (usize, usize), to: (usize, usize)) -> Result<usize> {
    let from_x = isize::try_from(from.0)?;
    let from_y = isize::try_from(from.1)?;
    let to_x = isize::try_from(to.0)?;
    let to_y = isize::try_from(to.1)?;

    Ok(usize::try_from((from_x - to_x).abs())? + usize::try_from((from_y - to_y).abs())?)
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<MazeData, usize>(AoCYear::AOC2024, AoCDay::AOCD20, setup, find2).map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<MazeData, usize>(bench, AoCYear::AOC2024, AoCDay::AOCD20, setup, find2)
        .map(|_| 0)
}

// For part two find all the paths through walls that are up to 19 long between any two empty spots.  Add to removes list and check.

#[allow(clippy::needless_pass_by_value)]
fn find2(data: MazeData) -> usize {
    find_res(data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1), 20)?;
        assert_eq!(find(data), 5);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1), 50)?;
        assert_eq!(find2(data), 285);
        Ok(())
    }
}
