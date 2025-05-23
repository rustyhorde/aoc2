// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2019 ---***
//!
//! **--- Day 7: Amplification Circuit ---**
//!
//! Based on the navigational maps, you're going to need to send more power to your ship's thrusters to reach Santa in time. To do this, you'll need to configure a series of amplifiers already installed on the ship.
//!
//! There are five amplifiers connected in series; each one receives an input signal and produces an output signal. They are connected such that the first amplifier's output leads to the second amplifier's input, the second amplifier's output leads to the third amplifier's input, and so on. The first amplifier's input value is 0, and the last amplifier's output leads to your ship's thrusters.
//!
//! ```text
//!     O-------O  O-------O  O-------O  O-------O  O-------O
//! 0 ->| Amp A |->| Amp B |->| Amp C |->| Amp D |->| Amp E |-> (to thrusters)
//!     O-------O  O-------O  O-------O  O-------O  O-------O
//! ```
//!
//! The Elves have sent you some Amplifier Controller Software (your puzzle input), a program that should run on your existing Intcode computer. Each amplifier will need to run a copy of the program.
//!
//! When a copy of the program starts running on an amplifier, it will first use an input instruction to ask the amplifier for its current phase setting (an integer from 0 to 4). Each phase setting is used exactly once, but the Elves can't remember which amplifier needs which phase setting.
//!
//! The program will then call another input instruction to get the amplifier's input signal, compute the correct output signal, and supply it back to the amplifier with an output instruction. (If the amplifier has not yet received an input signal, it waits until one arrives.)
//!
//! Your job is to find the largest output signal that can be sent to the thrusters by trying every possible combination of phase settings on the amplifiers. Make sure that memory is not shared or reused between copies of the program.
//!
//! For example, suppose you want to try the phase setting sequence 3,1,2,4,0, which would mean setting amplifier A to phase setting 3, amplifier B to setting 1, C to 2, D to 4, and E to 0. Then, you could determine the output signal that gets sent from amplifier E to the thrusters with the following steps:
//!
//! ```text
//!     Start the copy of the amplifier controller software that will run on amplifier A. At its first input instruction, provide it the amplifier's phase setting, 3. At its second input instruction, provide it the input signal, 0. After some calculations, it will use an output instruction to indicate the amplifier's output signal.
//!     Start the software for amplifier B. Provide it the phase setting (1) and then whatever output signal was produced from amplifier A. It will then produce a new output signal destined for amplifier C.
//!     Start the software for amplifier C, provide the phase setting (2) and the value from amplifier B, then collect its output signal.
//!     Run amplifier D's software, provide the phase setting (4) and input value, and collect its output signal.
//!     Run amplifier E's software, provide the phase setting (0) and input value, and collect its output signal.
//! ```
//!
//! The final output signal from amplifier E would be sent to the thrusters. However, this phase setting sequence may not have been the best one; another sequence might have sent a higher signal to the thrusters.
//!
//! Here are some example programs:
//!
//! ```text
//!     Max thruster signal 43210 (from phase setting sequence 4,3,2,1,0):
//!
//!     3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0
//!
//!     Max thruster signal 54321 (from phase setting sequence 0,1,2,3,4):
//!
//!     3,23,3,24,1002,24,10,24,1002,23,-1,23,
//!     101,5,23,23,1,24,23,23,4,23,99,0,0
//!
//!     Max thruster signal 65210 (from phase setting sequence 1,0,4,3,2):
//!
//!     3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
//!     1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0
//! ```
//!
//! Try every combination of phase settings on the amplifiers. What is the highest signal that can be sent to the thrusters?
//!
//! **--- Part Two ---**
//!
//! It's no good - in this configuration, the amplifiers can't generate a large enough output signal to produce the thrust you'll need. The Elves quickly talk you through rewiring the amplifiers into a feedback loop:
//!
//! ```text
//! O-------O  O-------O  O-------O  O-------O  O-------O
//! 0 -+->| Amp A |->| Amp B |->| Amp C |->| Amp D |->| Amp E |-.
//! |  O-------O  O-------O  O-------O  O-------O  O-------O |
//! |                                                        |
//! '--------------------------------------------------------+
//!                                                       |
//!                                                       v
//!                                                (to thrusters)
//! ```
//!
//! Most of the amplifiers are connected as they were before; amplifier A's output is connected to amplifier B's input, and so on. However, the output from amplifier E is now connected into amplifier A's input. This creates the feedback loop: the signal will be sent through the amplifiers many times.
//!
//! In feedback loop mode, the amplifiers need totally different phase settings: integers from 5 to 9, again each used exactly once. These settings will cause the Amplifier Controller Software to repeatedly take input and produce output many times before halting. Provide each amplifier its phase setting at its first input instruction; all further input/output instructions are for signals.
//!
//! Don't restart the Amplifier Controller Software on any amplifier during this process. Each one should continue receiving and sending signals until it halts.
//!
//! All signals sent or received in this process will be between pairs of amplifiers except the very first signal and the very last signal. To start the process, a 0 signal is sent to amplifier A's input exactly once.
//!
//! Eventually, the software on the amplifiers will halt after they have processed the final loop. When this happens, the last output signal from amplifier E is sent to the thrusters. Your job is to find the largest output signal that can be sent to the thrusters using the new phase settings and feedback loop arrangement.
//!
//! Here are some example programs:
//!
//! ```text
//! Max thruster signal 139629729 (from phase setting sequence 9,8,7,6,5):
//!
//! 3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
//! 27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5
//! ```
//!
//! ```text
//! Max thruster signal 18216 (from phase setting sequence 9,7,8,5,6):
//!
//! 3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
//! -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
//! 53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10
//! ```
//!
//! Try every combination of the new phase settings on the amplifier feedback loop. What is the highest signal that can be sent to the thrusters?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use bnum::types::I256;
use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    thread::spawn,
};

