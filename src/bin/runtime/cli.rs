// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `aoc` Command Line Interface

use aoc2::constants::{get_day_about, AoCDay};
use clap::{crate_authors, crate_name, crate_version, App, Arg, SubCommand};

pub(crate) const DEFAULT_YEAR: &str = "2021";
pub(crate) const YEAR: &str = "year";

pub(crate) fn app<'a, 'b>() -> App<'a, 'b> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about("Run Advent of Code daily problems")
        .usage("\u{1f31f}   solution: aoc <day>\n    \u{1f31f}\u{1f31f} solution: aoc <day> -s")
        .arg(
            Arg::with_name(YEAR)
                .short("y")
                .long(YEAR)
                .default_value(DEFAULT_YEAR)
                .required(true)
                .help("Specify the year you wish to work with"),
        )
        .arg(
            Arg::with_name("time")
                .short("t")
                .long("time")
                .help("Generate benchmark time"),
        )
        .subcommand(subcommand(AoCDay::AOCD01))
        .subcommand(subcommand(AoCDay::AOCD02))
        .subcommand(subcommand(AoCDay::AOCD03))
        .subcommand(subcommand(AoCDay::AOCD04))
        .subcommand(subcommand(AoCDay::AOCD05))
        .subcommand(subcommand(AoCDay::AOCD06))
        .subcommand(subcommand(AoCDay::AOCD07))
        .subcommand(subcommand(AoCDay::AOCD08))
        .subcommand(subcommand(AoCDay::AOCD09))
        .subcommand(subcommand(AoCDay::AOCD10))
        .subcommand(subcommand(AoCDay::AOCD11))
        .subcommand(subcommand(AoCDay::AOCD12))
        .subcommand(subcommand(AoCDay::AOCD13))
        .subcommand(subcommand(AoCDay::AOCD14))
        .subcommand(subcommand(AoCDay::AOCD15))
        .subcommand(subcommand(AoCDay::AOCD16))
        .subcommand(subcommand(AoCDay::AOCD17))
        .subcommand(subcommand(AoCDay::AOCD18))
        .subcommand(subcommand(AoCDay::AOCD19))
        .subcommand(subcommand(AoCDay::AOCD20))
        .subcommand(subcommand(AoCDay::AOCD21))
        .subcommand(subcommand(AoCDay::AOCD22))
        .subcommand(subcommand(AoCDay::AOCD23))
        .subcommand(subcommand(AoCDay::AOCD24))
        .subcommand(subcommand(AoCDay::AOCD25))
}

/// Advent of Code `SubCommand`
fn subcommand<'a, 'b>(day: AoCDay) -> App<'a, 'b> {
    SubCommand::with_name(day.into())
        .about(get_day_about(day))
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .required(true)
                .default_value("data_file"),
        )
        .arg(
            Arg::with_name("second")
                .short("s")
                .long("second")
                .help("Run the alrgorithm to calculate the value for the 2nd star"),
        )
}
