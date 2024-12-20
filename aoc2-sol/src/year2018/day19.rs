// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2018 ---**
//!
//! **--- Day 19: Go With The Flow ---**
//!
//! With the Elves well on their way constructing the North Pole base, you turn your attention back to understanding the inner workings of programming the device.
//!
//! You can't help but notice that the device's opcodes don't contain any flow control like jump instructions. The device's manual goes on to explain:
//!
//! "In programs where flow control is required, the instruction pointer can be bound to a register so that it can be manipulated directly. This way, setr/seti can function as absolute jumps, addr/addi can function as relative jumps, and other opcodes can cause truly fascinating effects."
//!
//! This mechanism is achieved through a declaration like #ip 1, which would modify register 1 so that accesses to it let the program indirectly access the instruction pointer itself. To compensate for this kind of binding, there are now six registers (numbered 0 through 5); the five not bound to the instruction pointer behave as normal. Otherwise, the same rules apply as the last time you worked with this device.
//!
//! When the instruction pointer is bound to a register, its value is written to that register just before each instruction is executed, and the value of that register is written back to the instruction pointer immediately after each instruction finishes execution. Afterward, move to the next instruction by adding one to the instruction pointer, even if the value in the instruction pointer was just updated by an instruction. (Because of this, instructions must effectively set the instruction pointer to the instruction before the one they want executed next.)
//!
//! The instruction pointer is 0 during the first instruction, 1 during the second, and so on. If the instruction pointer ever causes the device to attempt to load an instruction outside the instructions defined in the program, the program instead immediately halts. The instruction pointer starts at 0.
//!
//! It turns out that this new information is already proving useful: the CPU in the device is not very powerful, and a background process is occupying most of its time. You dump the background process' declarations and instructions to a file (your puzzle input), making sure to use the names of the opcodes rather than the numbers.
//!
//! For example, suppose you have the following program:
//!
//! ```text
//! #ip 0
//! seti 5 0 1
//! seti 6 0 2
//! addi 0 1 0
//! addr 1 2 3
//! setr 1 0 0
//! seti 8 0 4
//! seti 9 0 5
//! ```
//!
//! When executed, the following instructions are executed. Each line contains the value of the instruction pointer at the time the instruction started, the values of the six registers before executing the instructions (in square brackets), the instruction itself, and the values of the six registers after executing the instruction (also in square brackets).
//!
//! ```text
//! ip=0 [0, 0, 0, 0, 0, 0] seti 5 0 1 [0, 5, 0, 0, 0, 0]
//! ip=1 [1, 5, 0, 0, 0, 0] seti 6 0 2 [1, 5, 6, 0, 0, 0]
//! ip=2 [2, 5, 6, 0, 0, 0] addi 0 1 0 [3, 5, 6, 0, 0, 0]
//! ip=4 [4, 5, 6, 0, 0, 0] setr 1 0 0 [5, 5, 6, 0, 0, 0]
//! ip=6 [6, 5, 6, 0, 0, 0] seti 9 0 5 [6, 5, 6, 0, 0, 9]
//! ```
//!
//! In detail, when running this program, the following events occur:
//!
//! ```text
//!     The first line (#ip 0) indicates that the instruction pointer should be bound to register 0 in this program. This is not an instruction, and so the value of the instruction pointer does not change during the processing of this line.
//!     The instruction pointer contains 0, and so the first instruction is executed (seti 5 0 1). It updates register 0 to the current instruction pointer value (0), sets register 1 to 5, sets the instruction pointer to the value of register 0 (which has no effect, as the instruction did not modify register 0), and then adds one to the instruction pointer.
//!     The instruction pointer contains 1, and so the second instruction, seti 6 0 2, is executed. This is very similar to the instruction before it: 6 is stored in register 2, and the instruction pointer is left with the value 2.
//!     The instruction pointer is 2, which points at the instruction addi 0 1 0. This is like a relative jump: the value of the instruction pointer, 2, is loaded into register 0. Then, addi finds the result of adding the value in register 0 and the value 1, storing the result, 3, back in register 0. Register 0 is then copied back to the instruction pointer, which will cause it to end up 1 larger than it would have otherwise and skip the next instruction (addr 1 2 3) entirely. Finally, 1 is added to the instruction pointer.
//!     The instruction pointer is 4, so the instruction setr 1 0 0 is run. This is like an absolute jump: it copies the value contained in register 1, 5, into register 0, which causes it to end up in the instruction pointer. The instruction pointer is then incremented, leaving it at 6.
//!     The instruction pointer is 6, so the instruction seti 9 0 5 stores 9 into register 5. The instruction pointer is incremented, causing it to point outside the program, and so the program ends.
//! ```
//!
//! What value is left in register 0 when the background process halts?
//!
//! **--- Part Two ---**
//!
//! A new background process immediately spins up in its place. It appears identical, but on closer inspection, you notice that this time, register 0 started with the value 1.
//!
//! What value is left in register 0 when this new background process halts?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::{anyhow, Error, Result};
use regex::Regex;
use std::{
    collections::HashMap,
    fmt,
    fs::File,
    io::{BufRead, BufReader},
};

