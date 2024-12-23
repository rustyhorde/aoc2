// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2019 ---**
//!
//! **--- Day 11: Space Police ---**
//!  
//!  On the way to Jupiter, you're pulled over by the Space Police.
//!  
//!  "Attention, unmarked spacecraft! You are in violation of Space Law! All spacecraft must have a clearly visible registration identifier! You have 24 hours to comply or be sent to Space Jail!"
//!  
//!  Not wanting to be sent to Space Jail, you radio back to the Elves on Earth for help. Although it takes almost three hours for their reply signal to reach you, they send instructions for how to power up the emergency hull painting robot and even provide a small Intcode program (your puzzle input) that will cause it to paint your ship appropriately.
//!  
//!  There's just one problem: you don't have an emergency hull painting robot.
//!  
//!  You'll need to build a new emergency hull painting robot. The robot needs to be able to move around on the grid of square panels on the side of your ship, detect the color of its current panel, and paint its current panel black or white. (All of the panels are currently black.)
//!  
//!  The Intcode program will serve as the brain of the robot. The program uses input instructions to access the robot's camera: provide 0 if the robot is over a black panel or 1 if the robot is over a white panel. Then, the program will output two values:
//!  
//!  ```text
//!      First, it will output a value indicating the color to paint the panel the robot is over: 0 means to paint the panel black, and 1 means to paint the panel white.
//!      Second, it will output a value indicating the direction the robot should turn: 0 means it should turn left 90 degrees, and 1 means it should turn right 90 degrees.
//!  ```
//!
//!  After the robot turns, it should always move forward exactly one panel. The robot starts facing up.
//!  
//!  The robot will continue running for a while like this and halt when it is finished drawing. Do not restart the Intcode computer inside the robot during this process.
//!  
//!  For example, suppose the robot is about to start running. Drawing black panels as ., white panels as #, and the robot pointing the direction it is facing (< ^ > v), the initial state and region near the robot looks like this:
//!
//!  ```text
//!  .....
//!  .....
//!  ..^..
//!  .....
//!  .....
//!  ```
//!
//!  The panel under the robot (not visible here because a ^ is shown instead) is also black, and so any input instructions at this point should be provided 0. Suppose the robot eventually outputs 1 (paint white) and then 0 (turn left). After taking these actions and moving forward one panel, the region now looks like this:
//!
//!  ```text
//!  .....
//!  .....
//!  .<#..
//!  .....
//!  .....
//!  ```
//!
//!  Input instructions should still be provided 0. Next, the robot might output 0 (paint black) and then 0 (turn left):
//!
//!  ```text
//!  .....
//!  .....
//!  ..#..
//!  .v...
//!  .....
//!  ```
//!
//!  After more outputs (1,0, 1,0):
//!
//!  ```text
//!  .....
//!  .....
//!  ..^..
//!  .##..
//!  .....
//!  ```
//!
//!  The robot is now back where it started, but because it is now on a white panel, input instructions should be provided 1. After several more outputs (0,1, 1,0, 1,0), the area looks like this:
//!
//!  ```text
//!  .....
//!  ..<#.
//!  ...#.
//!  .##..
//!  .....
//!  ```
//!
//!  Before you deploy the robot, you should probably have an estimate of the area it will cover: specifically, you need to know the number of panels it paints at least once, regardless of color. In the example above, the robot painted 6 panels at least once. (It painted its starting panel twice, but that panel is still only counted once; it also never painted the panel it ended on.)
//!  
//!  Build a new emergency hull painting robot and run the Intcode program on it. How many panels does it paint at least once?
//!
//! **--- Part Two ---**
//!  
//!  You're not sure what it's trying to paint, but it's definitely not a registration identifier. The Space Police are getting impatient.
//!  
//!  Checking your external ship cameras again, you notice a white panel marked "emergency hull painting robot starting panel". The rest of the panels are still black, but it looks like the robot was expecting to start on a white panel, not a black one.
//!  
//!  Based on the Space Law Space Brochure that the Space Police attached to one of your windows, a valid registration identifier is always eight capital letters. After starting the robot on a single white panel instead, what registration identifier does it paint on your hull?

