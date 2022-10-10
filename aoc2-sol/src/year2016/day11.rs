// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 11 "Radioisotope Thermoelectric Generators"
//!
//! **--- Day 11: Radioisotope Thermoelectric Generators ---**
//!
//! **--- Part 1 ---**
//!
//! You come upon a column of four floors that have been entirely sealed off
//! from the rest of the building except for a small dedicated lobby. There
//! are some radiation warnings and a big sign which reads "Radioisotope
//! Testing Facility".
//!
//! According to the project status board, this facility is currently being
//! used to experiment with Radioisotope Thermoelectric Generators (RTGs, or
//! simply "generators") that are designed to be paired with specially-constructed
//! microchips. Basically, an RTG is a highly radioactive rock that generates
//! electricity through heat.
//!
//! The experimental RTGs have poor radiation containment, so they're dangerously
//! radioactive. The chips are prototypes and don't have normal radiation shielding,
//! but they do have the ability to generate an electromagnetic radiation shield
//! when powered. Unfortunately, they can only be powered by their corresponding
//! RTG. An RTG powering a microchip is still dangerous to other microchips.
//!
//! In other words, if a chip is ever left in the same area as another RTG, and
//! it's not connected to its own RTG, the chip will be fried. Therefore, it is
//! assumed that you will follow procedure and keep chips connected to their
//! corresponding RTG when they're in the same room, and away from other RTGs otherwise.
//!
//! These microchips sound very interesting and useful to your current activities,
//! and you'd like to try to retrieve them. The fourth floor of the facility has
//! an assembling machine which can make a self-contained, shielded computer for you
//! to take with you - that is, if you can bring it all of the RTGs and microchips.
//!
//! Within the radiation-shielded part of the facility (in which it's safe to have
//! these pre-assembly RTGs), there is an elevator that can move between the four
//! floors. Its capacity rating means it can carry at most yourself and two RTGs or
//! microchips in any combination. (They're rigged to some heavy diagnostic equipment -
//! the assembling machine will detach it for you.) As a security measure, the
//! elevator will only function if it contains at least one RTG or microchip. The
//! elevator always stops on each floor to recharge, and this takes long enough that
//! the items within it and the items on that floor can irradiate each other.
//! (You can prevent this if a Microchip and its Generator end up on the same floor
//! in this way, as they can be connected while the elevator is recharging.)
//!
//! You make some notes of the locations of each component of interest (your puzzle
//! input). Before you don a hazmat suit and start moving things around, you'd like
//! to have an idea of what you need to do.
//!
//! When you enter the containment area, you and the elevator will start on the first floor.
//!
//! For example, suppose the isolated area has the following arrangement:
//!
//! ```text
//! The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
//! The second floor contains a hydrogen generator.
//! The third floor contains a lithium generator.
//! The fourth floor contains nothing relevant.
//! ```
//!
//! As a diagram (F# for a Floor number, E for Elevator, H for Hydrogen, L for Lithium,
//! M for Microchip, and G for Generator), the initial state looks like this:
//!
//! ```text
//! F4 .  .  .  .  .
//! F3 .  .  .  LG .
//! F2 .  HG .  .  .
//! F1 E  .  HM .  LM
//! ```
//!
//! Then, to get everything up to the assembling machine on the fourth floor, the
//! following steps could be taken:
//!
//! Bring the Hydrogen-compatible Microchip to the second floor, which is safe
//! because it can get power from the Hydrogen Generator:
//!
//! ```text
//! F4 .  .  .  .  .
//! F3 .  .  .  LG .
//! F2 E  HG HM .  .
//! F1 .  .  .  .  LM
//! ```
//!
//! Bring both Hydrogen-related items to the third floor, which is safe because the
//! Hydrogen-compatible microchip is getting power from its generator:
//!
//! ```text
//! F4 .  .  .  .  .
//! F3 E  HG HM LG .
//! F2 .  .  .  .  .
//! F1 .  .  .  .  LM
//! ```
//!
//! Leave the Hydrogen Generator on floor three, but bring the Hydrogen-compatible
//! Microchip back down with you so you can still use the elevator:
//!
//! ```text
//! F4 .  .  .  .  .
//! F3 .  HG .  LG .
//! F2 E  .  HM .  .
//! F1 .  .  .  .  LM
//! ```
//!
//! At the first floor, grab the Lithium-compatible Microchip, which is safe because
//! Microchips don't affect each other:
//!
//! ```text
//! F4 .  .  .  .  .
//! F3 .  HG .  LG .
//! F2 .  .  .  .  .
//! F1 E  .  HM .  LM
//! ```
//!
//! Bring both Microchips up one floor, where there is nothing to fry them:
//!
//! ```text
//! F4 .  .  .  .  .
//! F3 .  HG .  LG .
//! F2 E  .  HM .  LM
//! F1 .  .  .  .  .
//! ```
//!
//! Bring both Microchips up again to floor three, where they can be temporarily
//! connected to their corresponding generators while the elevator recharges,
//! preventing either of them from being fried:
//!
//! ```text
//! F4 .  .  .  .  .
//! F3 E  HG HM LG LM
//! F2 .  .  .  .  .
//! F1 .  .  .  .  .
//! ```
//!
//! Bring both Microchips to the fourth floor:
//!
//! ```text
//! F4 E  .  HM .  LM
//! F3 .  HG .  LG .
//! F2 .  .  .  .  .
//! F1 .  .  .  .  .
//! ```
//!
//! Leave the Lithium-compatible microchip on the fourth floor, but bring the
//! Hydrogen-compatible one so you can still use the elevator; this is safe because
//! although the Lithium Generator is on the destination floor, you can connect
//! Hydrogen-compatible microchip to the Hydrogen Generator there:
//!
//! ```text
//! F4 .  .  .  .  LM
//! F3 E  HG HM LG .
//! F2 .  .  .  .  .
//! F1 .  .  .  .  .
//! ```
//!
//! Bring both Generators up to the fourth floor, which is safe because you can
//! connect the Lithium-compatible Microchip to the Lithium Generator upon arrival:
//!
//! ```text
//! F4 E  HG .  LG LM
//! F3 .  .  HM .  .
//! F2 .  .  .  .  .
//! F1 .  .  .  .  .
//! ```
//!
//! Bring the Lithium Microchip with you to the third floor so you can use the elevator:
//!
//! ```text
//! F4 .  HG .  LG .
//! F3 E  .  HM .  LM
//! F2 .  .  .  .  .
//! F1 .  .  .  .  .
//! ```
//!
//! Bring both Microchips to the fourth floor:
//!
//! ```text
//! F4 E  HG HM LG LM
//! F3 .  .  .  .  .
//! F2 .  .  .  .  .
//! F1 .  .  .  .  .
//! ```
//!
//! In this arrangement, it takes 11 steps to collect all of the objects at the fourth
//! floor for assembly. (Each elevator stop counts as one step, even if nothing is added
//! to or removed from it.)
//!
//! In your situation, what is the minimum number of steps required to bring all of
//! the objects to the fourth floor?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{get_cap, run_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use indexmap::IndexSet;
use regex::Regex;
use std::{
    collections::{hash_map::DefaultHasher, BTreeMap},
    fs::File,
    hash::{Hash, Hasher},
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum PosKind {
    Generator(u64),
    Microchip(u64),
    Space,
    Elevator,
}

#[allow(dead_code)]
struct Pos {
    row: usize,
    col: usize,
    kind: PosKind,
}

#[allow(dead_code)]
struct State {
    visited: Vec<Vec<Vec<Pos>>>,
    data: Vec<Vec<Pos>>,
}

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2016, AoCDay::AOCD11, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader).unwrap_or_default()
}

