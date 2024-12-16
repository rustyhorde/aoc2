// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2024 ---**
//!
//! **--- Day 16: Reindeer Maze ---**
//!
//! It's time again for the Reindeer Olympics! This year, the big event is the Reindeer Maze, where the Reindeer compete for the lowest score.
//!
//! You and The Historians arrive to search for the Chief right as the event is about to start. It wouldn't hurt to watch a little, right?
//!
//! The Reindeer start on the Start Tile (marked S) facing East and need to reach the End Tile (marked E). They can move forward one tile at a time (increasing their score by 1 point), but never into a wall (#). They can also rotate clockwise or counterclockwise 90 degrees at a time (increasing their score by 1000 points).
//!
//! To figure out the best place to sit, you start by grabbing a map (your puzzle input) from a nearby kiosk. For example:
//!
//! ```text
//! ###############
//! #.......#....E#
//! #.#.###.#.###.#
//! #.....#.#...#.#
//! #.###.#####.#.#
//! #.#.#.......#.#
//! #.#.#####.###.#
//! #...........#.#
//! ###.#.#####.#.#
//! #...#.....#.#.#
//! #.#.#.###.#.#.#
//! #.....#...#.#.#
//! #.###.#.#.#.#.#
//! #S..#.....#...#
//! ###############
//! ```
//!
//! There are many paths through this maze, but taking any of the best paths would incur a score of only 7036. This can be achieved by taking a total of 36 steps forward and turning 90 degrees a total of 7 times:
//!
//! ```text
//! ###############
//! #.......#....E#
//! #.#.###.#.###^#
//! #.....#.#...#^#
//! #.###.#####.#^#
//! #.#.#.......#^#
//! #.#.#####.###^#
//! #..>>>>>>>>v#^#
//! ###^#.#####v#^#
//! #>>^#.....#v#^#
//! #^#.#.###.#v#^#
//! #^....#...#v#^#
//! #^###.#.#.#v#^#
//! #S..#.....#>>^#
//! ###############
//! ```
//!
//! Here's a second example:
//!
//! ```text
//! #################
//! #...#...#...#..E#
//! #.#.#.#.#.#.#.#.#
//! #.#.#.#...#...#.#
//! #.#.#.#.###.#.#.#
//! #...#.#.#.....#.#
//! #.#.#.#.#.#####.#
//! #.#...#.#.#.....#
//! #.#.#####.#.###.#
//! #.#.#.......#...#
//! #.#.###.#####.###
//! #.#.#...#.....#.#
//! #.#.#.#####.###.#
//! #.#.#.........#.#
//! #.#.#.#########.#
//! #S#.............#
//! #################
//! ```
//!
//! In this maze, the best paths cost 11048 points; following one such path would look like this:
//!
//! ```text
//! #################
//! #...#...#...#..E#
//! #.#.#.#.#.#.#.#^#
//! #.#.#.#...#...#^#
//! #.#.#.#.###.#.#^#
//! #>>v#.#.#.....#^#
//! #^#v#.#.#.#####^#
//! #^#v..#.#.#>>>>^#
//! #^#v#####.#^###.#
//! #^#v#..>>>>^#...#
//! #^#v###^#####.###
//! #^#v#>>^#.....#.#
//! #^#v#^#####.###.#
//! #^#v#^........#.#
//! #^#v#^#########.#
//! #S#>>^..........#
//! #################
//! ```
//!
//! Note that the path shown above includes one 90 degree turn as the very first move, rotating the Reindeer from facing East to facing North.
//!
//! Analyze your map carefully. What is the lowest score a Reindeer could possibly get?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use ndarray::Array2;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

type MazeData = Vec<String>;

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<MazeData, usize>(AoCYear::AOC2024, AoCDay::AOCD16, setup, find).map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<MazeData, usize>(bench, AoCYear::AOC2024, AoCDay::AOCD16, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> MazeData {
    setup_br(reader).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T) -> Result<MazeData>
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
fn find(data: MazeData) -> usize {
    find_res(&data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(data: &MazeData, _second_star: bool) -> Result<usize> {
    let max_x = data[0].len();
    let max_y = data.len();
    let mut maze_data = Array2::<bool>::default((max_x, max_y));
    let mut visited = Array2::<bool>::default((max_x, max_y));
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut min_dist = usize::MAX;

    for (y, line) in data.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                // maze data is already false
                '#' => {}
                '.' => maze_data[[x, y]] = true,
                'S' => {
                    start = (x, y);
                    maze_data[[x, y]] = true;
                }
                'E' => {
                    end = (x, y);
                    maze_data[[x, y]] = true;
                }
                _ => return Err(anyhow!("invalid maze data: '{ch}'")),
            }
        }
    }

    find_shortest_path(
        &maze_data,
        &mut visited,
        start,
        end,
        &mut min_dist,
        0,
        Direction::East,
    );
    Ok(min_dist)
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    West,
    East,
    North,
    South,
}

