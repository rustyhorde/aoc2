// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code - Day 4 ---**
//!
//! **--- Day 4: Ceres Search ---**
//!
//! "Looks like the Chief's not here. Next!" One of The Historians pulls out a device and pushes the only button on it. After a brief flash, you recognize the interior of the Ceres monitoring station!
//!
//! As the search for the Chief continues, a small Elf who lives on the station tugs on your shirt; she'd like to know if you could help her with her word search (your puzzle input). She only has to find one word: XMAS.
//!
//! This word search allows words to be horizontal, vertical, diagonal, written backwards, or even overlapping other words. It's a little unusual, though, as you don't merely need to find one instance of XMAS - you need to find all of them. Here are a few ways XMAS might appear, where irrelevant characters have been replaced with .:
//!
//! ```text
//! ..X...
//! .SAMX.
//! .A..A.
//! XMAS.S
//! .X....
//! ```
//!
//! The actual word search will be full of letters instead. For example:
//!
//! ```text
//! MMMSXXMASM
//! MSAMXMSMSA
//! AMXSXMAAMM
//! MSAMASMSMX
//! XMASAMXAMM
//! XXAMMXXAMA
//! SMSMSASXSS
//! SAXAMASAAA
//! MAMMMXMMMM
//! MXMXAXMASX
//! ```
//!
//! In this word search, XMAS occurs a total of 18 times; here's the same word search again, but where letters not involved in any XMAS have been replaced with .:
//!
//! ```text
//! ....XXMAS.
//! .SAMXMS...
//! ...S..A...
//! ..A.A.MS.X
//! XMASAMX.MM
//! X.....XA.A
//! S.S.S.S.SS
//! .A.A.A.A.A
//! ..M.M.M.MM
//! .X.X.XMASX
//! ```
//!
//! Take a look at the little Elf's word search. How many times does XMAS appear?
//!
//! **--- Part Two ---**
//!
//! The Elf looks quizzically at you. Did you misunderstand the assignment?
//!
//! Looking for the instructions, you flip over the word search to find that this isn't actually an XMAS puzzle; it's an X-MAS puzzle in which you're supposed to find two MAS in the shape of an X. One way to achieve that is like this:
//!
//! ```text
//! M.S
//! .A.
//! M.S
//! ```
//!
//! Irrelevant characters have again been replaced with . in the above diagram. Within the X, each MAS can be written forwards or backwards.
//!
//! Here's the same example from before, but this time all of the X-MASes have been kept instead:
//!
//! ```text
//! .M.S......
//! ..A..MSMS.
//! .M.S.MAA..
//! ..A.ASMSM.
//! .M.S.M....
//! ..........
//! S.S.S.S.S.
//! .A.A.A.A..
//! M.M.M.M.M.
//! ..........
//! ```
//!
//! In this example, an X-MAS appears 9 times.
//!
//! Flip the word search from the instructions back over to the word search side and try again. How many times does an X-MAS appear?

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
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2024, AoCDay::AOCD04, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader)
}

