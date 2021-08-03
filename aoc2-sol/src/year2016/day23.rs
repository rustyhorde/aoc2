// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Safe Cracking
//!
//! **--- Day 23: Safe Cracking ---**
//!
//! **--- Part 1 ---**
//!
//! This is one of the top floors of the nicest tower in EBHQ. The Easter Bunny's
//! private office is here, complete with a safe hidden behind a painting,
//! and who wouldn't hide a star in a safe behind a painting?
//!
//! The safe has a digital screen and keypad for code entry. A sticky note
//! attached to the safe has a password hint on it: "eggs". The painting is of a
//! large rabbit coloring some eggs. You see `7`.
//!
//! When you go to type the code, though, nothing appears on the display; instead,
//! the keypad comes apart in your hands, apparently having been smashed. Behind
//! it is some kind of socket - one that matches a connector in your prototype
//! computer! You pull apart the smashed keypad and extract the logic circuit,
//! plug it into your computer, and plug your computer into the safe.
//!
//! Now, you just need to figure out what output the keypad would have sent to
//! the safe. You extract the assembunny code from the logic chip (your puzzle input).
//!
//! The code looks like it uses almost the same architecture and instruction set
//! that the monorail computer used! You should be able to use the same assembunny
//! interpreter for this as you did there, but with one new instruction:
//!
//! `tgl x` toggles the instruction x away (pointing at instructions like jnz does:
//! positive means forward; negative means backward):
//!
//! ```text
//! For one-argument instructions, inc becomes dec, and all other one-argument instructions become inc.
//! For two-argument instructions, jnz becomes cpy, and all other two-instructions become jnz.
//! The arguments of a toggled instruction are not affected.
//! If an attempt is made to toggle an instruction outside the program, nothing happens.
//! If toggling produces an invalid instruction (like cpy 1 2) and an attempt is later made to execute that instruction, skip it instead.
//! If tgl toggles itself (for example, if a is 0, tgl a would target itself and become inc a), the resulting instruction is not executed until the next time it is reached.
//! ```
//!
//! For example, given this program:
//!
//! ```text
//! cpy 2 a
//! tgl a
//! tgl a
//! tgl a
//! cpy 1 a
//! dec a
//! dec a
//! ```
//!
//! ```text
//! cpy 2 a initializes register a to 2.
//! The first tgl a toggles an instruction a (2) away from it, which changes the third tgl a into inc a.
//! The second tgl a also modifies an instruction 2 away from it, which changes the cpy 1 a into jnz 1 a.
//! The fourth line, which is now inc a, increments a to 3.
//! Finally, the fifth line, which is now jnz 1 a, jumps a (3) instructions ahead, skipping the dec a instructions.
//! ```
//!
//! In this example, the final value in register `a` is `3`.
//!
//! The rest of the electronics seem to place the keypad entry (the number of eggs, 7) in register `a`,
//! run the code, and then send the value left in register `a` to the safe.
//!
//! What value should be sent to the safe?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{get_cap_x, print_err, run_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use regex::Regex;
use std::{
    collections::BTreeMap,
    convert::TryFrom,
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
    JumpNotZeroValReg(isize, char),
    JumpNotZeroRegReg(char, char),
    Toggle(char),
    Skip,
}

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_solution::<isize>(AoCYear::AOC2016, AoCDay::AOCD23, find).map(|_| 0)
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
        *regs.entry('a').or_default() = 12;
    } else {
        *regs.entry('a').or_default() = 7;
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
                Instructions::JumpNotZeroValReg(val, reg) => {
                    if *val > 0 {
                        inst_ptr += get_reg(&regs, *reg)?;
                        continue;
                    }
                }
                Instructions::JumpNotZeroRegReg(reg_1, reg_2) => {
                    if get_reg(&regs, *reg_1)? > 0 {
                        inst_ptr += get_reg(&regs, *reg_2)?;
                        continue;
                    }
                }
                Instructions::Toggle(reg) => {
                    let val = get_reg(&regs, *reg)?;
                    if val > 0 {
                        let tmp_ptr = usize::try_from(inst_ptr + val)?;
                        if tmp_ptr < inst.len() {
                            let inst_to_toggle = inst[tmp_ptr];
                            let new_inst = match inst_to_toggle {
                                Instructions::Toggle(reg) | Instructions::Decrement(reg) => {
                                    Instructions::Increment(reg)
                                }
                                Instructions::Increment(reg) => Instructions::Decrement(reg),
                                Instructions::CopyVal(val, reg) => {
                                    Instructions::JumpNotZeroValReg(val, reg)
                                }
                                Instructions::CopyReg(reg_1, reg_2) => {
                                    Instructions::JumpNotZeroRegReg(reg_1, reg_2)
                                }
                                Instructions::JumpNotZeroVal(_val1, _val2) => Instructions::Skip,
                                Instructions::JumpNotZeroReg(_reg, _val) => Instructions::Skip,
                                Instructions::JumpNotZeroValReg(val, reg) => {
                                    Instructions::CopyVal(val, reg)
                                }
                                Instructions::JumpNotZeroRegReg(reg_1, reg_2) => {
                                    Instructions::CopyReg(reg_1, reg_2)
                                }
                                Instructions::Skip => Instructions::Skip,
                            };
                            inst[tmp_ptr] = new_inst;
                        }
                    }
                }
                Instructions::Skip => {}
            }
        } else {
            return Err(anyhow!(format!("no instruction at ptr: {:?}", ptr)));
        }

        inst_ptr += 1;
    }
    Ok(*regs.entry('a').or_default())
}

