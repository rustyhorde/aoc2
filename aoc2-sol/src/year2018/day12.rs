// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **-- Advent of Code 2018 --**
//!
//! **--- Day 12: Subterranean Sustainability ---**
//!
//! The year 518 is significantly more underground than your history books implied. Either that, or you've arrived in a vast cavern network under the North Pole.
//!
//! After exploring a little, you discover a long tunnel that contains a row of small pots as far as you can see to your left and right. A few of them contain plants - someone is trying to grow things in these geothermally-heated caves.
//!
//! The pots are numbered, with 0 in front of you. To the left, the pots are numbered -1, -2, -3, and so on; to the right, 1, 2, 3.... Your puzzle input contains a list of pots from 0 to the right and whether they do (#) or do not (.) currently contain a plant, the initial state. (No other pots currently contain plants.) For example, an initial state of #..##.... indicates that pots 0, 3, and 4 currently contain plants.
//!
//! Your puzzle input also contains some notes you find on a nearby table: someone has been trying to figure out how these plants spread to nearby pots. Based on the notes, for each generation of plants, a given pot has or does not have a plant based on whether that pot (and the two pots on either side of it) had a plant in the last generation. These are written as LLCRR => N, where L are pots to the left, C is the current pot being considered, R are the pots to the right, and N is whether the current pot will have a plant in the next generation. For example:
//!
//! ```text
//!     A note like ..#.. => . means that a pot that contains a plant but with no plants within two pots of it will not have a plant in it during the next generation.
//!     A note like ##.## => . means that an empty pot with two plants on each side of it will remain empty in the next generation.
//!     A note like .##.# => # means that a pot has a plant in a given generation if, in the previous generation, there were plants in that pot, the one immediately to the left, and the one two pots to the right, but not in the ones immediately to the right and two to the left.
//! ```
//!
//! It's not clear what these plants are for, but you're sure it's important, so you'd like to make sure the current configuration of plants is sustainable by determining what will happen after 20 generations.
//!
//! For example, given the following input:
//!
//! ```text
//! initial state: #..#.#..##......###...###
//!
//! ...## => #
//! ..#.. => #
//! .#... => #
//! .#.#. => #
//! .#.## => #
//! .##.. => #
//! .#### => #
//! #.#.# => #
//! #.### => #
//! ##.#. => #
//! ##.## => #
//! ###.. => #
//! ###.# => #
//! ####. => #
//! ```
//!
//! For brevity, in this example, only the combinations which do produce a plant are listed. (Your input includes all possible combinations.) Then, the next 20 generations will look like this:
//!
//! ```text
//!                  1         2         3     
//!        0         0         0         0     
//!  0: ...#..#.#..##......###...###...........
//!  1: ...#...#....#.....#..#..#..#...........
//!  2: ...##..##...##....#..#..#..##..........
//!  3: ..#.#...#..#.#....#..#..#...#..........
//!  4: ...#.#..#...#.#...#..#..##..##.........
//!  5: ....#...##...#.#..#..#...#...#.........
//!  6: ....##.#.#....#...#..##..##..##........
//!  7: ...#..###.#...##..#...#...#...#........
//!  8: ...#....##.#.#.#..##..##..##..##.......
//!  9: ...##..#..#####....#...#...#...#.......
//! 10: ..#.#..#...#.##....##..##..##..##......
//! 11: ...#...##...#.#...#.#...#...#...#......
//! 12: ...##.#.#....#.#...#.#..##..##..##.....
//! 13: ..#..###.#....#.#...#....#...#...#.....
//! 14: ..#....##.#....#.#..##...##..##..##....
//! 15: ..##..#..#.#....#....#..#.#...#...#....
//! 16: .#.#..#...#.#...##...#...#.#..##..##...
//! 17: ..#...##...#.#.#.#...##...#....#...#...
//! 18: ..##.#.#....#####.#.#.#...##...##..##..
//! 19: .#..###.#..#.#.#######.#.#.#..#.#...#..
//! 20: .#....##....#####...#######....#.#..##.
//! ```
//!
//! The generation is shown along the left, where 0 is the initial state. The pot numbers are shown along the top, where 0 labels the center pot, negative-numbered pots extend to the left, and positive pots extend toward the right. Remember, the initial state begins at pot 0, which is not the leftmost pot used in this example.
//!
//! After one generation, only seven plants remain. The one in pot 0 matched the rule looking for ..#.., the one in pot 4 matched the rule looking for .#.#., pot 9 matched .##.., and so on.
//!
//! In this example, after 20 generations, the pots shown as # contain plants, the furthest left of which is pot -2, and the furthest right of which is pot 34. Adding up all the numbers of plant-containing pots after the 20th generation produces 325.
//!
//! After 20 generations, what is the sum of the numbers of all pots which contain a plant?
//!
//! **--- Part Two ---**
//!
//! You realize that 20 generations aren't enough. After all, these plants will need to last another 1500 years to even reach your timeline, not to mention your future.
//!
//! After fifty billion (50000000000) generations, what is the sum of the numbers of all pots which contain a plant?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use indexmap::IndexMap;
use regex::Regex;
use sliding_windows::{IterExt, Storage};
use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::{BufRead, BufReader},
};

