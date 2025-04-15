// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2018 ---**
//!
//! **--- Day 16: Chronal Classification ---**
//!
//! As you see the Elves defend their hot chocolate successfully, you go back to falling through time. This is going to become a problem.
//!
//! If you're ever going to return to your own time, you need to understand how this device on your wrist works. You have a little while before you reach your next destination, and with a bit of trial and error, you manage to pull up a programming manual on the device's tiny screen.
//!
//! According to the manual, the device has four registers (numbered 0 through 3) that can be manipulated by instructions containing one of 16 opcodes. The registers start with the value 0.
//!
//! Every instruction consists of four values: an opcode, two inputs (named A and B), and an output (named C), in that order. The opcode specifies the behavior of the instruction and how the inputs are interpreted. The output, C, is always treated as a register.
//!
//! In the opcode descriptions below, if something says "value A", it means to take the number given as A literally. (This is also called an "immediate" value.) If something says "register A", it means to use the number given as A to read from (or write to) the register with that number. So, if the opcode addi adds register A and value B, storing the result in register C, and the instruction addi 0 7 3 is encountered, it would add 7 to the value contained by register 0 and store the sum in register 3, never modifying registers 0, 1, or 2 in the process.
//!
//! Many opcodes are similar except for how they interpret their arguments. The opcodes fall into seven general categories:
//!
//! ```text
//! Addition:
//!
//!     addr (add register) stores into register C the result of adding register A and register B.
//!     addi (add immediate) stores into register C the result of adding register A and value B.
//!
//! Multiplication:
//!
//!     mulr (multiply register) stores into register C the result of multiplying register A and register B.
//!     muli (multiply immediate) stores into register C the result of multiplying register A and value B.
//!
//! Bitwise AND:
//!
//!     banr (bitwise AND register) stores into register C the result of the bitwise AND of register A and register B.
//!     bani (bitwise AND immediate) stores into register C the result of the bitwise AND of register A and value B.
//!
//! Bitwise OR:
//!
//!     borr (bitwise OR register) stores into register C the result of the bitwise OR of register A and register B.
//!     bori (bitwise OR immediate) stores into register C the result of the bitwise OR of register A and value B.
//!
//! Assignment:
//!
//!     setr (set register) copies the contents of register A into register C. (Input B is ignored.)
//!     seti (set immediate) stores value A into register C. (Input B is ignored.)
//!
//! Greater-than testing:
//!
//!     gtir (greater-than immediate/register) sets register C to 1 if value A is greater than register B. Otherwise, register C is set to 0.
//!     gtri (greater-than register/immediate) sets register C to 1 if register A is greater than value B. Otherwise, register C is set to 0.
//!     gtrr (greater-than register/register) sets register C to 1 if register A is greater than register B. Otherwise, register C is set to 0.
//!
//! Equality testing:
//!
//!     eqir (equal immediate/register) sets register C to 1 if value A is equal to register B. Otherwise, register C is set to 0.
//!     eqri (equal register/immediate) sets register C to 1 if register A is equal to value B. Otherwise, register C is set to 0.
//!     eqrr (equal register/register) sets register C to 1 if register A is equal to register B. Otherwise, register C is set to 0.
//! ```
//!
//! Unfortunately, while the manual gives the name of each opcode, it doesn't seem to indicate the number. However, you can monitor the CPU to see the contents of the registers before and after instructions are executed to try to work them out. Each opcode has a number from 0 through 15, but the manual doesn't say which is which. For example, suppose you capture the following sample:
//!
//! ```text
//! Before: [3, 2, 1, 1]
//! 9 2 1 2
//! After:  [3, 2, 2, 1]
//! ```
//!
//! This sample shows the effect of the instruction 9 2 1 2 on the registers. Before the instruction is executed, register 0 has value 3, register 1 has value 2, and registers 2 and 3 have value 1. After the instruction is executed, register 2's value becomes 2.
//!
//! The instruction itself, 9 2 1 2, means that opcode 9 was executed with A=2, B=1, and C=2. Opcode 9 could be any of the 16 opcodes listed above, but only three of them behave in a way that would cause the result shown in the sample:
//!
//! ```text
//!     Opcode 9 could be mulr: register 2 (which has a value of 1) times register 1 (which has a value of 2) produces 2, which matches the value stored in the output register, register 2.
//!     Opcode 9 could be addi: register 2 (which has a value of 1) plus value 1 produces 2, which matches the value stored in the output register, register 2.
//!     Opcode 9 could be seti: value 2 matches the value stored in the output register, register 2; the number given for B is irrelevant.
//! ```
//!
//! None of the other opcodes produce the result captured in the sample. Because of this, the sample above behaves like three opcodes.
//!
//! You collect many of these samples (the first section of your puzzle input). The manual also includes a small test program (the second section of your puzzle input) - you can ignore it for now.
//!
//! Ignoring the opcode numbers, how many samples in your puzzle input behave like three or more opcodes?
//!
//! **--- Part Two ---**
//!
//! Using the samples you collected, work out the number of each opcode and execute the test program (the second section of your puzzle input).
//!
//! What value is contained in register 0 after executing the test program?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use regex::Regex;
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
};