use super::intcode::{Intcode, IntcodeData};

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<IntcodeData, usize>(AoCYear::AOC2019, AoCDay::AOCD07, setup, find)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<IntcodeData, usize>(bench, AoCYear::AOC2019, AoCDay::AOCD07, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> IntcodeData {
    match setup_br(reader) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("{e}");
            IntcodeData::default()
        }
    }
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

fn as_isize(x: &str) -> Option<I256> {
    x.parse::<I256>().ok()
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: IntcodeData) -> usize {
    match find_res(&data, false) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("{e}");
            0
        }
    }
}

#[allow(clippy::similar_names, clippy::unnecessary_wraps, tail_expr_drop_order)]
fn find_res(intcodes: &IntcodeData, second_star: bool) -> Result<usize> {
    let mut max_thrust = I256::MIN;

    let range = if second_star { 5..=9 } else { 0..=4 };
    for phase_settings in range.permutations(5) {
        let (send_a, mut amp_a) = Intcode::new(intcodes.clone());
        let (send_b, mut amp_b) = Intcode::new(intcodes.clone());
        let (send_c, mut amp_c) = Intcode::new(intcodes.clone());
        let (send_d, mut amp_d) = Intcode::new(intcodes.clone());
        let (send_e, mut amp_e) = Intcode::new(intcodes.clone());

        let _ = amp_a.set_sender_opt(Some(send_b.clone()));
        let _ = amp_b.set_sender_opt(Some(send_c.clone()));
        let _ = amp_c.set_sender_opt(Some(send_d.clone()));
        let _ = amp_d.set_sender_opt(Some(send_e.clone()));
        if second_star {
            let _ = amp_e.set_sender_opt(Some(send_a.clone()));
        }

        let amp_a_handle = spawn(move || amp_a.start());
        let amp_b_handle = spawn(move || amp_b.start());
        let amp_c_handle = spawn(move || amp_c.start());
        let amp_d_handle = spawn(move || amp_d.start());
        let amd_e_handle = spawn(move || amp_e.start());

        for (idx, phase_setting) in phase_settings.iter().enumerate() {
            if idx == 0 {
                send_a.send(I256::from(*phase_setting))?;
            } else if idx == 1 {
                send_b.send(I256::from(*phase_setting))?;
            } else if idx == 2 {
                send_c.send(I256::from(*phase_setting))?;
            } else if idx == 3 {
                send_d.send(I256::from(*phase_setting))?;
            } else if idx == 4 {
                send_e.send(I256::from(*phase_setting))?;
            }
        }

        // Start the chaos
        send_a.send(I256::ZERO)?;

        let _res = amp_a_handle.join();
        let _res = amp_b_handle.join();
        let _res = amp_c_handle.join();
        let _res = amp_d_handle.join();
        match amd_e_handle.join() {
            Ok(res) => match res {
                Ok(r) => {
                    if r > max_thrust {
                        max_thrust = r;
                    }
                }
                Err(e) => eprintln!("{e}"),
            },
            Err(e) => eprintln!("{e:?}"),
        }
    }

    usize::try_from(max_thrust).map_err(|e| anyhow!("{e}"))
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<IntcodeData, usize>(AoCYear::AOC2019, AoCDay::AOCD07, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<IntcodeData, usize>(bench, AoCYear::AOC2019, AoCDay::AOCD07, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: IntcodeData) -> usize {
    find_res(&data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
    const TEST_2: &str =
        r"3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
    const TEST_3: &str = r"3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find(data), 43210);
        let data = setup_br(Cursor::new(TEST_2))?;
        assert_eq!(find(data), 54321);
        let data = setup_br(Cursor::new(TEST_3))?;
        assert_eq!(find(data), 65210);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str =
        r"3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
    const TEST_2: &str = r"3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find2(data), 139_629_729);
        let data = setup_br(Cursor::new(TEST_2))?;
        assert_eq!(find2(data), 18216);
        Ok(())
    }
}
