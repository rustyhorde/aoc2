// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

mod cli;
mod config;
mod header;

use anyhow::Result;

pub(crate) fn run() -> Result<()> {
    // Parse the command line
    let _matches = cli::app().get_matches_safe()?;

    // Load the configuration
    // let config = load(&matches)?;

    // Output the pretty header
    // header::header(&matches, &mut io::stdout())?;

    Ok(())
}
