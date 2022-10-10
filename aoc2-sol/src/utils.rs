// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `aoc` utilities

use crate::constants::{AoCDay, AoCYear};
use anyhow::{anyhow, Context, Error, Result};
use regex::Captures;
use std::{
    convert::TryFrom,
    fmt,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    str::FromStr,
    time::{Duration, Instant},
};

#[derive(Clone, Copy, Debug)]
pub(crate) enum TimeUnits {
    Microseconds,
    Milliseconds,
    Nanoseconds,
    Seconds,
}

impl From<TimeUnits> for &'static str {
    fn from(unit: TimeUnits) -> &'static str {
        match unit {
            TimeUnits::Microseconds => "\u{b5}s",
            TimeUnits::Milliseconds => "ms",
            TimeUnits::Nanoseconds => "ns",
            TimeUnits::Seconds => "s",
        }
    }
}

impl fmt::Display for TimeUnits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&'static str>::from(*self))
    }
}

pub(crate) fn elapsed_parts(elapsed: Duration) -> Result<(usize, usize, TimeUnits)> {
    if elapsed.as_nanos() < 1000 && elapsed.as_nanos() > 0 {
        let nanos = usize::try_from(elapsed.as_nanos())?;
        Ok((nanos, 0, TimeUnits::Nanoseconds))
    } else if elapsed.as_micros() < 1000 && elapsed.as_micros() > 0 {
        let micros = usize::try_from(elapsed.as_micros())?;
        let nanos: usize = (elapsed.subsec_nanos() as usize) - (micros * 1000);
        Ok((micros, nanos, TimeUnits::Microseconds))
    } else if elapsed.as_millis() < 1000 && elapsed.as_millis() > 0 {
        let millis = usize::try_from(elapsed.as_millis())?;
        let micros: usize = (elapsed.subsec_micros() as usize) - (millis * 1000);
        Ok((millis, micros, TimeUnits::Milliseconds))
    } else if let Ok(seconds) = usize::try_from(elapsed.as_secs()) {
        Ok((
            seconds,
            elapsed.subsec_millis() as usize,
            TimeUnits::Seconds,
        ))
    } else {
        Err(anyhow!("Invalid duration: {:?}", elapsed))
    }
}

fn load_data(year: AoCYear, day: AoCDay) -> Result<BufReader<File>> {
    let year_str: &str = year.into();
    let day_str: &str = day.into();
    let mut filepath = PathBuf::from("data");
    filepath.push(year_str);
    filepath.push(day_str);
    filepath.push("data_file");

    Ok(BufReader::new(
        File::open(filepath).context("unable to open data_file")?,
    ))
}

pub(crate) fn run_solution<T>(year: AoCYear, day: AoCDay, f: fn(BufReader<File>) -> T) -> Result<T>
where
    T: fmt::Display,
{
    let data = load_data(year, day)?;
    let now = Instant::now();
    let res = f(data);
    let (whole, frac, units) = elapsed_parts(now.elapsed())?;
    println!("Answer:  {res}");
    println!("Elapsed: {whole}.{frac}{units}");
    Ok(res)
}

#[inline]
pub(crate) fn valid_lines<T>(reader: T) -> impl Iterator<Item = String>
where
    T: BufRead,
{
    reader.lines().filter_map(std::result::Result::ok)
}

pub(crate) fn get_cap(idx: usize, caps: &Captures<'_>) -> Result<String> {
    Ok(caps
        .get(idx)
        .ok_or_else(|| anyhow!("invalid cap"))?
        .as_str()
        .to_owned())
}

pub(crate) fn get_cap_u16(idx: usize, caps: &Captures<'_>) -> Result<u16> {
    get_cap_x(idx, caps)
}

pub(crate) fn get_cap_x<T>(idx: usize, caps: &Captures<'_>) -> Result<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    Ok(caps
        .get(idx)
        .ok_or_else(|| anyhow!("invalid cap"))?
        .as_str()
        .parse::<T>()?)
}

pub(crate) fn print_err(e: Error) -> Error {
    eprintln!("{e}");
    e
}
