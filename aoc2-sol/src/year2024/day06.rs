// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code - Day 6 ---**
//!
//! **--- Day 6: Guard Gallivant ---**
//!
//! The Historians use their fancy device again, this time to whisk you all away to the North Pole prototype suit manufacturing lab... in the year 1518! It turns out that having direct access to history is very convenient for a group of historians.
//!
//! You still have to be careful of time paradoxes, and so it will be important to avoid anyone from 1518 while The Historians search for the Chief. Unfortunately, a single guard is patrolling this part of the lab.
//!
//! Maybe you can work out where the guard will go ahead of time so that The Historians can search safely?
//!
//! You start by making a map (your puzzle input) of the situation. For example:
//!
//! ```text
//! ....#.....
//! .........#
//! ..........
//! ..#.......
//! .......#..
//! ..........
//! .#..^.....
//! ........#.
//! #.........
//! ......#...
//! ```
//!
//! The map shows the current position of the guard with ^ (to indicate the guard is currently facing up from the perspective of the map). Any obstructions - crates, desks, alchemical reactors, etc. - are shown as #.
//!
//! Lab guards in 1518 follow a very strict patrol protocol which involves repeatedly following these steps:
//!  
//! ```text
//!     If there is something directly in front of you, turn right 90 degrees.
//!     Otherwise, take a step forward.
//! ```
//!
//! Following the above protocol, the guard moves up several times until she reaches an obstacle (in this case, a pile of failed suit prototypes):
//!
//! ```text
//! ....#.....
//! ....^....#
//! ..........
//! ..#.......
//! .......#..
//! ..........
//! .#........
//! ........#.
//! #.........
//! ......#...
//! ```
//!
//! Because there is now an obstacle in front of the guard, she turns right before continuing straight in her new facing direction:
//!
//! ```text
//! ....#.....
//! ........>#
//! ..........
//! ..#.......
//! .......#..
//! ..........
//! .#........
//! ........#.
//! #.........
//! ......#...
//! ```
//!
//! Reaching another obstacle (a spool of several very long polymers), she turns right again and continues downward:
//!
//! ```text
//! ....#.....
//! .........#
//! ..........
//! ..#.......
//! .......#..
//! ..........
//! .#......v.
//! ........#.
//! #.........
//! ......#...
//! ```
//!
//! This process continues for a while, but the guard eventually leaves the mapped area (after walking past a tank of universal solvent):
//!
//! ```text
//! ....#.....
//! .........#
//! ..........
//! ..#.......
//! .......#..
//! ..........
//! .#........
//! ........#.
//! #.........
//! ......#v..
//! ```
//!
//! By predicting the guard's route, you can determine which specific positions in the lab will be in the patrol path. Including the guard's starting position, the positions visited by the guard before leaving the area are marked with an X:
//!
//! ```text
//! ....#.....
//! ....XXXXX#
//! ....X...X.
//! ..#.X...X.
//! ..XXXXX#X.
//! ..X.X.X.X.
//! .#XXXXXXX.
//! .XXXXXXX#.
//! #XXXXXXX..
//! ......#X..
//! ```
//!
//! In this example, the guard will visit 41 distinct positions on your map.
//!
//! Predict the path of the guard. How many distinct positions will the guard visit before leaving the mapped area?
//!
//! **--- Part Two ---**
//!
//! While The Historians begin working around the guard's patrol route, you borrow their fancy device and step outside the lab. From the safety of a supply closet, you time travel through the last few months and record the nightly status of the lab's guard post on the walls of the closet.
//!
//! Returning after what seems like only a few seconds to The Historians, they explain that the guard's patrol area is simply too large for them to safely search the lab without getting caught.
//!
//! Fortunately, they are pretty sure that adding a single new obstruction won't cause a time paradox. They'd like to place the new obstruction in such a way that the guard will get stuck in a loop, making the rest of the lab safe to search.
//!
//! To have the lowest chance of creating a time paradox, The Historians would like to know all of the possible positions for such an obstruction. The new obstruction can't be placed at the guard's starting position - the guard is there right now and would notice.
//!
//! In the above example, there are only 6 different positions where a new obstruction would cause the guard to get stuck in a loop. The diagrams of these six situations use O to mark the new obstruction, | to show a position where the guard moves up/down, - to show a position where the guard moves left/right, and + to show a position where the guard moves both up/down and left/right.
//!
//! Option one, put a printing press next to the guard's starting position:
//!
//! ```text
//! ....#.....
//! ....+---+#
//! ....|...|.
//! ..#.|...|.
//! ....|..#|.
//! ....|...|.
//! .#.O^---+.
//! ........#.
//! #.........
//! ......#...
//! ```
//!
//! Option two, put a stack of failed suit prototypes in the bottom right quadrant of the mapped area:
//!
//! ```text
//! ....#.....
//! ....+---+#
//! ....|...|.
//! ..#.|...|.
//! ..+-+-+#|.
//! ..|.|.|.|.
//! .#+-^-+-+.
//! ......O.#.
//! #.........
//! ......#...
//! ```
//!
//! Option three, put a crate of chimney-squeeze prototype fabric next to the standing desk in the bottom right quadrant:
//!
//! ```text
//! ....#.....
//! ....+---+#
//! ....|...|.
//! ..#.|...|.
//! ..+-+-+#|.
//! ..|.|.|.|.
//! .#+-^-+-+.
//! .+----+O#.
//! #+----+...
//! ......#...
//! ```
//!
//! Option four, put an alchemical retroencabulator near the bottom left corner:
//!
//! ```text
//! ....#.....
//! ....+---+#
//! ....|...|.
//! ..#.|...|.
//! ..+-+-+#|.
//! ..|.|.|.|.
//! .#+-^-+-+.
//! ..|...|.#.
//! #O+---+...
//! ......#...
//! ```
//!
//! Option five, put the alchemical retroencabulator a bit to the right instead:
//!
//! ```text
//! ....#.....
//! ....+---+#
//! ....|...|.
//! ..#.|...|.
//! ..+-+-+#|.
//! ..|.|.|.|.
//! .#+-^-+-+.
//! ....|.|.#.
//! #..O+-+...
//! ......#...
//! ```
//!
//! Option six, put a tank of sovereign glue right next to the tank of universal solvent:
//!
//! ```text
//! ....#.....
//! ....+---+#
//! ....|...|.
//! ..#.|...|.
//! ..+-+-+#|.
//! ..|.|.|.|.
//! .#+-^-+-+.
//! .+----++#.
//! #+----++..
//! ......#O..
//! ```
//! It doesn't really matter what you choose to use as an obstacle so long as you and The Historians can put it into position without the guard noticing. The important thing is having enough options that you can find one that minimizes time paradoxes, and in this example, there are 6 different positions you could choose.
//!
//! You need to get the guard stuck in a loop by adding a single new obstruction. How many different positions could you choose for this obstruction?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_solution, valid_lines},
};
use anyhow::Result;
use core::fmt;
use indexmap::IndexMap;
use std::{
    collections::HashSet,
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
    run_solution::<usize>(AoCYear::AOC2024, AoCDay::AOCD06, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader)
}