fn find_br<T>(reader: T) -> usize
where
    T: BufRead,
{
    let mut matrix = vec![];
    let mut count = 0;
    for line in valid_lines(reader) {
        let chars = line.chars().collect::<Vec<char>>();
        matrix.push(chars);
    }

    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            if matrix[row][col] == 'X' {
                if find_right(&matrix, row, col) {
                    count += 1;
                }
                if find_left(&matrix, row, col) {
                    count += 1;
                }
                if find_up(&matrix, row, col) {
                    count += 1;
                }
                if find_down(&matrix, row, col) {
                    count += 1;
                }
                if find_down_right(&matrix, row, col) {
                    count += 1;
                }
                if find_down_left(&matrix, row, col) {
                    count += 1;
                }
                if find_up_right(&matrix, row, col) {
                    count += 1;
                }
                if find_up_left(&matrix, row, col) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn is_mas(m_opt: Option<&char>, a_opt: Option<&char>, s_opt: Option<&char>) -> bool {
    let mut is_mas = false;
    if let Some(blah) = m_opt.zip(a_opt).zip(s_opt) {
        if blah == ((&'M', &'A'), &'S') {
            is_mas = true;
        }
    }
    is_mas
}

fn find_right(matrix: &[Vec<char>], row: usize, col: usize) -> bool {
    let m_opt = matrix.get(row).and_then(|v| v.get(col + 1));
    let a_opt = matrix.get(row).and_then(|v| v.get(col + 2));
    let s_opt = matrix.get(row).and_then(|v| v.get(col + 3));

    is_mas(m_opt, a_opt, s_opt)
}

fn find_down(matrix: &[Vec<char>], row: usize, col: usize) -> bool {
    let m_opt = matrix.get(row + 1).and_then(|v| v.get(col));
    let a_opt = matrix.get(row + 2).and_then(|v| v.get(col));
    let s_opt = matrix.get(row + 3).and_then(|v| v.get(col));

    is_mas(m_opt, a_opt, s_opt)
}

fn find_down_right(matrix: &[Vec<char>], row: usize, col: usize) -> bool {
    let m_opt = matrix.get(row + 1).and_then(|v| v.get(col + 1));
    let a_opt = matrix.get(row + 2).and_then(|v| v.get(col + 2));
    let s_opt = matrix.get(row + 3).and_then(|v| v.get(col + 3));

    is_mas(m_opt, a_opt, s_opt)
}

fn find_down_left(matrix: &[Vec<char>], row: usize, col: usize) -> bool {
    let idx_1 = col.checked_sub(1);
    let idx_2 = col.checked_sub(2);
    let idx_3 = col.checked_sub(3);

    idx_1
        .zip(idx_2)
        .zip(idx_3)
        .is_some_and(|((idx_1, idx_2), idx_3)| {
            let m_opt = matrix.get(row + 1).and_then(|v| v.get(idx_1));
            let a_opt = matrix.get(row + 2).and_then(|v| v.get(idx_2));
            let s_opt = matrix.get(row + 3).and_then(|v| v.get(idx_3));

            is_mas(m_opt, a_opt, s_opt)
        })
}

fn find_left(matrix: &[Vec<char>], row: usize, col: usize) -> bool {
    let idx_1 = col.checked_sub(1);
    let idx_2 = col.checked_sub(2);
    let idx_3 = col.checked_sub(3);

    idx_1
        .zip(idx_2)
        .zip(idx_3)
        .is_some_and(|((idx_1, idx_2), idx_3)| {
            let m_opt = matrix.get(row).and_then(|v| v.get(idx_1));
            let a_opt = matrix.get(row).and_then(|v| v.get(idx_2));
            let s_opt = matrix.get(row).and_then(|v| v.get(idx_3));

            is_mas(m_opt, a_opt, s_opt)
        })
}

fn find_up(matrix: &[Vec<char>], row: usize, col: usize) -> bool {
    let idx_1 = row.checked_sub(1);
    let idx_2 = row.checked_sub(2);
    let idx_3 = row.checked_sub(3);

    idx_1
        .zip(idx_2)
        .zip(idx_3)
        .is_some_and(|((idx_1, idx_2), idx_3)| {
            let m_opt = matrix.get(idx_1).and_then(|v| v.get(col));
            let a_opt = matrix.get(idx_2).and_then(|v| v.get(col));
            let s_opt = matrix.get(idx_3).and_then(|v| v.get(col));

            is_mas(m_opt, a_opt, s_opt)
        })
}

fn find_up_right(matrix: &[Vec<char>], row: usize, col: usize) -> bool {
    let idx_1 = row.checked_sub(1);
    let idx_2 = row.checked_sub(2);
    let idx_3 = row.checked_sub(3);

    idx_1
        .zip(idx_2)
        .zip(idx_3)
        .is_some_and(|((idx_1, idx_2), idx_3)| {
            let m_opt = matrix.get(idx_1).and_then(|v| v.get(col + 1));
            let a_opt = matrix.get(idx_2).and_then(|v| v.get(col + 2));
            let s_opt = matrix.get(idx_3).and_then(|v| v.get(col + 3));

            is_mas(m_opt, a_opt, s_opt)
        })
}

fn find_up_left(matrix: &[Vec<char>], row: usize, col: usize) -> bool {
    let idx_1 = row.checked_sub(1);
    let idx_2 = row.checked_sub(2);
    let idx_3 = row.checked_sub(3);

    idx_1
        .zip(idx_2)
        .zip(idx_3)
        .is_some_and(|((idx_1, idx_2), idx_3)| {
            let col_idx_1 = col.checked_sub(1);
            let col_idx_2 = col.checked_sub(2);
            let col_idx_3 = col.checked_sub(3);
            col_idx_1.zip(col_idx_2).zip(col_idx_3).is_some_and(
                |((col_idx_1, col_idx_2), col_idx_3)| {
                    let m_opt = matrix.get(idx_1).and_then(|v| v.get(col_idx_1));
                    let a_opt = matrix.get(idx_2).and_then(|v| v.get(col_idx_2));
                    let s_opt = matrix.get(idx_3).and_then(|v| v.get(col_idx_3));

                    is_mas(m_opt, a_opt, s_opt)
                },
            )
        })
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2024, AoCDay::AOCD04, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader)
}

fn find2_br<T>(reader: T) -> usize
where
    T: BufRead,
{
    let mut matrix = vec![];
    let mut count = 0;
    for line in valid_lines(reader) {
        let chars = line.chars().collect::<Vec<char>>();
        matrix.push(chars);
    }

    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            if matrix[row][col] == 'A' && find_x_mas(&matrix, row, col) {
                count += 1;
            }
        }
    }
    count
}