type Registers = [usize; 4];
type Instruction = [usize; 4];

#[derive(Clone, Debug, Eq, PartialEq)]
enum OpCode {
    /// (add register) stores into register C the result of adding register A and register B.
    Addr,
    /// (add immediate) stores into register C the result of adding register A and value B.
    Addi,
    /// (multiply register) stores into register C the result of multiplying register A and register B.
    Mulr,
    /// (multiply immediate) stores into register C the result of multiplying register A and value B.
    Muli,
    /// (bitwise AND register) stores into register C the result of the bitwise AND of register A and register B.
    Banr,
    /// (bitwise AND immediate) stores into register C the result of the bitwise AND of register A and value B.
    Bani,
    /// (bitwise OR register) stores into register C the result of the bitwise OR of register A and register B.
    Borr,
    /// (bitwise OR immediate) stores into register C the result of the bitwise OR of register A and value B.
    Bori,
    /// (set register) copies the contents of register A into register C. (Input B is ignored.)
    Setr,
    /// (set immediate) stores value A into register C. (Input B is ignored.)
    Seti,
    /// (greater-than immediate/register) sets register C to 1 if value A is greater than register B. Otherwise, register C is set to 0.
    Gtir,
    /// (greater-than register/immediate) sets register C to 1 if register A is greater than value B. Otherwise, register C is set to 0.
    Gtri,
    /// (greater-than register/register) sets register C to 1 if register A is greater than register B. Otherwise, register C is set to 0.
    Gtrr,
    /// (equal immediate/register) sets register C to 1 if value A is equal to register B. Otherwise, register C is set to 0.
    Eqir,
    /// (equal register/immediate) sets register C to 1 if register A is equal to value B. Otherwise, register C is set to 0.
    Eqri,
    /// (equal register/register) sets register C to 1 if register A is equal to register B. Otherwise, register C is set to 0.
    Eqrr,
}

impl OpCode {
    fn execute(&self, reg: &mut Registers, ins: Instruction) {
        match self {
            OpCode::Addr => reg[ins[3]] = reg[ins[1]] + reg[ins[2]],
            OpCode::Addi => reg[ins[3]] = reg[ins[1]] + ins[2],
            OpCode::Mulr => reg[ins[3]] = reg[ins[1]] * reg[ins[2]],
            OpCode::Muli => reg[ins[3]] = reg[ins[1]] * ins[2],
            OpCode::Banr => reg[ins[3]] = reg[ins[1]] & reg[ins[2]],
            OpCode::Bani => reg[ins[3]] = reg[ins[1]] & ins[2],
            OpCode::Borr => reg[ins[3]] = reg[ins[1]] | reg[ins[2]],
            OpCode::Bori => reg[ins[3]] = reg[ins[1]] | ins[2],
            OpCode::Setr => reg[ins[3]] = reg[ins[1]],
            OpCode::Seti => reg[ins[3]] = ins[1],
            OpCode::Gtir => reg[ins[3]] = usize::from(ins[1] > reg[ins[2]]),
            OpCode::Gtri => reg[ins[3]] = usize::from(reg[ins[1]] > ins[2]),
            OpCode::Gtrr => reg[ins[3]] = usize::from(reg[ins[1]] > reg[ins[2]]),
            OpCode::Eqir => reg[ins[3]] = usize::from(ins[1] == reg[ins[2]]),
            OpCode::Eqri => reg[ins[3]] = usize::from(reg[ins[1]] == ins[2]),
            OpCode::Eqrr => reg[ins[3]] = usize::from(reg[ins[1]] == reg[ins[2]]),
        }
    }
}

