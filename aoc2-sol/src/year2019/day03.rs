// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 3
//! --- Day 3: Crossed Wires ---
//!
//! The gravity assist was successful, and you're well on your way to the Venus refuelling station. During the rush back on Earth, the fuel management system wasn't completely installed, so that's next on the priority list.
//!
//! Opening the front panel reveals a jumble of wires. Specifically, two wires are connected to a central port and extend outward on a grid. You trace the path each wire takes as it leaves the central port, one wire per line of text (your puzzle input).
//!
//! The wires twist and turn, but the two wires occasionally cross paths. To fix the circuit, you need to find the intersection point closest to the central port. Because the wires are on a grid, use the Manhattan distance for this measurement. While the wires do technically cross right at the central port where they both start, this point does not count, nor does a wire count as crossing with itself.
//!
//! For example, if the first wire's path is R8,U5,L5,D3, then starting from the central port (o), it goes right 8, up 5, left 5, and finally down 3:
//!
//! > ...........
//! > ...........
//! > ...........
//! > ....+----+.
//! > ....|....|.
//! > ....|....|.
//! > ....|....|.
//! > .........|.
//! > .o-------+.
//! > ...........
//!
//! Then, if the second wire's path is U7,R6,D4,L4, it goes up 7, right 6, down 4, and left 4:
//!
//! > ...........
//! > .+-----+...
//! > .|.....|...
//! > .|..+--X-+.
//! > .|..|..|.|.
//! > .|.-X--+.|.
//! > .|..|....|.
//! > .|.......|.
//! > .o-------+.
//! > ...........
//!
//! These wires cross at two locations (marked X), but the lower-left one is closer to the central port: its distance is 3 + 3 = 6.
//!
//! Here are a few more examples:
//!
//! > R75,D30,R83,U83,L12,D49,R71,U7,L72
//! > U62,R66,U55,R34,D71,R55,D58,R83 = distance 159
//! > R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
//! > U98,R91,D20,R16,D67,R40,U7,R15,U6,R7 = distance 135
//!
//! What is the Manhattan distance from the central port to the closest intersection?
//!
//! --- Part Two ---
//!
//! It turns out that this circuit is very timing-sensitive; you actually need to minimize the signal delay.
//!
//! To do this, calculate the number of steps each wire takes to reach each intersection; choose the intersection where the sum of both wires' steps is lowest. If a wire visits a position on the grid multiple times, use the steps value from the first time it visits that position when calculating the total value of a specific intersection.
//!
//! The number of steps a wire takes is the total number of grid squares the wire has entered to get to that location, including the intersection being considered. Again consider the example from above:
//!
//! > ...........
//! > .+-----+...
//! > .|.....|...
//! > .|..+--X-+.
//! > .|..|..|.|.
//! > .|.-X--+.|.
//! > .|..|....|.
//! > .|.......|.
//! > .o-------+.
//! > ...........
//!
//! In the above example, the intersection closest to the central port is reached after 8+5+5+2 = 20 steps by the first wire and 7+6+4+3 = 20 steps by the second wire for a total of 20+20 = 40 steps.
//!
//! However, the top-right intersection is better: the first wire takes only 8+5+2 = 15 and the second wire takes only 7+6+2 = 15, a total of 15+15 = 30 steps.
//!
//! Here are the best steps for the extra examples from above:
//!
//! > R75,D30,R83,U83,L12,D49,R71,U7,L72
//! > U62,R66,U55,R34,D71,R55,D58,R83 = 610 steps
//! > R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
//! > U98,R91,D20,R16,D67,R40,U7,R15,U6,R7 = 410 steps
//!
//! What is the fewest combined steps the wires must take to reach an intersection?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use indexmap::IndexSet;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};
use tracing::error;

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_solution::<i32>(AoCYear::AOC2019, AoCDay::AOCD03, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> i32 {
    find_br(reader)
        .map_err(|e| {
            error!("{e}");
            e
        })
        .unwrap_or_default()
}

fn find_br<T>(reader: T) -> Result<i32>
where
    T: BufRead,
{
    let mut paths_def = vec![];

    for line in valid_lines(reader) {
        let inst: Vec<String> = line.split(',').map(str::to_string).collect();
        paths_def.push(inst);
    }

    let paths = build_path(&paths_def)?;
    let intersection: HashSet<&Point> = paths[0].intersection(&paths[1]).collect();
    let min_dis = intersection
        .into_iter()
        .map(|point| point.x.abs() + point.y.abs())
        .min();

    min_dis.ok_or_else(|| anyhow!("bad min"))
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn build_path(paths_def: &Vec<Vec<String>>) -> Result<Vec<HashSet<Point>>> {
    Ok(build_full_path(paths_def)?
        .into_iter()
        .map(HashSet::from_iter)
        .collect())
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2019, AoCDay::AOCD03, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader)
        .map_err(|e| {
            error!("{e}");
            e
        })
        .unwrap_or_default()
}

fn find2_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let mut paths_def = vec![];

    for line in valid_lines(reader) {
        let inst: Vec<String> = line.split(',').map(str::to_string).collect();
        paths_def.push(inst);
    }

    let full_paths = build_full_path(&paths_def)?;
    let paths: Vec<HashSet<Point>> = full_paths.iter().cloned().map(HashSet::from_iter).collect();
    let intersection: IndexSet<&Point> = paths[0].intersection(&paths[1]).collect();

    let mut steps = vec![];
    for point in intersection {
        let mut total = 2;
        for path in &full_paths {
            let pos = path
                .iter()
                .position(|x| x == point)
                .ok_or_else(|| anyhow!("bad pos"))?;
            total += pos;
        }
        steps.push(total);
    }
    steps.into_iter().min().ok_or_else(|| anyhow!("bad min"))
}

fn build_full_path(paths_def: &Vec<Vec<String>>) -> Result<Vec<Vec<Point>>> {
    let mut paths = vec![];

    for inst in paths_def {
        let mut coords = vec![];
        let mut x = 0;
        let mut y = 0;

        for next in inst {
            let (dir, count_str) = next.split_at(1);
            let count = count_str.parse::<u32>()?;
            for _ in 0..count {
                match dir {
                    "U" => {
                        y += 1;
                    }
                    "D" => {
                        y -= 1;
                    }
                    "L" => {
                        x -= 1;
                    }
                    "R" => {
                        x += 1;
                    }
                    _ => {
                        return Err(anyhow!("unknown direction: {dir}"));
                    }
                }
                coords.push(Point::new(x, y));
            }
        }
        paths.push(coords);
    }

    Ok(paths)
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";
    const TEST_2: &str = r"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1))?, 159);
        assert_eq!(find_br(Cursor::new(TEST_2))?, 135);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";
    const TEST_2: &str = r"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find2_br(Cursor::new(TEST_1))?, 610);
        assert_eq!(find2_br(Cursor::new(TEST_2))?, 410);
        Ok(())
    }
}
