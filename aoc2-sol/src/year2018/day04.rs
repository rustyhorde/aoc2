// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2018 ---**
//!
//! **--- Day 4: Repose Record ---**
//!
//! You've sneaked into another supply closet - this time, it's across from the prototype suit manufacturing lab. You need to sneak inside and fix the issues with the suit, but there's a guard stationed outside the lab, so this is as close as you can safely get.
//!
//! As you search the closet for anything that might help, you discover that you're not the first person to want to sneak in. Covering the walls, someone has spent an hour starting every midnight for the past few months secretly observing this guard post! They've been writing down the ID of the one guard on duty that night - the Elves seem to have decided that one guard was enough for the overnight shift - as well as when they fall asleep or wake up while at their post (your puzzle input).
//!
//! For example, consider the following records, which have already been organized into chronological order:
//!
//! ```text
//! [1518-11-01 00:00] Guard #10 begins shift
//! [1518-11-01 00:05] falls asleep
//! [1518-11-01 00:25] wakes up
//! [1518-11-01 00:30] falls asleep
//! [1518-11-01 00:55] wakes up
//! [1518-11-01 23:58] Guard #99 begins shift
//! [1518-11-02 00:40] falls asleep
//! [1518-11-02 00:50] wakes up
//! [1518-11-03 00:05] Guard #10 begins shift
//! [1518-11-03 00:24] falls asleep
//! [1518-11-03 00:29] wakes up
//! [1518-11-04 00:02] Guard #99 begins shift
//! [1518-11-04 00:36] falls asleep
//! [1518-11-04 00:46] wakes up
//! [1518-11-05 00:03] Guard #99 begins shift
//! [1518-11-05 00:45] falls asleep
//! [1518-11-05 00:55] wakes up
//! ```
//!
//! Timestamps are written using year-month-day hour:minute format. The guard falling asleep or waking up is always the one whose shift most recently started. Because all asleep/awake times are during the midnight hour (00:00 - 00:59), only the minute portion (00 - 59) is relevant for those events.
//!
//! Visually, these records show that the guards are asleep at these times:
//!
//! ```text
//! Date   ID   Minute
//!             000000000011111111112222222222333333333344444444445555555555
//!             012345678901234567890123456789012345678901234567890123456789
//! 11-01  #10  .....####################.....#########################.....
//! 11-02  #99  ........................................##########..........
//! 11-03  #10  ........................#####...............................
//! 11-04  #99  ....................................##########..............
//! 11-05  #99  .............................................##########.....
//! ```
//!
//! The columns are Date, which shows the month-day portion of the relevant day; ID, which shows the guard on duty that day; and Minute, which shows the minutes during which the guard was asleep within the midnight hour. (The Minute column's header shows the minute's ten's digit in the first row and the one's digit in the second row.) Awake is shown as ., and asleep is shown as #.
//!
//! Note that guards count as asleep on the minute they fall asleep, and they count as awake on the minute they wake up. For example, because Guard #10 wakes up at 00:25 on 1518-11-01, minute 25 is marked as awake.
//!
//! If you can figure out the guard most likely to be asleep at a specific time, you might be able to trick that guard into working tonight so you can have the best chance of sneaking in. You have two strategies for choosing the best guard/minute combination.
//!
//! Strategy 1: Find the guard that has the most minutes asleep. What minute does that guard spend asleep the most?
//!
//! In the example above, Guard #10 spent the most minutes asleep, a total of 50 minutes (20+25+5), while Guard #99 only slept for a total of 30 minutes (10+10+10). Guard #10 was asleep most during minute 24 (on two days, whereas any other minute the guard was asleep was only seen on one day).
//!
//! While this example listed the entries in chronological order, your entries are in the order you found them. You'll need to organize them before they can be analyzed.
//!
//! What is the ID of the guard you chose multiplied by the minute you chose? (In the above example, the answer would be 10 * 24 = 240.)
//!
//! **--- Part Two ---**
//!
//! Strategy 2: Of all guards, which guard is most frequently asleep on the same minute?
//!
//! In the example above, Guard #99 spent minute 45 asleep more than any other guard or minute - three times in total. (In all other cases, any guard spent any minute asleep at most twice.)
//!
//! What is the ID of the guard you chose multiplied by the minute you chose? (In the above example, the answer would be 99 * 45 = 4455.)

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use regex::Regex;
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
};
use time::{Month, OffsetDateTime};

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<BTreeMap<OffsetDateTime, String>, u32>(
        AoCYear::AOC2018,
        AoCDay::AOCD04,
        setup,
        find,
    )
    .map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<BTreeMap<OffsetDateTime, String>, u32>(
        bench,
        AoCYear::AOC2018,
        AoCDay::AOCD04,
        setup,
        find,
    )
    .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> BTreeMap<OffsetDateTime, String> {
    setup_br(reader).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T) -> Result<BTreeMap<OffsetDateTime, String>>
