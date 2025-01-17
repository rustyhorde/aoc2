// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! An Elephant Named Joseph
//!
//! **--- Day 19: An Elephant Named Joseph ---**
//!
//! **--- Part 1 ---**
//!
//! The Elves contact you over a highly secure emergency channel. Back at the
//! North Pole, the Elves are busy misunderstanding White Elephant parties.
//!
//! Each Elf brings a present. They all sit in a circle, numbered starting with
//! position `1`. Then, starting with the first Elf, they take turns stealing all
//! the presents from the Elf to their left. An Elf with no presents is removed
//! from the circle and does not take turns.
//!
//! For example, with five Elves (numbered 1 to 5):
//!
//! ```text
//!   1
//! 5   2
//!  4 3
//! ```
//!
//! ```text
//! Elf 1 takes Elf 2's present.
//! Elf 2 has no presents and is skipped.
//! Elf 3 takes Elf 4's present.
//! Elf 4 has no presents and is also skipped.
//! Elf 5 takes Elf 1's two presents.
//! Neither Elf 1 nor Elf 2 have any presents, so both are skipped.
//! Elf 3 takes Elf 5's three presents.
//! ```
//!
//! So, with five Elves, the Elf that sits starting in position `3` gets all the presents.
//!
//! With the number of Elves given in your puzzle input, which Elf gets all the presents?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{print_err, run_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2016, AoCDay::AOCD19, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader).map_err(print_err).unwrap_or_default()
}

fn find_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    // let mut elf_count = 0;
    // for line in valid_lines(reader) {
    //     elf_count = line.parse::<usize>()?;
    // }
    // let mut elf_circle = BTreeMap::new();

    // for i in 0..elf_count {
    //     _ = elf_circle.entry(i).or_insert(1_usize);
    // }

    // let mut elf_ptr = 0;

    // 'outer: loop {
    //     if *elf_circle
    //         .get(&elf_ptr)
    //         .ok_or_else(|| anyhow!("invalid elf"))?
    //         > 0
    //     {
    //         let mut take_ptr = (elf_ptr + 1) % elf_count;
    //         loop {
    //             if take_ptr == elf_ptr {
    //                 break 'outer;
    //             }
    //             let their_presents = elf_circle.entry(take_ptr).or_default();
    //             if *their_presents > 0 {
    //                 *their_presents = 0;
    //                 break;
    //             }
    //             take_ptr = (take_ptr + 1) % elf_count;
    //         }
    //     }

    //     elf_ptr = (elf_ptr + 1) % elf_count;
    // }

    // let elf: usize = elf_circle
    //     .iter()
    //     .filter_map(|(k, v)| if *v > 0 { Some(k) } else { None })
    //     .sum();
    // Ok(elf + 1)
    setup(reader, false)
}

fn setup<T>(reader: T, part2: bool) -> Result<usize>
where
    T: BufRead,
{
    let mut elf_count = 0;
    for line in valid_lines(reader) {
        elf_count = line.parse::<usize>()?;
    }

    let mut elf_queue = VecDeque::new();

    if part2 {
        let mut elf_queue_2 = VecDeque::new();

        for i in 1..=elf_count {
            if i < (elf_count / 2) + 1 {
                elf_queue.push_back(i);
            } else {
                elf_queue_2.push_back(i);
            }
        }

        loop {
            let elf_queue_len = elf_queue.len();
            let elf_queue_2_len = elf_queue_2.len();
            if elf_queue_len == 1 || elf_queue_2_len == 1 {
                break;
            }

            if elf_queue_len > elf_queue_2_len {
                let _taken = elf_queue.pop_back().ok_or_else(|| anyhow!("badness"))?;
            } else {
                let _taken = elf_queue_2.pop_front().ok_or_else(|| anyhow!("badness"))?;
            }

            elf_queue_2.push_back(elf_queue.pop_front().ok_or_else(|| anyhow!("badness"))?);
            elf_queue.push_back(elf_queue_2.pop_front().ok_or_else(|| anyhow!("badness"))?);
        }

        Ok(if elf_queue_2.len() == 1 {
            elf_queue_2[0]
        } else {
            elf_queue[0]
        })
    } else {
        for i in 1..=elf_count {
            elf_queue.push_back(i);
        }

        loop {
            if elf_queue.len() == 1 {
                break;
            }

            let taker = elf_queue.pop_front().ok_or_else(|| anyhow!("badness"))?;
            elf_queue.push_back(taker);
            _ = elf_queue.pop_front();
        }

        Ok(elf_queue[0])
    }
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2016, AoCDay::AOCD19, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader).map_err(print_err).unwrap_or_default()
}

fn find2_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    setup(reader, true)
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"5";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1))?, 3);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"5";
    const TEST_2: &str = r"10";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find2_br(Cursor::new(TEST_1))?, 2);
        assert_eq!(find2_br(Cursor::new(TEST_2))?, 1);
        Ok(())
    }
}