type Registers = [usize; 6];
type Instruction = [usize; 3];

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
            OpCode::Addr => reg[ins[2]] = reg[ins[0]] + reg[ins[1]],
            OpCode::Addi => reg[ins[2]] = reg[ins[0]] + ins[1],
            OpCode::Mulr => reg[ins[2]] = reg[ins[0]] * reg[ins[1]],
            OpCode::Muli => reg[ins[2]] = reg[ins[0]] * ins[1],
            OpCode::Banr => reg[ins[2]] = reg[ins[0]] & reg[ins[1]],
            OpCode::Bani => reg[ins[2]] = reg[ins[0]] & ins[1],
            OpCode::Borr => reg[ins[2]] = reg[ins[0]] | reg[ins[1]],
            OpCode::Bori => reg[ins[2]] = reg[ins[0]] | ins[1],
            OpCode::Setr => reg[ins[2]] = reg[ins[0]],
            OpCode::Seti => reg[ins[2]] = ins[0],
            OpCode::Gtir => reg[ins[2]] = usize::from(ins[0] > reg[ins[1]]),
            OpCode::Gtri => reg[ins[2]] = usize::from(reg[ins[0]] > ins[1]),
            OpCode::Gtrr => reg[ins[2]] = usize::from(reg[ins[0]] > reg[ins[1]]),
            OpCode::Eqir => reg[ins[2]] = usize::from(ins[0] == reg[ins[1]]),
            OpCode::Eqri => reg[ins[2]] = usize::from(reg[ins[0]] == ins[1]),
            OpCode::Eqrr => reg[ins[2]] = usize::from(reg[ins[0]] == reg[ins[1]]),
        }
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OpCode::Addr => "addr",
                OpCode::Addi => "addi",
                OpCode::Mulr => "mulr",
                OpCode::Muli => "muli",
                OpCode::Banr => "banr",
                OpCode::Bani => "bani",
                OpCode::Borr => "borr",
                OpCode::Bori => "bori",
                OpCode::Setr => "setr",
                OpCode::Seti => "seti",
                OpCode::Gtir => "gtir",
                OpCode::Gtri => "gtri",
                OpCode::Gtrr => "gtrr",
                OpCode::Eqir => "eqir",
                OpCode::Eqri => "eqri",
                OpCode::Eqrr => "eqrr",
            }
        )
    }
}
impl TryFrom<&str> for OpCode {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        Ok(match s {
            "addr" => OpCode::Addr,
            "addi" => OpCode::Addi,
            "mulr" => OpCode::Mulr,
            "muli" => OpCode::Muli,
            "banr" => OpCode::Banr,
            "bani" => OpCode::Bani,
            "borr" => OpCode::Borr,
            "bori" => OpCode::Bori,
            "setr" => OpCode::Setr,
            "seti" => OpCode::Seti,
            "gtir" => OpCode::Gtir,
            "gtri" => OpCode::Gtri,
            "gtrr" => OpCode::Gtrr,
            "eqir" => OpCode::Eqir,
            "eqri" => OpCode::Eqri,
            "eqrr" => OpCode::Eqrr,
            _ => return Err(anyhow!("invalid opcode")),
        })
    }
}