fn find_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let floor_re = Regex::new(r"^The (.*) floor")?;
    let generator_re = Regex::new(r"a [a-z]+ generator[ ,\.]")?;
    let microchip_re = Regex::new(r"a [a-z]+-compatible[ ,\.]")?;
    let mut floors = BTreeMap::new();
    let mut items = IndexSet::new();

    for line in valid_lines(reader) {
        let mut floor = 0;
        for caps in floor_re.captures_iter(&line) {
            let floor_str = get_cap(1, &caps)?;
            floor = match &floor_str[..] {
                "first" => 0,
                "second" => 1,
                "third" => 2,
                "fourth" => 3,
                _ => return Err(anyhow!(format!("invalid floor: {floor_str}"))),
            };
            let _ = floors.entry(floor).or_insert_with(Vec::new);
        }
        for m in generator_re.find_iter(&line) {
            let generator_match = &line[m.start()..m.end()];
            let split = generator_match.split(' ').collect::<Vec<&str>>();
            let mut generator = split[1].to_string();
            generator.push('G');
            let floor_vec = floors.entry(floor).or_default();
            floor_vec.push(generator.clone());
            let _ = items.insert(generator);
        }
        for m in microchip_re.find_iter(&line) {
            let microchip_match = &line[m.start()..m.end()];
            let split = microchip_match.split(' ').collect::<Vec<&str>>();
            let mut microchip =
                split[1]
                    .chars()
                    .take_while(|c| *c != '-')
                    .fold(String::new(), |acc, ch| {
                        let mut val = acc;
                        val.push(ch);
                        val
                    });
            microchip.push('M');
            let floor_vec = floors.entry(floor).or_default();
            floor_vec.push(microchip.clone());
            let _ = items.insert(microchip);
        }
    }

    let items_count = items.len() + 1;

    let mut data = vec![];

    for (floor, items_on_floor) in floors {
        let mut floor_vec = vec![PosKind::Space; items_count];

        if floor == 0 {
            floor_vec[0] = PosKind::Elevator;
        }
        for item in &items_on_floor {
            let idx = items
                .get_index_of(item)
                .ok_or_else(|| anyhow!("bad item"))?;
            if item.ends_with('M') {
                let blah = item.trim_end_matches('M');
                let mut hasher = DefaultHasher::new();
                blah.hash(&mut hasher);
                floor_vec[idx + 1] = PosKind::Microchip(hasher.finish());
            } else {
                let blah = item.trim_end_matches('G');
                let mut hasher = DefaultHasher::new();
                blah.hash(&mut hasher);
                floor_vec[idx + 1] = PosKind::Generator(hasher.finish());
            }
        }
        data.push(floor_vec);
    }

    let mut goal = vec![vec![PosKind::Space; items_count]; 4];
    goal[3][0] = PosKind::Elevator;

    for row in &data {
        for (idx, pos) in row.iter().enumerate() {
            if *pos != PosKind::Space {
                goal[3][idx] = *pos;
            }
        }
    }
    println!("{data:?}");
    println!("{goal:?}");
    Ok(0)
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2016, AoCDay::AOCD11, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader)
}

fn find2_br<T>(reader: T) -> usize
where
    T: BufRead,
{
    for _line in valid_lines(reader) {}
    0
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant.";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1))?, 0);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    // use super::find2_br;
    use anyhow::Result;
    // use std::io::Cursor;

    // const TEST_1: &str = r"^v";
    // const TEST_2: &str = r"^>v<";
    // const TEST_3: &str = r"^v^v^v^v^v";

    #[test]
    fn solution() -> Result<()> {
        // assert_eq!(find2_br(Cursor::new(TEST_1))?, 3);
        Ok(())
    }
}
