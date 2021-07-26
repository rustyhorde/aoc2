// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 24 "It Hangs in the Balance"
//!
//! **--- Day 24: It Hangs in the Balance ---**
//!
//! **--- Part 1 ---**
//!
//! It's Christmas Eve, and Santa is loading up the sleigh for this year's deliveries.
//! However, there's one small problem: he can't get the sleigh to balance.
//! If it isn't balanced, he can't defy physics, and nobody gets presents this year.
//!
//! No pressure.
//!
//! Santa has provided you a list of the weights of every package he needs to fit
//! on the sleigh. The packages need to be split into three groups of exactly the
//! same weight, and every package has to fit. The first group goes in the passenger
//! compartment of the sleigh, and the second and third go in containers on either side.
//! Only when all three groups weigh exactly the same amount will the sleigh be
//! able to fly. Defying physics has rules, you know!
//!
//! Of course, that's not the only problem. The first group - the one going in the
//! passenger compartment - needs as few packages as possible so that Santa has some
//! legroom left over. It doesn't matter how many packages are in either of the
//! other two groups, so long as all of the groups weigh the same.
//!
//! Furthermore, Santa tells you, if there are multiple ways to arrange the packages
//! such that the fewest possible are in the first group, you need to choose the way
//! where the first group has the smallest quantum entanglement to reduce the chance
//! of any "complications". The quantum entanglement of a group of packages is the
//! product of their weights, that is, the value you get when you multiply their weights
//! together. Only consider quantum entanglement if the first group has the fewest
//! possible number of packages in it and all groups weigh the same amount.
//!
//! For example, suppose you have ten packages with weights `1` through `5` and `7`
//! through `11`. For this situation, some of the unique first groups, their quantum
//! entanglements, and a way to divide the remaining packages are as follows:
//!
//! ```text
//! Group 1;             Group 2; Group 3
//! 11 9       (QE= 99); 10 8 2;  7 5 4 3 1
//! 10 9 1     (QE= 90); 11 7 2;  8 5 4 3
//! 10 8 2     (QE=160); 11 9;    7 5 4 3 1
//! 10 7 3     (QE=210); 11 9;    8 5 4 2 1
//! 10 5 4 1   (QE=200); 11 9;    8 7 3 2
//! 10 5 3 2   (QE=300); 11 9;    8 7 4 1
//! 10 4 3 2 1 (QE=240); 11 9;    8 7 5
//! 9 8 3      (QE=216); 11 7 2;  10 5 4 1
//! 9 7 4      (QE=252); 11 8 1;  10 5 3 2
//! 9 5 4 2    (QE=360); 11 8 1;  10 7 3
//! 8 7 5      (QE=280); 11 9;    10 4 3 2 1
//! 8 5 4 3    (QE=480); 11 9;    10 7 2 1
//! 7 5 4 3 1  (QE=420); 11 9;    10 8 2
//! ```
//!
//! Of these, although `10 9 1` has the smallest quantum entanglement (`90`), the
//! configuration with only two packages, `11 9`, in the passenger compartment gives
//! Santa the most legroom and wins. In this situation, the quantum entanglement for
//! the ideal configuration is therefore `99`. Had there been two configurations with
//! only two packages in the first group, the one with the smaller quantum entanglement
//! would be chosen.
//!
//! What is the quantum entanglement of the first group of packages in the ideal configuration?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_solution, valid_lines},
};
use anyhow::Result;
use std::{
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
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD24, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader).unwrap_or_default()
}

fn find_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let mut weights = vec![];

    for line in valid_lines(reader) {
        weights.push(line.parse::<usize>()?);
    }

    let group_size = weights.iter().sum::<usize>() / 3;
    println!("Group Size: {}", group_size);
    // let mut equals = vec![];
    // let mut min_len = usize::MAX;
    // for set in weights.iter().powerset().permutations(3).filter(|set| set[0].is_empty() || set[1].is_empty() || set[2].is_empty()) {
    //     if set[0].len() > min_len {
    //         continue;
    //     }
    //     let s0 = set[0].iter().copied().sum::<usize>();
    //     let s1 = set[1].iter().copied().sum::<usize>();
    //     let s2 = set[2].iter().copied().sum::<usize>();
    //     if s0 == s1 && s1 == s2 {
    //         if set[0].len() < min_len {
    //             min_len = set[0].len();
    //         }
    //         equals.push(s0);
    //     }
    // }
    Ok(0)
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD24, find2).map(|_| 0)
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

    const TEST_1: &str = r"1
2
3
4
5
7
8
9
10
11";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1))?, 99);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    // use super::find2_br;
    use anyhow::Result;
    // use std::io::Cursor;

    // const TEST_1: &str = r"turn on 0,0 through 0,0";
    // const TEST_2: &str = r"toggle 0,0 through 999,999";

    #[test]
    fn solution() -> Result<()> {
        // assert_eq!(find2_br(Cursor::new(TEST_1))?, 1);
        // assert_eq!(find2_br(Cursor::new(TEST_2))?, 2_000_000);
        Ok(())
    }
}
