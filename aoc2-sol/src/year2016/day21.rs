// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Scrambled Letters and Hash
//!
//! **--- Day 21: Scrambled Letters and Hash ---**
//!
//! **--- Part 1 ---**
//!
//! The computer system you're breaking into uses a weird scrambling function
//! to store its passwords. It shouldn't be much trouble to create your own scrambled
//! password so you can add it to the system; you just have to implement the scrambler.
//!
//! The scrambling function is a series of operations (the exact list is provided
//! in your puzzle input). Starting with the password to be scrambled, apply each
//! operation in succession to the string. The individual operations behave as follows:
//!
//! ```text
//! swap position X with position Y means that the letters at indexes X and Y (counting from 0) should be swapped.
//! swap letter X with letter Y means that the letters X and Y should be swapped (regardless of where they appear in the string).
//! rotate left/right X steps means that the whole string should be rotated; for example, one right rotation would turn abcd into dabc.
//! rotate based on position of letter X means that the whole string should be rotated to the right based on the index of letter X (counting from 0) as determined before this instruction does any rotations. Once the index is determined, rotate the string to the right one time, plus a number of times equal to that index, plus one additional time if the index was at least 4.
//! reverse positions X through Y means that the span of letters at indexes X through Y (including the letters at X and Y) should be reversed in order.
//! move position X to position Y means that the letter which is at index X should be removed from the string, then inserted such that it ends up at index Y.
//! ```
//!
//! For example, suppose you start with `abcde` and perform the following operations:
//!
//! ```text
//! swap position 4 with position 0 swaps the first and last letters, producing the input for the next step, ebcda.
//! swap letter d with letter b swaps the positions of d and b: edcba.
//! reverse positions 0 through 4 causes the entire string to be reversed, producing abcde.
//! rotate left 1 step shifts all letters left one position, causing the first letter to wrap to the end of the string: bcdea.
//! move position 1 to position 4 removes the letter at position 1 (c), then inserts it at position 4 (the end of the string): bdeac.
//! move position 3 to position 0 removes the letter at position 3 (a), then inserts it at position 0 (the front of the string): abdec.
//! rotate based on position of letter b finds the index of letter b (1), then rotates the string right once plus a number of times equal to that index (2): ecabd.
//! rotate based on position of letter d finds the index of letter d (4), then rotates the string right once, plus a number of times equal to that index, plus an additional time because the index was at least 4, for a total of 6 right rotations: decab.
//! ```
//!
//! After these steps, the resulting scrambled password is `decab`.
//!
//! Now, you just need to generate a new scrambled password and you can access the system.
//!
//! Given the list of scrambling operations in your puzzle input, what is the result of scrambling
//! `abcdefgh`?
//!
//! **--- Part Two ---**
//!
//! You scrambled the password correctly, but you discover that you can't actually modify
//! the password file on the system. You'll need to un-scramble one of the existing passwords
//! by reversing the scrambling process.
//!
//! What is the un-scrambled version of the scrambled password `fbgdceah`?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{get_cap, get_cap_x, print_err, run_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use itertools::Itertools;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Instructions {
    Swap(usize, usize),
    SwapChar(char, char),
    Rev(usize, usize),
    RotLeft(usize),
    RotRight(usize),
    Move(usize, usize),
    RotB(char),
}

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_solution::<String>(AoCYear::AOC2016, AoCDay::AOCD21, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> String {
    find_br(reader, "abcdefgh")
        .map_err(print_err)
        .unwrap_or_default()
}

fn find_br<T>(reader: T, input: &str) -> Result<String>
where
    T: BufRead,
{
    scramble(reader, input, false)
}

fn scramble<T>(reader: T, input: &str, part2: bool) -> Result<String>
where
    T: BufRead,
{
    let swap_pos_re = Regex::new(r"^swap position (\d+) with position (\d+)$")?;
    let swap_let_re = Regex::new(r"^swap letter ([a-z]) with letter ([a-z])$")?;
    let rev_pos_re = Regex::new(r"^reverse positions (\d+) through (\d+)$")?;
    let rot_lr_re = Regex::new(r"^rotate (left|right) (\d+) steps?$")?;
    let move_re = Regex::new(r"^move position (\d+) to position (\d+)$")?;
    let rot_b_re = Regex::new(r"^rotate based on position of letter ([a-z])$")?;

    let mut instructions = vec![];

    for line in valid_lines(reader) {
        if swap_pos_re.is_match(&line) {
            for caps in swap_pos_re.captures_iter(&line) {
                let x = get_cap_x::<usize>(1, &caps)?;
                let y = get_cap_x::<usize>(2, &caps)?;
                instructions.push(Instructions::Swap(x, y));
            }
        } else if swap_let_re.is_match(&line) {
            for caps in swap_let_re.captures_iter(&line) {
                let x = get_cap_x::<char>(1, &caps)?;
                let y = get_cap_x::<char>(2, &caps)?;
                instructions.push(Instructions::SwapChar(x, y));
            }
        } else if rev_pos_re.is_match(&line) {
            for caps in rev_pos_re.captures_iter(&line) {
                let x = get_cap_x::<usize>(1, &caps)?;
                let y = get_cap_x::<usize>(2, &caps)?;
                instructions.push(Instructions::Rev(x, y));
            }
        } else if rot_lr_re.is_match(&line) {
            for caps in rot_lr_re.captures_iter(&line) {
                let direction = get_cap(1, &caps)?;
                let x = get_cap_x::<usize>(2, &caps)?;
                match &direction[..] {
                    "left" => instructions.push(Instructions::RotLeft(x)),
                    "right" => instructions.push(Instructions::RotRight(x)),
                    _ => return Err(anyhow!(format!("invalid instruction: {}", line))),
                }
            }
        } else if move_re.is_match(&line) {
            for caps in move_re.captures_iter(&line) {
                let x = get_cap_x::<usize>(1, &caps)?;
                let y = get_cap_x::<usize>(2, &caps)?;
                instructions.push(Instructions::Move(x, y));
            }
        } else if rot_b_re.is_match(&line) {
            for caps in rot_b_re.captures_iter(&line) {
                let x = get_cap_x::<char>(1, &caps)?;
                instructions.push(Instructions::RotB(x));
            }
        } else {
            return Err(anyhow!(format!("invalid instruction: {}", line)));
        }
    }

    if part2 {
        let mut result = vec![];
        for inp in input.chars().permutations(8) {
            let mut inp_c = inp.clone();
            let output = process_input(&mut inp_c, &instructions);

            if output == input {
                result = inp;
                break;
            }
        }
        Ok(result.iter().fold(String::new(), |acc, x| {
            let mut tmp = acc;
            tmp.push(*x);
            tmp
        }))
    } else {
        let mut in_vec = input.chars().collect::<Vec<char>>();
        Ok(process_input(&mut in_vec, &instructions))
    }
}

fn process_input(in_vec: &mut Vec<char>, instructions: &[Instructions]) -> String {
    for inst in instructions {
        match inst {
            Instructions::Swap(x, y) => swap_pos(in_vec, *x, *y),
            Instructions::SwapChar(x, y) => swap_char(in_vec, *x, *y),
            Instructions::Rev(x, y) => reverse(in_vec, *x, *y),
            Instructions::RotRight(x) => rot_r(in_vec, *x),
            Instructions::RotLeft(x) => rot_l(in_vec, *x),
            Instructions::Move(x, y) => move_pos(in_vec, *x, *y),
            Instructions::RotB(x) => rot_b(in_vec, *x),
        }
    }

    in_vec.iter().fold(String::new(), |acc, x| {
        let mut tmp = acc;
        tmp.push(*x);
        tmp
    })
}

fn swap_pos(input: &mut [char], x: usize, y: usize) {
    input.swap(x, y);
}

fn swap_char(input: &mut [char], x: char, y: char) {
    let idx_x: usize = input
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, ch)| if *ch == x { acc + idx } else { acc });
    let idx_y: usize = input
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, ch)| if *ch == y { acc + idx } else { acc });
    swap_pos(input, idx_x, idx_y);
}

