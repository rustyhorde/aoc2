// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 1 "Not Quite Lisp" Solution
//!
//! **--- Day 1: Not Quite Lisp ---**
//!
//! **--- Part 1 ---**
//!
//! Santa was hoping for a white Christmas, but his weather machine's "snow"
//! function is powered by stars, and he's fresh out! To save Christmas, he
//! needs you to collect fifty stars by December 25th.
//!
//! Collect stars by helping Santa solve puzzles. Two puzzles will be made
//! available on each day in the Advent calendar; the second puzzle is unlocked
//! when you complete the first. Each puzzle grants one star. Good luck!
//!
//! Here's an easy puzzle to warm you up.
//!
//! Santa is trying to deliver presents in a large apartment building, but
//! he can't find the right floor - the directions he got are a little confusing.
//! He starts on the ground floor (floor 0) and then follows the instructions
//! one character at a time.
//!
//! An opening parenthesis, (, means he should go up one floor, and a closing
//! parenthesis, ), means he should go down one floor.
//!
//! The apartment building is very tall, and the basement is very deep;
//! he will never find the top or bottom floors.
//!
//! For example:
//!
//! * `(())` and `()()` both result in floor 0.
//! * `(((` and `(()(()(` both result in floor 3.
//! * `))(((((` also results in floor 3.
//! * `())` and `))(` both result in floor -1 (the first basement level).
//! * `)))` and `)())())` both result in floor -3.
//!
//! To what floor do the instructions take Santa?
//!
//! **--- Part Two ---**
//!
//! Now, given the same instructions, find the position of the first character
//! that causes him to enter the basement (floor -1). The first character in
//! the instructions has position 1, the second character has position 2, and so on.
//!
//! For example:
//!
//! * `)` causes him to enter the basement at character position 1.
//! * `()())` causes him to enter the basement at character position 5.
//!
//! What is the position of the character that causes Santa to first enter the basement?

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
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`Duration`](std::time::Duration) is invalid.
pub fn part_1() -> Result<u32> {
    run_solution::<isize>(AoCYear::AOC2015, AoCDay::AOCD01, find_floor).map(|_| 0)
}

fn find_floor(reader: BufReader<File>) -> isize {
    find_floor_br(reader)
}

fn find_floor_br<T>(reader: T) -> isize
where
    T: BufRead,
{
    valid_lines(reader).fold(0, handle_line)
}

#[allow(clippy::needless_pass_by_value)]
#[inline]
fn handle_line(acc: isize, line: String) -> isize {
    line.chars().fold(acc, up_or_down)
}

#[inline]
fn up_or_down(acc: isize, ch: char) -> isize {
    match ch {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc,
    }
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<i32>(AoCYear::AOC2015, AoCDay::AOCD01, find_basement).map(|_| 0)
}

fn find_basement(reader: BufReader<File>) -> i32 {
    find_basement_br(reader)
}

fn find_basement_br<T>(reader: T) -> i32
where
    T: BufRead,
{
    let mut state = (0, 0);

    valid_lines(reader)
        .map(|line| line.chars().scan(&mut state, handle_ch).for_each(|_| ()))
        .for_each(|()| ());

    state.0
}

#[allow(clippy::mut_mut)]
fn handle_ch(state: &mut &mut (i32, i32), ch: char) -> Option<(i32, i32)> {
    state.0 += 1;

    if state.1 == 0 && ch == ')' {
        None
    } else {
        match ch {
            '(' => state.1 += 1,
            ')' => state.1 -= 1,
            _ => {}
        }
        Some(**state)
    }
}

#[cfg(test)]
mod one_star {
    use super::find_floor_br;
    use std::io::Cursor;

    const TEST_CHAIN: &str = r"(())";
    const TEST_CHAIN_1: &str = r"()()";
    const TEST_CHAIN_2: &str = r"(((";
    const TEST_CHAIN_3: &str = r"(()(()(";
    const TEST_CHAIN_4: &str = r"))(((((";
    const TEST_CHAIN_5: &str = r"())";
    const TEST_CHAIN_6: &str = r"))(";
    const TEST_CHAIN_7: &str = r")))";
    const TEST_CHAIN_8: &str = r")())())";

    #[test]
    fn solution() {
        assert_eq!(find_floor_br(Cursor::new(TEST_CHAIN)), 0);
        assert_eq!(find_floor_br(Cursor::new(TEST_CHAIN_1)), 0);
        assert_eq!(find_floor_br(Cursor::new(TEST_CHAIN_2)), 3);
        assert_eq!(find_floor_br(Cursor::new(TEST_CHAIN_3)), 3);
        assert_eq!(find_floor_br(Cursor::new(TEST_CHAIN_4)), 3);
        assert_eq!(find_floor_br(Cursor::new(TEST_CHAIN_5)), -1);
        assert_eq!(find_floor_br(Cursor::new(TEST_CHAIN_6)), -1);
        assert_eq!(find_floor_br(Cursor::new(TEST_CHAIN_7)), -3);
        assert_eq!(find_floor_br(Cursor::new(TEST_CHAIN_8)), -3);
    }
}

#[cfg(test)]
mod two_star {
    use super::find_basement_br;
    use std::io::Cursor;

    const TEST_CHAIN: &str = r")";
    const TEST_CHAIN_1: &str = r"()())";

    #[test]
    fn solution() {
        assert_eq!(find_basement_br(Cursor::new(TEST_CHAIN)), 1,);
        assert_eq!(find_basement_br(Cursor::new(TEST_CHAIN_1)), 5);
    }
}
