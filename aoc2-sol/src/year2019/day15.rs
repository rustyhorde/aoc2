// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2019 ---**
//!
//! **--- Day 15: Oxygen System ---**
//!
//! Out here in deep space, many things can go wrong. Fortunately, many of those things have indicator lights. Unfortunately, one of those lights is lit: the oxygen system for part of the ship has failed!
//!
//! According to the readouts, the oxygen system must have failed days ago after a rupture in oxygen tank two; that section of the ship was automatically sealed once oxygen levels went dangerously low. A single remotely-operated repair droid is your only option for fixing the oxygen system.
//!
//! The Elves' care package included an Intcode program (your puzzle input) that you can use to remotely control the repair droid. By running that program, you can direct the repair droid to the oxygen system and fix the problem.
//!
//! The remote control program executes the following steps in a loop forever:
//!
//! ```text
//!     Accept a movement command via an input instruction.
//!     Send the movement command to the repair droid.
//!     Wait for the repair droid to finish the movement operation.
//!     Report on the status of the repair droid via an output instruction.
//! ```
//!
//! Only four movement commands are understood: north (1), south (2), west (3), and east (4). Any other command is invalid. The movements differ in direction, but not in distance: in a long enough east-west hallway, a series of commands like 4,4,4,4,3,3,3,3 would leave the repair droid back where it started.
//!
//! The repair droid can reply with any of the following status codes:
//!
//! ```text
//!     0: The repair droid hit a wall. Its position has not changed.
//!     1: The repair droid has moved one step in the requested direction.
//!     2: The repair droid has moved one step in the requested direction; its new position is the location of the oxygen system.
//! ```
//!
//! You don't know anything about the area around the repair droid, but you can figure it out by watching the status codes.
//!
//! For example, we can draw the area using D for the droid, # for walls, . for locations the droid can traverse, and empty space for unexplored locations. Then, the initial state looks like this:
//!
//! ```text
//!       
//!       
//!    D  
//!       
//!       
//! ```
//!
//! To make the droid go north, send it 1. If it replies with 0, you know that location is a wall and that the droid didn't move:
//!
//! ```text
//!       
//!    #  
//!    D  
//!       
//!       
//! ```
//!
//! To move east, send 4; a reply of 1 means the movement was successful:
//!
//! ```text
//!       
//!    #  
//!    .D
//!       
//!       
//! ```
//!
//! Then, perhaps attempts to move north (1), south (2), and east (4) are all met with replies of 0:
//!
//! ```text
//!       
//!    ##
//!    .D#
//!     #
//!       
//! ```
//!
//! Now, you know the repair droid is in a dead end. Backtrack with 3 (which you already know will get a reply of 1 because you already know that location is open):
//!
//! ```text
//!       
//!    ##
//!    D.#
//!     #
//!       
//! ```
//!
//! Then, perhaps west (3) gets a reply of 0, south (2) gets a reply of 1, south again (2) gets a reply of 0, and then west (3) gets a reply of 2:
//!
//! ```text
//!       
//!    ##
//!   #..#
//!   D.#
//!    #  
//! ```
//!
//! Now, because of the reply of 2, you know you've found the oxygen system! In this example, it was only 2 moves away from the repair droid's starting position.
//!
//! What is the fewest number of movement commands required to move the repair droid from its starting position to the location of the oxygen system?

use crate::year2019::intcode::{as_isize, Intcode, IntcodeData};
use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use bnum::types::I256;
use console::style;
use crossterm::{
    cursor::{Hide, MoveToNextLine, RestorePosition, SavePosition, Show},
    style::Print,
    ExecutableCommand, QueueableCommand,
};
use std::io::{stdout, Write};
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
    sync::mpsc::channel,
    thread::spawn,
};
#[cfg(feature = "intcode_debug")]
use {crate::year2019::intcode::DebugMessage, tracing::info};

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<IntcodeData, usize>(AoCYear::AOC2019, AoCDay::AOCD15, setup, find)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<IntcodeData, usize>(bench, AoCYear::AOC2019, AoCDay::AOCD15, setup, find)
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