struct Ip {
    register: usize,
    value: usize,
}

impl fmt::Display for Ip {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.register, self.value)
    }
}

type AsmData = (Vec<HashMap<OpCode, Instruction>>, usize, bool);

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<AsmData, usize>(AoCYear::AOC2018, AoCDay::AOCD19, setup, find).map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<AsmData, usize>(bench, AoCYear::AOC2018, AoCDay::AOCD19, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> AsmData {
    setup_br(reader, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T, test: bool) -> Result<AsmData>
where
    T: BufRead,
{
    let ip_re = Regex::new(r"#ip (\d+)")?;
    let instruction_re = Regex::new(r"([a-z]+) (\d+) (\d+) (\d+)")?;
    let mut instructions_vec = vec![];
    let mut register = 0;

    for line in valid_lines(reader) {
        if ip_re.is_match(&line) {
            for caps in ip_re.captures_iter(&line) {
                register = (caps[1]).parse::<usize>()?;
            }
        } else if instruction_re.is_match(&line) {
            for caps in instruction_re.captures_iter(&line) {
                let mut instruction_map = HashMap::new();
                let opcode = OpCode::try_from(&caps[1])?;
                let reg_a = (caps[2]).parse::<usize>()?;
                let reg_b = (caps[3]).parse::<usize>()?;
                let reg_c = (caps[4]).parse::<usize>()?;

                let _ = instruction_map.insert(opcode, [reg_a, reg_b, reg_c]);
                instructions_vec.push(instruction_map);
            }
        }
    }
    Ok((instructions_vec, register, test))
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: AsmData) -> usize {
    find_res(data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(data: AsmData, second_star: bool) -> Result<usize> {
    let (instructions_vec, register, _test) = data;

    let mut ip = Ip { register, value: 0 };
    let mut registers: Registers = if second_star {
        [1, 0, 0, 0, 0, 0]
    } else {
        [0, 0, 0, 0, 0, 0]
    };

    while is_ins(&ip, &instructions_vec).is_some() {
        update_register_with_ip(&mut registers, &ip);
        execute(&mut registers, &ip, &instructions_vec);
        update_ip_with_register(&registers, &mut ip);
    }

    Ok(registers[0])
}

fn is_ins(ip: &Ip, ins_vec: &[HashMap<OpCode, Instruction>]) -> Option<()> {
    ins_vec.get(ip.value).map(|_| ())
}

fn update_register_with_ip(registers: &mut Registers, ip: &Ip) {
    registers[ip.register] = ip.value;
}

fn execute(registers: &mut Registers, ip: &Ip, ins_vec: &[HashMap<OpCode, Instruction>]) {
    if let Some(ins_map) = ins_vec.get(ip.value) {
        if ins_map.len() == 1 {
            for (opcode, ins) in ins_map {
                opcode.execute(registers, *ins);
            }
        }
    }
}

fn update_ip_with_register(registers: &Registers, ip: &mut Ip) {
    ip.value = registers[ip.register];
    ip.value += 1;
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<AsmData, usize>(AoCYear::AOC2018, AoCDay::AOCD19, setup, find2).map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<AsmData, usize>(bench, AoCYear::AOC2018, AoCDay::AOCD19, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: AsmData) -> usize {
    find_res(data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1), true)?;
        assert_eq!(find(data), 6);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {}