use crate::year2019::intcode::{as_isize, Intcode, IntcodeData};
use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use bnum::types::I256;
use std::collections::HashMap;
use std::fmt::Formatter;
use std::sync::mpsc::channel;
use std::thread::spawn;
use std::{
    fmt,
    fs::File,
    io::{BufRead, BufReader},
};

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](AoCYear) and
///   [`AoCDay`](AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<IntcodeData, usize>(AoCYear::AOC2019, AoCDay::AOCD11, setup, find)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<IntcodeData, usize>(bench, AoCYear::AOC2019, AoCDay::AOCD11, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> IntcodeData {
    setup_br(reader).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T) -> Result<IntcodeData>
where
    T: BufRead,
{
    let mut intcodes = vec![];
    for line in valid_lines(reader) {
        intcodes = line.split(',').filter_map(as_isize).collect();
    }
    Ok(intcodes)
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: IntcodeData) -> usize {
    find_res(data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(intcodes: IntcodeData, second_star: bool) -> Result<usize> {
    let (sender, receiver) = channel();
    let (send_a, mut amp_a) = Intcode::new(intcodes.clone());
    let _ = amp_a.set_sender_opt(Some(sender));

    let amp_a_handle = spawn(move || amp_a.start());

    let initial_input = if second_star { I256::ONE } else { I256::ZERO };
    send_a.send(initial_input)?;

    let mut paint_map = HashMap::new();
    let mut curr_loc = (0_isize, 0_isize);
    let mut curr_dir = Direction::Up;
    let mut paint = true;

    while let Ok(res) = receiver.recv() {
        if let Some(send) = hpr(
            res,
            &mut paint,
            &mut curr_loc,
            &mut curr_dir,
            &mut paint_map,
        )? {
            send_a.send(send)?;
        }
    }

    match amp_a_handle.join() {
        Ok(res) => match res {
            Ok(_r) => {}
            Err(e) => eprintln!("{e}"),
        },
        Err(e) => eprintln!("{e:?}"),
    }

    if second_star {
        display_map(&paint_map)?;
    }
    Ok(paint_map.keys().len())
}

fn display_map(paint_map: &HashMap<(isize, isize), char>) -> Result<()> {
    let min_x = paint_map.keys().map(|(x, _)| *x).min().unwrap_or_default();
    let max_x = paint_map.keys().map(|(x, _)| *x).max().unwrap_or_default();
    let min_y = paint_map.keys().map(|(_, y)| *y).min().unwrap_or_default();
    let max_y = paint_map.keys().map(|(_, y)| *y).max().unwrap_or_default();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let ch = paint_map.get(&(x, y)).unwrap_or(&'.');
            print!("{}", ch);
        }
        println!();
    }

    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn hpr(
    res: I256,
    paint: &mut bool,
    curr_loc: &mut (isize, isize),
    curr_dir: &mut Direction,
    paint_map: &mut HashMap<(isize, isize), char>,
) -> Result<Option<I256>> {
    if res == I256::ZERO && *paint {
        // eprint!("painting {curr_loc:?} black, ");
        let _ = paint_map
            .entry(*curr_loc)
            .and_modify(|loc| *loc = '.')
            .or_insert('.');
        *paint = false;
        Ok(None)
    } else if res == I256::ONE && *paint {
        // eprint!("painting {curr_loc:?} white, ");
        let _ = paint_map
            .entry(*curr_loc)
            .and_modify(|loc| *loc = '#')
            .or_insert('#');
        *paint = false;
        Ok(None)
    } else if res == I256::ZERO && !(*paint) {
        match *curr_dir {
            Direction::Up => {
                *curr_dir = Direction::Left;
                curr_loc.0 -= 1;
            }
            Direction::Down => {
                *curr_dir = Direction::Right;
                curr_loc.0 += 1;
            }
            Direction::Left => {
                *curr_dir = Direction::Down;
                curr_loc.1 += 1;
            }
            Direction::Right => {
                *curr_dir = Direction::Up;
                curr_loc.1 -= 1;
            }
        }
        // eprint!("turned left to {curr_dir} and moved to {curr_loc:?}");
        let _ = paint_map.entry(*curr_loc).or_insert('.');
        let mut send = None;
        if let Some(below) = paint_map.get(&curr_loc) {
            match below {
                '.' => send = Some(I256::ZERO),
                '#' => send = Some(I256::ONE),
                _ => return Err(anyhow!("Invalid paint color")),
            }
        }

        *paint = true;
        Ok(send)
    } else if res == I256::ONE && !(*paint) {
        match *curr_dir {
            Direction::Up => {
                *curr_dir = Direction::Right;
                curr_loc.0 += 1;
            }
            Direction::Down => {
                *curr_dir = Direction::Left;
                curr_loc.0 -= 1;
            }
            Direction::Left => {
                *curr_dir = Direction::Up;
                curr_loc.1 -= 1;
            }
            Direction::Right => {
                *curr_dir = Direction::Down;
                curr_loc.1 += 1;
            }
        }
        // eprint!("turned right to {curr_dir} and moved to {curr_loc:?}");
        let _ = paint_map.entry(*curr_loc).or_insert('.');
        let mut send = None;
        if let Some(below) = paint_map.get(&curr_loc) {
            match below {
                '.' => send = Some(I256::ZERO),
                '#' => send = Some(I256::ONE),
                _ => return Err(anyhow!("Invalid paint color")),
            }
        }

        *paint = true;
        Ok(send)
    } else {
        Err(anyhow!("Invalid robot input"))
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Up => '^',
                Direction::Down => 'v',
                Direction::Left => '<',
                Direction::Right => '>',
            }
        )
    }
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](AoCYear) and
///   [`AoCDay`](AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<IntcodeData, usize>(AoCYear::AOC2019, AoCDay::AOCD11, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<IntcodeData, usize>(bench, AoCYear::AOC2019, AoCDay::AOCD11, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: IntcodeData) -> usize {
    find_res(data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {
    use super::{hpr, Direction};
    use anyhow::Result;
    use bnum::types::I256;
    use std::collections::HashMap;

    #[test]
    fn solution() -> Result<()> {
        let mut paint_map = HashMap::new();
        let mut curr_loc = (0, 0);
        let mut curr_dir = Direction::Up;
        let mut paint = true;
        // Paint
        let _ = hpr(
            I256::ONE,
            &mut paint,
            &mut curr_loc,
            &mut curr_dir,
            &mut paint_map,
        )?;
        assert_eq!(curr_loc, (0, 0));
        assert_eq!(paint_map.get(&curr_loc), Some(&'#'));
        assert!(!paint);
        assert_eq!(curr_dir, Direction::Up);
        // Move
        let _ = hpr(
            I256::ZERO,
            &mut paint,
            &mut curr_loc,
            &mut curr_dir,
            &mut paint_map,
        )?;
        assert_eq!(curr_loc, (-1, 0));
        assert_eq!(paint_map.get(&curr_loc), Some(&'.'));
        assert!(paint);
        assert_eq!(curr_dir, Direction::Left);

        //Paint
        let _ = hpr(
            I256::ZERO,
            &mut paint,
            &mut curr_loc,
            &mut curr_dir,
            &mut paint_map,
        )?;
        assert_eq!(curr_loc, (-1, 0));
        assert_eq!(paint_map.get(&curr_loc), Some(&'.'));
        assert!(!paint);
        assert_eq!(curr_dir, Direction::Left);
        // Move
        let _ = hpr(
            I256::ZERO,
            &mut paint,
            &mut curr_loc,
            &mut curr_dir,
            &mut paint_map,
        )?;
        assert_eq!(curr_loc, (-1, -1));
        assert_eq!(paint_map.get(&curr_loc), Some(&'.'));
        assert!(paint);
        assert_eq!(curr_dir, Direction::Down);

        // Paint
        let _ = hpr(
            I256::ONE,
            &mut paint,
            &mut curr_loc,
            &mut curr_dir,
            &mut paint_map,
        )?;
        assert_eq!(curr_loc, (-1, -1));
        assert_eq!(paint_map.get(&curr_loc), Some(&'#'));
        assert!(!paint);
        assert_eq!(curr_dir, Direction::Down);
        // Move
        let _ = hpr(
            I256::ZERO,
            &mut paint,
            &mut curr_loc,
            &mut curr_dir,
            &mut paint_map,
        )?;
        assert_eq!(curr_loc, (0, -1));
        assert_eq!(paint_map.get(&curr_loc), Some(&'.'));
        assert!(paint);
        assert_eq!(curr_dir, Direction::Right);

        // Paint
        let _ = hpr(
            I256::ONE,
            &mut paint,
            &mut curr_loc,
            &mut curr_dir,
            &mut paint_map,
        )?;
        assert_eq!(curr_loc, (0, -1));
        assert_eq!(paint_map.get(&curr_loc), Some(&'#'));
        assert!(!paint);
        assert_eq!(curr_dir, Direction::Right);
        // Move
        let _ = hpr(
            I256::ZERO,
            &mut paint,
            &mut curr_loc,
            &mut curr_dir,
            &mut paint_map,
        )?;
        assert_eq!(curr_loc, (0, 0));
        assert_eq!(paint_map.get(&curr_loc), Some(&'#'));
        assert!(paint);
        assert_eq!(curr_dir, Direction::Up);
        assert_eq!(4, paint_map.keys().len());

        // Paint
        let _ = hpr(
            I256::ZERO,
            &mut paint,
            &mut curr_loc,
            &mut curr_dir,
            &mut paint_map,
        )?;
        assert_eq!(curr_loc, (0, 0));
        assert_eq!(paint_map.get(&curr_loc), Some(&'.'));
        assert!(!paint);
        assert_eq!(curr_dir, Direction::Up);
        // Move
        let _ = hpr(
            I256::ONE,
            &mut paint,
            &mut curr_loc,
            &mut curr_dir,
            &mut paint_map,
        )?;
        assert_eq!(curr_loc, (1, 0));
        assert_eq!(paint_map.get(&curr_loc), Some(&'.'));
        assert!(paint);
        assert_eq!(curr_dir, Direction::Right);

        // Paint
        let _ = hpr(
            I256::ONE,
            &mut paint,
            &mut curr_loc,
            &mut curr_dir,
            &mut paint_map,
        )?;
        assert_eq!(curr_loc, (1, 0));
        assert_eq!(paint_map.get(&curr_loc), Some(&'#'));
        assert!(!paint);
        assert_eq!(curr_dir, Direction::Right);
        // Move
        let _ = hpr(
            I256::ZERO,
            &mut paint,
            &mut curr_loc,
            &mut curr_dir,
            &mut paint_map,
        )?;
        assert_eq!(curr_loc, (1, 1));
        assert_eq!(paint_map.get(&curr_loc), Some(&'.'));
        assert!(paint);
        assert_eq!(curr_dir, Direction::Up);

        // Paint
        let _ = hpr(
            I256::ONE,
            &mut paint,
            &mut curr_loc,
            &mut curr_dir,
            &mut paint_map,
        )?;
        assert_eq!(curr_loc, (1, 1));
        assert_eq!(paint_map.get(&curr_loc), Some(&'#'));
        assert!(!paint);
        assert_eq!(curr_dir, Direction::Up);
        // Move
        let _ = hpr(
            I256::ZERO,
            &mut paint,
            &mut curr_loc,
            &mut curr_dir,
            &mut paint_map,
        )?;
        assert_eq!(curr_loc, (0, 1));
        assert_eq!(paint_map.get(&curr_loc), Some(&'.'));
        assert!(paint);
        assert_eq!(curr_dir, Direction::Left);
        assert_eq!(7, paint_map.keys().len());

        Ok(())
    }
}

#[cfg(test)]
mod two_star {}
