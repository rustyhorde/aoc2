// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 14 "Reindeer Olympics"
//!
//! **--- Day 14: Reindeer Olympics ---**
//!
//! **--- Part 1 ---**
//!
//! This year is the Reindeer Olympics! Reindeer can fly at high speeds, but
//! must rest occasionally to recover their energy. Santa would like to know which
//! of his reindeer is fastest, and so he has them race.
//!
//! Reindeer can only either be flying (always at their top speed) or resting
//! (not moving at all), and always spend whole seconds in either state.
//!
//! For example, suppose you have the following Reindeer:
//!
//! ```text
//! Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
//! Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
//! ```
//!
//! After one second, Comet has gone 14 km, while Dancer has gone 16 km. After ten seconds,
//! Comet has gone 140 km, while Dancer has gone 160 km. On the eleventh second, Comet begins
//! resting (staying at 140 km), and Dancer continues on for a total distance of 176 km.
//! On the 12th second, both reindeer are resting. They continue to rest until the 138th
//! second, when Comet flies for another ten seconds. On the 174th second,
//! Dancer flies for another 11 seconds.
//!
//! In this example, after the 1000th second, both reindeer are resting, and Comet
//! is in the lead at 1120 km (poor Dancer has only gotten 1056 km by that point). So,
//! in this situation, Comet would win (if the race ended at 1000 seconds).
//!
//! Given the descriptions of each reindeer (in your puzzle input), after exactly 2503 seconds,
//! what distance has the winning reindeer traveled?
//!
//! **--- Part Two ---**
//!
//! Seeing how reindeer move in bursts, Santa decides he's not pleased with the old scoring system.
//!
//! Instead, at the end of each second, he awards one point to the reindeer currently
//! in the lead. (If there are multiple reindeer tied for the lead, they each get one point.)
//! He keeps the traditional 2503 second time limit, of course, as doing otherwise would be entirely ridiculous.
//!
//! Given the example reindeer from above, after the first second, Dancer is in the lead and
//! gets one point. He stays in the lead until several seconds into Comet's second burst:
//! after the 140th second, Comet pulls into the lead and gets his first point. Of course,
//! since Dancer had been in the lead for the 139 seconds before that, he has accumulated 139 points
//! by the 140th second.
//!
//! After the 1000th second, Dancer has accumulated 689 points, while poor Comet, our old champion, only has 312.
//! So, with the new scoring system, Dancer would win (if the race ended at 1000 seconds).
//!
//! Again given the descriptions of each reindeer (in your puzzle input), after exactly 2503 seconds,
//! how many points does the winning reindeer have?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{get_cap, get_cap_x, run_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use regex::Regex;
use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum ReindeerState {
    Flying,
    Resting,
}

impl Default for ReindeerState {
    fn default() -> Self {
        Self::Flying
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Reindeer {
    state: ReindeerState,
    speed: usize,
    f_dur: usize,
    r_dur: usize,
    distance: usize,
    seconds_flying: usize,
    seconds_resting: usize,
    points: usize,
}

impl Reindeer {
    fn fly(&mut self) -> &mut Self {
        self.distance += self.speed;
        self.seconds_flying += 1;

        if self.seconds_flying == self.f_dur {
            self.state = ReindeerState::Resting;
            self.seconds_flying = 0;
        }

        self
    }

    fn rest(&mut self) -> &mut Self {
        self.seconds_resting += 1;

        if self.seconds_resting == self.r_dur {
            self.state = ReindeerState::Flying;
            self.seconds_resting = 0;
        }

        self
    }
}

impl Ord for Reindeer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl PartialOrd for Reindeer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD14, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader, 2503).unwrap_or_default()
}

fn find_br<T>(reader: T, dur: usize) -> Result<usize>
where
    T: BufRead,
{
    let mut reindeers = setup(reader)?;

    race(&mut reindeers, dur, false)?;

    let winner = reindeers
        .values()
        .max()
        .ok_or_else(|| anyhow!("no reindeer won!"))?;
    Ok(winner.distance)
}

fn setup<T>(reader: T) -> Result<HashMap<String, Reindeer>>
where
    T: BufRead,
{
    let re = Regex::new(
        r"^(.*) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds\.$",
    )?;

    let mut reindeers = HashMap::new();

    for line in valid_lines(reader) {
        for caps in re.captures_iter(&line) {
            let name = get_cap(1, &caps)?;
            let speed = get_cap_x::<usize>(2, &caps)?;
            let f_dur = get_cap_x::<usize>(3, &caps)?;
            let r_dur = get_cap_x::<usize>(4, &caps)?;
            let reindeer = Reindeer {
                state: ReindeerState::Flying,
                speed,
                f_dur,
                r_dur,
                distance: 0,
                seconds_flying: 0,
                seconds_resting: 0,
                points: 0,
            };

            _ = reindeers.insert(name, reindeer);
        }
    }

    Ok(reindeers)
}

fn race(reindeers: &mut HashMap<String, Reindeer>, dur: usize, award: bool) -> Result<()> {
    for _i in 0..dur {
        for reindeer in reindeers.values_mut() {
            if reindeer.state == ReindeerState::Flying {
                _ = reindeer.fly();
            } else {
                _ = reindeer.rest();
            }
        }

        if award {
            award_leaders(reindeers)?;
        }
    }
    Ok(())
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD14, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader, 2503).unwrap_or_default()
}

fn find2_br<T>(reader: T, dur: usize) -> Result<usize>
where
    T: BufRead,
{
    let mut reindeers = setup(reader)?;

    race(&mut reindeers, dur, true)?;

    let winner = reindeers
        .values()
        .reduce(|r1, r2| if r1.points > r2.points { r1 } else { r2 })
        .ok_or_else(|| anyhow!("no reindeer won!"))?;
    Ok(winner.points)
}

fn award_leaders(reindeers: &mut HashMap<String, Reindeer>) -> Result<()> {
    let max_distance = reindeers
        .values()
        .max()
        .ok_or_else(|| anyhow!("bad"))?
        .distance;

    for reindeer in reindeers.values_mut() {
        if reindeer.distance == max_distance {
            reindeer.points += 1;
        }
    }
    Ok(())
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1), 1_000)?, 1120);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find2_br(Cursor::new(TEST_1), 1_000)?, 689);
        Ok(())
    }
}
