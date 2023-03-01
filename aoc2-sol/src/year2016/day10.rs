// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 10 "Balance Bot"
//!
//! **--- Day 10: Balance Bots ---**
//!
//! **--- Part 1 ---**
//!
//! You come upon a factory in which many robots are zooming around handing small
//! microchips to each other.
//!
//! Upon closer examination, you notice that each bot only proceeds when it
//! has two microchips, and once it does, it gives each one to a different bot
//! or puts it in a marked "output" bin. Sometimes, bots take microchips from
//! "input" bins, too.
//!
//! Inspecting one of the microchips, it seems like they each contain a single
//! number; the bots must use some logic to decide what to do with each chip.
//! You access the local control computer and download the bots' instructions
//! (your puzzle input).
//!
//! Some of the instructions specify that a specific-valued microchip should
//! be given to a specific bot; the rest of the instructions indicate what a
//! given bot should do with its lower-value or higher-value chip.
//!
//! For example, consider the following instructions:
//!
//! ```text
//! value 5 goes to bot 2
//! bot 2 gives low to bot 1 and high to bot 0
//! value 3 goes to bot 1
//! bot 1 gives low to output 1 and high to bot 0
//! bot 0 gives low to output 2 and high to output 0
//! value 2 goes to bot 2
//! ```
//!
//! ```text
//! Initially, bot 1 starts with a value-3 chip, and bot 2 starts with a value-2 chip and a value-5 chip.
//! Because bot 2 has two microchips, it gives its lower one (2) to bot 1 and its higher one (5) to bot 0.
//! Then, bot 1 has two microchips; it puts the value-2 chip in output 1 and gives the value-3 chip to bot 0.
//! Finally, bot 0 has two microchips; it puts the 3 in output 2 and the 5 in output 0.
//!```
//!
//! In the end, output bin 0 contains a value-5 microchip, output bin 1 contains a value-2 microchip,
//! and output bin 2 contains a value-3 microchip. In this configuration, bot number 2 is responsible
//! for comparing value-5 microchips with value-2 microchips.
//!
//! Based on your instructions, what is the number of the bot that is responsible for comparing
//! value-61 microchips with value-17 microchips?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{get_cap, get_cap_x, run_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use indexmap::IndexMap;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy, Debug)]
enum GiveTo {
    Robot(usize),
    Output(usize),
}

impl Default for GiveTo {
    fn default() -> Self {
        Self::Robot(0)
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Giving {
    gt_low: GiveTo,
    gt_high: GiveTo,
    val1: Option<usize>,
    val2: Option<usize>,
}

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2016, AoCDay::AOCD10, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader, 61, 17).unwrap_or_default()
}

fn find_br<T>(reader: T, m1: usize, m2: usize) -> Result<usize>
where
    T: BufRead,
{
    let mut bots_map = IndexMap::new();
    setup(reader, &mut bots_map)?;
    Ok(move_microchips(&mut bots_map, m1, m2, true))
}

fn setup<T>(reader: T, bots_map: &mut IndexMap<usize, Giving>) -> Result<()>
where
    T: BufRead,
{
    let value_re = Regex::new(r"^value (\d+) goes to bot (\d+)$")?;
    let bot_re =
        Regex::new(r"^bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)$")?;
    let mut values_map = IndexMap::new();

    for line in valid_lines(reader) {
        if value_re.is_match(&line) {
            for caps in value_re.captures_iter(&line) {
                let bot = get_cap_x::<usize>(2, &caps)?;
                let value = get_cap_x::<usize>(1, &caps)?;
                let values = values_map.entry(bot).or_insert_with(Vec::new);
                values.push(value);
            }
        } else if bot_re.is_match(&line) {
            for caps in bot_re.captures_iter(&line) {
                let bot = get_cap_x::<usize>(1, &caps)?;
                let give_to_low_txt = get_cap(2, &caps)?;
                let low = get_cap_x::<usize>(3, &caps)?;
                let give_to_high_txt = get_cap(4, &caps)?;
                let high = get_cap_x::<usize>(5, &caps)?;
                let gt_low = if give_to_low_txt == "bot" {
                    GiveTo::Robot(low)
                } else {
                    GiveTo::Output(low)
                };
                let gt_high = if give_to_high_txt == "bot" {
                    GiveTo::Robot(high)
                } else {
                    GiveTo::Output(high)
                };
                _ = bots_map.insert(
                    bot,
                    Giving {
                        gt_low,
                        gt_high,
                        val1: None,
                        val2: None,
                    },
                );
            }
        } else {
            return Err(anyhow!(format!("invalid instruction: {line}")));
        }
    }

    for (bot, values) in values_map {
        let giving = bots_map.entry(bot).or_default();
        for value in values {
            give_to_bot(giving, value);
        }
    }

    Ok(())
}

fn move_microchips(
    bots_map: &mut IndexMap<usize, Giving>,
    m1: usize,
    m2: usize,
    part1: bool,
) -> usize {
    let mut output_vec = Vec::with_capacity(3);

    loop {
        let mut should_give = vec![];
        let (found, bot) = find_givers(bots_map, &mut should_give, m1, m2, part1);
        if found && part1 {
            return bot;
        }

        if should_give.is_empty() {
            break;
        }
        for (bot, bot_giving, low, high) in &should_give {
            match bot_giving.gt_low {
                GiveTo::Robot(bot_r) => {
                    let giving = bots_map.entry(bot_r).or_default();
                    give_to_bot(giving, *low);
                }
                GiveTo::Output(output) => {
                    if output == 0 || output == 1 || output == 2 {
                        output_vec.push(*low);
                    }
                }
            }
            match bot_giving.gt_high {
                GiveTo::Robot(bot_r) => {
                    let giving = bots_map.entry(bot_r).or_default();
                    give_to_bot(giving, *high);
                }
                GiveTo::Output(output) => {
                    if output == 0 || output == 1 || output == 2 {
                        output_vec.push(*low);
                    }
                }
            }

            let bot_giving = bots_map.entry(*bot).or_default();
            bot_giving.val1 = None;
            bot_giving.val2 = None;
        }
        should_give.clear();
    }
    output_vec.iter().product()
}

fn give_to_bot(giving: &mut Giving, value: usize) {
    if giving.val1.is_none() {
        giving.val1 = Some(value);
    } else if giving.val2.is_none() {
        giving.val2 = Some(value);
    }
}

fn find_givers(
    bots_map: &mut IndexMap<usize, Giving>,
    should_give: &mut Vec<(usize, Giving, usize, usize)>,
    m1: usize,
    m2: usize,
    part1: bool,
) -> (bool, usize) {
    for (bot, giving) in bots_map.iter() {
        if let (Some(val1), Some(val2)) = (giving.val1, giving.val2) {
            if part1 && ((m1 == val1 && m2 == val2) || (m2 == val1 && m1 == val2)) {
                return (true, *bot);
            }
            let (low, high) = if val1 < val2 {
                (val1, val2)
            } else {
                (val2, val1)
            };
            should_give.push((*bot, *giving, low, high));
        }
    }
    (false, 0)
}

// fn give_microchips(_bots_map: &mut HashMap<usize, Giving>, _giving: &Giving) {}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2016, AoCDay::AOCD10, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader).unwrap_or_default()
}

fn find2_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let mut bots_map = IndexMap::new();
    setup(reader, &mut bots_map)?;
    Ok(move_microchips(&mut bots_map, 0, 0, false))
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1), 5, 2)?, 2);
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
