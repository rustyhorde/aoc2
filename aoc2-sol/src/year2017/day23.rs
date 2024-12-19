// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2017 ---**
//!
//! **--- Day 23: Coprocessor Conflagration ---**
//!
//! You decide to head directly to the CPU and fix the printer from there. As you get close, you find an experimental coprocessor doing so much work that the local programs are afraid it will halt and catch fire. This would cause serious issues for the rest of the computer, so you head in and see what you can do.
//!
//! The code it's running seems to be a variant of the kind you saw recently on that tablet. The general functionality seems very similar, but some of the instructions are different:
//!
//! ```text
//!     set X Y sets register X to the value of Y.
//!     sub X Y decreases register X by the value of Y.
//!     mul X Y sets register X to the result of multiplying the value contained in register X by the value of Y.
//!     jnz X Y jumps with an offset of the value of Y, but only if the value of X is not zero. (An offset of 2 skips the next instruction, an offset of -1 jumps to the previous instruction, and so on.)
//!
//!     Only the instructions listed above are used. The eight registers here, named a through h, all start at 0.
//! ```
//!
//! The coprocessor is currently set to some kind of debug mode, which allows for testing, but prevents it from doing any meaningful work.
//!
//! If you run the program (your puzzle input), how many times is the mul instruction invoked?
//!
//! **--- Part Two ---**
//!
//! Now, it's time to fix the problem.
//!
//! The debug mode switch is wired directly to register a. You flip the switch, which makes register a now start at 1 when the program is executed.
//!
//! Immediately, the coprocessor begins to overheat. Whoever wrote this program obviously didn't choose a very efficient implementation. You'll need to optimize the program if it has any hope of completing before Santa needs that printer working.
//!
//! The coprocessor's ultimate goal is to determine the final value left in register h once the program completes. Technically, if it had that... it wouldn't even need to run the program.
//!
//! After setting register a to 1, if the program were to run to completion, what value would be left in register h?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

/// A value can either be a pointer to a register or a number.
#[derive(Clone, Debug, Eq, PartialEq)]
enum Value {
    /// A number value.
    Number(i64),
    /// A registe pointer value.
    Register(String),
}

type CoData = HashMap<i64, (String, String, Option<Value>)>;

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<CoData, usize>(AoCYear::AOC2017, AoCDay::AOCD23, setup, find).map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<CoData, usize>(bench, AoCYear::AOC2017, AoCDay::AOCD23, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> CoData {
    setup_br(reader).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T) -> Result<CoData>
where
    T: BufRead,
{
    let mut commands: HashMap<i64, (String, String, Option<Value>)> = HashMap::new();
    for (idx, line) in valid_lines(reader).enumerate() {
        let _res = commands.insert(i64::try_from(idx)?, parse_command(&line)?);
    }
    Ok(commands)
}

/// Parse a command into (command, register, value)
fn parse_command(command: &str) -> Result<(String, String, Option<Value>)> {
    let token_strs: Vec<&str> = command.split(' ').collect();

    if token_strs.len() == 3 {
        let value = if let Ok(number) = token_strs[2].parse::<i64>() {
            Value::Number(number)
        } else {
            Value::Register(token_strs[2].to_string())
        };
        Ok((
            token_strs[0].to_string(),
            token_strs[1].to_string(),
            Some(value),
        ))
    } else if token_strs.len() == 2 {
        Ok((token_strs[0].to_string(), token_strs[1].to_string(), None))
    } else {
        Err(anyhow!("Invalid command"))
    }
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: CoData) -> usize {
    find_res(data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(commands: CoData, second_star: bool) -> Result<usize> {
    let mut register_map: HashMap<String, i64> = HashMap::new();

    initialize_register_map(&commands, &mut register_map)?;

    let mul_count = if second_star {
        let b = 106_700;
        let c = 123_700;
        let mut h = 0;
        for b in (b..=c).step_by(17) {
            if !primal::is_prime(b) {
                h += 1
            }
        }
        h
    } else {
        let mut id = 0;
        let mut count = 0;
        loop {
            if id < 0 || id == commands.len() as i64 {
                break;
            }
            let next_command = commands.get(&id).ok_or(anyhow!("invalid command"))?;
            let (new_id, new_mul_count) =
                run_command((id, next_command), &mut register_map, count)?;
            id = new_id;
            count = new_mul_count;
        }
        count
    };

    Ok(mul_count)
}

/// Initialize the register map.
fn initialize_register_map(
    commands: &HashMap<i64, (String, String, Option<Value>)>,
    register_map: &mut HashMap<String, i64>,
) -> Result<()> {
    for (_, command) in commands.iter() {
        let _ = register_map.entry(command.1.clone()).or_insert(0);
    }
    Ok(())
}

/// Run a command
fn run_command(
    (id, command): (i64, &(String, String, Option<Value>)),
    register_map: &mut HashMap<String, i64>,
    count: usize,
) -> Result<(i64, usize)> {
    let cmd = &command.0;
    let register = &command.1;
    let value = &command.2;
    let mut mul_count = count;

    match &cmd[..] {
        "set" => {
            let actual_value = match *value {
                Some(Value::Number(x)) => x,
                Some(Value::Register(ref x)) => {
                    *register_map.get(x).ok_or(anyhow!("invalid register"))?
                }
                _ => return Err(anyhow!("Invalid set command")),
            };
            *register_map
                .get_mut(register)
                .ok_or(anyhow!("invalid register"))? = actual_value;
        }
        "sub" => {
            let actual_value = match *value {
                Some(Value::Number(x)) => x,
                Some(Value::Register(ref x)) => {
                    *register_map.get(x).ok_or(anyhow!("invalid register"))?
                }
                _ => return Err(anyhow!("Invalid set command")),
            };
            let x = register_map
                .get_mut(register)
                .ok_or(anyhow!("invalid register"))?;
            *x -= actual_value;
        }
        "mul" => {
            let actual_value = match *value {
                Some(Value::Number(x)) => x,
                Some(Value::Register(ref x)) => {
                    *register_map.get(x).ok_or(anyhow!("invalid register"))?
                }
                _ => return Err(anyhow!("Invalid set command")),
            };
            let x = register_map
                .get_mut(register)
                .ok_or(anyhow!("invalid register"))?;
            *x *= actual_value;
            mul_count += 1;
        }
        "jnz" => {
            let should_jump = if let Ok(val) = register.parse::<i64>() {
                val
            } else {
                *register_map
                    .get(register)
                    .ok_or(anyhow!("invalid register"))?
            };

            if should_jump != 0 {
                let actual_value = match *value {
                    Some(Value::Number(x)) => x,
                    Some(Value::Register(ref x)) => {
                        *register_map.get(x).ok_or(anyhow!("invalid register"))?
                    }
                    _ => return Err(anyhow!("Invalid set command")),
                };
                return Ok((id + actual_value, mul_count));
            }
        }
        _ => {}
    }

    Ok((id + 1, mul_count))
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<CoData, usize>(AoCYear::AOC2017, AoCDay::AOCD23, setup, find2).map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<CoData, usize>(bench, AoCYear::AOC2017, AoCDay::AOCD23, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: CoData) -> usize {
    find_res(data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {}

#[cfg(test)]
mod two_star {}
