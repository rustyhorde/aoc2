// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2024 ---**
//!
//! **--- Day 17: Chronospatial Computer ---**
//!
//! The Historians push the button on their strange device, but this time, you all just feel like you're falling.
//!
//! "Situation critical", the device announces in a familiar voice. "Bootstrapping process failed. Initializing debugger...."
//!
//! The small handheld device suddenly unfolds into an entire computer! The Historians look around nervously before one of them tosses it to you.
//!
//! This seems to be a 3-bit computer: its program is a list of 3-bit numbers (0 through 7), like 0,1,2,3. The computer also has three registers named A, B, and C, but these registers aren't limited to 3 bits and can instead hold any integer.
//!
//! The computer knows eight instructions, each identified by a 3-bit number (called the instruction's opcode). Each instruction also reads the 3-bit number after it as an input; this is called its operand.
//!
//! A number called the instruction pointer identifies the position in the program from which the next opcode will be read; it starts at 0, pointing at the first 3-bit number in the program. Except for jump instructions, the instruction pointer increases by 2 after each instruction is processed (to move past the instruction's opcode and its operand). If the computer tries to read an opcode past the end of the program, it instead halts.
//!
//! So, the program 0,1,2,3 would run the instruction whose opcode is 0 and pass it the operand 1, then run the instruction having opcode 2 and pass it the operand 3, then halt.
//!
//! There are two types of operands; each instruction specifies the type of its operand. The value of a literal operand is the operand itself. For example, the value of the literal operand 7 is the number 7. The value of a combo operand can be found as follows:
//!
//! ```text
//!     Combo operands 0 through 3 represent literal values 0 through 3.
//!     Combo operand 4 represents the value of register A.
//!     Combo operand 5 represents the value of register B.
//!     Combo operand 6 represents the value of register C.
//!     Combo operand 7 is reserved and will not appear in valid programs.
//! ```
//!
//! The eight instructions are as follows:
//!
//! ```text
//! The adv instruction (opcode 0) performs division. The numerator is the value in the A register. The denominator is found by raising 2 to the power of the instruction's combo operand. (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.) The result of the division operation is truncated to an integer and then written to the A register.
//!
//! The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand, then stores the result in register B.
//!
//! The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits), then writes that value to the B register.
//!
//! The jnz instruction (opcode 3) does nothing if the A register is 0. However, if the A register is not zero, it jumps by setting the instruction pointer to the value of its literal operand; if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
//!
//! The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then stores the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)
//!
//! The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value. (If a program outputs multiple values, they are separated by commas.)
//!
//! The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register. (The numerator is still read from the A register.)
//!
//! The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register. (The numerator is still read from the A register.)
//! ```
//!
//! Here are some examples of instruction operation:
//!
//! ```text
//!     If register C contains 9, the program 2,6 would set register B to 1.
//!     If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
//!     If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
//!     If register B contains 29, the program 1,7 would set register B to 26.
//!     If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354.
//! ```
//!
//! The Historians' strange device has finished initializing its debugger and is displaying some information about the program it is trying to run (your puzzle input). For example:
//!
//! ```text
//! Register A: 729
//! Register B: 0
//! Register C: 0
//!
//! Program: 0,1,5,4,3,0
//! ```
//!
//! Your first task is to determine what the program is trying to output. To do this, initialize the registers to the given values, then run the given program, collecting any output produced by out instructions. (Always join the values produced by out instructions with commas.) After the above program halts, its final output will be 4,6,3,5,6,3,5,2,1,0.
//!
//! Using the information provided by the debugger, initialize the registers to the given values, then run the program. Once it halts, what do you get if you use commas to join the values it output into a single string?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{get_cap, get_cap_x, run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use console::style;
use crossterm::{
    cursor::{Hide, MoveToNextLine, RestorePosition, SavePosition, Show},
    terminal::{Clear, ClearType},
    ExecutableCommand, QueueableCommand,
};
use getset::{CopyGetters, Setters};
use itertools::Itertools;
use regex::Regex;
use std::{
    fs::File,
    io::{stdout, BufRead, BufReader, Write},
    thread::sleep,
    time::Duration,
};

#[derive(Clone, Copy, CopyGetters, Debug, Default, Eq, PartialEq, Setters)]
#[getset(get_copy = "pub(crate)", set = "pub(crate)")]
struct Registers {
    a: usize,
    b: usize,
    c: usize,
}