fn reverse(input: &mut [char], x: usize, y: usize) {
    let mut curr_x = x;
    let mut curr_y = y;
    loop {
        if curr_x == curr_y {
            break;
        }
        swap_pos(input, curr_x, curr_y);
        if curr_x + 1 == curr_y {
            break;
        }
        curr_x += 1;
        curr_y -= 1;
    }
}

fn rot_r(input: &mut [char], x: usize) {
    let rot = x % input.len();
    input.rotate_right(rot);
}

fn rot_l(input: &mut [char], x: usize) {
    let rot = x % input.len();
    input.rotate_left(rot);
}

fn rot_b(input: &mut [char], x: char) {
    let idx_x: usize = input
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, ch)| if *ch == x { acc + idx } else { acc });
    let mut rot = 1 + idx_x;
    if idx_x >= 4 {
        rot += 1;
    }
    rot_r(input, rot);
}

fn move_pos(input: &mut Vec<char>, x: usize, y: usize) {
    let rem = input.remove(x);
    input.insert(y, rem);
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<String>(AoCYear::AOC2016, AoCDay::AOCD21, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> String {
    find2_br(reader, "fbgdceah")
        .map_err(print_err)
        .unwrap_or_default()
}

fn find2_br<T>(reader: T, input: &str) -> Result<String>
where
    T: BufRead,
{
    scramble(reader, input, true)
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 step
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1), "abcde")?, "decab");
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
