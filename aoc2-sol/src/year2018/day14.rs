// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 14

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::Result;
use std::{
    collections::VecDeque,
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
    run_setup_solution::<Vec<String>, String>(AoCYear::AOC2018, AoCDay::AOCD14, setup, find)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<String>, String>(bench, AoCYear::AOC2018, AoCDay::AOCD14, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> Vec<String> {
    setup_br(reader).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T) -> Result<Vec<String>>
where
    T: BufRead,
{
    let mut data = vec![];
    for line in valid_lines(reader) {
        data.push(line);
    }
    Ok(data)
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: Vec<String>) -> String {
    find_res(data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(data: Vec<String>, second_star: bool) -> Result<String> {
    let mut recipe_count = String::new();

    for line in data {
        recipe_count = line;
    }

    score_recipes(&recipe_count, second_star)
}

fn check_patt(recipes: &VecDeque<u8>, pattern: &str, len: usize) -> bool {
    if recipes.len() >= len {
        let rev_patt: String = pattern.chars().rev().collect();
        let mut check_str = String::new();
        for i in recipes.iter().rev().take(len) {
            check_str.push_str(&i.to_string());
        }

        if rev_patt == check_str {
            return true;
        }
    }
    false
}

fn score_recipes(count: &str, second_star: bool) -> Result<String> {
    let len = count.len();
    let count_u = count.parse::<usize>()?;
    let mut recipe_deque = VecDeque::<u8>::new();
    recipe_deque.push_back(3);
    recipe_deque.push_back(7);

    let mut idx_e1 = 0;
    let mut idx_e2 = 1;

    loop {
        if !second_star && recipe_deque.len() > count_u + 10 {
            break;
        }
        let e1_r = recipe_deque[idx_e1];
        let e2_r = recipe_deque[idx_e2];
        let next = e1_r + e2_r;

        if next > 9 {
            let tens = next / 10;
            let ones = next % 10;

            recipe_deque.push_back(tens);
            if second_star && check_patt(&recipe_deque, count, len) {
                break;
            }
            recipe_deque.push_back(ones);
            if second_star && check_patt(&recipe_deque, count, len) {
                break;
            }
        } else {
            recipe_deque.push_back(next);
            if second_star && check_patt(&recipe_deque, count, len) {
                break;
            }
        }

        let len = recipe_deque.len();

        idx_e1 = (idx_e1 + e1_r as usize + 1) % len;
        idx_e2 = (idx_e2 + e2_r as usize + 1) % len;
    }

    let output = if second_star {
        let total_len = recipe_deque.len();
        let _res = recipe_deque.split_off(total_len - len);
        recipe_deque.len().to_string()
    } else {
        let after_count = recipe_deque.split_off(count_u);
        after_count
            .into_iter()
            .take(10)
            .map(|x| x.to_string())
            .collect()
    };

    Ok(output)
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<Vec<String>, String>(AoCYear::AOC2018, AoCDay::AOCD14, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<String>, String>(bench, AoCYear::AOC2018, AoCDay::AOCD14, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: Vec<String>) -> String {
    find_res(data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"5";
    const TEST_2: &str = r"9";
    const TEST_3: &str = r"18";
    const TEST_4: &str = r"2018";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find(data), "0124515891");
        let data = setup_br(Cursor::new(TEST_2))?;
        assert_eq!(find(data), "5158916779");
        let data = setup_br(Cursor::new(TEST_3))?;
        assert_eq!(find(data), "9251071085");
        let data = setup_br(Cursor::new(TEST_4))?;
        assert_eq!(find(data), "5941429882");
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"51589";
    const TEST_2: &str = r"01245";
    const TEST_3: &str = r"92510";
    const TEST_4: &str = r"59414";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find2(data), "9");
        let data = setup_br(Cursor::new(TEST_2))?;
        assert_eq!(find2(data), "5");
        let data = setup_br(Cursor::new(TEST_3))?;
        assert_eq!(find2(data), "18");
        let data = setup_br(Cursor::new(TEST_4))?;
        assert_eq!(find2(data), "2018");
        Ok(())
    }
}
