// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 23 "Opening the Turing Lock"
//!
//! **--- Day 23: Opening the Turing Lock ---**
//!
//! **--- Part 1 ---**
//!
//! Little Jane Marie just got her very first computer for Christmas from some
//! unknown benefactor. It comes with instructions and an example program, but the
//! computer itself seems to be malfunctioning. She's curious what the program does,
//! and would like you to help her run it.
//!
//! The manual explains that the computer supports two registers and six instructions
//! (truly, it goes on to remind the reader, a state-of-the-art technology).
//! The registers are named `a` and `b`, can hold any `non-negative integer`,
//! and begin with a value of `0`. The instructions are as follows:
//!
//! ```text
//! `hlf r` sets register `r` to half its current value, then continues with the next instruction.
//! `tpl r` sets register `r` to triple its current value, then continues with the next instruction.
//! `inc r` increments register `r`, adding 1 to it, then continues with the next instruction.
//! `jmp offset  is a jump; it continues with the instruction offset away relative to itself.
//! `jie r, offset `is like jmp, but only jumps if register `r` is even ("jump if even").
//! `jio r, offset` is like jmp, but only jumps if register `r` is 1 ("jump if one", not odd).
//! ```
//!
//! All three jump instructions work with an offset relative to that instruction. The offset
//! is always written with a prefix `+` or `-` to indicate the direction of the jump
//! (forward or backward, respectively). For example, `jmp +1` would simply continue with
//! the next instruction, while `jmp +0` would continuously jump back to itself forever.
//!
//! The program exits when it tries to run an instruction beyond the ones defined.
//!
//! For example, this program sets `a` to 2, because the `jio` instruction causes
//! it to skip the `tpl` instruction:
//!
//! ```text
//! inc a
//! jio a, +2
//! tpl a
//! inc a
//! ```
//!
//! What is the value in register `b` when the program in your puzzle input is finished executing?
//!
//! **--- Part Two ---**
//!
//! The unknown benefactor is very thankful for releasi-- er, helping little Jane Marie
//! with her computer. Definitely not to distract you, what is the value in register `b` after
//! the program is finished executing if register a starts as 1 instead?
//!

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{get_cap, get_cap_x, run_solution, valid_lines},
};
use anyhow::{anyhow, Context, Result};
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD23, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader).unwrap_or_default()
}

#[derive(Clone, Debug)]
enum Inst {
    Half(usize),
    Triple(usize),
    Inc(usize),
    Jmp(String, usize),
    Jie(usize, String, usize),
    Jio(usize, String, usize),
}

fn find_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let hlf_re = Regex::new(r"^hlf (a|b)$")?;
    let tpl_re = Regex::new(r"^tpl (a|b)$")?;
    let inc_re = Regex::new(r"^inc (a|b)$")?;
    let jmp_re = Regex::new(r"^jmp (\+|-)(\d+)$")?;
    let jump_if_even_re = Regex::new(r"^jie (a|b), (\+|-)(\d+)$")?;
    let jump_if_one_re = Regex::new(r"^jio (a|b), (\+|-)(\d+)$")?;

    let mut inst = vec![];

    for line in valid_lines(reader) {
        if hlf_re.is_match(&line) {
            let reg = get_cap(1, &hlf_re.captures(&line).context("bad cap")?)?;
            inst.push(Inst::Half(usize::from(&reg != "a")));
        } else if tpl_re.is_match(&line) {
            let reg = get_cap(1, &tpl_re.captures(&line).context("bad cap")?)?;
            inst.push(Inst::Triple(usize::from(&reg != "a")));
        } else if inc_re.is_match(&line) {
            let reg = get_cap(1, &inc_re.captures(&line).context("bad cap")?)?;
            inst.push(Inst::Inc(usize::from(&reg != "a")));
        } else if jmp_re.is_match(&line) {
            let caps = jmp_re.captures(&line).context("bad cap")?;
            let sign = get_cap(1, &caps)?;
            let offset = get_cap_x::<usize>(2, &caps)?;
            inst.push(Inst::Jmp(sign, offset));
        } else if jump_if_even_re.is_match(&line) {
            let caps = jump_if_even_re.captures(&line).context("bad cap")?;
            let reg = get_cap(1, &caps)?;
            let sign = get_cap(2, &caps)?;
            let offset = get_cap_x::<usize>(3, &caps)?;
            inst.push(Inst::Jie(usize::from(&reg != "a"), sign, offset));
        } else if jump_if_one_re.is_match(&line) {
            let caps = jump_if_one_re.captures(&line).context("bad cap")?;
            let reg = get_cap(1, &caps)?;
            let sign = get_cap(2, &caps)?;
            let offset = get_cap_x::<usize>(3, &caps)?;
            inst.push(Inst::Jio(usize::from(&reg != "a"), sign, offset));
        } else {
            return Err(anyhow!(format!("Invalid instruction: {line}")));
        }
    }

    let mut regs = vec![0, 0];
    let mut ptr = 0;

    while let Some(next) = inst.get(ptr) {
        match next {
            Inst::Half(reg) => {
                regs[*reg] /= 2;
                ptr += 1;
            }
            Inst::Triple(reg) => {
                regs[*reg] *= 3;
                ptr += 1;
            }
            Inst::Inc(reg) => {
                regs[*reg] += 1;
                ptr += 1;
            }
            Inst::Jmp(sign, offset) => {
                if sign == "+" {
                    ptr += offset;
                } else {
                    ptr -= offset;
                }
            }
            Inst::Jie(reg, sign, offset) => {
                if regs[*reg] % 2 == 0 {
                    if sign == "+" {
                        ptr += offset;
                    } else {
                        ptr -= offset;
                    }
                } else {
                    ptr += 1;
                }
            }
            Inst::Jio(reg, sign, offset) => {
                if regs[*reg] == 1 {
                    if sign == "+" {
                        ptr += offset;
                    } else {
                        ptr -= offset;
                    }
                } else {
                    ptr += 1;
                }
            }
        }
    }
    Ok(regs[1])
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD23, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader).unwrap_or_default()
}

