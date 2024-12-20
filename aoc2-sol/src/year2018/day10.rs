// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2018 ---**
//!
//! **--- Day 10: The Stars Align ---**
//!
//! It's no use; your navigation system simply isn't capable of providing walking directions in the arctic circle, and certainly not in 1018.
//!
//! The Elves suggest an alternative. In times like these, North Pole rescue operations will arrange points of light in the sky to guide missing Elves back to base. Unfortunately, the message is easy to miss: the points move slowly enough that it takes hours to align them, but have so much momentum that they only stay aligned for a second. If you blink at the wrong time, it might be hours before another message appears.
//!
//! You can see these points of light floating in the distance, and record their position in the sky and their velocity, the relative change in position per second (your puzzle input). The coordinates are all given from your perspective; given enough time, those positions and velocities will move the points into a cohesive message!
//!
//! Rather than wait, you decide to fast-forward the process and calculate what the points will eventually spell.
//!
//! For example, suppose you note the following points:
//!
//! ```text
//! position=< 9,  1> velocity=< 0,  2>
//! position=< 7,  0> velocity=<-1,  0>
//! position=< 3, -2> velocity=<-1,  1>
//! position=< 6, 10> velocity=<-2, -1>
//! position=< 2, -4> velocity=< 2,  2>
//! position=<-6, 10> velocity=< 2, -2>
//! position=< 1,  8> velocity=< 1, -1>
//! position=< 1,  7> velocity=< 1,  0>
//! position=<-3, 11> velocity=< 1, -2>
//! position=< 7,  6> velocity=<-1, -1>
//! position=<-2,  3> velocity=< 1,  0>
//! position=<-4,  3> velocity=< 2,  0>
//! position=<10, -3> velocity=<-1,  1>
//! position=< 5, 11> velocity=< 1, -2>
//! position=< 4,  7> velocity=< 0, -1>
//! position=< 8, -2> velocity=< 0,  1>
//! position=<15,  0> velocity=<-2,  0>
//! position=< 1,  6> velocity=< 1,  0>
//! position=< 8,  9> velocity=< 0, -1>
//! position=< 3,  3> velocity=<-1,  1>
//! position=< 0,  5> velocity=< 0, -1>
//! position=<-2,  2> velocity=< 2,  0>
//! position=< 5, -2> velocity=< 1,  2>
//! position=< 1,  4> velocity=< 2,  1>
//! position=<-2,  7> velocity=< 2, -2>
//! position=< 3,  6> velocity=<-1, -1>
//! position=< 5,  0> velocity=< 1,  0>
//! position=<-6,  0> velocity=< 2,  0>
//! position=< 5,  9> velocity=< 1, -2>
//! position=<14,  7> velocity=<-2,  0>
//! position=<-3,  6> velocity=< 2, -1>
//! ```
//!
//! Each line represents one point. Positions are given as <X, Y> pairs: X represents how far left (negative) or right (positive) the point appears, while Y represents how far up (negative) or down (positive) the point appears.
//!
//! At 0 seconds, each point has the position given. Each second, each point's velocity is added to its position. So, a point with velocity <1, -2> is moving to the right, but is moving upward twice as quickly. If this point's initial position were <3, 9>, after 3 seconds, its position would become <6, 3>.
//!
//! Over time, the points listed above would move like this:
//!
//! ```text
//! Initially:
//! ........#.............
//! ................#.....
//! .........#.#..#.......
//! ......................
//! #..........#.#.......#
//! ...............#......
//! ....#.................
//! ..#.#....#............
//! .......#..............
//! ......#...............
//! ...#...#.#...#........
//! ....#..#..#.........#.
//! .......#..............
//! ...........#..#.......
//! #...........#.........
//! ...#.......#..........
//!
//! After 1 second:
//! ......................
//! ......................
//! ..........#....#......
//! ........#.....#.......
//! ..#.........#......#..
//! ......................
//! ......#...............
//! ....##.........#......
//! ......#.#.............
//! .....##.##..#.........
//! ........#.#...........
//! ........#...#.....#...
//! ..#...........#.......
//! ....#.....#.#.........
//! ......................
//! ......................
//!
//! After 2 seconds:
//! ......................
//! ......................
//! ......................
//! ..............#.......
//! ....#..#...####..#....
//! ......................
//! ........#....#........
//! ......#.#.............
//! .......#...#..........
//! .......#..#..#.#......
//! ....#....#.#..........
//! .....#...#...##.#.....
//! ........#.............
//! ......................
//! ......................
//! ......................
//!
//! After 3 seconds:
//! ......................
//! ......................
//! ......................
//! ......................
//! ......#...#..###......
//! ......#...#...#.......
//! ......#...#...#.......
//! ......#####...#.......
//! ......#...#...#.......
//! ......#...#...#.......
//! ......#...#...#.......
//! ......#...#..###......
//! ......................
//! ......................
//! ......................
//! ......................
//!
//! After 4 seconds:
//! ......................
//! ......................
//! ......................
//! ............#.........
//! ........##...#.#......
//! ......#.....#..#......
//! .....#..##.##.#.......
//! .......##.#....#......
//! ...........#....#.....
//! ..............#.......
//! ....#......#...#......
//! .....#.....##.........
//! ...............#......
//! ...............#......
//! ......................
//! ......................
//! ```
//!
//! After 3 seconds, the message appeared briefly: HI. Of course, your message will be much longer and will take many more seconds to appear.
//!
//! What message will eventually appear in the sky?
//!
//! **--- Part Two ---**
//!
//! Good thing you didn't have to wait, because that would have taken a long time - much longer than the 3 seconds in the example above.
//!
//! Impressed by your sub-hour communication capabilities, the Elves are curious: exactly how many seconds would they have needed to wait for that message to appear?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::Result;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