fn find_br<T>(reader: T) -> usize
where
    T: BufRead,
{
    let mut matrix = vec![];
    for line in valid_lines(reader) {
        let cols = line.chars().collect::<Vec<char>>();
        matrix.push(cols);
    }

    let (mut curr_pos, mut curr_dir) = curr_dir_and_pos(&matrix);
    try_move(&matrix, &mut curr_pos, &mut curr_dir)
}

fn try_move(matrix: &[Vec<char>], curr_pos: &mut (usize, usize), curr_dir: &mut Dir) -> usize {
    let mut visited = HashSet::new();
    let _old = visited.insert(*curr_pos);

    while let Some((next_row, next_col)) = next_pos(curr_pos, *curr_dir) {
        if let Some(next) = matrix.get(next_row).and_then(|v| v.get(next_col)) {
            if next == &'#' {
                change_dir(curr_dir);
            } else {
                *curr_pos = (next_row, next_col);
                let _old = visited.insert(*curr_pos);
            }
        } else {
            break;
        }
    }
    visited.len()
}

fn curr_dir_and_pos(matrix: &[Vec<char>]) -> ((usize, usize), Dir) {
    let mut curr_dir = Dir::North;
    let mut curr_pos = (0, 0);
    for (row, columns) in matrix.iter().enumerate() {
        for col in 0..columns.len() {
            let curr_char = matrix[row][col];
            if curr_char == '^' {
                curr_pos = (row, col);
                curr_dir = Dir::North;
            } else if curr_char == '>' {
                curr_pos = (row, col);
                curr_dir = Dir::East;
            } else if curr_char == 'v' {
                curr_pos = (row, col);
                curr_dir = Dir::South;
            } else if curr_char == '<' {
                curr_pos = (row, col);
                curr_dir = Dir::West;
            }
        }
    }
    (curr_pos, curr_dir)
}