type ChronalData = (Vec<[usize; 4]>, Vec<[usize; 4]>, Vec<[usize; 4]>);

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<ChronalData, usize>(AoCYear::AOC2018, AoCDay::AOCD16, setup, find)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<ChronalData, usize>(bench, AoCYear::AOC2018, AoCDay::AOCD16, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> ChronalData {
    setup_br(reader).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T) -> Result<ChronalData>
where
    T: BufRead,
{
    let before_re = Regex::new(r"Before: \[(\d), (\d), (\d), (\d)\]")?;
    let after_re = Regex::new(r"After:  \[(\d), (\d), (\d), (\d)\]")?;
    let instruction_re = Regex::new(r"(\d+) (\d) (\d) (\d)")?;

    let mut before_vec = Vec::new();
    let mut after_vec = Vec::new();
    let mut instructions_vec = Vec::new();

    for line in valid_lines(reader) {
        if before_re.is_match(&line) {
            for caps in before_re.captures_iter(&line) {
                let opcode = (caps[1]).parse::<usize>()?;
                let reg_a = (caps[2]).parse::<usize>()?;
                let reg_b = (caps[3]).parse::<usize>()?;
                let reg_c = (caps[4]).parse::<usize>()?;

                before_vec.push([opcode, reg_a, reg_b, reg_c]);
            }
        } else if after_re.is_match(&line) {
            for caps in after_re.captures_iter(&line) {
                let opcode = (caps[1]).parse::<usize>()?;
                let reg_a = (caps[2]).parse::<usize>()?;
                let reg_b = (caps[3]).parse::<usize>()?;
                let reg_c = (caps[4]).parse::<usize>()?;

                after_vec.push([opcode, reg_a, reg_b, reg_c]);
            }
        } else if instruction_re.is_match(&line) {
            for caps in instruction_re.captures_iter(&line) {
                let opcode = (caps[1]).parse::<usize>()?;
                let reg_a = (caps[2]).parse::<usize>()?;
                let reg_b = (caps[3]).parse::<usize>()?;
                let reg_c = (caps[4]).parse::<usize>()?;

                instructions_vec.push([opcode, reg_a, reg_b, reg_c]);
            }
        }
    }
    Ok((before_vec, after_vec, instructions_vec))
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: ChronalData) -> usize {
    find_res(data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps, clippy::too_many_lines)]
fn find_res(data: ChronalData, second_star: bool) -> Result<usize> {
    let (before_vec, after_vec, mut instructions_vec) = data;
    if before_vec.len() != after_vec.len() {
        return Err(anyhow!("Bad input file"));
    }

    let instructions = instructions_vec.split_off(before_vec.len());

    let tuples_vec: Vec<(Registers, Instruction, Registers)> = before_vec
        .into_iter()
        .zip(instructions_vec)
        .zip(after_vec)
        .map(|((rb, i), ra)| (rb, i, ra))
        .collect();

    let mut three_or_more = 0;
    let mut opcode_map = BTreeMap::new();

    for (before, ins, after) in tuples_vec {
        let mut count = 0;

        execute_opcode(
            OpCode::Addr,
            before,
            ins,
            after,
            &mut count,
            second_star,
            &mut opcode_map,
        );
        execute_opcode(
            OpCode::Addi,
            before,
            ins,
            after,
            &mut count,
            second_star,
            &mut opcode_map,
        );
        execute_opcode(
            OpCode::Mulr,
            before,
            ins,
            after,
            &mut count,
            second_star,
            &mut opcode_map,
        );
        execute_opcode(
            OpCode::Muli,
            before,
            ins,
            after,
            &mut count,
            second_star,
            &mut opcode_map,
        );
        execute_opcode(
            OpCode::Banr,
            before,
            ins,
            after,
            &mut count,
            second_star,
            &mut opcode_map,
        );
        execute_opcode(
            OpCode::Bani,
            before,
            ins,
            after,
            &mut count,
            second_star,
            &mut opcode_map,
        );
        execute_opcode(
            OpCode::Borr,
            before,
            ins,
            after,
            &mut count,
            second_star,
            &mut opcode_map,
        );
        execute_opcode(
            OpCode::Bori,
            before,
            ins,
            after,
            &mut count,
            second_star,
            &mut opcode_map,
        );
        execute_opcode(
            OpCode::Setr,
            before,
            ins,
            after,
            &mut count,
            second_star,
            &mut opcode_map,
        );
        execute_opcode(
            OpCode::Seti,
            before,
            ins,
            after,
            &mut count,
            second_star,
            &mut opcode_map,
        );
        execute_opcode(
            OpCode::Gtir,
            before,
            ins,
            after,
            &mut count,
            second_star,
            &mut opcode_map,
        );
        execute_opcode(
            OpCode::Gtri,
            before,
            ins,
            after,
            &mut count,
            second_star,
            &mut opcode_map,
        );
        execute_opcode(
            OpCode::Gtrr,
            before,
            ins,
            after,
            &mut count,
            second_star,
            &mut opcode_map,
        );
        execute_opcode(
            OpCode::Eqir,
            before,
            ins,
            after,
            &mut count,
            second_star,
            &mut opcode_map,
        );
        execute_opcode(
            OpCode::Eqri,
            before,
            ins,
            after,
            &mut count,
            second_star,
            &mut opcode_map,
        );
        execute_opcode(
            OpCode::Eqrr,
            before,
            ins,
            after,
            &mut count,
            second_star,
            &mut opcode_map,
        );

        if count >= 3 {
            three_or_more += 1;
        }
    }

    if second_star {
        let mut known = OpCode::Addr;
        let mut op_map = BTreeMap::new();

        while op_map.len() != 16 {
            for (opcode, poss) in &opcode_map {
                if poss.len() == 1 {
                    known = poss[0].clone();
                    let _ = op_map.insert(*opcode, known.clone());
                    break;
                }
            }

            for poss in opcode_map.values_mut() {
                poss.retain(|x| *x != known);
            }
        }

        let mut registers = [0, 0, 0, 0];
        for ins in instructions {
            let opcode = op_map.get(&ins[0]).ok_or(anyhow!("invalid opcode"))?;
            opcode.execute(&mut registers, ins);
        }
        Ok(registers[0])
    } else {
        Ok(three_or_more)
    }
}

fn execute_opcode(
    opcode: OpCode,
    before: [usize; 4],
    ins: [usize; 4],
    after: [usize; 4],
    count: &mut usize,
    second_star: bool,
    opcode_map: &mut BTreeMap<usize, Vec<OpCode>>,
) {
    let mut regs = before;
    opcode.execute(&mut regs, ins);
    if regs == after {
        *count += 1;
    }
    if second_star && regs == after {
        let opcode_vec = opcode_map.entry(ins[0]).or_default();

        if !opcode_vec.contains(&opcode) {
            opcode_vec.push(opcode);
        }
    }
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<ChronalData, usize>(AoCYear::AOC2018, AoCDay::AOCD16, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<ChronalData, usize>(bench, AoCYear::AOC2018, AoCDay::AOCD16, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: ChronalData) -> usize {
    find_res(data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {}

#[cfg(test)]
mod two_star {}
