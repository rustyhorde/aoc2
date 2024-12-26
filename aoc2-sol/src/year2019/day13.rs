// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2019 ---**
//!
//! --- Day 13: Care Package ---
//!  
//!  As you ponder the solitude of space and the ever-increasing three-hour round trip for messages between you and Earth, you notice that the Space Mail Indicator Light is blinking. To help keep you sane, the Elves have sent you a care package.
//!  
//!  It's a new game for the ship's arcade cabinet! Unfortunately, the arcade is all the way on the other end of the ship. Surely, it won't be hard to build your own - the care package even comes with schematics.
//!  
//!  The arcade cabinet runs Intcode software like the game the Elves sent (your puzzle input). It has a primitive screen capable of drawing square tiles on a grid. The software draws tiles to the screen with output instructions: every three output instructions specify the x position (distance from the left), y position (distance from the top), and tile id. The tile id is interpreted as follows:
//!  
//! ```text
//!      0 is an empty tile. No game object appears in this tile.
//!      1 is a wall tile. Walls are indestructible barriers.
//!      2 is a block tile. Blocks can be broken by the ball.
//!      3 is a horizontal paddle tile. The paddle is indestructible.
//!      4 is a ball tile. The ball moves diagonally and bounces off objects.
//!  ```
//!
//!  For example, a sequence of output values like 1,2,3,6,5,4 would draw a horizontal paddle tile (1 tile from the left and 2 tiles from the top) and a ball tile (6 tiles from the left and 5 tiles from the top).
//!  
//!  Start the game. How many block tiles are on the screen when the game exits?
//!
//! **--- Part Two ---**
//!
//! The game didn't run because you didn't put in any quarters. Unfortunately, you did not bring any quarters. Memory address 0 represents the number of quarters that have been inserted; set it to 2 to play for free.
//!
//! The arcade cabinet has a joystick that can move left and right. The software reads the position of the joystick with input instructions:
//!
//! ```text
//!     If the joystick is in the neutral position, provide 0.
//!     If the joystick is tilted to the left, provide -1.
//!     If the joystick is tilted to the right, provide 1.
//! ```
//!
//! The arcade cabinet also has a segment display capable of showing a single number that represents the player's current score. When three output instructions specify X=-1, Y=0, the third output instruction is not a tile; the value instead specifies the new score to show in the segment display. For example, a sequence of output values like -1,0,12345 would show 12345 as the player's current score.
//!
//! Beat the game by breaking all the blocks. What is your score after the last block is broken?

#[cfg(feature = "intcode_debug")]
use crate::year2019::intcode::DebugMessage;
use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
    year2019::intcode::{as_isize, Intcode, IntcodeData},
};
use anyhow::{anyhow, Result};
use bnum::types::I256;
use console::style;
use crossterm::cursor::{
    Hide, MoveToColumn, MoveToNextLine, MoveToRow, RestorePosition, SavePosition, Show,
};
use crossterm::style::Print;
use crossterm::{ExecutableCommand, QueueableCommand};
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
    sync::mpsc::channel,
    thread::spawn,
};
#[cfg(feature = "intcode_debug")]
use tracing::info;

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](AoCYear) and
///   [`AoCDay`](AoCDay) cannot be read.
/// * This function will error if the elapsed [`Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<IntcodeData, usize>(AoCYear::AOC2019, AoCDay::AOCD13, setup, find)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<IntcodeData, usize>(bench, AoCYear::AOC2019, AoCDay::AOCD13, setup, find)
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
fn find_res(mut intcodes: IntcodeData, second_star: bool) -> Result<usize> {
    let (sender, receiver) = channel();
    if second_star {
        intcodes[0] = I256::from(2);
    }
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
    // Start the chaos
    // let input = if second_star {
    //     I256::from(2)
    // } else {
    //     I256::ZERO
    // };
    // send_a.send(input)?;
    let input = vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for i in input {
        send_a.send(I256::from(i)).unwrap_or_default();
    }

    let mut grid_map = BTreeMap::new();
    let mut out_count = 0;
    let mut x = isize::MAX;
    let mut y = isize::MAX;
    let mut score = usize::MIN;
    let mut after_initial = false;

    while let Ok(res) = receiver.recv() {
        if out_count == 0 {
            x = isize::try_from(res).unwrap_or_default();
            out_count += 1;
        } else if out_count == 1 {
            y = isize::try_from(res).unwrap_or_default();
            out_count += 1;
        } else if out_count == 2 {
            if x == -1 && y == 0 {
                score += usize::try_from(res).unwrap_or_default();
                out_count = 0;
                x = isize::MAX;
                y = isize::MAX;
                continue;
            }
            let id = usize::try_from(res).unwrap_or_default();
            let ch = match id {
                0 => ' ',
                1 => '#',
                2 => '-',
                3 => '_',
                4 => 'O',
                _ => return Err(anyhow!("Invalid id: {}", id)),
            };
            let _ = grid_map.entry((x, y)).and_modify(|x| *x = ch).or_insert(ch);

            screen(&grid_map, true, score).unwrap_or_default();
            if after_initial {
                sleep(Duration::from_millis(100));
            }
            if x == 37 && y == 21 {
                after_initial = true;
            }
            x = isize::MAX;
            y = isize::MAX;
            out_count = 0;
        } else {
            return Err(anyhow!("Invalid output count: {}", out_count));
        }
    }

    screen(&grid_map, false, score).unwrap_or_default();

    match amp_a_handle.join() {
        Ok(res) => match res {
            Ok(_r) => {}
            Err(e) => eprintln!("{e}"),
        },
        Err(e) => eprintln!("{e:?}"),
    }
    Ok(grid_map.values().filter(|x| **x == '-').count())
}

fn screen(screen_data: &BTreeMap<(isize, isize), char>, restore: bool, score: usize) -> Result<()> {
    let mut stdout = stdout();

    let _ = stdout.execute(Hide)?;
    let _ = stdout.queue(SavePosition)?;
    let _ = stdout.queue(MoveToNextLine(1))?;
    let _ = stdout.queue(Print(format!("Score: {score}")))?;
    let _ = stdout.queue(MoveToNextLine(1))?;
    for ((x, y), ch) in screen_data {
        let _ = stdout.queue(MoveToColumn(u16::try_from(*x)?))?;
        let _ = stdout.queue(MoveToRow(u16::try_from(*y + 5)?))?;
        match ch {
            '#' => {
                let _ = stdout.queue(Print(style(ch.to_string()).green().bold()))?;
            }
            '-' => {
                let _ = stdout.queue(Print(style(ch.to_string()).red().bold()))?;
            }
            '_' => {
                let _ = stdout.queue(Print(style(ch.to_string()).yellow().bold()))?;
            }
            'O' => {
                let _ = stdout.queue(Print(style(ch.to_string()).cyan().bold()))?;
            }
            ' ' => {
                let _ = stdout.queue(Print(ch))?;
            }
            _ => {}
        }
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
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](AoCYear) and
///   [`AoCDay`](AoCDay) cannot be read.
/// * This function will error if the elapsed [`Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<IntcodeData, usize>(AoCYear::AOC2019, AoCDay::AOCD13, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<IntcodeData, usize>(bench, AoCYear::AOC2019, AoCDay::AOCD13, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: IntcodeData) -> usize {
    find_res(data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {}

#[cfg(test)]
mod two_star {}
