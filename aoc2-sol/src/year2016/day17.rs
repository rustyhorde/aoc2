// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 17 "Two Steps Forward"
//!
//! **--- Day 17: Two Steps Forward ---**
//!
//! **--- Part 1 ---**
//!
//! You're trying to access a secure vault protected by a `4x4` grid of small
//! rooms connected by doors. You start in the top-left room (marked S), and
//! you can access the vault (marked V) once you reach the bottom-right room:
//!
//! #########
//! #S| | | #
//! #-#-#-#-#
//! # | | | #
//! #-#-#-#-#
//! # | | | #
//! #-#-#-#-#
//! # | | |
//! ####### V
//!
//! Fixed walls are marked with #, and doors are marked with - or |.
//!
//! The doors in your current room are either open or closed (and locked) based
//! on the hexadecimal MD5 hash of a passcode (your puzzle input) followed by a
//! sequence of uppercase characters representing the path you have taken so far
//! (U for up, D for down, L for left, and R for right).
//!
//! Only the first four characters of the hash are used; they represent, respectively,
//! the doors up, down, left, and right from your current position. Any `b`, `c`,
//! `d`, `e`, or `f` means that the corresponding door is open; any other character
//! (any number or `a`) means that the corresponding door is closed and locked.
//!
//! To access the vault, all you need to do is reach the bottom-right room; reaching
//! this room opens the vault and all doors in the maze.
//!
//! For example, suppose the passcode is `hijkl`. Initially, you have taken no steps,
//! and so your path is empty: you simply find the MD5 hash of `hijkl` alone. The first
//! four characters of this hash are `ced9`, which indicate that up is open (c), down
//! is open (e), left is open (d), and right is closed and locked (9). Because you
//! start in the top-left corner, there are no "up" or "left" doors to be open, so your
//! only choice is down.
//!
//! Next, having gone only one step (down, or D), you find the hash of `hijklD`. This
//! produces `f2bc`, which indicates that you can go back up, left (but that's a wall),
//! or right. Going right means hashing `hijklDR` to get `5745` - all doors closed and
//! locked. However, going up instead is worthwhile: even though it returns you to the
//! room you started in, your path would then be `DU`, opening a different set of doors.
//!
//! After going `DU` (and then hashing `hijklDU` to get `528e`), only the right door
//! is open; after going `DUR`, all doors lock. (Fortunately, your actual passcode is not
//! `hijkl`).
//!
//! Passcodes actually used by Easter Bunny Vault Security do allow access to the vault
//! if you know the right path. For example:
//!
//! ```text
//! If your passcode were ihgpwlah, the shortest path would be DDRRRD.
//! With kglvqrro, the shortest path would be DDUDRLRRUDRD.
//! With ulqzkmiv, the shortest would be DRURDRUDDLLDLUURRDULRLDUUDDDRR.
//! ```
//!
//! Given your vault's passcode, what is the shortest path (the actual path, not just
//! the length) to reach the vault?
//!
//! **--- Part Two ---**
//!
//! You're curious how robust this security solution really is, and so you decide to
//! find longer and longer paths which still provide access to the vault. You remember
//! that paths always end the first time they reach the bottom-right room (that is, they
//! can never pass through it, only end in it).
//!
//! For example:
//!
//! ```text
//! If your passcode were ihgpwlah, the longest path would take 370 steps.
//! With kglvqrro, the longest path would be 492 steps long.
//! With ulqzkmiv, the longest path would be 830 steps long.
//! ```
//!
//! What is the length of the longest path that reaches the vault?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{print_err, run_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use md5::{Digest, Md5};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct State {
    base: String,
    path: String,
    x: usize,
    y: usize,
}

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_solution::<String>(AoCYear::AOC2016, AoCDay::AOCD17, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> String {
    find_br(reader).map_err(print_err).unwrap_or_default()
}

fn find_br<T>(reader: T) -> Result<String>
where
    T: BufRead,
{
    find_path(reader, false)
}

fn find_path<T>(reader: T, part2: bool) -> Result<String>
where
    T: BufRead,
{
    let mut base = String::new();
    for line in valid_lines(reader) {
        base = line;
    }
    let state = State {
        base,
        ..State::default()
    };
    let mut results = vec![];
    search(&state, &mut results, part2);
    if part2 {
        results
            .into_iter()
            .max_by(|x, y| x.len().cmp(&y.len()))
            .ok_or_else(|| anyhow!("invalid longest"))
    } else {
        results
            .into_iter()
            .min_by(|x, y| x.len().cmp(&y.len()))
            .ok_or_else(|| anyhow!("invalid shortest"))
    }
}

fn search(state: &State, results: &mut Vec<String>, part2: bool) {
    let mut directions = directions(state);
    valid_from_pos(&mut directions, state);
    'outer: for direction in directions {
        let mut state = state.clone();
        walk(&mut state, direction);

        if !part2 {
            for result in results.iter() {
                if result.len() < state.path.len() {
                    continue 'outer;
                }
            }
        }
        if state.x == 3 && state.y == 3 {
            results.push(state.path);
        } else {
            search(&state, results, part2);
        }
    }
}