fn find_x_mas(matrix: &[Vec<char>], row: usize, col: usize) -> bool {
    let prev_row = row.checked_sub(1);
    let next_row = row + 1;
    let prev_col = col.checked_sub(1);
    let next_col = col + 1;

    prev_row.zip(prev_col).is_some_and(|(prev_row, prev_col)| {
        let top_left = matrix.get(prev_row).and_then(|v| v.get(prev_col));
        let top_right = matrix.get(prev_row).and_then(|v| v.get(next_col));
        let bottom_left = matrix.get(next_row).and_then(|v| v.get(prev_col));
        let bottom_right = matrix.get(next_row).and_then(|v| v.get(next_col));

        is_xmas(top_left, top_right, bottom_left, bottom_right)
    })
}

fn is_xmas(
    top_left: Option<&char>,
    top_right: Option<&char>,
    bottom_left: Option<&char>,
    bottom_right: Option<&char>,
) -> bool {
    let diag_one = top_left
        .zip(bottom_right)
        .is_some_and(|(top_left, bottom_right)| {
            (top_left == &'M' && bottom_right == &'S') || (top_left == &'S' && bottom_right == &'M')
        });
    let diag_two = top_right
        .zip(bottom_left)
        .is_some_and(|(top_right, bottom_left)| {
            (top_right == &'M' && bottom_left == &'S') || (top_right == &'S' && bottom_left == &'M')
        });
    diag_one && diag_two
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use std::io::Cursor;

    const TEST_1: &str = r"..X...
.SAMX.
.A..A.
XMAS.S
.X....";
    const TEST_2: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn solution() {
        assert_eq!(find_br(Cursor::new(TEST_1)), 4);
        assert_eq!(find_br(Cursor::new(TEST_2)), 18);
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use std::io::Cursor;

    const TEST_1: &str = r"M.S
.A.
M.S";
    const TEST_2: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn solution() {
        assert_eq!(find2_br(Cursor::new(TEST_1)), 1);
        assert_eq!(find2_br(Cursor::new(TEST_2)), 9);
    }
}