fn next_pos(curr_pos: &(usize, usize), curr_dir: Dir) -> Option<(usize, usize)> {
    match curr_dir {
        Dir::North => {
            let row_idx = curr_pos.0.checked_sub(1);
            let col_idx = Some(curr_pos.1);
            row_idx.zip(col_idx)
        }
        Dir::East => {
            let row_idx = Some(curr_pos.0);
            let col_idx = Some(curr_pos.1 + 1);
            row_idx.zip(col_idx)
        }
        Dir::South => {
            let row_idx = Some(curr_pos.0 + 1);
            let col_idx = Some(curr_pos.1);
            row_idx.zip(col_idx)
        }
        Dir::West => {
            let row_idx = Some(curr_pos.0);
            let col_idx = curr_pos.1.checked_sub(1);
            row_idx.zip(col_idx)
        }
    }
}

fn change_dir(dir: &mut Dir) {
    match dir {
        Dir::North => *dir = Dir::East,
        Dir::East => *dir = Dir::South,
        Dir::South => *dir = Dir::West,
        Dir::West => *dir = Dir::North,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl fmt::Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Dir::North => "North",
                Dir::East => "East",
                Dir::South => "South",
                Dir::West => "West",
            }
        )
    }
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2024, AoCDay::AOCD06, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader)
}

fn find2_br<T>(reader: T) -> usize
where
    T: BufRead,
{
    let mut matrix = vec![];
    for line in valid_lines(reader) {
        let cols = line.chars().collect::<Vec<char>>();
        matrix.push(cols);
    }

    let (curr_pos, curr_dir) = curr_dir_and_pos(&matrix);
    let (a_loop, _visited) = try_move_part_2(&matrix, &curr_pos, curr_dir);

    let mut loop_count = 0;
    if a_loop {
        loop_count += 1;
    } else {
        let matricies = find_new_blocks(&matrix);
        for matrix in &matricies {
            let (curr_pos, curr_dir) = curr_dir_and_pos(matrix);
            let (loopy, _) = try_move_part_2(matrix, &curr_pos, curr_dir);
            if loopy {
                loop_count += 1;
            }
        }
    }
    loop_count
}

fn try_move_part_2(
    matrix: &[Vec<char>],
    initial_pos: &(usize, usize),
    initial_dir: Dir,
) -> (bool, IndexMap<(usize, usize), Vec<Dir>>) {
    let mut visited = IndexMap::new();
    let mut curr_pos = *initial_pos;
    let mut curr_dir = initial_dir;
    let _old = visited.insert(curr_pos, vec![initial_dir]);
    let mut loopy = false;

    while let Some((next_row, next_col)) = next_pos(&curr_pos, curr_dir) {
        if let Some(visits) = visited.get(&(next_row, next_col)) {
            if visits.contains(&curr_dir) {
                loopy = true;
                visited.clear();
                break;
            }
        }
        if let Some(next) = matrix.get(next_row).and_then(|v| v.get(next_col)) {
            if next == &'#' {
                change_dir(&mut curr_dir);
            } else {
                let _ = visited
                    .entry((next_row, next_col))
                    .and_modify(|dirs| dirs.push(curr_dir))
                    .or_insert_with(|| vec![curr_dir]);
                curr_pos = (next_row, next_col);
            }
        } else {
            break;
        }
    }

    (loopy, visited)
}

fn find_new_blocks(matrix: &[Vec<char>]) -> Vec<Vec<Vec<char>>> {
    let mut new_matricies = vec![];
    for row in 0..matrix.len() {
        for col in 0..matrix.len() {
            let mut new_matrix = Vec::from(matrix);
            let spot = matrix[row][col];

            if spot == '.' {
                new_matrix[row][col] = '#';
            }

            new_matricies.push(new_matrix);
        }
    }
    new_matricies
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use std::io::Cursor;

    const TEST_1: &str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn solution() {
        assert_eq!(find_br(Cursor::new(TEST_1)), 41);
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use std::io::Cursor;

    const TEST_1: &str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    const TEST_2: &str = r".#..
.^.#
#...
..#.";
    const TEST_3: &str = r"...>#
.#...
....#
#....
...#.";
    const TEST_4: &str = r"...#.
>...#
.....
..#..
...#.";

    #[test]
    fn solution() {
        assert_eq!(find2_br(Cursor::new(TEST_1)), 6);
        assert_eq!(find2_br(Cursor::new(TEST_2)), 1);
        assert_eq!(find2_br(Cursor::new(TEST_3)), 1);
        assert_eq!(find2_br(Cursor::new(TEST_4)), 1);
    }
}
