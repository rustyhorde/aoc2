// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2019 ---**
//!
//! **--- Day 9: Sensor Boost ---**
//!
//! You've just said goodbye to the rebooted rover and left Mars when you receive a faint distress signal coming from the asteroid belt. It must be the Ceres monitoring station!
//!
//! In order to lock on to the signal, you'll need to boost your sensors. The Elves send up the latest BOOST program - Basic Operation Of System Test.
//!
//! While BOOST (your puzzle input) is capable of boosting your sensors, for tenuous safety reasons, it refuses to do so until the computer it runs on passes some checks to demonstrate it is a complete Intcode computer.
//!
//! Your existing Intcode computer is missing one key feature: it needs support for parameters in relative mode.
//!
//! Parameters in mode 2, relative mode, behave very similarly to parameters in position mode: the parameter is interpreted as a position. Like position mode, parameters in relative mode can be read from or written to.
//!
//! The important difference is that relative mode parameters don't count from address 0. Instead, they count from a value called the relative base. The relative base starts at 0.
//!
//! The address a relative mode parameter refers to is itself plus the current relative base. When the relative base is 0, relative mode parameters and position mode parameters with the same value refer to the same address.
//!
//! For example, given a relative base of 50, a relative mode parameter of -7 refers to memory address 50 + -7 = 43.
//!
//! The relative base is modified with the relative base offset instruction:
//!
//! ```text
//!     Opcode 9 adjusts the relative base by the value of its only parameter. The relative base increases (or decreases, if the value is negative) by the value of the parameter.
//! ```
//!
//! For example, if the relative base is 2000, then after the instruction 109,19, the relative base would be 2019. If the next instruction were 204,-34, then the value at address 1985 would be output.
//!
//! Your Intcode computer will also need a few other capabilities:
//!
//! ```text
//!     The computer's available memory should be much larger than the initial program. Memory beyond the initial program starts with the value 0 and can be read or written like any other memory. (It is invalid to try to access memory at a negative address, though.)
//!     The computer should have support for large numbers. Some instructions near the beginning of the BOOST program will verify this capability.
//! ```
//!
//! Here are some example programs that use these features:
//!
//! ```text
//!     109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99 takes no input and produces a copy of itself as output.
//!     1102,34915192,34915192,7,4,7,99,0 should output a 16-digit number.
//!     104,1125899906842624,99 should output the large number in the middle.
//! ```
//!
//! The BOOST program will ask for a single input; run it in test mode by providing it the value 1. It will perform a series of checks on each opcode, output any opcodes (and the associated parameter modes) that seem to be functioning incorrectly, and finally output a BOOST keycode.
//!
//! Once your Intcode computer is fully functional, the BOOST program should report no malfunctioning opcodes when run in test mode; it should only output a single value, the BOOST keycode. What BOOST keycode does it produce?

use crate::year2019::intcode::{as_isize, Intcode, IntcodeData};
use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::Result;
use bnum::types::I256;
use itertools::Itertools;
use std::sync::mpsc::channel;
use std::thread::spawn;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](AoCYear) and
///   [`AoCDay`](AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<IntcodeData, String>(AoCYear::AOC2019, AoCDay::AOCD09, setup, find)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<IntcodeData, String>(bench, AoCYear::AOC2019, AoCDay::AOCD09, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> IntcodeData {
    setup_br(reader).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T) -> Result<IntcodeData>
where
    T: BufRead,
{
    let mut intcodes = vec![];
    for line in valid_lines(reader) {
        intcodes = line.split(',').filter_map(as_isize).collect();
    }
    Ok(intcodes)
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: IntcodeData) -> String {
    find_res(&data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(intcodes: &IntcodeData, second_star: bool) -> Result<String> {
    let (sender, receiver) = channel();
    let (send_a, mut amp_a) = Intcode::new(intcodes.clone());
    let _ = amp_a.set_sender_opt(Some(sender));

    let amp_a_handle = spawn(move || amp_a.start());

    // Start the chaos
    let input = if second_star {
        I256::from(2)
    } else {
        I256::ONE
    };
    send_a.send(input)?;

    let mut output = vec![];
    while let Ok(res) = receiver.recv() {
        output.push(res);
    }

    match amp_a_handle.join() {
        Ok(res) => match res {
            Ok(_r) => {}
            Err(e) => eprintln!("{e}"),
        },
        Err(e) => eprintln!("{e:?}"),
    }

    Ok(output.iter().map(I256::to_string).join(","))
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](AoCYear) and
///   [`AoCDay`](AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<IntcodeData, String>(AoCYear::AOC2019, AoCDay::AOCD09, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<IntcodeData, String>(bench, AoCYear::AOC2019, AoCDay::AOCD09, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: IntcodeData) -> String {
    find_res(&data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
    const TEST_2: &str = r"1102,34915192,34915192,7,4,7,99,0 ";
    const TEST_3: &str = r"104,1125899906842624,99 ";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(
            find(data),
            "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"
        );
        let data = setup_br(Cursor::new(TEST_2))?;
        assert_eq!(find(data), "1219070632396864");
        let data = setup_br(Cursor::new(TEST_3))?;
        assert_eq!(find(data), "1125899906842624");
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find2(data), "");
        Ok(())
    }
}
