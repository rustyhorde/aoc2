// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `aoc` utilities

use anyhow::{anyhow, Result};
use std::{convert::TryFrom, fmt, time::Duration};

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
