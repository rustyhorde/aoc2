// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 25 "Let It Snow"
//!
//! **--- Day 25: Let It Snow ---**
//!
//! **--- Part 1 ---**
//!
//! Merry Christmas! Santa is booting up his weather machine; looks like you
//! might get a white Christmas after all.
//!
//! The weather machine beeps! On the console of the machine is a copy
//! protection message asking you to enter a code from the instruction manual.
//! Apparently, it refuses to run unless you give it that code. No problem;
//! you'll just look up the code in the--
//!
//! "Ho ho ho", Santa ponders aloud. "I can't seem to find the manual."
//!
//! You look up the support number for the manufacturer and give them a call.
//! Good thing, too - that 49th star wasn't going to earn itself.
//!
//! "Oh, that machine is quite old!", they tell you. "That model went out of
//! support six minutes ago, and we just finished shredding all of the manuals.
//! I bet we can find you the code generation algorithm, though."
//!
//! After putting you on hold for twenty minutes (your call is very important
//! to them, it reminded you repeatedly), they finally find an engineer that
//! remembers how the code system works.
//!
//! The codes are printed on an infinite sheet of paper, starting in the
//! top-left corner. The codes are filled in by diagonals: starting with the first
//! row with an empty first box, the codes are filled in diagonally up and
//! to the right. This process repeats until the infinite paper is covered. So,
//! the first few codes are filled in in this order:
//!
//! ```text
//!    | 1   2   3   4   5   6
//! ---+---+---+---+---+---+---+
//!  1 |  1   3   6  10  15  21
//!  2 |  2   5   9  14  20
//!  3 |  4   8  13  19
//!  4 |  7  12  18
//!  5 | 11  17
//!  6 | 16
//! ```
//!
//! For example, the 12th code would be written to row 4, column 2; the 15th code
//! would be written to row 1, column 5.
//!
//! The voice on the other end of the phone continues with how the codes are
//! actually generated. The first code is `20151125`. After that, each code is generated
//! by taking the previous one, multiplying it by `252533`, and then keeping the
//! remainder from dividing that value by `33554393`.
//!
//! So, to find the second code (which ends up in row 2, column 1), start with the
//! previous value, `20151125`. Multiply it by `252533` to get `5088824049625`.
//! Then, divide that by `33554393`, which leaves a remainder of `31916031`.
//! That remainder is the second code.
//!
//! "Oh!", says the voice. "It looks like we missed a scrap from one of the manuals.
//! Let me read it to you." You write down his numbers:
//!
//!    |    1         2         3         4         5         6
//! ---+---------+---------+---------+---------+---------+---------+
//!  1 | 20151125  18749137  17289845  30943339  10071777  33511524
//!  2 | 31916031  21629792  16929656   7726640  15514188   4041754
//!  3 | 16080970   8057251   1601130   7981243  11661866  16474243
//!  4 | 24592653  32451966  21345942   9380097  10600672  31527494
//!  5 |    77061  17552253  28094349   6899651   9250759  31663883
//!  6 | 33071741   6796745  25397450  24659492   1534922  27995004
//!
//! "Now remember", the voice continues, "that's not even all of the first few numbers;
//! for example, you're missing the one at 7,1 that would come before 6,2. But, it
//! should be enough to let your-- oh, it's time for lunch! Bye!" The call disconnects.
//!
//! Santa looks nervous. Your puzzle input contains the message on the machine's console.
//! What code do you give the machine?
//!
//! **--- Part 2 ---**
//!
//! You fill the weather machine with fifty stars. It comes to life!
//!
//! Snow begins to fall.
//!
//! Congratulations! You've finished every puzzle in Advent of Code 2015! I hope you
//! had as much fun solving them as I had making them for you. I'd love to hear about
//! your adventure; you can get in touch with me via contact info on my website or through Twitter.
//!
//! If you'd like to see more things like this in the future, please consider
//! supporting Advent of Code and sharing it with others.
//!
//! To hear about future projects, you can follow me on Twitter.
//!
//! I've highlighted the easter eggs in each puzzle, just in case you missed any. Hover
//! your mouse over them, and the easter egg will appear.

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{get_cap_x, run_solution, valid_lines},
};
use anyhow::Result;
use regex::Regex;
use std::{
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
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD25, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader).unwrap_or_default()
}

fn find_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let line_re = Regex::new(r"Enter the code at row (\d+), column (\d+)\.$")?;
    let mut final_y = usize::MAX;
    let mut final_x = usize::MAX;

    for line in valid_lines(reader) {
        for caps in line_re.captures_iter(&line) {
            final_y = get_cap_x::<usize>(1, &caps)?;
            final_x = get_cap_x::<usize>(2, &caps)?;
        }
    }

    Ok(calculate(final_x - 1, final_y - 1))
}

fn calculate(final_x: usize, final_y: usize) -> usize {
    let mut x;
    let mut y;
    let mut diagonal = 0;
    let mut current = 20_151_125;

    'outer: loop {
        y = diagonal;
        x = 0;

        loop {
            if x >= final_x && y >= final_y {
                break 'outer;
            }
            current = (current * 252_533) % 33_554_393;
            if y == 0 {
                break;
            }

            y -= 1;
            x += 1;
        }

        diagonal += 1;
    }

    current
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD08, find2).map(|_| 0)
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

    const TEST_1: &str = r"To continue, please consult the code grid in the manual.  Enter the code at row 6, column 6.";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1))?, 27_995_004);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    // use super::find2_br;
    // use std::io::Cursor;

    // const TEST_1: &str = r"turn on 0,0 through 0,0";
    // const TEST_2: &str = r"toggle 0,0 through 999,999";

    #[test]
    fn solution() {
        // assert_eq!(find2_br(Cursor::new(TEST_1))?, 1);
        // assert_eq!(find2_br(Cursor::new(TEST_2))?, 2_000_000);
    }
}