type ThreeBitData = (Registers, Vec<u8>, bool);

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<ThreeBitData, String>(AoCYear::AOC2024, AoCDay::AOCD17, setup, find)
        .map(|_| 0)
}
/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<ThreeBitData, String>(bench, AoCYear::AOC2024, AoCDay::AOCD17, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> ThreeBitData {
    setup_br(reader, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T, test: bool) -> Result<ThreeBitData>
where
    T: BufRead,
{
    let reg_re = Regex::new(r"^Register (.): (\d+)$")?;
    let prog_re = Regex::new(r"^Program: (.+)$")?;
    let mut registers = Registers::default();
    let mut program = vec![];

    for line in valid_lines(reader) {
        for caps in reg_re.captures_iter(&line) {
            let reg = get_cap(1, &caps)?;
            let val = get_cap_x::<usize>(2, &caps)?;
            let _ = match &reg[..] {
                "A" => registers.set_a(val),
                "B" => registers.set_b(val),
                "C" => registers.set_c(val),
                _ => return Err(anyhow!("invalid register")),
            };
        }

        for caps in prog_re.captures_iter(&line) {
            let prog = get_cap(1, &caps)?;
            program = prog
                .split(',')
                .map(str::parse::<u8>)
                .filter_map(Result::ok)
                .collect();
        }
    }
    Ok((registers, program, test))
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: ThreeBitData) -> String {
    find_res(data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(data: ThreeBitData, second_star: bool) -> Result<String> {
    let (mut regs, program, test) = data;
    let cmp_prog: Vec<usize> = program.iter().copied().map(usize::from).collect();
    let mut out = vec![];
    let len = cmp_prog.len();
    let mut a = 0;

    if second_star {
        let mut ind = cmp_prog.len() - 1;
        loop {
            let _ = regs.set_a(a);
            run_program(&mut regs, &program, &mut out, second_star, test)?;

            if out.len() == len && out[ind] == cmp_prog[ind] {
                if ind == 0 {
                    break;
                }
                ind -= 1;
            } else {
                a += 8_usize.pow(u32::try_from(ind)?);
            }
            out.clear();
        }
    } else {
        if !test {
            display_regs(&regs, &out, true, "")?;
        }
        run_program(&mut regs, &program, &mut out, second_star, test)?;
    }

    Ok(if second_star {
        a.to_string()
    } else {
        out.iter().map(ToString::to_string).join(",")
    })
}

fn run_program(
    regs: &mut Registers,
    program: &[u8],
    out: &mut Vec<usize>,
    second_star: bool,
    test: bool,
) -> Result<()> {
    let len = program.len();
    let mut ptr = 0;
    while ptr < len {
        let (opcode, operand) = read_instruction(program, &mut ptr)?;
        match opcode {
            0 => adv(regs, operand, &mut ptr)?,
            1 => bxl(regs, operand, &mut ptr),
            2 => bst(regs, operand, &mut ptr)?,
            3 => jnz(regs, operand, &mut ptr),
            4 => bxc(regs, operand, &mut ptr),
            5 => outp(regs, operand, &mut ptr, out)?,
            6 => bdv(regs, operand, &mut ptr)?,
            7 => cdv(regs, operand, &mut ptr)?,
            _ => {}
        }

        if !second_star && !test {
            display_regs(regs, out, true, "")?;
            sleep(Duration::from_millis(200));
        }
    }
    if !second_star && !test {
        display_regs(regs, out, false, "")?;
    }
    Ok(())
}

fn read_instruction(program: &[u8], ptr: &mut usize) -> Result<(u8, u8)> {
    let opcode = program
        .get(*ptr)
        .ok_or_else(|| anyhow!("invalid program"))?;
    let operand = program
        .get(*ptr + 1)
        .ok_or_else(|| anyhow!("invalid program"))?;
    Ok((*opcode, *operand))
}

fn adv(regs: &mut Registers, op: u8, ptr: &mut usize) -> Result<()> {
    let op_val = get_op_val(regs, op)?;
    let num = regs.a();
    let dem = 2_usize.pow(u32::try_from(op_val)?);
    let _ = regs.set_a(num / dem);
    *ptr += 2;
    Ok(())
}

fn bxl(regs: &mut Registers, op: u8, ptr: &mut usize) {
    let _ = regs.set_b(regs.b() ^ usize::from(op));
    *ptr += 2;
}

fn bst(regs: &mut Registers, op: u8, ptr: &mut usize) -> Result<()> {
    let op_val = get_op_val(regs, op)?;
    let _ = regs.set_b(op_val % 8);
    *ptr += 2;
    Ok(())
}

fn jnz(regs: &mut Registers, op: u8, ptr: &mut usize) {
    if regs.a() > 0 {
        *ptr = usize::from(op);
    } else {
        *ptr += 2;
    }
}

fn outp(regs: &mut Registers, op: u8, ptr: &mut usize, out: &mut Vec<usize>) -> Result<()> {
    let blah = get_op_val(regs, op)? % 8;
    out.push(blah);
    *ptr += 2;
    Ok(())
}

fn bxc(regs: &mut Registers, _op: u8, ptr: &mut usize) {
    let _ = regs.set_b(regs.b() ^ regs.c());
    *ptr += 2;
}

fn bdv(regs: &mut Registers, op: u8, ptr: &mut usize) -> Result<()> {
    let op_val = get_op_val(regs, op)?;
    let num = regs.a();
    let dem = 2_usize.pow(u32::try_from(op_val)?);
    let _ = regs.set_b(num / dem);
    *ptr += 2;
    Ok(())
}

fn cdv(regs: &mut Registers, op: u8, ptr: &mut usize) -> Result<()> {
    let op_val = get_op_val(regs, op)?;
    let num = regs.a();
    let dem = 2_usize.pow(u32::try_from(op_val)?);
    let _ = regs.set_c(num / dem);
    *ptr += 2;
    Ok(())
}

fn get_op_val(regs: &mut Registers, op: u8) -> Result<usize> {
    Ok(match op {
        0..=3 => usize::from(op),
        4 => regs.a(),
        5 => regs.b(),
        6 => regs.c(),
        _ => return Err(anyhow!("invalid operand")),
    })
}

fn display_regs(regs: &Registers, out: &[usize], restore: bool, header: &str) -> Result<()> {
    let mut stdout = stdout();

    let _ = stdout.execute(Hide)?;
    let _ = stdout.queue(SavePosition)?;
    let _ = stdout.write(format!("{} {header}", style("Registers:").bold().yellow()).as_bytes())?;
    let _ = stdout.queue(MoveToNextLine(1))?;
    let _ = stdout.queue(Clear(ClearType::CurrentLine))?;
    let _ = stdout.write(format!("A: {}", regs.a()).as_bytes())?;
    let _ = stdout.queue(MoveToNextLine(1))?;
    let _ = stdout.queue(Clear(ClearType::CurrentLine))?;
    let _ = stdout.write(format!("B: {}", regs.b()).as_bytes())?;
    let _ = stdout.queue(MoveToNextLine(1))?;
    let _ = stdout.queue(Clear(ClearType::CurrentLine))?;
    let _ = stdout.write(format!("C: {}", regs.c()).as_bytes())?;
    let _ = stdout.queue(MoveToNextLine(1))?;
    let _ = stdout.queue(MoveToNextLine(1))?;
    let _ = stdout.queue(Clear(ClearType::CurrentLine))?;
    let _ = stdout
        .write(format!("Output: {}", out.iter().map(ToString::to_string).join(",")).as_bytes())?;
    if restore {
        let _ = stdout.queue(RestorePosition)?;
    } else {
        let _ = stdout.queue(MoveToNextLine(1))?;
    }
    let _ = stdout.execute(Show)?;
    Ok(())
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<ThreeBitData, String>(AoCYear::AOC2024, AoCDay::AOCD17, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<ThreeBitData, String>(
        bench,
        AoCYear::AOC2024,
        AoCDay::AOCD17,
        setup,
        find2,
    )
    .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: ThreeBitData) -> String {
    find_res(data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const TEST_2: &str = r"Register A: 0
Register B: 0
Register C: 9

Program: 2,6";

    const TEST_3: &str = r"Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4";

    const TEST_4: &str = r"Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const TEST_5: &str = r"Register A: 0
Register B: 29
Register C: 0

Program: 1,7";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1), true)?;
        assert_eq!(find(data), "4,6,3,5,6,3,5,2,1,0");
        let data = setup_br(Cursor::new(TEST_2), true)?;
        assert_eq!(find(data), "");
        let data = setup_br(Cursor::new(TEST_3), true)?;
        assert_eq!(find(data), "0,1,2");
        let data = setup_br(Cursor::new(TEST_4), true)?;
        assert_eq!(find(data), "4,2,5,6,7,7,7,7,3,1,0");
        let data = setup_br(Cursor::new(TEST_5), true)?;
        assert_eq!(find(data), "");
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1), true)?;
        assert_eq!(find2(data), "117440");
        Ok(())
    }
}
