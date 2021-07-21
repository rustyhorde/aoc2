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
use aoc2_sol::constants::{
    AoCDay, AoCYear, DAY_1, DAY_10, DAY_11, DAY_12, DAY_13, DAY_14, DAY_15, DAY_16, DAY_17, DAY_18,
    DAY_19, DAY_2, DAY_20, DAY_21, DAY_22, DAY_23, DAY_24, DAY_25, DAY_3, DAY_4, DAY_5, DAY_6,
    DAY_7, DAY_8, DAY_9,
};
use clap::ArgMatches;
use lazy_static::lazy_static;
use std::{collections::HashMap, convert::TryFrom, io};

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
        (DAY_1, Some(matches)) => (matches, AoCDay::AOCD01),
        (DAY_2, Some(matches)) => (matches, AoCDay::AOCD02),
        (DAY_3, Some(matches)) => (matches, AoCDay::AOCD03),
        (DAY_4, Some(matches)) => (matches, AoCDay::AOCD04),
        (DAY_5, Some(matches)) => (matches, AoCDay::AOCD05),
        (DAY_6, Some(matches)) => (matches, AoCDay::AOCD06),
        (DAY_7, Some(matches)) => (matches, AoCDay::AOCD07),
        (DAY_8, Some(matches)) => (matches, AoCDay::AOCD08),
        (DAY_9, Some(matches)) => (matches, AoCDay::AOCD09),
        (DAY_10, Some(matches)) => (matches, AoCDay::AOCD10),
        (DAY_11, Some(matches)) => (matches, AoCDay::AOCD11),
        (DAY_12, Some(matches)) => (matches, AoCDay::AOCD12),
        (DAY_13, Some(matches)) => (matches, AoCDay::AOCD13),
        (DAY_14, Some(matches)) => (matches, AoCDay::AOCD14),
        (DAY_15, Some(matches)) => (matches, AoCDay::AOCD15),
        (DAY_16, Some(matches)) => (matches, AoCDay::AOCD16),
        (DAY_17, Some(matches)) => (matches, AoCDay::AOCD17),
        (DAY_18, Some(matches)) => (matches, AoCDay::AOCD18),
        (DAY_19, Some(matches)) => (matches, AoCDay::AOCD19),
        (DAY_20, Some(matches)) => (matches, AoCDay::AOCD20),
        (DAY_21, Some(matches)) => (matches, AoCDay::AOCD21),
        (DAY_22, Some(matches)) => (matches, AoCDay::AOCD22),
        (DAY_23, Some(matches)) => (matches, AoCDay::AOCD23),
        (DAY_24, Some(matches)) => (matches, AoCDay::AOCD24),
        (DAY_25, Some(matches)) => (matches, AoCDay::AOCD25),
        _ => return Err(anyhow!("Unable to determine the day you wish to run")),
    };

    let _ = find_solution(match_tuple.0, year, match_tuple.1)?;

    Ok(())
}

type FnMap = HashMap<(AoCYear, AoCDay, bool), fn() -> Result<u32>>;

lazy_static! {
    static ref FN_MAP: FnMap = {
        let mut fn_map: FnMap = HashMap::new();
        let _ = fn_map.insert(
            (AoCYear::AOC2015, AoCDay::AOCD01, false),
            aoc2_sol::year2015::day01::part_1,
        );
        let _ = fn_map.insert(
            (AoCYear::AOC2015, AoCDay::AOCD01, true),
            aoc2_sol::year2015::day01::part_2,
        );
        fn_map
    };
}

/// Find the solution.
fn find_solution(matches: &ArgMatches<'_>, year: AoCYear, day: AoCDay) -> Result<u32> {
    let is_second_star = matches.is_present("second");

    (*FN_MAP)
        .get(&(year, day, is_second_star))
        .and_then(|f| f().ok())
        .ok_or_else(|| anyhow!("blah"))
}
