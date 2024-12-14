// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code - Day 14 ---**
//!
//! --- Day 14: Restroom Redoubt ---
//!
//! One of The Historians needs to use the bathroom; fortunately, you know there's a bathroom near an unvisited location on their list, and so you're all quickly teleported directly to the lobby of Easter Bunny Headquarters.
//!
//! Unfortunately, EBHQ seems to have "improved" bathroom security again after your last visit. The area outside the bathroom is swarming with robots!
//!
//! To get The Historian safely to the bathroom, you'll need a way to predict where the robots will be in the future. Fortunately, they all seem to be moving on the tile floor in predictable straight lines.
//!
//! You make a list (your puzzle input) of all of the robots' current positions (p) and velocities (v), one robot per line. For example:
//!
//! ```text
//! p=0,4 v=3,-3
//! p=6,3 v=-1,-3
//! p=10,3 v=-1,2
//! p=2,0 v=2,-1
//! p=0,0 v=1,3
//! p=3,0 v=-2,-2
//! p=7,6 v=-1,-3
//! p=3,0 v=-1,-2
//! p=9,3 v=2,3
//! p=7,3 v=-1,2
//! p=2,4 v=2,-3
//! p=9,5 v=-3,-3
//! ```
//!
//! Each robot's position is given as p=x,y where x represents the number of tiles the robot is from the left wall and y represents the number of tiles from the top wall (when viewed from above). So, a position of p=0,0 means the robot is all the way in the top-left corner.
//!
//! Each robot's velocity is given as v=x,y where x and y are given in tiles per second. Positive x means the robot is moving to the right, and positive y means the robot is moving down. So, a velocity of v=1,-2 means that each second, the robot moves 1 tile to the right and 2 tiles up.
//!
//! The robots outside the actual bathroom are in a space which is 101 tiles wide and 103 tiles tall (when viewed from above). However, in this example, the robots are in a space which is only 11 tiles wide and 7 tiles tall.
//!
//! The robots are good at navigating over/under each other (due to a combination of springs, extendable legs, and quadcopters), so they can share the same tile and don't interact with each other. Visually, the number of robots on each tile in this example looks like this:
//!
//! ```text
//! 1.12.......
//! ...........
//! ...........
//! ......11.11
//! 1.1........
//! .........1.
//! .......1...
//! ```
//!
//! These robots have a unique feature for maximum bathroom security: they can teleport. When a robot would run into an edge of the space they're in, they instead teleport to the other side, effectively wrapping around the edges. Here is what robot p=2,4 v=2,-3 does for the first few seconds:
//!
//! ```text
//! Initial state:
//! ...........
//! ...........
//! ...........
//! ...........
//! ..1........
//! ...........
//! ...........
//!
//! After 1 second:
//! ...........
//! ....1......
//! ...........
//! ...........
//! ...........
//! ...........
//! ...........
//!
//! After 2 seconds:
//! ...........
//! ...........
//! ...........
//! ...........
//! ...........
//! ......1....
//! ...........
//!
//! After 3 seconds:
//! ...........
//! ...........
//! ........1..
//! ...........
//! ...........
//! ...........
//! ...........
//!
//! After 4 seconds:
//! ...........
//! ...........
//! ...........
//! ...........
//! ...........
//! ...........
//! ..........1
//!
//! After 5 seconds:
//! ...........
//! ...........
//! ...........
//! .1.........
//! ...........
//! ...........
//! ...........
//! ```
//!
//! The Historian can't wait much longer, so you don't have to simulate the robots for very long. Where will the robots be after 100 seconds?
//!
//! In the above example, the number of robots on each tile after 100 seconds has elapsed looks like this:
//!
//! ```text
//! ......2..1.
//! ...........
//! 1..........
//! .11........
//! .....1.....
//! ...12......
//! .1....1....
//! ```
//!
//! To determine the safest area, count the number of robots in each quadrant after 100 seconds. Robots that are exactly in the middle (horizontally or vertically) don't count as being in any quadrant, so the only relevant robots are:
//!
//! ```text
//! ..... 2..1.
//! ..... .....
//! 1.... .....
//!            
//! ..... .....
//! ...12 .....
//! .1... 1....
//! ```
//!
//! In this example, the quadrants contain 1, 3, 4, and 1 robot. Multiplying these together gives a total safety factor of 12.
//!
//! Predict the motion of the robots in your list within a space which is 101 tiles wide and 103 tiles tall. What will the safety factor be after exactly 100 seconds have elapsed?
//!
//! **--- Part Two ---**
//!
//! During the bathroom break, someone notices that these robots seem awfully similar to ones built and used at the North Pole. If they're the same type of robots, they should have a hard-coded Easter egg: very rarely, most of the robots should arrange themselves into a picture of a Christmas tree.
//!
//! What is the fewest number of seconds that must elapse for the robots to display the Easter egg?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{get_cap_x, run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::Result;
use crossterm::{
    cursor::{Hide, MoveToNextLine, RestorePosition, SavePosition, Show},
    ExecutableCommand, QueueableCommand,
};
use getset::{CopyGetters, Setters};
use regex::Regex;
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{stdout, BufRead, BufReader, Write},
};

