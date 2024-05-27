// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Air Duct Spelunking
//!
//! **--- Day 24: Air Duct Spelunking ---**
//!
//! **--- Part 1 ---**
//!
//! You've finally met your match; the doors that provide access to the roof are
//! locked tight, and all of the controls and related electronics are inaccessible.
//! You simply can't reach them.
//!
//! The robot that cleans the air ducts, however, can.
//!
//! It's not a very fast little robot, but you reconfigure it to be able to interface
//! with some of the exposed wires that have been routed through the HVAC system.
//! If you can direct it to each of those locations, you should be able to bypass
//! the security controls.
//!
//! You extract the duct layout for this area from some blueprints you acquired and
//! create a map with the relevant locations marked (your puzzle input). `0` is your
//! current location, from which the cleaning robot embarks; the other numbers are
//! (in no particular order) the locations the robot needs to visit at least once each.
//! Walls are marked as #, and open passages are marked as .. Numbers behave like open
//! passages.
//!
//! For example, suppose you have a map like the following:
//!
//! ```text
//! ###########
//! #0.1.....2#
//! #.#######.#
//! #4.......3#
//! ###########
//! ```
//!
//! To reach all of the points of interest as quickly as possible, you would have
//! the robot take the following path:
//!
//! ```text
//! 0 to 4 (2 steps)
//! 4 to 1 (4 steps; it can't move diagonally)
//! 1 to 2 (6 steps)
//! 2 to 3 (2 steps)
//! ```
//!
//! Since the robot isn't very fast, you need to find it the shortest route. This path
//! is the fewest steps (in the above example, a total of `14`) required to start at `0`
//! and then visit every other location at least once.
//!
//! Given your actual map, and starting from location `0`, what is the fewest number of
//! steps required to visit every non-0 number marked on the map at least once?
//!
//! **--- Part Two ---**
//!
//! Of course, if you leave the cleaning robot somewhere weird, someone is bound to notice.
//!
//! What is the fewest number of steps required to start at `0`, visit every non-0 number
//! marked on the map at least once, and then return to `0`?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{print_err, run_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use itertools::Itertools;
use pathfinding::prelude::astar;
use std::{
    collections::HashMap,
    fmt,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum PosKind {
    Wall,
    Point(usize),
    Space,
}

impl fmt::Display for PosKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PosKind::Wall => "#".to_string(),
                PosKind::Space => ".".to_string(),
                PosKind::Point(val) => val.to_string(),
            }
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos {
    row: usize,
    col: usize,
    kind: PosKind,
}

impl Pos {
    fn is_open(&self) -> bool {
        !matches!(self.kind, PosKind::Wall)
    }

    fn distance(&self, other: &Pos) -> usize {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }

    fn is_point(&self) -> bool {
        matches!(self.kind, PosKind::Point(_))
    }
}

#[derive(Clone, Debug)]
struct Grid {
    points: usize,
    max_row: usize,
    max_col: usize,
    data: Vec<Vec<Pos>>,
}

impl Grid {
    fn successors(&self, pos: &Pos) -> Vec<(Pos, usize)> {
        let &Pos { row, col, kind } = pos;
        let mut successors = vec![];
        if kind == PosKind::Wall {
            vec![]
        } else {
            if col < self.max_col - 1 {
                successors.push((self.data[row][col + 1], 1));
            }

            if row < self.max_row - 1 {
                successors.push((self.data[row + 1][col], 1));
            }

            if col > 0 {
                successors.push((self.data[row][col - 1], 1));
            }

            if row > 0 {
                successors.push((self.data[row - 1][col], 1));
            }
            successors
                .iter()
                .filter(|(pos, _weight)| pos.is_open())
                .copied()
                .collect()
        }
    }

