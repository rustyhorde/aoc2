// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Clock Signal
//!
//! **--- Day 25: Clock Signal ---**
//!
//! You open the door and find yourself on the roof. The city sprawls away from
//! you for miles and miles.
//!
//! There's not much time now - it's already Christmas, but you're nowhere near
//! the North Pole, much too far to deliver these stars to the sleigh in time.
//!
//! However, maybe the huge antenna up here can offer a solution. After all,
//! the sleigh doesn't need the stars, exactly; it needs the timing data they
//! provide, and you happen to have a massive signal generator right here.
//!
//! You connect the stars you have to your prototype computer, connect that to
//! the antenna, and begin the transmission.
//!
//! Nothing happens.
//!
//! You call the service number printed on the side of the antenna and quickly
//! explain the situation. "I'm not sure what kind of equipment you have connected
//! over there," he says, "but you need a clock signal." You try to explain that
//! this is a signal for a clock.
//!
//! "No, no, a clock signal - timing information so the antenna computer knows
//! how to read the data you're sending it. An endless, alternating pattern of
//! 0, 1, 0, 1, 0, 1, 0, 1, 0, 1...." He trails off.
//!
//! You ask if the antenna can handle a clock signal at the frequency you would
//! need to use for the data from the stars. "There's no way it can! The only
//! antenna we've installed capable of that is on top of a top-secret Easter Bunny
//! installation, and you're definitely not-" You hang up the phone.
//!
//! You've extracted the antenna's clock signal generation assembunny code
//! (your puzzle input); it looks mostly compatible with code you worked on
//! just recently.
//!
//! This antenna code, being a signal generator, uses one extra instruction:
//!
//! ```text
//! out x transmits x (either an integer or the value of a register) as the next value for the clock signal.
//! ```
//!
//! The code takes a value (via register `a`) that describes the signal to generate,
//!  but you're not sure how it's used. You'll have to find the input to produce the right
//! signal through experimentation.
//!
//! What is the lowest positive integer that can be used to initialize register `a` and cause the code to
//! output a clock signal of 0, 1, 0, 1... repeating forever?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{get_cap_x, print_err, run_solution, valid_lines},
};
use anyhow::{anyhow, Context, Result};
use console::Term;
use regex::Regex;
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader, Write},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Instructions {
    CopyVal(isize, char),
    CopyReg(char, char),
    Increment(char),
    Decrement(char),
    JumpNotZeroVal(isize, isize),
    JumpNotZeroReg(char, isize),
    Out(char),
}

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_solution::<isize>(AoCYear::AOC2016, AoCDay::AOCD25, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> isize {
    find_br(reader).map_err(print_err).unwrap_or_default()
}

fn find_br<T>(reader: T) -> Result<isize>
where
    T: BufRead,
{
    run_assembunny(reader, false)
}

fn run_assembunny<T>(reader: T, _part2: bool) -> Result<isize>
where
    T: BufRead,
{
    let mut regs: BTreeMap<char, isize> = BTreeMap::new();
    let mut inst = vec![];

    setup(reader, &mut regs, &mut inst)?;

    let mut inst_ptr: isize = 0;
    let mut counter = 0;
    *regs.entry('a').or_default() = 196;

    // let mut term = Term::stdout();
    loop {
        let ptr = usize::try_from(inst_ptr)?;
        if ptr >= inst.len() || counter == 100 {
            break;
        }
        // show_regs(&regs, &mut term)?;

        if let Some(instruction) = inst.get(ptr) {
            match instruction {
                Instructions::CopyReg(register_1, register_2) => {
                    let val = regs.entry(*register_1).or_default();
                    *regs.entry(*register_2).or_default() = *val;
                }
                Instructions::CopyVal(val, reg) => {
                    *regs.entry(*reg).or_default() = *val;
                }
                Instructions::Increment(reg) => {
                    *regs.entry(*reg).or_default() += 1;
                }
                Instructions::Decrement(reg) => {
                    *regs.entry(*reg).or_default() -= 1;
                }
                Instructions::JumpNotZeroReg(reg, offset) => {
                    if get_reg(&regs, *reg)? > 0 {
                        inst_ptr += offset;
                        continue;
                    }
                }
                Instructions::JumpNotZeroVal(val, offset) => {
                    if *val > 0 {
                        inst_ptr += offset;
                        continue;
                    }
                }
                Instructions::Out(reg) => {
                    print!("{}", get_reg(&regs, *reg)?);
                    counter += 1;
                }
            }
        } else {
            return Err(anyhow!(format!("no instruction at ptr: {ptr:?}")));
        }

        inst_ptr += 1;
    }
    println!();

    Ok(*regs.entry('a').or_default())
}

fn setup<T>(reader: T, regs: &mut BTreeMap<char, isize>, inst: &mut Vec<Instructions>) -> Result<()>
where
    T: BufRead,
{
    let cpy_int_re = Regex::new(r"^cpy (\d+) ([a-z])$")?;
    let cpy_reg_re = Regex::new(r"^cpy ([a-z]+) ([a-z])$")?;
    let inc_re = Regex::new(r"^inc ([a-z])$")?;
    let dec_re = Regex::new(r"^dec ([a-z])$")?;
    let jnz_int_re = Regex::new(r"^jnz (\d+) (-?\d+)$")?;
    let jnz_reg_re = Regex::new(r"^jnz ([a-z]) (-?\d+)$")?;
    let out_re = Regex::new(r"^out ([a-z])$")?;

    for line in valid_lines(reader) {
        if cpy_int_re.is_match(&line) {
            for caps in cpy_int_re.captures_iter(&line) {
                let value_1 = get_cap_x::<isize>(1, &caps)?;
                let register_1 = get_cap_x::<char>(2, &caps)?;
                _ = regs.entry(register_1).or_insert(0);
                inst.push(Instructions::CopyVal(value_1, register_1));
            }
        } else if cpy_reg_re.is_match(&line) {
            for caps in cpy_reg_re.captures_iter(&line) {
                let register_1 = get_cap_x::<char>(1, &caps)?;
                let register_2 = get_cap_x::<char>(2, &caps)?;
                _ = regs.entry(register_1).or_insert(0);
                _ = regs.entry(register_2).or_insert(0);
                inst.push(Instructions::CopyReg(register_1, register_2));
            }
        } else if inc_re.is_match(&line) {
            for caps in inc_re.captures_iter(&line) {
                let register_1 = get_cap_x::<char>(1, &caps)?;
                _ = regs.entry(register_1).or_insert(0);
                inst.push(Instructions::Increment(register_1));
            }
        } else if dec_re.is_match(&line) {
            for caps in dec_re.captures_iter(&line) {
                let register_1 = get_cap_x::<char>(1, &caps)?;
                _ = regs.entry(register_1).or_insert(0);
                inst.push(Instructions::Decrement(register_1));
            }
        } else if jnz_int_re.is_match(&line) {
            for caps in jnz_int_re.captures_iter(&line) {
                let val1 = get_cap_x::<isize>(1, &caps)?;
                let val2 = get_cap_x::<isize>(2, &caps)?;
                inst.push(Instructions::JumpNotZeroVal(val1, val2));
            }
        } else if jnz_reg_re.is_match(&line) {
            for caps in jnz_reg_re.captures_iter(&line) {
                let register_1 = get_cap_x::<char>(1, &caps)?;
                let value_1 = get_cap_x::<isize>(2, &caps)?;
                _ = regs.entry(register_1).or_insert(0);
                inst.push(Instructions::JumpNotZeroReg(register_1, value_1));
            }
        } else if out_re.is_match(&line) {
            for caps in out_re.captures_iter(&line) {
                let register_1 = get_cap_x::<char>(1, &caps)?;
                _ = regs.entry(register_1).or_insert(0);
                inst.push(Instructions::Out(register_1));
            }
        } else {
            return Err(anyhow!(format!("invalid instruction: {line}")));
        }
    }
    Ok(())
}

fn get_reg(regs: &BTreeMap<char, isize>, reg: char) -> Result<isize> {
    if let Some(val) = regs.get(&reg) {
        Ok(*val)
    } else {
        Err(anyhow!(format!("invalid register: {reg}")))
    }
}

#[allow(dead_code)]
fn show_regs(regs: &BTreeMap<char, isize>, term: &mut Term) -> Result<()> {
    use std::fmt::Write;

    let mut buf = String::new();
    for (idx, (k, v)) in regs.iter().enumerate() {
        write!(buf, "{k}: {v}").with_context(|| "Unable to write string")?;
        if idx < regs.len() - 1 {
            buf.push_str(", ");
        }
    }
    term.clear_line()?;
    term.write_all(buf.as_bytes())?;
    Ok(())
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<isize>(AoCYear::AOC2016, AoCDay::AOCD25, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> isize {
    find2_br(reader).map_err(print_err).unwrap_or_default()
}

fn find2_br<T>(reader: T) -> Result<isize>
where
    T: BufRead,
{
    run_assembunny(reader, true)
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"cpy a d";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1))?, 196);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    // use super::find2_br;
    // use std::io::Cursor;

    // const TEST_1: &str = r"^v";
    // const TEST_2: &str = r"^>v<";
    // const TEST_3: &str = r"^v^v^v^v^v";

    #[test]
    fn solution() {
        // assert_eq!(find2_br(Cursor::new(TEST_1))?, 3);
    }
}