fn find_shortest_path(
    maze: &Array2<bool>,
    visited: &mut Array2<bool>,
    start: (usize, usize),
    end: (usize, usize),
    min_dist: &mut usize,
    dist: usize,
    my_dir: Direction,
) {
    if start == end {
        *min_dist = dist.min(*min_dist);
    } else {
        visited[start] = true;

        // visit to the east
        if let Some(next_start) = is_valid_path(maze, visited, start, Direction::East) {
            let next_dist = add_points(my_dir, Direction::East) + dist + 1;
            find_shortest_path(
                maze,
                visited,
                next_start,
                end,
                min_dist,
                next_dist,
                Direction::East,
            );
        }
        // visit to the left
        if let Some(next_start) = is_valid_path(maze, visited, start, Direction::West) {
            let next_dist = add_points(my_dir, Direction::West) + dist + 1;
            find_shortest_path(
                maze,
                visited,
                next_start,
                end,
                min_dist,
                next_dist,
                Direction::West,
            );
        }
        // visit up
        if let Some(next_start) = is_valid_path(maze, visited, start, Direction::North) {
            let next_dist = add_points(my_dir, Direction::North) + dist + 1;
            find_shortest_path(
                maze,
                visited,
                next_start,
                end,
                min_dist,
                next_dist,
                Direction::North,
            );
        }
        // visit down
        if let Some(next_start) = is_valid_path(maze, visited, start, Direction::South) {
            let next_dist = add_points(my_dir, Direction::South) + dist + 1;
            find_shortest_path(
                maze,
                visited,
                next_start,
                end,
                min_dist,
                next_dist,
                Direction::South,
            );
        }

        visited[start] = false;
    }
}

fn add_points(curr_dir: Direction, next_dir: Direction) -> usize {
    match curr_dir {
        Direction::West => match next_dir {
            Direction::West => 0,
            Direction::East => 2000,
            Direction::North | Direction::South => 1000,
        },
        Direction::East => match next_dir {
            Direction::West => 2000,
            Direction::East => 0,
            Direction::North | Direction::South => 1000,
        },
        Direction::North => match next_dir {
            Direction::West | Direction::East => 1000,
            Direction::North => 0,
            Direction::South => 2000,
        },
        Direction::South => match next_dir {
            Direction::West | Direction::East => 1000,
            Direction::North => 2000,
            Direction::South => 0,
        },
    }
}

fn is_valid_path(
    maze: &Array2<bool>,
    visited: &mut Array2<bool>,
    loc: (usize, usize),
    direction: Direction,
) -> Option<(usize, usize)> {
    let mut next_loc = None;
    let next_opt = match direction {
        Direction::West => loc.0.checked_sub(1).map(|x| (x, loc.1)),
        Direction::East => Some((loc.0 + 1, loc.1)),
        Direction::North => loc.1.checked_sub(1).map(|x| (loc.0, x)),
        Direction::South => Some((loc.0, loc.1 + 1)),
    };

    if let Some((next_x, next_y)) = next_opt {
        if let Some(safe) = maze.get((next_x, next_y)) {
            if let Some(visited) = visited.get((next_x, next_y)) {
                if *safe && !visited {
                    next_loc = Some((next_x, next_y));
                }
            }
        }
    }
    next_loc
}
/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<MazeData, usize>(AoCYear::AOC2024, AoCDay::AOCD16, setup, find2).map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<MazeData, usize>(bench, AoCYear::AOC2024, AoCDay::AOCD16, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: MazeData) -> usize {
    find_res(&data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const TEST_2: &str = r"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

const TEST_3: &str = r"#######
#....E#
#.#####
#.#...#
#...#.#
#S#...#
#######";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find(data), 7036);
        let data = setup_br(Cursor::new(TEST_2))?;
        assert_eq!(find(data), 11048);
        let data = setup_br(Cursor::new(TEST_3))?;
        assert_eq!(find(data), 2008);
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
