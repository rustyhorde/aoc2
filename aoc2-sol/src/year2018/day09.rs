// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code ---**
//!
//! **--- Day 9: Marble Mania ---**
//!
//! You talk to the Elves while you wait for your navigation system to initialize. To pass the time, they introduce you to their favorite marble game.
//!
//! The Elves play this game by taking turns arranging the marbles in a circle according to very particular rules. The marbles are numbered starting with 0 and increasing by 1 until every marble has a number.
//!
//! First, the marble numbered 0 is placed in the circle. At this point, while it contains only a single marble, it is still a circle: the marble is both clockwise from itself and counter-clockwise from itself. This marble is designated the current marble.
//!
//! Then, each Elf takes a turn placing the lowest-numbered remaining marble into the circle between the marbles that are 1 and 2 marbles clockwise of the current marble. (When the circle is large enough, this means that there is one marble between the marble that was just placed and the current marble.) The marble that was just placed then becomes the current marble.
//!
//! However, if the marble that is about to be placed has a number which is a multiple of 23, something entirely different happens. First, the current player keeps the marble they would have placed, adding it to their score. In addition, the marble 7 marbles counter-clockwise from the current marble is removed from the circle and also added to the current player's score. The marble located immediately clockwise of the marble that was removed becomes the new current marble.
//!
//! For example, suppose there are 9 players. After the marble with value 0 is placed in the middle, each player (shown in square brackets) takes a turn. The result of each of those turns would produce circles of marbles like this, where clockwise is to the right and the resulting current marble is in parentheses:
//!
//! ```text
//! [-] (0)
//! [1]  0 (1)
//! [2]  0 (2) 1
//! [3]  0  2  1 (3)
//! [4]  0 (4) 2  1  3
//! [5]  0  4  2 (5) 1  3
//! [6]  0  4  2  5  1 (6) 3
//! [7]  0  4  2  5  1  6  3 (7)
//! [8]  0 (8) 4  2  5  1  6  3  7
//! [9]  0  8  4 (9) 2  5  1  6  3  7
//! [1]  0  8  4  9  2(10) 5  1  6  3  7
//! [2]  0  8  4  9  2 10  5(11) 1  6  3  7
//! [3]  0  8  4  9  2 10  5 11  1(12) 6  3  7
//! [4]  0  8  4  9  2 10  5 11  1 12  6(13) 3  7
//! [5]  0  8  4  9  2 10  5 11  1 12  6 13  3(14) 7
//! [6]  0  8  4  9  2 10  5 11  1 12  6 13  3 14  7(15)
//! [7]  0(16) 8  4  9  2 10  5 11  1 12  6 13  3 14  7 15
//! [8]  0 16  8(17) 4  9  2 10  5 11  1 12  6 13  3 14  7 15
//! [9]  0 16  8 17  4(18) 9  2 10  5 11  1 12  6 13  3 14  7 15
//! [1]  0 16  8 17  4 18  9(19) 2 10  5 11  1 12  6 13  3 14  7 15
//! [2]  0 16  8 17  4 18  9 19  2(20)10  5 11  1 12  6 13  3 14  7 15
//! [3]  0 16  8 17  4 18  9 19  2 20 10(21) 5 11  1 12  6 13  3 14  7 15
//! [4]  0 16  8 17  4 18  9 19  2 20 10 21  5(22)11  1 12  6 13  3 14  7 15
//! [5]  0 16  8 17  4 18(19) 2 20 10 21  5 22 11  1 12  6 13  3 14  7 15
//! [6]  0 16  8 17  4 18 19  2(24)20 10 21  5 22 11  1 12  6 13  3 14  7 15
//! [7]  0 16  8 17  4 18 19  2 24 20(25)10 21  5 22 11  1 12  6 13  3 14  7 15
//! ```
//!
//! The goal is to be the player with the highest score after the last marble is used up. Assuming the example above ends after the marble numbered 25, the winning score is 23+9=32 (because player 5 kept marble 23 and removed marble 9, while no other player got any points in this very short example game).
//!
//! Here are a few more examples:
//!
//! ```text
//!     10 players; last marble is worth 1618 points: high score is 8317
//!     13 players; last marble is worth 7999 points: high score is 146373
//!     17 players; last marble is worth 1104 points: high score is 2764
//!     21 players; last marble is worth 6111 points: high score is 54718
//!     30 players; last marble is worth 5807 points: high score is 37305
//! ```
//!
//! What is the winning Elf's score?
//!
//! **--- Part Two ---**
//!
//! Amused by the speed of your answer, the Elves are curious:
//!
//! What would the new winning Elf's score be if the number of the last marble were 100 times larger?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use regex::Regex;
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
    run_setup_solution::<(usize, usize), usize>(AoCYear::AOC2018, AoCDay::AOCD09, setup, find)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<(usize, usize), usize>(
        bench,
        AoCYear::AOC2018,
        AoCDay::AOCD09,
        setup,
        find,
    )
    .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> (usize, usize) {
    setup_br(reader).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T) -> Result<(usize, usize)>