    fn get_point(&self, point: usize) -> Option<Pos> {
        for row in &self.data {
            for pos in row {
                if let PosKind::Point(val) = pos.kind {
                    if val == point {
                        return Some(*pos);
                    }
                }
            }
        }
        None
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "P: {}, MR: {}, MC: {}",
            self.points, self.max_row, self.max_col
        )?;
        let mut buf = String::new();
        for row in &self.data {
            for col in row {
                buf.push_str(&col.kind.to_string());
            }
            writeln!(f, "{buf}")?;
            buf.clear();
        }
        Ok(())
    }
}

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2016, AoCDay::AOCD24, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader).map_err(print_err).unwrap_or_default()
}

fn find_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let grid = setup(reader)?;
    let path_lens = get_path_lengths(&grid);
    find_min_distance(&grid, &path_lens, false)
}

fn setup<T>(reader: T) -> Result<Grid>
where
    T: BufRead,
{
    let mut data = vec![];
    for (row, line) in valid_lines(reader).enumerate() {
        let mut row_vec = vec![];
        for (col, ch) in line.chars().enumerate() {
            match ch {
                '#' => row_vec.push(Pos {
                    row,
                    col,
                    kind: PosKind::Wall,
                }),
                '.' => row_vec.push(Pos {
                    row,
                    col,
                    kind: PosKind::Space,
                }),
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    let digit = ch.to_digit(10).ok_or_else(|| anyhow!("invalid point"))?;
                    let kind = PosKind::Point(usize::try_from(digit)?);
                    row_vec.push(Pos { row, col, kind });
                }
                _ => return Err(anyhow!(format!("invalid input: {ch}"))),
            }
        }
        data.push(row_vec);
    }
    let points = data.iter().flatten().filter(|x| x.is_point()).count();
    let max_col = data[0].len();
    let max_row = data.len();
    Ok(Grid {
        points,
        max_row,
        max_col,
        data,
    })
}

fn get_path_lengths(grid: &Grid) -> HashMap<(usize, usize), usize> {
    let mut path_lens = HashMap::new();
    for point_tuple in (0..grid.points)
        .tuple_combinations()
        .filter_map(|(p_1, p_2)| {
            if let (Some(pos_p_1), Some(pos_p_2)) = (grid.get_point(p_1), grid.get_point(p_2)) {
                Some((pos_p_1, pos_p_2, p_1, p_2))
            } else {
                None
            }
        })
    {
        if let Some((_path, len)) = astar(
            &point_tuple.0,
            |p| grid.successors(p),
            |p| p.distance(&point_tuple.1) / 3,
            |p| *p == point_tuple.1,
        ) {
            _ = path_lens.insert((point_tuple.2, point_tuple.3), len);
            _ = path_lens.insert((point_tuple.3, point_tuple.2), len);
        }
    }

    path_lens
}

fn find_min_distance(
    grid: &Grid,
    path_lens: &HashMap<(usize, usize), usize>,
    part2: bool,
) -> Result<usize> {
    let mut min_distance = usize::MAX;
    for mut points_to_visit in (1..grid.points).permutations(grid.points - 1) {
        points_to_visit.insert(0, 0);

        if part2 {
            points_to_visit.push(0);
        }

        let mut len = 0;
        for pair in points_to_visit.windows(2) {
            let pair_tuple = (pair[0], pair[1]);
            let path_len = path_lens
                .get(&pair_tuple)
                .ok_or_else(|| anyhow!("no len"))?;
            len += *path_len;
        }

        if len < min_distance {
            min_distance = len;
        }
    }

    Ok(min_distance)
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2016, AoCDay::AOCD24, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader).map_err(print_err).unwrap_or_default()
}

fn find2_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let grid = setup(reader)?;
    let path_lens = get_path_lengths(&grid);
    find_min_distance(&grid, &path_lens, true)
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"###########
#0.1.....2#
#.#######.#
#4.......3#
###########";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1))?, 14);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"###########
#0.1.....2#
#.#######.#
#4.......3#
###########";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find2_br(Cursor::new(TEST_1))?, 20);
        Ok(())
    }
}