fn walk(state: &mut State, direction: Direction) {
    match direction {
        Direction::Up => {
            state.y -= 1;
            state.path.push('U');
        }
        Direction::Down => {
            state.y += 1;
            state.path.push('D');
        }
        Direction::Left => {
            state.x -= 1;
            state.path.push('L');
        }
        Direction::Right => {
            state.x += 1;
            state.path.push('R');
        }
    }
}

fn valid_from_pos(dirs: &mut Vec<Direction>, state: &State) {
    let x = state.x;
    let y = state.y;
    if x == 0 {
        dirs.retain(|dir| *dir != Direction::Left);
    } else if x == 3 {
        dirs.retain(|dir| *dir != Direction::Right);
    }

    if y == 0 {
        dirs.retain(|dir| *dir != Direction::Up);
    } else if y == 3 {
        dirs.retain(|dir| *dir != Direction::Down);
    }
}

fn directions(state: &State) -> Vec<Direction> {
    let mut open_doors = vec![];
    let mut md5 = Md5::new();
    md5.update(format!("{}{}", state.base, state.path));
    let digest = md5.finalize_reset();
    let hash = format!("{digest:x}");
    let hash_dirs_str = &hash[0..4];
    for (idx, dir) in hash_dirs_str.chars().enumerate() {
        match dir {
            'b' | 'c' | 'd' | 'e' | 'f' => {
                if idx == 0 {
                    open_doors.push(Direction::Up);
                } else if idx == 1 {
                    open_doors.push(Direction::Down);
                } else if idx == 2 {
                    open_doors.push(Direction::Left);
                } else if idx == 3 {
                    open_doors.push(Direction::Right);
                }
            }
            _ => {}
        }
    }
    open_doors
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2016, AoCDay::AOCD17, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader).map_err(print_err).unwrap_or_default()
}

fn find2_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    find_path(reader, true).map(|x| x.len())
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"ihgpwlah";
    const TEST_2: &str = r"kglvqrro";
    const TEST_3: &str = r"ulqzkmiv";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1))?, "DDRRRD");
        assert_eq!(find_br(Cursor::new(TEST_2))?, "DDUDRLRRUDRD");
        assert_eq!(
            find_br(Cursor::new(TEST_3))?,
            "DRURDRUDDLLDLUURRDULRLDUUDDDRR"
        );
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"ihgpwlah";
    const TEST_2: &str = r"kglvqrro";
    const TEST_3: &str = r"ulqzkmiv";

    #[test]
    #[ignore]
    fn solution() -> Result<()> {
        assert_eq!(find2_br(Cursor::new(TEST_1))?, 370);
        assert_eq!(find2_br(Cursor::new(TEST_2))?, 492);
        assert_eq!(find2_br(Cursor::new(TEST_3))?, 830);
        Ok(())
    }
}
