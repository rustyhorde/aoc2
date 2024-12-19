// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code ---**
//!
//! **--- Day 25: The Halting Problem ---**
//!
//! Following the twisty passageways deeper and deeper into the CPU, you finally reach the core of the computer. Here, in the expansive central chamber, you find a grand apparatus that fills the entire room, suspended nanometers above your head.
//!
//! You had always imagined CPUs to be noisy, chaotic places, bustling with activity. Instead, the room is quiet, motionless, and dark.
//!
//! Suddenly, you and the CPU's garbage collector startle each other. "It's not often we get many visitors here!", he says. You inquire about the stopped machinery.
//!
//! "It stopped milliseconds ago; not sure why. I'm a garbage collector, not a doctor." You ask what the machine is for.
//!
//! "Programs these days, don't know their origins. That's the Turing machine! It's what makes the whole computer work." You try to explain that Turing machines are merely models of computation, but he cuts you off. "No, see, that's just what they want you to think. Ultimately, inside every CPU, there's a Turing machine driving the whole thing! Too bad this one's broken. We're doomed!"
//!
//! You ask how you can help. "Well, unfortunately, the only way to get the computer running again would be to create a whole new Turing machine from scratch, but there's no way you can-" He notices the look on your face, gives you a curious glance, shrugs, and goes back to sweeping the floor.
//!
//! You find the Turing machine blueprints (your puzzle input) on a tablet in a nearby pile of debris. Looking back up at the broken Turing machine above, you can start to identify its parts:
//!
//! ```text
//!     A tape which contains 0 repeated infinitely to the left and right.
//!     A cursor, which can move left or right along the tape and read or write values at its current position.
//!     A set of states, each containing rules about what to do based on the current value under the cursor.
//! ```
//!
//! Each slot on the tape has two possible values: 0 (the starting value for all slots) and 1. Based on whether the cursor is pointing at a 0 or a 1, the current state says what value to write at the current position of the cursor, whether to move the cursor left or right one slot, and which state to use next.
//!
//! For example, suppose you found the following blueprint:
//!
//! ```text
//! Begin in state A.
//! Perform a diagnostic checksum after 6 steps.
//!
//! In state A:
//!   If the current value is 0:
//!     - Write the value 1.
//!     - Move one slot to the right.
//!     - Continue with state B.
//!   If the current value is 1:
//!     - Write the value 0.
//!     - Move one slot to the left.
//!     - Continue with state B.
//!
//! In state B:
//!   If the current value is 0:
//!     - Write the value 1.
//!     - Move one slot to the left.
//!     - Continue with state A.
//!   If the current value is 1:
//!     - Write the value 1.
//!     - Move one slot to the right.
//!     - Continue with state A.
//! ```
//!
//! Running it until the number of steps required to take the listed diagnostic checksum would result in the following tape configurations (with the cursor marked in square brackets):
//!
//! ```text
//! ... 0  0  0 [0] 0  0 ... (before any steps; about to run state A)
//! ... 0  0  0  1 [0] 0 ... (after 1 step;     about to run state B)
//! ... 0  0  0 [1] 1  0 ... (after 2 steps;    about to run state A)
//! ... 0  0 [0] 0  1  0 ... (after 3 steps;    about to run state B)
//! ... 0 [0] 1  0  1  0 ... (after 4 steps;    about to run state A)
//! ... 0  1 [1] 0  1  0 ... (after 5 steps;    about to run state B)
//! ... 0  1  1 [0] 1  0 ... (after 6 steps;    about to run state A)
//! ```
//!
//! The CPU can confirm that the Turing machine is working by taking a diagnostic checksum after a specific number of steps (given in the blueprint). Once the specified number of steps have been executed, the Turing machine should pause; once it does, count the number of times 1 appears on the tape. In the above example, the diagnostic checksum is 3.
//!
//! Recreate the Turing machine and save the computer! What is the diagnostic checksum it produces once it's working again?
//!
//! **--- Part Two ---**
//!
//! The Turing machine, and soon the entire computer, springs back to life. A console glows dimly nearby, awaiting your command.
//!
//! ```text
//! > reboot printer
//! Error: That command requires priority 50. You currently have priority 0.
//! You must deposit 50 stars to increase your priority to the required level.
//! ```
//! The console flickers for a moment, and then prints another message:
//!
//! ```text
//! Star accepted.
//! You must deposit 49 stars to increase your priority to the required level.
//! ```
//!
//! The garbage collector winks at you, then continues sweeping.

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::{anyhow, Error, Result};
use regex::Regex;
use std::{
    collections::BTreeMap,
    fmt,
    fs::File,
    io::{BufRead, BufReader},
};

