// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Memory Reallocation
//!
//! **--- Day 6: Memory Reallocation ---**
//!
//! **--- Part 1 ---**
//!
//! A debugger program here is having an issue: it is trying to repair a
//! memory reallocation routine, but it keeps getting stuck in an infinite loop.
//!
//! In this area, there are sixteen memory banks; each memory bank can
//! hold any number of blocks. The goal of the reallocation routine is to
//! balance the blocks between the memory banks.
//!
//! The reallocation routine operates in cycles. In each cycle, it finds the
//! memory bank with the most blocks (ties won by the lowest-numbered memory bank)
//! and redistributes those blocks among the banks. To do this, it removes all of
//! the blocks from the selected bank, then moves to the next (by index) memory
//! bank and inserts one of the blocks. It continues doing this until it runs out
//! of blocks; if it reaches the last memory bank, it wraps around to the first one.
//!
//! The debugger would like to know how many redistributions can be done before a
//! blocks-in-banks configuration is produced that has been seen before.
//!
//! For example, imagine a scenario with only four memory banks:
//!
//! ```text
//! The banks start with 0, 2, 7, and 0 blocks. The third bank has the most blocks, so it is chosen for redistribution.
//! Starting with the next bank (the fourth bank) and then continuing to the first bank, the second bank, and so on, the 7 blocks are spread out over the memory banks. The fourth, first, and second banks get two blocks each, and the third bank gets one back. The final result looks like this: 2 4 1 2.
//! Next, the second bank is chosen because it contains the most blocks (four). Because there are four memory banks, each gets one block. The result is: 3 1 2 3.
//! Now, there is a tie between the first and fourth memory banks, both of which have three blocks. The first bank wins the tie, and its three blocks are distributed evenly over the other three banks, leaving it with none: 0 2 3 4.
//! The fourth bank is chosen, and its four blocks are distributed such that each of the four banks receives one: 1 3 4 1.
//! The third bank is chosen, and the same thing happens: 2 4 1 2.
//! ```
//!
//! At this point, we've reached a state we've seen before: `2 4 1 2` was already seen.
//! The infinite loop is detected after the fifth block redistribution cycle, and so
//! the answer in this example is `5`.
//!
//! Given the initial block counts in your puzzle input, how many redistribution cycles
//! must be completed before a configuration is produced that has been seen before?
//!
//! **--- Part Two ---**
//!
//! Out of curiosity, the debugger would also like to know the size of the loop: starting
//! from a state that has already been seen, how many block redistribution cycles must be performed
//! before that same state is seen again?
//!
//! In the example above, `2 4 1 2` is seen again after four cycles, and so the answer in
//! that example would be `4`.
//!
//! How many cycles are in the infinite loop that arises from the configuration in your puzzle input?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{print_err, run_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use std::{
    cmp::Ordering,
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2017, AoCDay::AOCD06, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader).map_err(print_err).unwrap_or_default()
}

fn find_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    reallocate_memory(reader, false)
}

fn reallocate_memory<T>(reader: T, part2: bool) -> Result<usize>
where
    T: BufRead,
{
    let mut input = String::new();
    for line in valid_lines(reader) {
        input = line;
    }

    // Convert the line to a vector of usize.
    let mut vals_vec = input
        .split_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect::<Vec<usize>>();

    // Setup some state.
    let len = vals_vec.len();
    let mut once_more = part2;
    let mut steps = 1;
    let mut seen = HashSet::new();

    loop {
        // We've seen the current vec, so put into set.
        _ = seen.insert(vals_vec.clone());

        // Find the first position of max value.
        let mut iter = vals_vec.iter().copied().enumerate();
        let init = iter.next().ok_or_else(|| anyhow!("no valid values"))?;
        let (pos, max) = iter.fold(init, |acc, x| {
            if x.1.cmp(&acc.1) == Ordering::Greater {
                x
            } else {
                acc
            }
        });

        // Reset the max to 0
        vals_vec[pos] = 0;

        // Cycle through the vec, reallocating
        for i in 0..max {
            let idx = (pos + i + 1) % len;
            vals_vec[idx] += 1;
        }

        // Check if we have seen the resulting vec
        if seen.contains(&vals_vec) {
            // If we have, but we want to find the next occurence
            // then reset some state and continue.
            if once_more {
                steps = 1;
                seen.clear();
                once_more = false;
                continue;
            }
            // Otherwise we are done.
            break;
        }
        // If we haven't, increment the step count and loop
        steps += 1;
    }
    Ok(steps)
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2017, AoCDay::AOCD06, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader).map_err(print_err).unwrap_or_default()
}

fn find2_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    reallocate_memory(reader, true)
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"0 2 7 0";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1))?, 5);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"0 2 7 0";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find2_br(Cursor::new(TEST_1))?, 4);
        Ok(())
    }
}