#[derive(Clone, Copy, Debug)]
enum CoordKind {
    Wall,
    Empty,
    Oxygen,
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(intcodes: IntcodeData, _second_star: bool) -> Result<usize> {
    let (sender, receiver) = channel();
    let (send_a, mut amp_a) = Intcode::new(intcodes);
    let _ = amp_a.set_sender_opt(Some(sender));
    #[cfg(feature = "intcode_debug")]
    let (d_sender, d_receiver) = channel();
    #[cfg(feature = "intcode_debug")]
    {
        let _ = amp_a.set_debug_opt(Some(d_sender));
        let _ = amp_a.set_debug(false);
    }
    #[cfg(feature = "intcode_debug")]
    let _debug_handle = spawn(move || {
        while let Ok(res) = d_receiver.recv() {
            if let Ok(dm) = bincode::deserialize::<DebugMessage>(&res) {
                info!("{dm:?}");
            }
        }
    });

    let amp_a_handle = spawn(move || amp_a.start());

    let mut room_map = BTreeMap::new();
    let _ = room_map.insert((0_isize, 0_isize), CoordKind::Empty);
    let mut curr_coord = (0, 0);
    let mut curr_dir = 1;
    send_a.send(I256::ONE).unwrap_or_default();

    while let Ok(res) = receiver.recv() {
        match res {
            I256::ONE => {
                if curr_dir == 1 {
                    curr_coord = (curr_coord.0, curr_coord.1 - 1);
                    curr_dir = 3;
                    send_a.send(I256::THREE).unwrap_or_default();
                } else if curr_dir == 2 {
                    curr_coord = (curr_coord.0, curr_coord.1 + 1);
                    curr_dir = 4;
                    send_a.send(I256::FOUR).unwrap_or_default();
                } else if curr_dir == 3 {
                    curr_coord = (curr_coord.0 - 1, curr_coord.1);
                    curr_dir = 2;
                    send_a.send(I256::TWO).unwrap_or_default();
                } else if curr_dir == 4 {
                    curr_coord = (curr_coord.0 + 1, curr_coord.1);
                    curr_dir = 1;
                    send_a.send(I256::ONE).unwrap_or_default();
                } else {
                    return Err(anyhow!("Invalid direction"));
                }
                let _ = room_map.insert(curr_coord, CoordKind::Empty);
            }
            I256::ZERO => {
                let wall_coord = if curr_dir == 1 {
                    curr_dir = 4;
                    send_a.send(I256::FOUR).unwrap_or_default();
                    (curr_coord.0, curr_coord.1 - 1)
                } else if curr_dir == 2 {
                    curr_dir = 3;
                    send_a.send(I256::THREE).unwrap_or_default();
                    (curr_coord.0, curr_coord.1 + 1)
                } else if curr_dir == 3 {
                    curr_dir = 1;
                    send_a.send(I256::ONE).unwrap_or_default();
                    (curr_coord.0 - 1, curr_coord.1)
                } else if curr_dir == 4 {
                    curr_dir = 2;
                    send_a.send(I256::TWO).unwrap_or_default();
                    (curr_coord.0 + 1, curr_coord.1)
                } else {
                    return Err(anyhow!("Invalid direction"));
                };
                let _ = room_map.insert(wall_coord, CoordKind::Wall);
            }
            I256::TWO => {
                let oxy_coord = if curr_dir == 1 {
                    (curr_coord.0, curr_coord.1 - 1)
                } else if curr_dir == 2 {
                    (curr_coord.0, curr_coord.1 + 1)
                } else if curr_dir == 3 {
                    (curr_coord.0 - 1, curr_coord.1)
                } else if curr_dir == 4 {
                    (curr_coord.0 + 1, curr_coord.1)
                } else {
                    return Err(anyhow!("Invalid direction"));
                };
                let _ = room_map.insert(oxy_coord, CoordKind::Oxygen);
                eprintln!("Found oxygen at {oxy_coord:?}");
                break;
            }
            _ => return Err(anyhow!("Invalid response from Intcode")),
        }
    }

    screen(&room_map, false).unwrap_or_default();
    match amp_a_handle.join() {
        Ok(res) => match res {
            Ok(_r) => {}
            Err(e) => eprintln!("{e}"),
        },
        Err(e) => eprintln!("{e:?}"),
    }

    Ok(0)
}

fn screen(screen_data: &BTreeMap<(isize, isize), CoordKind>, restore: bool) -> Result<()> {
    let mut stdout = stdout();

    let min_x = screen_data
        .keys()
        .map(|(x, _)| *x)
        .min()
        .unwrap_or_default();
    let min_y = screen_data
        .keys()
        .map(|(_, y)| *y)
        .min()
        .unwrap_or_default();
    let max_x = screen_data
        .keys()
        .map(|(x, _)| *x)
        .max()
        .unwrap_or_default();
    let max_y = screen_data
        .keys()
        .map(|(_, y)| *y)
        .max()
        .unwrap_or_default();
    let _ = stdout.execute(Hide)?;
    let _ = stdout.queue(SavePosition)?;
    let _ = stdout.queue(Print("Score"))?;
    let _ = stdout.queue(MoveToNextLine(1))?;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if let Some(kind) = screen_data.get(&(x, y)) {
                match kind {
                    CoordKind::Wall => {
                        let _ = stdout.queue(Print(style("#").green().bold()))?;
                    }
                    CoordKind::Empty => {
                        if x == 0 && y == 0 {
                            let _ = stdout.queue(Print(style("S").red().bold()))?;
                        } else {
                            let _ = stdout.queue(Print('.'))?;
                        }
                    }
                    CoordKind::Oxygen => {
                        let _ = stdout.queue(Print(style("O").yellow().bold()))?;
                    }
                }
            } else {
                let _ = stdout.queue(Print(' '))?;
            }
        }
        let _ = stdout.queue(MoveToNextLine(1))?;
    }
    let _ = stdout.queue(MoveToNextLine(1))?;

    if restore {
        let _ = stdout.queue(RestorePosition)?;
    }
    let _ = stdout.execute(Show)?;
    stdout.flush()?;
    Ok(())
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<IntcodeData, usize>(AoCYear::AOC2019, AoCDay::AOCD15, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<IntcodeData, usize>(bench, AoCYear::AOC2019, AoCDay::AOCD15, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: IntcodeData) -> usize {
    find_res(data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r">";

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
