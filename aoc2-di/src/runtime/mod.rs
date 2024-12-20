// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::{
    ffi::OsString,
    fs::{OpenOptions, create_dir_all},
    io::Write,
    path::Path,
    sync::Arc,
};

use self::{cli::Args, cookie::cookie_jar};
use anyhow::{Context, Result};
use clap::Parser;
use reqwest::ClientBuilder;
use tracing::{info, trace};

use crate::{
    config::{ConfigAoc2Di, load},
    constants::{CONFIG_LOAD_ERROR, DOMAIN, HEADER_PREFIX, SESSION, TRACING_INIT_ERROR},
    header::header,
    tracing::initialize,
};

mod cli;
mod cookie;

pub(crate) async fn run<I, T>(args: Option<I>) -> Result<()>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    // Parse the command line
    let cli = if let Some(args) = args {
        Args::try_parse_from(args)?
    } else {
        Args::try_parse()?
    };
    let config =
        load::<Args, ConfigAoc2Di<'_>, Args>(&cli, &cli).with_context(|| CONFIG_LOAD_ERROR)?;
    let _guard = initialize(&cli, &config, None).with_context(|| TRACING_INIT_ERROR)?;
    header(HEADER_PREFIX)?;
    trace!("configuration loaded - {}", config.env());
    trace!("tracing initialized");

    let year = config.year();
    let day_range = config.day_range()?;
    let jar = cookie_jar(&config)?;
    let client = ClientBuilder::new()
        .cookie_provider(Arc::new(jar))
        .build()?;
    trace!("reqwest client created");

    for day in day_range {
        let base_path = Path::new("data").join(year).join(format!("day{:0>2}", day));
        create_dir_all(&base_path)?;
        let data_file_path = base_path.join("data_file");
        info!(
            "Downloading input data for year {year} day {day} at path {}",
            data_file_path.display()
        );
        let mut data_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(data_file_path)?;

        trace!("Beginning download...");
        let url = format!("https://adventofcode.com/{year}/day/{day}/input");
        let data = client.get(&url).send().await?.bytes().await?;
        trace!("Download complete");

        data_file.write_all(&data)?;
    }
    Ok(())
}