type StarMap = (usize, Vec<(isize, isize, isize, isize)>);

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<StarMap, String>(AoCYear::AOC2018, AoCDay::AOCD10, setup, find).map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<StarMap, String>(bench, AoCYear::AOC2018, AoCDay::AOCD10, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> StarMap {
    setup_br(reader, 10619).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T, max_step: usize) -> Result<StarMap>
where
    T: BufRead,
{
    let line_re = Regex::new(r"position=<(.*), (.*)> velocity=<(.*), (.*)>")?;
    let mut star_map: Vec<(isize, isize, isize, isize)> = Vec::new();

    for line in valid_lines(reader) {
        for cap in line_re.captures_iter(&line) {
            let x = (cap[1]).trim().parse::<isize>()?;
            let y = (cap[2]).trim().parse::<isize>()?;
            let vx = (cap[3]).trim().parse::<isize>()?;
            let vy = (cap[4]).trim().parse::<isize>()?;

            star_map.push((x, y, vx, vy));
        }
    }
    Ok((max_step, star_map))
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: StarMap) -> String {
    find_res(data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(data: StarMap, _second_star: bool) -> Result<String> {
    let (max_step, mut star_map) = data;
    for _ in 0..max_step {
        move_stars(&mut star_map);
    }

    Ok(show_stars(&star_map))
}

fn move_stars(star_map: &mut Vec<(isize, isize, isize, isize)>) {
    for star in star_map {
        star.0 += star.2;
        star.1 += star.3;
    }
}

fn show_stars(star_map: &[(isize, isize, isize, isize)]) -> String {
    let mut output = String::new();
    let mut min_x = isize::MAX;
    let mut min_y = isize::MAX;
    let mut max_x = isize::MIN;
    let mut max_y = isize::MIN;

    for star in star_map {
        if star.0 < min_x {
            min_x = star.0;
        }

        if star.0 > max_x {
            max_x = star.0;
        }

        if star.1 < min_y {
            min_y = star.1;
        }

        if star.1 > max_y {
            max_y = star.1;
        }
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let mut found_star = false;
            for star in star_map {
                if star.0 == x && star.1 == y {
                    output.push('#');
                    found_star = true;
                    break;
                }
            }

            if !found_star {
                output.push('.');
            }
        }
        output.push('\n');
    }

    output
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<StarMap, String>(AoCYear::AOC2018, AoCDay::AOCD10, setup, find2).map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<StarMap, String>(bench, AoCYear::AOC2018, AoCDay::AOCD10, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: StarMap) -> String {
    find_res(data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";

    const EXPECTED: &str = r"#...#..###
#...#...#.
#...#...#.
#####...#.
#...#...#.
#...#...#.
#...#...#.
#...#..###
";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1), 3)?;
        assert_eq!(find(data), EXPECTED);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    #[test]
    fn solution() {}
}
