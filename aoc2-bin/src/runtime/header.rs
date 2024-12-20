// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `aoc2` header
use anyhow::Result;
use console::Style;
use rand::Rng;
use vergen_pretty::{vergen_pretty_env, PrefixBuilder, PrettyBuilder};

fn from_u8(val: u8) -> Style {
    let style = Style::new();
    match val {
        0 => style.green(),
        1 => style.yellow(),
        2 => style.blue(),
        3 => style.magenta(),
        4 => style.cyan(),
        5 => style.white(),
        _ => style.red(),
    }
}

/// Generate a pretty header
///
/// # Errors
///
pub(crate) fn header(prefix: &'static str) -> Result<()> {
    let mut rng = rand::thread_rng();
    let app_style = from_u8(rng.gen_range(0..7));
    trace(app_style, prefix)?;
    Ok(())
}

fn trace(app_style: Style, prefix: &'static str) -> Result<()> {
    let prefix = PrefixBuilder::default()
        .lines(prefix.lines().map(str::to_string).collect())
        .style(app_style)
        .build()?;
    PrettyBuilder::default()
        .env(vergen_pretty_env!())
        .prefix(prefix)
        .build()?
        .trace();
    Ok(())
}
