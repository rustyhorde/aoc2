// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! A Maze of Twisty Trampolines, All Alike
//!
//! **--- Day 5: A Maze of Twisty Trampolines, All Alike ---**
//!
//! **--- Part 1 ---**
//!
//! An urgent interrupt arrives from the CPU: it's trapped in a maze of
//! jump instructions, and it would like assistance from any programs with spare
//! cycles to help find the exit.
//!
//! The message includes a list of the offsets for each jump. Jumps are relative:
//! -1 moves to the previous instruction, and 2 skips the next one. Start at the
//! first instruction in the list. The goal is to follow the jumps until one leads
//! outside the list.
//!
//! In addition, these instructions are a little strange; after each jump, the offset
//! of that instruction increases by 1. So, if you come across an offset of 3, you
//! would move three instructions forward, but change it to a 4 for the next time it
//! is encountered.
//!
//! For example, consider the following list of jump offsets:
//!
//! ```text
//! 0
//! 3
//! 0
//! 1
//! -3
//! ```
//!
//! Positive jumps ("forward") move downward; negative jumps move upward. For legibility
//! in this example, these offset values will be written all on one line, with the current
//! instruction marked in parentheses. The following steps would be taken before an exit is found:
//!
//! ```text
//! (0) 3  0  1  -3  - before we have taken any steps.
//! (1) 3  0  1  -3  - jump with offset 0 (that is, don't jump at all). Fortunately, the instruction is then incremented to 1.
//!  2 (3) 0  1  -3  - step forward because of the instruction we just modified. The first instruction is incremented again, now to 2.
//!  2  4  0  1 (-3) - jump all the way to the end; leave a 4 behind.
//!  2 (4) 0  1  -2  - go back to where we just were; increment -3 to -2.
//!  2  5  0  1  -2  - jump 4 steps forward, escaping the maze.
//! ```
//!
//! In this example, the exit is reached in 5 steps.
//!
//! How many steps does it take to reach the exit?
//!
//! **--- Part Two ---**
//!
//! Now, the jumps are even stranger: after each jump, if the offset was three or more,
//! instead decrease it by 1. Otherwise, increase it by 1 as before.
//!
//! Using this rule with the above example, the process now takes 10 steps, and the offset
//! values after finding the exit are left as `2 3 2 3 -1`.
//!
//! How many steps does it now take to reach the exit?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{print_err, run_solution, valid_lines},
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
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_solution::<isize>(AoCYear::AOC2017, AoCDay::AOCD05, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> isize {
    find_br(reader).map_err(print_err).unwrap_or_default()
}

fn find_br<T>(reader: T) -> Result<isize>
where
    T: BufRead,
{
    jump(reader, false)
}

fn jump<T>(reader: T, part2: bool) -> Result<isize>
where
    T: BufRead,
{
    let mut jump_vec = vec![];
    for line in valid_lines(reader) {
        jump_vec.push(line.parse::<isize>()?);
    }
    jump_away(&mut jump_vec, part2)
}

fn jump_away(jump_vec: &mut [isize], crazy_jumps: bool) -> Result<isize> {
    let list_len = jump_vec.len();
    let mut step = 0;
    let mut current_idx = 0;

    loop {
        if current_idx < 0 {
            break;
        }

        let idx = usize::try_from(current_idx)?;

        if idx < list_len {
            let next_idx = jump_vec[idx];
            jump_vec[idx] = if crazy_jumps {
                if next_idx > 2 {
                    next_idx - 1
                } else {
                    next_idx + 1
                }
            } else {
                next_idx + 1
            };
            current_idx += next_idx;
        } else {
            break;
        }

        step += 1;
    }

    Ok(step)
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<isize>(AoCYear::AOC2017, AoCDay::AOCD05, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> isize {
    find2_br(reader).map_err(print_err).unwrap_or_default()
}

fn find2_br<T>(reader: T) -> Result<isize>
where
    T: BufRead,
{
    jump(reader, true)
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"0
3
0
1
-3";

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

    const TEST_1: &str = r"0
3
0
1
-3";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find2_br(Cursor::new(TEST_1))?, 10);
        Ok(())
    }
}
