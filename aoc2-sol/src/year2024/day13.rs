// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code - Day 13 ---**
//!
//! **--- Day 13: Claw Contraption ---**
//!
//! Next up: the lobby of a resort on a tropical island. The Historians take a moment to admire the hexagonal floor tiles before spreading out.
//!
//! Fortunately, it looks like the resort has a new arcade! Maybe you can win some prizes from the claw machines?
//!
//! The claw machines here are a little unusual. Instead of a joystick or directional buttons to control the claw, these machines have two buttons labeled A and B. Worse, you can't just put in a token and play; it costs 3 tokens to push the A button and 1 token to push the B button.
//!
//! With a little experimentation, you figure out that each machine's buttons are configured to move the claw a specific amount to the right (along the X axis) and a specific amount forward (along the Y axis) each time that button is pressed.
//!
//! Each machine contains one prize; to win the prize, the claw must be positioned exactly above the prize on both the X and Y axes.
//!
//! You wonder: what is the smallest number of tokens you would have to spend to win as many prizes as possible? You assemble a list of every machine's button behavior and prize location (your puzzle input). For example:
//!
//! ```text
//! Button A: X+94, Y+34
//! Button B: X+22, Y+67
//! Prize: X=8400, Y=5400
//!
//! Button A: X+26, Y+66
//! Button B: X+67, Y+21
//! Prize: X=12748, Y=12176
//!
//! Button A: X+17, Y+86
//! Button B: X+84, Y+37
//! Prize: X=7870, Y=6450
//!
//! Button A: X+69, Y+23
//! Button B: X+27, Y+71
//! Prize: X=18641, Y=10279
//! ```
//!
//! This list describes the button configuration and prize location of four different claw machines.
//!
//! For now, consider just the first claw machine in the list:
//!
//! ```text
//!     Pushing the machine's A button would move the claw 94 units along the X axis and 34 units along the Y axis.
//!     Pushing the B button would move the claw 22 units along the X axis and 67 units along the Y axis.
//!     The prize is located at X=8400, Y=5400; this means that from the claw's initial position, it would need to move exactly 8400 units along the X axis and exactly 5400 units along the Y axis to be perfectly aligned with the prize in this machine.
//! ```
//!
//! The cheapest way to win the prize is by pushing the A button 80 times and the B button 40 times. This would line up the claw along the X axis (because 80*94 + 40*22 = 8400) and along the Y axis (because 80*34 + 40*67 = 5400). Doing this would cost 80*3 tokens for the A presses and 40*1 for the B presses, a total of 280 tokens.
//!
//! For the second and fourth claw machines, there is no combination of A and B presses that will ever win a prize.
//!
//! For the third claw machine, the cheapest way to win the prize is by pushing the A button 38 times and the B button 86 times. Doing this would cost a total of 200 tokens.
//!
//! So, the most prizes you could possibly win is two; the minimum tokens you would have to spend to win all (two) prizes is 480.
//!
//! You estimate that each button would need to be pressed no more than 100 times to win a prize. How else would someone be expected to play?
//!
//! Figure out how to win as many prizes as possible. What is the fewest tokens you would have to spend to win all possible prizes?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{get_cap, get_cap_x, run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use getset::{CopyGetters, Setters};
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<Vec<Machine>, f64>(AoCYear::AOC2024, AoCDay::AOCD13, setup, find)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<Machine>, f64>(bench, AoCYear::AOC2024, AoCDay::AOCD13, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> Vec<Machine> {
    setup_br(reader).unwrap_or_default()
}

#[derive(Clone, Copy, CopyGetters, Debug, Default, PartialEq, Setters)]
#[getset(get_copy = "pub(crate)", set = "pub(crate)")]
struct Machine {
    // button a, x value
    a: f64,
    // button b, x value
    b: f64,
    // button a, y value
    d: f64,
    // button b, y value
    e: f64,
    // x destination
    c: f64,
    // y deatination
    f: f64,
}

impl Machine {
    fn solve(&self, part2: bool) -> Result<f64> {
        let c = if part2 {
            self.c + 10_000_000_000_000.
        } else {
            self.c
        };

        let f = if part2 {
            self.f + 10_000_000_000_000.
        } else {
            self.f
        };
        // Cramer's Method
        // delta = (a * e) + (b * d)
        let delta = (self.a * self.e) - (self.b * self.d);
        // num_x = (c * e) - (b * f)
        let num_x = (c * self.e) - (self.b * f);
        // num_y = (a * f) - (c *  d)
        let num_y = (self.a * f) - (c * self.d);
        let x = num_x / delta;
        let y = num_y / delta;

        if x.fract() == 0. && y.fract() == 0. {
            Ok((3. * x) + y)
        } else {
            Err(anyhow!("broken machine"))
        }
    }
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T) -> Result<Vec<Machine>>
where
    T: BufRead,
{
    let button_re = Regex::new(r"^Button (A|B): X\+(\d+), Y\+(\d+)$")?;
    let prize_re = Regex::new(r"^Prize: X=(\d+), Y=(\d+)$")?;
    let mut machines = vec![];

    let mut curr_machine = Machine::default();

    for line in valid_lines(reader) {
        if let Some(caps) = button_re.captures(&line) {
            let button = get_cap(1, &caps)?;
            let x = f64::from(get_cap_x::<u32>(2, &caps)?);
            let y = f64::from(get_cap_x::<u32>(3, &caps)?);

            if button == "A" {
                let _ = curr_machine.set_a(x);
                let _ = curr_machine.set_d(y);
            } else if button == "B" {
                let _ = curr_machine.set_b(x);
                let _ = curr_machine.set_e(y);
            }
        } else if let Some(caps) = prize_re.captures(&line) {
            let c = f64::from(get_cap_x::<u32>(1, &caps)?);
            let f: f64 = f64::from(get_cap_x::<u32>(2, &caps)?);
            let _ = curr_machine.set_c(c);
            let _ = curr_machine.set_f(f);
            machines.push(curr_machine);
            curr_machine = Machine::default();
        }
    }
    Ok(machines)
}

#[allow(clippy::needless_pass_by_value)]
fn find(machines: Vec<Machine>) -> f64 {
    machines.iter().filter_map(|x| x.solve(false).ok()).sum()
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<Vec<Machine>, f64>(AoCYear::AOC2024, AoCDay::AOCD13, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<Machine>, f64>(bench, AoCYear::AOC2024, AoCDay::AOCD13, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(machines: Vec<Machine>) -> f64 {
    machines.iter().filter_map(|x| x.solve(true).ok()).sum()
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    #[allow(clippy::float_cmp)]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find(data), 480.);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    #[allow(clippy::float_cmp)]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find2(data), 875_318_608_908.);
        Ok(())
    }
}
