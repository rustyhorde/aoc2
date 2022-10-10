// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! I Heard You Like Registers
//!
//! **--- Day 8: I Heard You Like Registers ---**
//!
//! **--- Part 1 ---**
//!
//! You receive a signal directly from the CPU. Because of your recent
//! assistance with jump instructions, it would like you to compute the
//! result of a series of unusual register instructions.
//!
//! Each instruction consists of several parts: the register to modify,
//! whether to increase or decrease that register's value, the amount by
//! which to increase or decrease it, and a condition. If the condition fails,
//! skip the instruction without modifying the register. The registers all
//! start at `0`. The instructions look like this:
//!
//! ```text
//! b inc 5 if a > 1
//! a inc 1 if b < 5
//! c dec -10 if a >= 1
//! c inc -20 if c == 10
//! ```
//!
//! These instructions would be processed as follows:
//!
//! ```text
//! Because a starts at 0, it is not greater than 1, and so b is not modified.
//! a is increased by 1 (to 1) because b is less than 5 (it is 0).
//! c is decreased by -10 (to 10) because a is now greater than or equal to 1 (it is 1).
//! c is increased by -20 (to -10) because c is equal to 10.
//! ```
//!
//! After this process, the largest value in any register is `1`.
//!
//! You might also encounter `<=` (less than or equal to) or `!=` (not equal to).
//! However, the CPU doesn't have the bandwidth to tell you what all the registers
//! are named, and leaves that to you to determine.
//!
//! What is the largest value in any register after completing the instructions
//! in your puzzle input?
//!
//! **--- Part Two ---**
//!
//! To be safe, the CPU also needs to know the highest value held in any register
//! during this process so that it can decide how much memory to allocate to these
//! operations. For example, in the above instructions, the highest value ever held
//! was `10` (in register `c` after the third instruction was evaluated).

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{get_cap, get_cap_x, print_err, run_solution, valid_lines},
};
use anyhow::{anyhow, Error, Result};
use regex::Regex;
use std::{
    collections::BTreeMap,
    convert::TryFrom,
    fs::File,
    io::{BufRead, BufReader},
};

// Register commands
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Command {
    // Increment
    Inc,
    // Decrement
    Dec,
}

impl TryFrom<String> for Command {
    type Error = Error;
    fn try_from(command: String) -> Result<Self> {
        match &command[..] {
            "inc" => Ok(Command::Inc),
            "dec" => Ok(Command::Dec),
            _ => Err(anyhow!(format!("invalid command: {command}"))),
        }
    }
}

// Supported condition operators
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Operator {
    // `>`
    GreaterThan,
    // `>=`
    GreaterThanEqualTo,
    // `==`
    EqualTo,
    // `<=`
    LessThanEqualTo,
    // `<`
    LessThan,
    // `!=`
    NotEqualTo,
}

impl TryFrom<String> for Operator {
    type Error = Error;
    fn try_from(command: String) -> Result<Self> {
        match &command[..] {
            ">" => Ok(Operator::GreaterThan),
            ">=" => Ok(Operator::GreaterThanEqualTo),
            "==" => Ok(Operator::EqualTo),
            "<=" => Ok(Operator::LessThanEqualTo),
            "<" => Ok(Operator::LessThan),
            "!=" => Ok(Operator::NotEqualTo),
            _ => Err(anyhow!(format!("invalid operator: {command}"))),
        }
    }
}

// A condition that must be met before operating on a register.
#[derive(Clone, Debug, Eq, PartialEq)]
struct Condition {
    /// The register to check.
    register: String,
    /// The operater, e.g. `<`, `>`
    op: Operator,
    /// The value on the right of the condition.
    value: isize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Instruction {
    register: String,
    cmd: Command,
    value: isize,
    condition: Condition,
}

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_solution::<isize>(AoCYear::AOC2017, AoCDay::AOCD08, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> isize {
    find_br(reader).map_err(print_err).unwrap_or_default()
}

fn find_br<T>(reader: T) -> Result<isize>
where
    T: BufRead,
{
    execute(reader, false)
}

fn execute<T>(reader: T, part2: bool) -> Result<isize>
where
    T: BufRead,
{
    let (mut regs, instructions) = setup(reader)?;
    let mut max = isize::MIN;

    for inst in &instructions {
        if check_condition(&regs, &inst.condition)? {
            execute_instruction(&mut regs, inst)?;
        }

        if part2 {
            let curr_max = max_reg(&regs)?;
            if max < curr_max {
                max = curr_max;
            }
        }
    }

    if part2 {
        Ok(max)
    } else {
        max_reg(&regs)
    }
}

fn setup<T>(reader: T) -> Result<(BTreeMap<String, isize>, Vec<Instruction>)>
where
    T: BufRead,
{
    let inst_re = Regex::new(r"^([a-z]+) (inc|dec) (-?\d+) if ([a-z]+) ([=<>!]{1,2}) (-?\d+)$")?;
    let mut regs = BTreeMap::new();
    let mut inst = vec![];
    for line in valid_lines(reader) {
        for caps in inst_re.captures_iter(&line) {
            let reg = get_cap(1, &caps)?;
            let command = Command::try_from(get_cap(2, &caps)?)?;
            let value = get_cap_x::<isize>(3, &caps)?;
            let cond_reg = get_cap(4, &caps)?;
            let op = Operator::try_from(get_cap(5, &caps)?)?;
            let cond_val = get_cap_x::<isize>(6, &caps)?;
            let cond = Condition {
                register: cond_reg.clone(),
                op,
                value: cond_val,
            };
            let instruction = Instruction {
                register: reg.clone(),
                cmd: command,
                value,
                condition: cond,
            };
            let _ = regs.entry(reg).or_default();
            let _ = regs.entry(cond_reg).or_default();
            inst.push(instruction);
        }
    }
    Ok((regs, inst))
}

/// Check the command condition
fn check_condition(regs: &BTreeMap<String, isize>, condition: &Condition) -> Result<bool> {
    let register_value = regs
        .get(&condition.register)
        .ok_or_else(|| anyhow!("Cannot read value from register"))?;
    let condition_value = &condition.value;

    match condition.op {
        Operator::GreaterThan => Ok(register_value > condition_value),
        Operator::GreaterThanEqualTo => Ok(register_value >= condition_value),
        Operator::EqualTo => Ok(register_value == condition_value),
        Operator::LessThanEqualTo => Ok(register_value <= condition_value),
        Operator::LessThan => Ok(register_value < condition_value),
        Operator::NotEqualTo => Ok(register_value != condition_value),
    }
}

/// Execute the given command
fn execute_instruction(regs: &mut BTreeMap<String, isize>, inst: &Instruction) -> Result<()> {
    let map_entry = regs
        .get_mut(&inst.register)
        .ok_or_else(|| anyhow!("blah"))?;

    match inst.cmd {
        Command::Inc => *map_entry += inst.value,
        Command::Dec => *map_entry -= inst.value,
    }

    Ok(())
}

fn max_reg(regs: &BTreeMap<String, isize>) -> Result<isize> {
    regs.values()
        .copied()
        .max()
        .ok_or_else(|| anyhow!("No max found"))
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<isize>(AoCYear::AOC2017, AoCDay::AOCD08, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> isize {
    find2_br(reader).map_err(print_err).unwrap_or_default()
}

fn find2_br<T>(reader: T) -> Result<isize>
where
    T: BufRead,
{
    execute(reader, true)
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1))?, 1);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find2_br(Cursor::new(TEST_1))?, 10);
        Ok(())
    }
}
