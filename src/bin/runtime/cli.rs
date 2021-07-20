// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `aoc` Command Line Interface

use clap::{App, Arg, crate_authors, crate_name, crate_version};

pub(crate) const DEFAULT_YEAR: &str = "2021";

pub(crate) fn app<'a, 'b>() -> App<'a, 'b> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about("Run Advent of Code daily problems")
        .usage("\u{1f31f}   solution: aoc <day>\n    \u{1f31f}\u{1f31f} solution: aoc <day> -s")
        .arg(
            Arg::with_name("year")
                .short("y")
                .long("year")
                .default_value(DEFAULT_YEAR)
                .required(true)
                .help("Specify the year you wish to work with"),
        )
        .arg(
            Arg::with_name("time")
                .short("t")
                .long("time")
                .help("Generate benchmark time")
        )
}