type PlantData = (BTreeMap<isize, bool>, IndexMap<Vec<bool>, bool>);

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<(BTreeMap<isize, bool>, IndexMap<Vec<bool>, bool>), isize>(
        AoCYear::AOC2018,
        AoCDay::AOCD12,
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
    run_bench_solution::<PlantData, isize>(bench, AoCYear::AOC2018, AoCDay::AOCD12, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> PlantData {
    setup_br(reader).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T) -> Result<PlantData>
where
    T: BufRead,
{
    let initial_state_re = Regex::new(r"^initial state: ([\.#]+)")?;
    let patt_re = Regex::new(r"([\.#]+) => ([\.#])")?;
    let mut state_map = BTreeMap::new();
    let mut pattern_map = IndexMap::new();

    for line in valid_lines(reader) {
        for cap in initial_state_re.captures_iter(&line) {
            let state_str = &cap[1];

            for (idx, ch) in state_str.chars().enumerate() {
                let _ = match ch {
                    '#' => state_map.insert(isize::try_from(idx)?, true),
                    '.' => state_map.insert(isize::try_from(idx)?, false),
                    _ => return Err(anyhow!("invalid state character")),
                };
            }
        }

        for cap in patt_re.captures_iter(&line) {
            let pattern: Vec<bool> = (cap[1]).chars().map(|ch| ch == '#').collect();

            let _ = match &cap[2] {
                "#" => pattern_map.insert(pattern, true),
                "." => pattern_map.insert(pattern, false),
                _ => return Err(anyhow!("invalid pattern character")),
            };
        }
    }
    Ok((state_map, pattern_map))
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: PlantData) -> isize {
    find_res(data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(data: PlantData, second_star: bool) -> Result<isize> {
    let (mut state_map, pattern_map) = data;
    let res = if second_star {
        let mut sub_total = run_generations(95, &mut state_map, &pattern_map)?;
        sub_total += (50_000_000_000 - 95) * 91;
        sub_total
    } else {
        run_generations(20, &mut state_map, &pattern_map)?
    };
    Ok(res)
}

fn run_generations(
    gens: usize,
    state_map: &mut BTreeMap<isize, bool>,
    pattern_map: &IndexMap<Vec<bool>, bool>,
) -> Result<isize> {
    for _ in 0..gens {
        let mut action_map = HashMap::new();
        add_left(state_map)?;
        add_right(state_map)?;
        check_plants(state_map, pattern_map, &mut action_map);
        grow_plants(action_map, state_map);
    }
    let total: isize = state_map.iter().filter(|(_, v)| **v).map(|(k, _)| *k).sum();
    Ok(total)
}

fn add_left(state_map: &mut BTreeMap<isize, bool>) -> Result<()> {
    let min = find_min_plant(state_map)?;

    for i in (min - 4)..min {
        let _ = state_map.insert(i, false);
    }

    Ok(())
}

fn add_right(state_map: &mut BTreeMap<isize, bool>) -> Result<()> {
    let max = find_max_plant(state_map)?;

    for i in max + 1..max + 4 {
        let _ = state_map.insert(i, false);
    }

    Ok(())
}

fn find_min_plant(state_map: &BTreeMap<isize, bool>) -> Result<isize> {
    state_map
        .iter()
        .filter(|(_, v)| **v)
        .min_by_key(|(k, _)| *k)
        .map(|(k, _)| *k)
        .ok_or(anyhow!("no minimum key"))
}

fn find_max_plant(state_map: &BTreeMap<isize, bool>) -> Result<isize> {
    state_map
        .iter()
        .filter(|(_, v)| **v)
        .max_by_key(|(k, _)| *k)
        .map(|(k, _)| *k)
        .ok_or(anyhow!("no maximum key"))
}

fn check_plants(
    state_map: &BTreeMap<isize, bool>,
    pattern_map: &IndexMap<Vec<bool>, bool>,
    action_map: &mut HashMap<isize, bool>,
) {
    let mut window: Storage<(&isize, &bool)> = Storage::new(5);
    for x in state_map.iter().sliding_windows(&mut window) {
        let plants: Vec<bool> = x.iter().map(|(_, plant)| **plant).collect();
        let idx: Vec<isize> = x.iter().map(|(idx, _)| **idx).collect();
        let mut found = false;
        let mut action = false;

        for (pattern, outcome) in pattern_map {
            if pattern == &plants {
                found = true;
                action = *outcome;
            }
        }

        if found {
            let _ = action_map.insert(idx[2], action);
        } else {
            let _ = action_map.insert(idx[2], false);
        }
    }
}

fn grow_plants(action_map: HashMap<isize, bool>, state_map: &mut BTreeMap<isize, bool>) {
    for (idx, action) in action_map {
        *state_map.entry(idx).or_insert(false) = action;
    }
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<PlantData, isize>(AoCYear::AOC2018, AoCDay::AOCD12, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<PlantData, isize>(bench, AoCYear::AOC2018, AoCDay::AOCD12, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: PlantData) -> isize {
    find_res(data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find(data), 325);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    #[test]
    fn solution() {}
}