type RobotData = (isize, isize, HashMap<(isize, isize), Vec<Robot>>);

#[derive(Clone, Copy, CopyGetters, Debug, Default, Setters)]
#[getset(get_copy = "pub(crate)", set = "pub(crate)")]
struct Robot {
    v_x: isize,
    v_y: isize,
}

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<RobotData, usize>(AoCYear::AOC2024, AoCDay::AOCD14, setup, find).map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<(isize, isize, HashMap<(isize, isize), Vec<Robot>>), usize>(
        bench,
        AoCYear::AOC2024,
        AoCDay::AOCD14,
        setup,
        find,
    )
    .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> RobotData {
    setup_br(101, 103, reader).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(max_x: isize, max_y: isize, reader: T) -> Result<RobotData>
where
    T: BufRead,
{
    let robots_re = Regex::new(r"^p=(\d+),(\d+) v=(-?\d+),(-?\d+)$")?;
    let mut robots = HashMap::new();
    for line in valid_lines(reader) {
        if let Some(caps) = robots_re.captures(&line) {
            let mut robot = Robot::default();
            let x = get_cap_x::<isize>(1, &caps)?;
            let y = get_cap_x::<isize>(2, &caps)?;
            let _ = robot.set_v_x(get_cap_x::<isize>(3, &caps)?);
            let _ = robot.set_v_y(get_cap_x::<isize>(4, &caps)?);
            let _ = robots
                .entry((x, y))
                .and_modify(|robots: &mut Vec<Robot>| robots.push(robot))
                .or_insert(vec![robot]);
        }
    }
    Ok((max_x, max_y, robots))
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: RobotData) -> usize {
    let (max_x, max_y, mut robots_map) = data;

    for _ in 0..100 {
        move_robots(max_x, max_y, &mut robots_map);
    }
    count_robots(max_x, max_y, &robots_map)
}

fn move_robots(max_x: isize, max_y: isize, robots_map: &mut HashMap<(isize, isize), Vec<Robot>>) {
    let mut moves = vec![];
    for ((curr_x, curr_y), robots) in robots_map.drain() {
        for robot in robots {
            let next_x = (curr_x + robot.v_x()).rem_euclid(max_x);
            let next_y = (curr_y + robot.v_y()).rem_euclid(max_y);
            moves.push((next_x, next_y, robot));
        }
    }
    for (next_x, next_y, robot) in moves {
        let _ = robots_map
            .entry((next_x, next_y))
            .and_modify(|robots: &mut Vec<Robot>| robots.push(robot))
            .or_insert(vec![robot]);
    }
}

fn count_robots(
    max_x: isize,
    max_y: isize,
    robots_map: &HashMap<(isize, isize), Vec<Robot>>,
) -> usize {
    let no_x = max_x / 2;
    let no_y = max_y / 2;

    let q1: usize = robots_map
        .iter()
        .filter_map(|((x, y), robot)| {
            if *x < no_x && *y < no_y {
                Some(robot.len())
            } else {
                None
            }
        })
        .sum();
    let q2: usize = robots_map
        .iter()
        .filter_map(|((x, y), robot)| {
            if *x > no_x && *y < no_y {
                Some(robot.len())
            } else {
                None
            }
        })
        .sum();
    let q3: usize = robots_map
        .iter()
        .filter_map(|((x, y), robot)| {
            if *x < no_x && *y > no_y {
                Some(robot.len())
            } else {
                None
            }
        })
        .sum();

    let q4: usize = robots_map
        .iter()
        .filter_map(|((x, y), robot)| {
            if *x > no_x && *y > no_y {
                Some(robot.len())
            } else {
                None
            }
        })
        .sum();
    q1 * q2 * q3 * q4
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<RobotData, usize>(AoCYear::AOC2024, AoCDay::AOCD14, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<RobotData, usize>(bench, AoCYear::AOC2024, AoCDay::AOCD14, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: RobotData) -> usize {
    let (max_x, max_y, mut robots_map) = data;

    let blah: Vec<usize> = (0..10000).collect();
    for (outer, i) in blah.chunks(1000).enumerate() {
        for j in i {
            move_robots(max_x, max_y, &mut robots_map);
            if *j < 7344 {
                display_grid(*j, max_x, max_y, &robots_map).unwrap();
            }
            if *j == 7343 {
                let mut output = OpenOptions::new()
                    .create(true)
                    .truncate(true)
                    .write(true)
                    .open(format!("data/2024/day14/robots_map_{outer}.txt"))
                    .unwrap();
                write_grid(*j, max_x, max_y, &robots_map, &mut output).unwrap();
            }
        }
    }
    0
}

fn write_grid(
    idx: usize,
    max_x: isize,
    max_y: isize,
    robots_map: &HashMap<(isize, isize), Vec<Robot>>,
    file: &mut File,
) -> Result<()> {
    writeln!(file, "Elapsed Seconds: {idx}")?;

    for x in 0..max_x {
        for y in 0..max_y {
            if let Some(robots) = robots_map.get(&(x, y)) {
                write!(file, "{}", robots.len())?;
            } else {
                write!(file, ".")?;
            }
        }
        writeln!(file)?;
    }
    writeln!(file)?;

    Ok(())
}

fn display_grid(
    idx: usize,
    max_x: isize,
    max_y: isize,
    robots_map: &HashMap<(isize, isize), Vec<Robot>>,
) -> Result<()> {
    let mut stdout = stdout();

    let _ = stdout.execute(Hide)?;
    let _ = stdout.queue(SavePosition)?;

    stdout.write_all(format!("Elapsed Seconds: {idx}").as_bytes())?;
    let _ = stdout.queue(MoveToNextLine(1))?;
    for x in 0..max_x {
        for y in 0..max_y {
            if let Some(robots) = robots_map.get(&(x, y)) {
                let _ = stdout.write(format!("{}", robots.len()).as_bytes())?;
            } else {
                let _ = stdout.write(".".as_bytes())?;
            }
        }
        let _ = stdout.queue(MoveToNextLine(1))?;
    }
    let _ = stdout.queue(RestorePosition)?;
    let _ = stdout.execute(Show)?;
    Ok(())
}

#[cfg(test)]
mod one_star {
    use super::{find, move_robots, setup_br, Robot};
    use anyhow::Result;
    use std::{collections::HashMap, io::Cursor};

    const TEST_1: &str = r"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn move_robots_works() {
        let mut robots_map = HashMap::new();
        let mut robot = Robot::default();
        let _ = robot.set_v_x(2);
        let _ = robot.set_v_y(-3);
        let _res = robots_map.insert((2_isize, 4_isize), vec![robot]);
        move_robots(11, 7, &mut robots_map);
        assert!(robots_map.contains_key(&(4, 1)));
        move_robots(11, 7, &mut robots_map);
        assert!(robots_map.contains_key(&(6, 5)));
        move_robots(11, 7, &mut robots_map);
        assert!(robots_map.contains_key(&(8, 2)));
        move_robots(11, 7, &mut robots_map);
        assert!(robots_map.contains_key(&(10, 6)));
        move_robots(11, 7, &mut robots_map);
        assert!(robots_map.contains_key(&(1, 3)));
    }

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(11, 7, Cursor::new(TEST_1))?;
        assert_eq!(find(data), 12);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {}