/// The direction to move on the tape
#[derive(Clone, Copy, Debug, Default)]
enum Move {
    #[default]
    /// Left
    Left,
    /// Right
    Right,
}

impl TryFrom<&str> for Move {
    type Error = Error;
    fn try_from(val: &str) -> Result<Self> {
        Ok(match val {
            "left" => Move::Left,
            "right" => Move::Right,
            _ => return Err(anyhow!(format!("Invalid move value: {val}"))),
        })
    }
}

impl fmt::Display for Move {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "{}",
            match *self {
                Move::Left => "left",
                Move::Right => "right",
            }
        )
    }
}

/// A state definition.
#[derive(Clone, Copy, Debug, Default)]
struct State {
    /// What to write if the current slot is 0.
    zero_write: u8,
    /// Where to move if the current slot is 0.
    zero_move: Move,
    /// What state is next if the current slot is 0.
    zero_next: char,
    /// What to write if the current slot is 1.
    one_write: u8,
    /// Where to move if the current slot is 1.
    one_move: Move,
    /// What state is next if the current slot is 1.
    one_next: char,
}

impl fmt::Display for State {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "State {{ ")?;
        write!(fmt, "zero_write: {}, ", self.zero_write)?;
        write!(fmt, "zero_move: {}, ", self.zero_move)?;
        write!(fmt, "zero_next: {}, ", self.zero_next)?;
        write!(fmt, "one_write: {}, ", self.one_write)?;
        write!(fmt, "one_move: {}, ", self.one_move)?;
        write!(fmt, "one_next: {} }}", self.one_next)
    }
}

type HaltingData = (BTreeMap<char, State>, char, usize);

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<HaltingData, usize>(AoCYear::AOC2017, AoCDay::AOCD25, setup, find)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<HaltingData, usize>(bench, AoCYear::AOC2017, AoCDay::AOCD25, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> HaltingData {
    setup_br(reader).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps, clippy::too_many_lines)]
