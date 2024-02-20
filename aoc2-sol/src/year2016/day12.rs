// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 12 "Leonardo's Monorail"
//!
//! **--- Day 12: Leonardo's Monorail ---**
//!
//! **--- Part 1 ---**
//!
//! You finally reach the top floor of this building: a garden with a slanted glass ceiling.
//! Looks like there are no more stars to be had.
//!
//! While sitting on a nearby bench amidst some tiger lilies, you manage to decrypt some
//! of the files you extracted from the servers downstairs.
//!
//! According to these documents, Easter Bunny HQ isn't just this building - it's a
//! collection of buildings in the nearby area. They're all connected by a local monorail,
//! and there's another building not far from here! Unfortunately, being night, the monorail
//! is currently not operating.
//!
//! You remotely connect to the monorail control systems and discover that the boot sequence
//! expects a password. The password-checking logic (your puzzle input) is easy to extract,
//! but the code it uses is strange: it's assembunny code designed for the new computer you
//! just assembled. You'll have to execute the code and get the password.
//!
//! The assembunny code you've extracted operates on four registers (a, b, c, and d) that
//! start at 0 and can hold any integer. However, it seems to make use of only a few
//! instructions:
//!
//! ```text
//! cpy x y copies x (either an integer or the value of a register) into register y.
//! inc x increases the value of register x by one.
//! dec x decreases the value of register x by one.
//! jnz x y jumps to an instruction y away (positive means forward; negative means backward), but only if x is not zero.
//! ```
//!
//! The jnz instruction moves relative to itself: an offset of -1 would continue at the
//! previous instruction, while an offset of 2 would skip over the next instruction.
//!
//! For example:
//!
//! ```text
//! cpy 41 a
//! inc a
//! inc a
//! dec a
//! jnz a 2
//! dec a
//! ```
//!
//! The above code would set register `a` to 41, increase its value by 2, decrease its value
//! by 1, and then skip the last dec `a` (because `a` is not zero, so the `jnz a 2`
//! skips it), leaving register a at 42. When you move past the last instruction,
//! the program halts.
//!
//! After executing the assembunny code in your puzzle input, what value is left in register `a`?
//!
//! **--- Part Two ---**
//!
//! As you head down the fire escape to the monorail, you notice it didn't start; register
//! `c` needs to be initialized to the position of the ignition key.
//!
//! If you instead initialize register `c` to be 1, what value is now left in register `a`?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{get_cap_x, print_err, run_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use regex::Regex;
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Instructions {
    CopyVal(isize, char),
    CopyReg(char, char),
    Increment(char),
    Decrement(char),
    JumpNotZeroVal(isize, isize),
    JumpNotZeroReg(char, isize),
}

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_solution::<isize>(AoCYear::AOC2016, AoCDay::AOCD12, find).map(|_| 0)
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

fn run_assembunny<T>(reader: T, part2: bool) -> Result<isize>
where
    T: BufRead,
{
    let mut regs: BTreeMap<char, isize> = BTreeMap::new();
    let mut inst = vec![];

    setup(reader, &mut regs, &mut inst)?;

    let mut inst_ptr: isize = 0;

    if part2 {
        *regs.entry('c').or_default() = 1;
    }

    loop {
        let ptr = usize::try_from(inst_ptr)?;
        if ptr >= inst.len() {
            break;
        }

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
                    if let Some(val) = regs.get(reg) {
                        if *val > 0 {
                            inst_ptr += offset;
                            continue;
                        }
                    } else {
                        return Err(anyhow!(format!(
                            "invalid jnz reg instruction: {instruction:?}"
                        )));
                    }
                }
                Instructions::JumpNotZeroVal(val, offset) => {
                    if *val > 0 {
                        inst_ptr += offset;
                        continue;
                    }
                }
            }
        } else {
            return Err(anyhow!(format!("no instruction at ptr: {ptr:?}")));
        }

        inst_ptr += 1;
    }
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
    let jnz_int_re = Regex::new(r"^jnz (\d)+ (\d+)$")?;
    let jnz_reg_re = Regex::new(r"^jnz ([a-z]) (-?\d+)$")?;

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
        } else {
            return Err(anyhow!(format!("invalid instruction: {line}")));
        }
    }
    Ok(())
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<isize>(AoCYear::AOC2016, AoCDay::AOCD12, find2).map(|_| 0)
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

    const TEST_1: &str = r"cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1))?, 42);
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
