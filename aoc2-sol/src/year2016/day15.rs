// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 15 "Timing is Everything"
//!
//! **--- Day 15: Timing is Everything ---**
//!
//! **--- Part 1 ---**
//!
//! The halls open into an interior plaza containing a large kinetic sculpture.
//! The sculpture is in a sealed enclosure and seems to involve a set of identical
//! spherical capsules that are carried to the top and allowed to bounce through
//! the maze of spinning pieces.
//!
//! Part of the sculpture is even interactive! When a button is pressed, a capsule
//! is dropped and tries to fall through slots in a set of rotating discs to finally
//! go through a little hole at the bottom and come out of the sculpture. If any
//! of the slots aren't aligned with the capsule as it passes, the capsule bounces
//! off the disc and soars away. You feel compelled to get one of those capsules.
//!
//! The discs pause their motion each second and come in different sizes; they
//! seem to each have a fixed number of positions at which they stop. You decide
//! to call the position with the slot `0`, and count up for each position it reaches
//! next.
//!
//! Furthermore, the discs are spaced out so that after you push the button, one
//! second elapses before the first disc is reached, and one second elapses as the
//! capsule passes from one disc to the one below it. So, if you push the button at
//! `time=100`, then the capsule reaches the top disc at `time=101`, the second disc
//! at `time=102`, the third disc at `time=103`, and so on.
//!
//! The button will only drop a capsule at an integer time - no fractional
//! seconds allowed.
//!
//! For example, at `time=0`, suppose you see the following arrangement:
//!
//! ```text
//! Disc #1 has 5 positions; at time=0, it is at position 4.
//! Disc #2 has 2 positions; at time=0, it is at position 1.
//! ```
//!
//! If you press the button exactly at `time=0`, the capsule would start to fall; it
//! would reach the first disc at `time=1`. Since the first disc was at position `4`
//! at `time=0`, by `time=1` it has ticked one position forward. As a five-position disc,
//! the next position is `0`, and the capsule falls through the slot.
//!
//! Then, at `time=2`, the capsule reaches the second disc. The second disc has ticked
//! forward two positions at this point: it started at position `1`, then continued to
//! position `0`, and finally ended up at position `1` again. Because there's only a slot
//! at position `0`, the capsule bounces away.
//!
//! If, however, you wait until `time=5` to push the button, then when the capsule reaches
//! each disc, the first disc will have ticked forward `5+1 = 6` times (to position 0), and
//! the second disc will have ticked forward `5+2 = 7` times (also to position 0). In this
//! case, the capsule would fall through the discs and come out of the machine.
//!
//! However, your situation has more than two discs; you've noted their positions in
//! your puzzle input. What is the first time you can press the button to get a capsule?
//!
//! **--- Part Two ---**
//!
//! After getting the first capsule (it contained a star! what great fortune!), the machine
//! detects your success and begins to rearrange itself.
//!
//! When it's done, the discs are back in their original configuration as if it were `time=0`
//! again, but a new disc with 11 positions and starting at position 0 has appeared exactly
//! one second below the previously-bottom disc.
//!
//! With this new disc, and counting again starting from `time=0` with the configuration in
//! your puzzle input, what is the first time you can press the button to get another capsule?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{get_cap_x, print_err, run_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use regex::Regex;
use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::{BufRead, BufReader},
};

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2016, AoCDay::AOCD15, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader, 1_000_000)
        .map_err(print_err)
        .unwrap_or_default()
}

fn find_br<T>(reader: T, map_size: usize) -> Result<usize>
where
    T: BufRead,
{
    let (num_discs, disc_map) = setup(reader, map_size, false)?;
    drop_the_balls(&disc_map, num_discs)
}

fn drop_the_balls(
    disc_map: &HashMap<usize, BTreeMap<usize, (usize, usize)>>,
    num_discs: usize,
) -> Result<usize> {
    let mut successful_start = 0;
    'outer: for start in 0.. {
        for (idx, map_idx) in ((start + 1)..=(start + num_discs)).enumerate() {
            let map = disc_map
                .get(&map_idx)
                .ok_or_else(|| anyhow!("not enough spins"))?;
            let (position, _) = map.get(&(idx + 1)).ok_or_else(|| anyhow!("invbjkfdaj"))?;
            if *position != 0 {
                continue 'outer;
            }
        }
        successful_start = start;
        break;
    }
    Ok(successful_start)
}

type DiscSpinMap = HashMap<usize, BTreeMap<usize, (usize, usize)>>;

fn setup<T>(reader: T, map_size: usize, part2: bool) -> Result<(usize, DiscSpinMap)>
where
    T: BufRead,
{
    let disc_re =
        Regex::new(r"^Disc #(\d+) has (\d+) positions; at time=0, it is at position (\d+)\.$")?;
    let mut discs = BTreeMap::new();
    let mut last_disc_number = 0;
    for line in valid_lines(reader) {
        for caps in disc_re.captures_iter(&line) {
            let disc_number = get_cap_x::<usize>(1, &caps)?;
            let total_positions = get_cap_x::<usize>(2, &caps)?;
            let initial_position = get_cap_x::<usize>(3, &caps)?;
            last_disc_number = disc_number;
            *discs.entry(disc_number).or_default() = (initial_position, total_positions);
        }
    }
    if part2 {
        *discs.entry(last_disc_number + 1).or_default() = (0, 11);
    }

    // Pre-compile the disc states at a given time
    let num_discs = discs.len();
    let mut disc_map = HashMap::new();
    let mut previous = disc_map.entry(0).or_insert(discs);

    for i in 1..map_size {
        let mut discs_state = previous.clone();
        rotate_discs(&mut discs_state);
        previous = disc_map.entry(i).or_insert(discs_state);
    }
    Ok((num_discs, disc_map))
}

fn rotate_discs(discs: &mut BTreeMap<usize, (usize, usize)>) {
    for (position, total_positions) in (*discs).values_mut() {
        *position = (*position + 1) % *total_positions;
    }
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2016, AoCDay::AOCD15, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader, 3_500_000)
        .map_err(print_err)
        .unwrap_or_default()
}

fn find2_br<T>(reader: T, map_size: usize) -> Result<usize>
where
    T: BufRead,
{
    let (num_discs, disc_map) = setup(reader, map_size, true)?;
    drop_the_balls(&disc_map, num_discs)
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"Disc #1 has 5 positions; at time=0, it is at position 4.
Disc #2 has 2 positions; at time=0, it is at position 1.";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1), 50)?, 5);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    // use super::find2_br;
    // use std::io::Cursor;

    // const TEST_1: &str = r"^v";
    // const TEST_2: &str = r"^>v<";
    // const TEST_3: &str = r"^v^v^v^v^v";

    #[test]
    fn solution() {
        // assert_eq!(find2_br(Cursor::new(TEST_1))?, 3);
    }
}
