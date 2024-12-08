// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `aoc2` runtime

mod cli;
mod config;
mod fnmap;
mod header;

use self::{
    cli::{AoC2Subcommand, Args, Command},
    fnmap::FN_MAP,
    header::header,
};
use anyhow::{anyhow, Context, Result};
use aoc2_sol::constants::{AoCDay, AoCYear};
use clap::Parser;
use config::{load, ConfigAoc2};
use fnmap::BENCH_MAP;
use std::convert::TryFrom;
use tracing::trace;

use crate::{
    constants::HEADER_PREFIX,
    error::Error::{ConfigLoad, TracingInit},
    tracing::initialize,
};

pub(crate) fn run() -> Result<()> {
    // Parse the command line
    let matches = Args::try_parse()?;

    // Load the configuration
    let config =
        load::<Args, ConfigAoc2<'_>, Args>(&matches, &matches).with_context(|| ConfigLoad)?;
    initialize(&config, None).with_context(|| TracingInit)?;
    trace!("configuration loaded - {}", config.env());
    trace!("tracing initialized");

    // Output the pretty header
    header(HEADER_PREFIX)?;

    let year = AoCYear::try_from(&matches.year()[..])?;

    let match_tuple = match matches.command() {
        Command::Day01(command) => (command, AoCDay::AOCD01),
        Command::Day02(command) => (command, AoCDay::AOCD02),
        Command::Day03(command) => (command, AoCDay::AOCD03),
        Command::Day04(command) => (command, AoCDay::AOCD04),
        Command::Day05(command) => (command, AoCDay::AOCD05),
        Command::Day06(command) => (command, AoCDay::AOCD06),
        Command::Day07(command) => (command, AoCDay::AOCD07),
        Command::Day08(command) => (command, AoCDay::AOCD08),
        Command::Day09(command) => (command, AoCDay::AOCD09),
        Command::Day10(command) => (command, AoCDay::AOCD10),
        Command::Day11(command) => (command, AoCDay::AOCD11),
        Command::Day12(command) => (command, AoCDay::AOCD12),
        Command::Day13(command) => (command, AoCDay::AOCD13),
        Command::Day14(command) => (command, AoCDay::AOCD14),
        Command::Day15(command) => (command, AoCDay::AOCD15),
        Command::Day16(command) => (command, AoCDay::AOCD16),
        Command::Day17(command) => (command, AoCDay::AOCD17),
        Command::Day18(command) => (command, AoCDay::AOCD18),
        Command::Day19(command) => (command, AoCDay::AOCD19),
        Command::Day20(command) => (command, AoCDay::AOCD20),
        Command::Day21(command) => (command, AoCDay::AOCD21),
        Command::Day22(command) => (command, AoCDay::AOCD22),
        Command::Day23(command) => (command, AoCDay::AOCD23),
        Command::Day24(command) => (command, AoCDay::AOCD24),
        Command::Day25(command) => (command, AoCDay::AOCD25),
    };

    _ = find_solution(&config, match_tuple.0, year, match_tuple.1)?;

    Ok(())
}

/// Find the solution.
fn find_solution(
    config: &ConfigAoc2<'_>,
    matches: &AoC2Subcommand,
    year: AoCYear,
    day: AoCDay,
) -> Result<u32> {
    let is_second_star = matches.second();

    if let Some(bench) = config.bench() {
        if let Some(f) = (*BENCH_MAP).get(&(year, day, *is_second_star)) {
            f(*bench)
        } else {
            Err(anyhow!("Unable to find year and day to run!"))
        }
    } else if let Some(f) = (*FN_MAP).get(&(year, day, *is_second_star)) {
        f()
    } else {
        Err(anyhow!("Unable to find year and day to run!"))
    }
}