fn find2_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let hlf_re = Regex::new(r"^hlf (a|b)$")?;
    let tpl_re = Regex::new(r"^tpl (a|b)$")?;
    let inc_re = Regex::new(r"^inc (a|b)$")?;
    let jmp_re = Regex::new(r"^jmp (\+|-)(\d+)$")?;
    let jump_if_even_re = Regex::new(r"^jie (a|b), (\+|-)(\d+)$")?;
    let jump_if_one_re = Regex::new(r"^jio (a|b), (\+|-)(\d+)$")?;

    let mut inst = vec![];

    for line in valid_lines(reader) {
        if hlf_re.is_match(&line) {
            let reg = get_cap(1, &hlf_re.captures(&line).context("bad cap")?)?;
            inst.push(Inst::Half(usize::from(&reg != "a")));
        } else if tpl_re.is_match(&line) {
            let reg = get_cap(1, &tpl_re.captures(&line).context("bad cap")?)?;
            inst.push(Inst::Triple(usize::from(&reg != "a")));
        } else if inc_re.is_match(&line) {
            let reg = get_cap(1, &inc_re.captures(&line).context("bad cap")?)?;
            inst.push(Inst::Inc(usize::from(&reg != "a")));
        } else if jmp_re.is_match(&line) {
            let caps = jmp_re.captures(&line).context("bad cap")?;
            let sign = get_cap(1, &caps)?;
            let offset = get_cap_x::<usize>(2, &caps)?;
            inst.push(Inst::Jmp(sign, offset));
        } else if jump_if_even_re.is_match(&line) {
            let caps = jump_if_even_re.captures(&line).context("bad cap")?;
            let reg = get_cap(1, &caps)?;
            let sign = get_cap(2, &caps)?;
            let offset = get_cap_x::<usize>(3, &caps)?;
            inst.push(Inst::Jie(usize::from(&reg != "a"), sign, offset));
        } else if jump_if_one_re.is_match(&line) {
            let caps = jump_if_one_re.captures(&line).context("bad cap")?;
            let reg = get_cap(1, &caps)?;
            let sign = get_cap(2, &caps)?;
            let offset = get_cap_x::<usize>(3, &caps)?;
            inst.push(Inst::Jio(usize::from(&reg != "a"), sign, offset));
        } else {
            return Err(anyhow!(format!("Invalid instruction: {line}")));
        }
    }

    let mut regs = vec![1, 0];
    let mut ptr = 0;

    while let Some(next) = inst.get(ptr) {
        match next {
            Inst::Half(reg) => {
                regs[*reg] /= 2;
                ptr += 1;
            }
            Inst::Triple(reg) => {
                regs[*reg] *= 3;
                ptr += 1;
            }
            Inst::Inc(reg) => {
                regs[*reg] += 1;
                ptr += 1;
            }
            Inst::Jmp(sign, offset) => {
                if sign == "+" {
                    ptr += offset;
                } else {
                    ptr -= offset;
                }
            }
            Inst::Jie(reg, sign, offset) => {
                if regs[*reg] % 2 == 0 {
                    if sign == "+" {
                        ptr += offset;
                    } else {
                        ptr -= offset;
                    }
                } else {
                    ptr += 1;
                }
            }
            Inst::Jio(reg, sign, offset) => {
                if regs[*reg] == 1 {
                    if sign == "+" {
                        ptr += offset;
                    } else {
                        ptr -= offset;
                    }
                } else {
                    ptr += 1;
                }
            }
        }
    }
    Ok(regs[1])
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"inc b
jio b, +2
tpl b
inc b";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1))?, 2);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    // use super::find2_br;
    // use std::io::Cursor;

    // const TEST_1: &str = r"turn on 0,0 through 0,0";
    // const TEST_2: &str = r"toggle 0,0 through 999,999";

    #[test]
    fn solution() {
        // assert_eq!(find2_br(Cursor::new(TEST_1))?, 1);
        // assert_eq!(find2_br(Cursor::new(TEST_2))?, 2_000_000);
    }
}
