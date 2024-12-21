// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::{
    ffi::OsString,
    fmt::Display,
    fs::{OpenOptions, create_dir_all},
    io::Write,
    path::Path,
    sync::Arc,
    time::{Duration, Instant},
};

use self::{cli::Args, cookie::cookie_jar};
use anyhow::{Context, Result, anyhow};
use chrono::{DateTime, Datelike, MappedLocalTime, TimeZone, Utc};
use chrono_tz::{Tz, US::Eastern};
use clap::Parser;
use reqwest::{Client, ClientBuilder};
use tokio::time::interval;
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

    let client = setup_client(&config)?;
    trace!("reqwest client created");

    if config.wait() {
        info!("Waiting for the next day to start a download");
        let now = Utc::now();
        let now_eastern = Eastern.from_utc_datetime(&now.naive_utc());
        if let MappedLocalTime::Single(tomorrow_midnight_eastern) = now_eastern
            .date_naive()
            .succ_opt()
            .and_then(|d| d.and_hms_opt(0, 0, 5))
            .ok_or_else(|| anyhow!("Cannot determine tomorrow's date"))?
            .and_local_timezone(Eastern)
        {
            let mut message_now = Instant::now();
            let (_, curr_ye) = tomorrow_midnight_eastern.year_ce();
            let curr_de = tomorrow_midnight_eastern.day();

            let mut interval = interval(Duration::from_secs(1));

            loop {
                if check_time_to_download(
                    &mut message_now,
                    &config,
                    &client,
                    &curr_ye.to_string(),
                    &curr_de.to_string(),
                    &tomorrow_midnight_eastern,
                )
                .await?
                {
                    break;
                }
                let _ = interval.tick().await;
            }
        }
    } else {
        let client = setup_client(&config)?;
        trace!("reqwest client created");

        for day in config.day_range()? {
            download(&config, &client, config.year(), &day.to_string()).await?;
        }
    }
    Ok(())
}

async fn check_time_to_download(
    message_now: &mut Instant,
    config: &ConfigAoc2Di<'_>,
    client: &Client,
    curr_ye: &str,
    curr_de: &str,
    midnight: &DateTime<Tz>,
) -> Result<bool> {
    let mut done = false;
    let now = Utc::now();
    let now_eastern = Eastern.from_utc_datetime(&now.naive_utc());
    if &now_eastern > midnight {
        download(config, client, curr_ye, curr_de).await?;
        done = true
    }

    let elapsed = message_now.elapsed();
    if elapsed > Duration::from_secs(20) {
        info!("Waiting for the next day to start a download");
        *message_now = Instant::now();
    }
    Ok(done)
}

fn setup_client(config: &ConfigAoc2Di<'_>) -> Result<Client> {
    let jar = cookie_jar(config)?;
    let client = ClientBuilder::new()
        .cookie_provider(Arc::new(jar))
        .build()?;
    Ok(client)
}

async fn download<S>(config: &ConfigAoc2Di<'_>, client: &Client, year: S, day: S) -> Result<()>
where
    S: AsRef<str> + Display + AsRef<Path>,
{
    let aoc2_path = Path::new(config.aoc2_path())
        .join("data")
        .join(&year)
        .join(format!("day{:0>2}", day));
    create_dir_all(&aoc2_path)?;
    let data_file_path = aoc2_path.join("data_file");
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
    Ok(())
}