where
    T: BufRead,
{
    let line_re = Regex::new(r"(\d+) players; last marble is worth (\d+) points")?;
    let mut players = 0;
    let mut final_marble = 0;

    for line in valid_lines(reader) {
        for cap in line_re.captures_iter(&line) {
            players = (cap[1]).parse::<usize>()?;
            final_marble = (cap[2]).parse::<usize>()?;
        }
    }
    Ok((players, final_marble))
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: (usize, usize)) -> usize {
    find_res(data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(data: (usize, usize), second_star: bool) -> Result<usize> {
    let (players, mut final_marble) = data;

    if second_star {
        final_marble *= 100;
    }

    play_game(players, final_marble)
}

fn play_game(players: usize, final_marble: usize) -> Result<usize> {
    let mut scores = vec![0; players];
    let mut circle = VecDeque::new();
    circle.push_front(0);

    for marble in 1..=final_marble {
        if marble % 23 == 0 {
            rotate_left(&mut circle, 7)?;
            scores[marble % players] += marble + circle.pop_front().ok_or(anyhow!("no front"))?;
        } else {
            rotate_right(&mut circle, 2)?;
            circle.push_front(marble);
        }
    }

    Ok(*scores.iter().max().ok_or(anyhow!("no max"))?)
}

fn rotate_left(circle: &mut VecDeque<usize>, amt: usize) -> Result<()> {
    for _ in 0..amt {
        let tmp = circle.pop_back().ok_or(anyhow!("rotate left err"))?;
        circle.push_front(tmp);
    }
    Ok(())
}

fn rotate_right(circle: &mut VecDeque<usize>, amt: usize) -> Result<()> {
    for _ in 0..amt {
        let tmp = circle.pop_front().ok_or(anyhow!("rotate right err"))?;
        circle.push_back(tmp);
    }
    Ok(())
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<(usize, usize), usize>(AoCYear::AOC2018, AoCDay::AOCD09, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<(usize, usize), usize>(
        bench,
        AoCYear::AOC2018,
        AoCDay::AOCD09,
        setup,
        find2,
    )
    .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: (usize, usize)) -> usize {
    find_res(data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"9 players; last marble is worth 25 points";
    const TEST_2: &str = r"10 players; last marble is worth 1618 points";
    const TEST_3: &str = r"13 players; last marble is worth 7999 points";
    const TEST_4: &str = r"17 players; last marble is worth 1104 points";
    const TEST_5: &str = r"21 players; last marble is worth 6111 points";
    const TEST_6: &str = r"30 players; last marble is worth 5807 points";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find(data), 32);
        let data = setup_br(Cursor::new(TEST_2))?;
        assert_eq!(find(data), 8317);
        let data = setup_br(Cursor::new(TEST_3))?;
        assert_eq!(find(data), 146_373);
        let data = setup_br(Cursor::new(TEST_4))?;
        assert_eq!(find(data), 2764);
        let data = setup_br(Cursor::new(TEST_5))?;
        assert_eq!(find(data), 54718);
        let data = setup_br(Cursor::new(TEST_6))?;
        assert_eq!(find(data), 37305);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"9 players; last marble is worth 25 points";
    const TEST_2: &str = r"10 players; last marble is worth 1618 points";
    const TEST_3: &str = r"13 players; last marble is worth 7999 points";
    const TEST_4: &str = r"17 players; last marble is worth 1104 points";
    const TEST_5: &str = r"21 players; last marble is worth 6111 points";
    const TEST_6: &str = r"30 players; last marble is worth 5807 points";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find2(data), 22563);
        let data = setup_br(Cursor::new(TEST_2))?;
        assert_eq!(find2(data), 74_765_078);
        let data = setup_br(Cursor::new(TEST_3))?;
        assert_eq!(find2(data), 1_406_506_154);
        let data = setup_br(Cursor::new(TEST_4))?;
        assert_eq!(find2(data), 20_548_882);
        let data = setup_br(Cursor::new(TEST_5))?;
        assert_eq!(find2(data), 507_583_214);
        let data = setup_br(Cursor::new(TEST_6))?;
        assert_eq!(find2(data), 320_997_431);
        Ok(())
    }
}
