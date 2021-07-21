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
mod header;

use self::cli::YEAR;
use anyhow::{anyhow, Result};
use aoc2_sol::constants::{self, AoCDay, AoCYear};
use clap::ArgMatches;
use std::{
    convert::TryFrom,
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

pub(crate) fn run() -> Result<()> {
    // Parse the command line
    let matches = cli::app().get_matches_safe()?;

    // Load the configuration
    // let config = load(&matches)?;

    // Output the pretty header
    header::header(&mut io::stdout())?;

    let year = AoCYear::try_from(
        matches
            .value_of(YEAR)
            .ok_or_else(|| anyhow!("invalid year"))?,
    )?;

    let match_tuple = match matches.subcommand() {
        (constants::DAY_1, Some(matches)) => (matches, AoCDay::AOCD01),
        (constants::DAY_2, Some(matches)) => (matches, AoCDay::AOCD02),
        (constants::DAY_3, Some(matches)) => (matches, AoCDay::AOCD03),
        (constants::DAY_4, Some(matches)) => (matches, AoCDay::AOCD04),
        (constants::DAY_5, Some(matches)) => (matches, AoCDay::AOCD05),
        (constants::DAY_6, Some(matches)) => (matches, AoCDay::AOCD06),
        (constants::DAY_7, Some(matches)) => (matches, AoCDay::AOCD07),
        (constants::DAY_8, Some(matches)) => (matches, AoCDay::AOCD08),
        (constants::DAY_9, Some(matches)) => (matches, AoCDay::AOCD09),
        (constants::DAY_10, Some(matches)) => (matches, AoCDay::AOCD10),
        (constants::DAY_11, Some(matches)) => (matches, AoCDay::AOCD11),
        (constants::DAY_12, Some(matches)) => (matches, AoCDay::AOCD12),
        (constants::DAY_13, Some(matches)) => (matches, AoCDay::AOCD13),
        (constants::DAY_14, Some(matches)) => (matches, AoCDay::AOCD14),
        (constants::DAY_15, Some(matches)) => (matches, AoCDay::AOCD15),
        (constants::DAY_16, Some(matches)) => (matches, AoCDay::AOCD16),
        (constants::DAY_17, Some(matches)) => (matches, AoCDay::AOCD17),
        (constants::DAY_18, Some(matches)) => (matches, AoCDay::AOCD18),
        (constants::DAY_19, Some(matches)) => (matches, AoCDay::AOCD19),
        (constants::DAY_20, Some(matches)) => (matches, AoCDay::AOCD20),
        (constants::DAY_21, Some(matches)) => (matches, AoCDay::AOCD21),
        (constants::DAY_22, Some(matches)) => (matches, AoCDay::AOCD22),
        (constants::DAY_23, Some(matches)) => (matches, AoCDay::AOCD23),
        (constants::DAY_24, Some(matches)) => (matches, AoCDay::AOCD24),
        (constants::DAY_25, Some(matches)) => (matches, AoCDay::AOCD25),
        _ => return Err(anyhow!("Unable to determine the day you wish to run")),
    };

    let _ = find_solution(match_tuple.0, year, match_tuple.1)?;

    Ok(())
}

/// Find the solution.
fn find_solution(matches: &ArgMatches<'_>, year: AoCYear, day: AoCDay) -> Result<u32> {
    let year_str: &str = year.into();
    let day_str: &str = day.into();
    let mut filepath = PathBuf::from("data");
    filepath.push(year_str);
    filepath.push(day_str);
    filepath.push(
        matches
            .value_of("file")
            .ok_or_else(|| anyhow!("Invalid filename!"))?,
    );

    let reader = BufReader::new(File::open(filepath)?);
    let is_second_star = matches.is_present("second");

    aoc2_sol::find_solution(reader.lines(), year, day, is_second_star)
}