where
    T: BufRead,
{
    let line_re = Regex::new(r"\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] (.*)")?;
    let mut sorted_events = BTreeMap::new();

    for line in valid_lines(reader) {
        for cap in line_re.captures_iter(&line) {
            let y = (cap[1]).parse::<i32>()?;
            let mon = (cap[2]).parse::<u8>()?;
            let d = (cap[3]).parse::<u8>()?;
            let h = (cap[4]).parse::<u8>()?;
            let m = (cap[5]).parse::<u8>()?;
            let rest = &cap[6];
            let dt = OffsetDateTime::now_utc()
                .replace_day(d)?
                .replace_month(Month::try_from(mon)?)?
                .replace_year(y)?
                .replace_hour(h)?
                .replace_minute(m)?
                .replace_second(0)?;
            let _res = sorted_events.insert(dt, rest.to_string());
        }
    }
    Ok(sorted_events)
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: BTreeMap<OffsetDateTime, String>) -> u32 {
    find_res(&data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(sorted_events: &BTreeMap<OffsetDateTime, String>, second_star: bool) -> Result<u32> {
    let guard_re = Regex::new(r"Guard #(\d+) begins shift")?;

    let mut guards_napping: BTreeMap<u32, BTreeMap<u8, u32>> = BTreeMap::new();
    let mut current_guard = 0;
    let mut minute_asleep = 0;
    for (dt, evt) in sorted_events {
        if guard_re.is_match(evt) {
            for cap in guard_re.captures_iter(evt) {
                current_guard = (cap[1]).parse::<u32>()?;
                let _ = guards_napping.entry(current_guard).or_insert_with(|| {
                    let mut minute_map = BTreeMap::new();
                    for i in 0..60 {
                        let _ = minute_map.insert(i, 0);
                    }
                    minute_map
                });
            }
        } else if current_guard > 0 && dt.hour() == 0 {
            if evt.contains("falls asleep") {
                minute_asleep = dt.minute();
            } else if evt.contains("wakes up") {
                let minutes_map = guards_napping
                    .get_mut(&current_guard)
                    .ok_or(anyhow!("Invalid Index"))?;
                for i in minute_asleep..dt.minute() {
                    *minutes_map.entry(i).or_insert(0) += 1;
                }
            }
        }
    }

    let mut max_id = 0;
    let mut max_minute_asleep = 0;
    if second_star {
        let mut max_times_asleep = 0;
        for (id, time_napping) in guards_napping {
            let (guard_max_minutes_asleep, guard_max_times_asleep): (u8, u32) = time_napping
                .iter()
                .max_by_key(|(_, v)| *v)
                .map_or((0, 0), |(x, y)| (*x, *y));

            if guard_max_times_asleep > max_times_asleep {
                max_times_asleep = guard_max_times_asleep;
                max_minute_asleep = guard_max_minutes_asleep;
                max_id = id;
            }
        }
    } else {
        let mut max_time_asleep = 0;
        for (id, time_napping) in guards_napping {
            let total_time_asleep: u32 = time_napping.values().sum();
            let (mma, _): (u8, u32) = time_napping
                .iter()
                .max_by_key(|(_, v)| *v)
                .map_or((0, 0), |(x, y)| (*x, *y));
            if total_time_asleep > max_time_asleep {
                max_id = id;
                max_time_asleep = total_time_asleep;
                max_minute_asleep = mma;
            }
        }
    }

    Ok(max_id * u32::from(max_minute_asleep))
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<BTreeMap<OffsetDateTime, String>, u32>(
        AoCYear::AOC2018,
        AoCDay::AOCD04,
        setup,
        find2,
    )
    .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<BTreeMap<OffsetDateTime, String>, u32>(
        bench,
        AoCYear::AOC2018,
        AoCDay::AOCD04,
        setup,
        find2,
    )
    .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: BTreeMap<OffsetDateTime, String>) -> u32 {
    find_res(&data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find(data), 240);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find2(data), 4455);
        Ok(())
    }
}