fn setup_br<T>(reader: T) -> Result<HaltingData>
where
    T: BufRead,
{
    let mut states: BTreeMap<char, State> = BTreeMap::new();

    let begin_re = Regex::new(r"^Begin in state ([A-Z])\.$")?;
    let dc_re = Regex::new(r"^Perform a diagnostic checksum after (\d+) steps\.$")?;
    let in_state_re = Regex::new(r"^In state ([A-Z]):$")?;
    let if_curr_re = Regex::new(r"If the current value is (\d+):$")?;
    let write_val_re = Regex::new(r" - Write the value (\d+)\.$")?;
    let move_re = Regex::new(r"- Move one slot to the (right|left)\.$")?;
    let cont_re = Regex::new(r"- Continue with state ([A-Z])\.$")?;

    let mut start_state = 'A';
    let mut step_count = 0;
    let mut parsing_state = false;
    let mut curr_state = 'A';
    let mut curr_val = 0;

    for line in valid_lines(reader) {
        if begin_re.is_match(&line) {
            let caps = begin_re
                .captures(&line)
                .ok_or(anyhow!("invalid begin captures"))?;
            let state_str = caps.get(1).ok_or(anyhow!("invalid state value"))?.as_str();
            let val = state_str.parse::<char>()?;
            start_state = val;
        } else if dc_re.is_match(&line) {
            let caps = dc_re
                .captures(&line)
                .ok_or(anyhow!("invalid diagnostic checksum captures"))?;
            let steps_str = caps
                .get(1)
                .ok_or(anyhow!("invalid diagnostic checksum value"))?
                .as_str();
            let steps = steps_str.parse::<usize>()?;
            step_count = steps;
        } else if in_state_re.is_match(&line) {
            let caps = in_state_re
                .captures(&line)
                .ok_or(anyhow!("invalid in state captures"))?;
            let state_str = caps
                .get(1)
                .ok_or(anyhow!("invalid in state value"))?
                .as_str();
            let val = state_str.parse::<char>()?;
            parsing_state = true;
            curr_state = val;
            let _ = states.insert(val, State::default());
        } else if if_curr_re.is_match(&line) && parsing_state {
            let caps = if_curr_re
                .captures(&line)
                .ok_or(anyhow!("invalid if current value captures"))?;
            let val_str = caps
                .get(1)
                .ok_or(anyhow!("invalid if current value"))?
                .as_str();
            let val = val_str.parse::<u8>()?;
            curr_val = val;
        } else if write_val_re.is_match(&line) && parsing_state {
            let caps = write_val_re
                .captures(&line)
                .ok_or(anyhow!("invalid write value captures"))?;
            let val_str = caps.get(1).ok_or(anyhow!("invalid write value"))?.as_str();
            let val = val_str.parse::<u8>()?;
            let state_ptr = states.entry(curr_state).or_default();

            if curr_val == 0 {
                state_ptr.zero_write = val;
            } else if curr_val == 1 {
                state_ptr.one_write = val;
            } else {
                return Err(anyhow!("Invalid curr value"));
            }
        } else if move_re.is_match(&line) {
            let caps = move_re
                .captures(&line)
                .ok_or(anyhow!("invalid move captures"))?;
            let move_str = caps.get(1).ok_or(anyhow!("invalid move value"))?.as_str();
            let state_ptr = states.entry(curr_state).or_default();

            if curr_val == 0 {
                state_ptr.zero_move = TryFrom::try_from(move_str)?;
            } else if curr_val == 1 {
                state_ptr.one_move = TryFrom::try_from(move_str)?;
            } else {
                return Err(anyhow!("Invalid curr value"));
            }
        } else if cont_re.is_match(&line) {
            let caps = cont_re
                .captures(&line)
                .ok_or(anyhow!("invalid continue captures"))?;
            let cont_str = caps
                .get(1)
                .ok_or(anyhow!("invalid continue value"))?
                .as_str();
            let val = cont_str.parse::<char>()?;
            let state_ptr = states.entry(curr_state).or_default();

            if curr_val == 0 {
                state_ptr.zero_next = val;
            } else if curr_val == 1 {
                state_ptr.one_next = val;
            } else {
                return Err(anyhow!("Invalid curr value"));
            }
        } else if line.is_empty() && parsing_state {
            parsing_state = false;
        } else if line.is_empty() {
            // Do nothing.
        } else {
            return Err(anyhow!(format!("Unable to parse line: {line}")));
        }
    }
    Ok((states, start_state, step_count))
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: HaltingData) -> usize {
    find_res(data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(data: HaltingData, _second_star: bool) -> Result<usize> {
    let (states, start_state, step_count) = data;
    let mut tape: Vec<u8> = vec![0; 10_000_000];
    let mut curr_idx = 5_000_000;
    let mut curr_state = start_state;

    for _ in 0..step_count {
        let tape_val = tape
            .get_mut(curr_idx)
            .ok_or(anyhow!("invalid tape value"))?;
        let state = states
            .get(&curr_state)
            .ok_or(anyhow!("invalid state value"))?;

        if *tape_val == 0 {
            *tape_val = state.zero_write;
            match state.zero_move {
                Move::Left => curr_idx -= 1,
                Move::Right => curr_idx += 1,
            }
            curr_state = state.zero_next;
        } else {
            *tape_val = state.one_write;
            match state.one_move {
                Move::Left => curr_idx -= 1,
                Move::Right => curr_idx += 1,
            }
            curr_state = state.one_next;
        }
    }

    Ok(bytecount::count(&tape, 1))
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<HaltingData, usize>(AoCYear::AOC2017, AoCDay::AOCD25, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<HaltingData, usize>(bench, AoCYear::AOC2017, AoCDay::AOCD25, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: HaltingData) -> usize {
    find_res(data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {}

#[cfg(test)]
mod two_star {}