fn get_reg(regs: &BTreeMap<char, isize>, reg: char) -> Result<isize> {
    if let Some(val) = regs.get(&reg) {
        Ok(*val)
    } else {
        Err(anyhow!(format!("invalid register: {}", reg)))
    }
}

#[allow(dead_code)]
fn show_regs(regs: &BTreeMap<char, isize>) {
    let mut buf = String::new();
    for (idx, (k, v)) in regs.iter().enumerate() {
        buf.push_str(&format!("{}: {}", k, v));
        if idx < regs.len() - 1 {
            buf.push_str(", ");
        }
    }
    println!("{}", buf);
}

fn setup<T>(reader: T, regs: &mut BTreeMap<char, isize>, inst: &mut Vec<Instructions>) -> Result<()>
where
    T: BufRead,
{
    let cpy_int_re = Regex::new(r"^cpy (-?\d+) ([a-z])$")?;
    let cpy_reg_re = Regex::new(r"^cpy ([a-z]+) ([a-z])$")?;
    let inc_re = Regex::new(r"^inc ([a-z])$")?;
    let dec_re = Regex::new(r"^dec ([a-z])$")?;
    let jnz_int_re = Regex::new(r"^jnz (\d+) (\d+)$")?;
    let jnz_reg_re = Regex::new(r"^jnz ([a-z]) (-?\d+)$")?;
    let jnz_vr_re = Regex::new(r"^jnz (\d+) ([a-z])$")?;
    let tgl_re = Regex::new(r"^tgl ([a-z])$")?;

    for line in valid_lines(reader) {
        if cpy_int_re.is_match(&line) {
            for caps in cpy_int_re.captures_iter(&line) {
                let value_1 = get_cap_x::<isize>(1, &caps)?;
                let register_1 = get_cap_x::<char>(2, &caps)?;
                let _ = regs.entry(register_1).or_insert(0);
                inst.push(Instructions::CopyVal(value_1, register_1));
            }
        } else if cpy_reg_re.is_match(&line) {
            for caps in cpy_reg_re.captures_iter(&line) {
                let register_1 = get_cap_x::<char>(1, &caps)?;
                let register_2 = get_cap_x::<char>(2, &caps)?;
                let _ = regs.entry(register_1).or_insert(0);
                let _ = regs.entry(register_2).or_insert(0);
                inst.push(Instructions::CopyReg(register_1, register_2));
            }
        } else if inc_re.is_match(&line) {
            for caps in inc_re.captures_iter(&line) {
                let register_1 = get_cap_x::<char>(1, &caps)?;
                let _ = regs.entry(register_1).or_insert(0);
                inst.push(Instructions::Increment(register_1));
            }
        } else if dec_re.is_match(&line) {
            for caps in dec_re.captures_iter(&line) {
                let register_1 = get_cap_x::<char>(1, &caps)?;
                let _ = regs.entry(register_1).or_insert(0);
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
                let _ = regs.entry(register_1).or_insert(0);
                inst.push(Instructions::JumpNotZeroReg(register_1, value_1));
            }
        } else if jnz_vr_re.is_match(&line) {
            for caps in jnz_vr_re.captures_iter(&line) {
                let value_1 = get_cap_x::<isize>(1, &caps)?;
                let register_1 = get_cap_x::<char>(2, &caps)?;
                let _ = regs.entry(register_1).or_insert(0);
                inst.push(Instructions::JumpNotZeroValReg(value_1, register_1));
            }
        } else if tgl_re.is_match(&line) {
            for caps in tgl_re.captures_iter(&line) {
                let register_1 = get_cap_x::<char>(1, &caps)?;
                let _ = regs.entry(register_1).or_insert(0);
                inst.push(Instructions::Toggle(register_1));
            }
        } else {
            return Err(anyhow!(format!("invalid instruction: {}", line)));
        }
    }
    Ok(())
}

/// Solution for Part 2
///
/// # Notes
/// This runs as is in 22 seconds.  This could be optimized by replacing the `inc a, dec b, jnz b -2, dec c, jnz c -5`
/// loops with a multiply instruction `mul b c a` where `(b * c) + a` is stored in `a`.  This would cut those loops
/// down to 1 instruction.  This would have to be done on the fly, as the last 5 instruction loop isn't the right
/// pattern until the toggle has been passed. And that's the 90 * 87 loop.
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<isize>(AoCYear::AOC2016, AoCDay::AOCD23, find2).map(|_| 0)
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

    const TEST_1: &str = r"cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1))?, 3);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    // use super::find2_br;
    use anyhow::Result;
    // use std::io::Cursor;

    // const TEST_1: &str = r"^v";
    // const TEST_2: &str = r"^>v<";
    // const TEST_3: &str = r"^v^v^v^v^v";

    #[test]
    fn solution() -> Result<()> {
        // assert_eq!(find2_br(Cursor::new(TEST_1))?, 3);
        Ok(())
    }
}
