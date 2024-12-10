// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 1
//!
//! --- Day 10: Hoof It ---
//!
//! You all arrive at a Lava Production Facility on a floating island in the sky. As the others begin to search the massive industrial complex, you feel a small nose boop your leg and look down to discover a reindeer wearing a hard hat.
//!
//! The reindeer is holding a book titled "Lava Island Hiking Guide". However, when you open the book, you discover that most of it seems to have been scorched by lava! As you're about to ask how you can help, the reindeer brings you a blank topographic map of the surrounding area (your puzzle input) and looks up at you excitedly.
//!
//! Perhaps you can help fill in the missing hiking trails?
//!
//! The topographic map indicates the height at each position using a scale from 0 (lowest) to 9 (highest). For example:
//!
//! ```text
//! 0123
//! 1234
//! 8765
//! 9876
//! ```
//!
//! Based on un-scorched scraps of the book, you determine that a good hiking trail is as long as possible and has an even, gradual, uphill slope. For all practical purposes, this means that a hiking trail is any path that starts at height 0, ends at height 9, and always increases by a height of exactly 1 at each step. Hiking trails never include diagonal steps - only up, down, left, or right (from the perspective of the map).
//!
//! You look up from the map and notice that the reindeer has helpfully begun to construct a small pile of pencils, markers, rulers, compasses, stickers, and other equipment you might need to update the map with hiking trails.
//!
//! A trailhead is any position that starts one or more hiking trails - here, these positions will always have height 0. Assembling more fragments of pages, you establish that a trailhead's score is the number of 9-height positions reachable from that trailhead via a hiking trail. In the above example, the single trailhead in the top left corner has a score of 1 because it can reach a single 9 (the one in the bottom left).
//!
//! This trailhead has a score of 2:
//!
//! ```text
//! ...0...
//! ...1...
//! ...2...
//! 6543456
//! 7.....7
//! 8.....8
//! 9.....9
//! ```
//!
//! (The positions marked . are impassable tiles to simplify these examples; they do not appear on your actual topographic map.)
//!
//! This trailhead has a score of 4 because every 9 is reachable via a hiking trail except the one immediately to the left of the trailhead:
//!
//! ```text
//! ..90..9
//! ...1.98
//! ...2..7
//! 6543456
//! 765.987
//! 876....
//! 987....
//! ```
//!
//! This topographic map contains two trailheads; the trailhead at the top has a score of 1, while the trailhead at the bottom has a score of 2:
//!
//! ```text
//! 10..9..
//! 2...8..
//! 3...7..
//! 4567654
//! ...8..3
//! ...9..2
//! .....01
//! ```
//!
//! Here's a larger example:
//!
//! ```text
//! 89010123
//! 78121874
//! 87430965
//! 96549874
//! 45678903
//! 32019012
//! 01329801
//! 10456732
//! ```
//!
//! This larger example has 9 trailheads. Considering the trailheads in reading order, they have scores of 5, 6, 5, 3, 1, 3, 5, 3, and 5. Adding these scores together, the sum of the scores of all trailheads is 36.
//!
//! The reindeer gleefully carries over a protractor and adds it to the pile. What is the sum of the scores of all trailheads on your topographic map?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::Result;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
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
    run_setup_solution::<Vec<Vec<usize>>, usize>(AoCYear::AOC2024, AoCDay::AOCD10, setup, find)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<Vec<usize>>, usize>(
        bench,
        AoCYear::AOC2024,
        AoCDay::AOCD10,
        setup,
        find,
    )
    .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> Vec<Vec<usize>> {
    setup_br(reader).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T) -> Result<Vec<Vec<usize>>>
where
    T: BufRead,
{
    let mut matrix = vec![];
    for line in valid_lines(reader) {
        let cols = line
            .chars()
            .map(|x| x.to_string().parse::<usize>())
            .filter_map(Result::ok)
            .collect::<Vec<usize>>();
        matrix.push(cols);
    }
    Ok(matrix)
}

#[allow(clippy::needless_pass_by_value)]
fn find(matrix: Vec<Vec<usize>>) -> usize {
    let mut trails: BTreeMap<usize, Vec<(usize, usize)>> = BTreeMap::new();
    for (row_id, cols) in matrix.iter().enumerate() {
        for (col_id, col) in cols.iter().enumerate() {
            let _ = trails
                .entry(*col)
                .and_modify(|locs| locs.push((row_id, col_id)))
                .or_insert(vec![(row_id, col_id)]);
        }
    }
    // eprintln!("trailheads: {trails:?}");
    let mut scores = HashMap::new();
    if let Some(trailheads) = trails.get(&0) {
        for trailhead in trailheads {
            check_path(&trails, *trailhead, 0, &[*trailhead], &mut scores);
        }
    }
    // eprintln!("scores: {scores:?}");
    scores.values().map(HashSet::len).sum()
}

fn check_path(
    trails: &BTreeMap<usize, Vec<(usize, usize)>>,
    curr_node: (usize, usize),
    curr_level: usize,
    curr_trail: &[(usize, usize)],
    scores: &mut HashMap<(usize, usize), HashSet<(usize, usize)>>,
) {
    let (row, col) = curr_node;
    if curr_level == 9 {
        let _ = scores
            .entry(curr_trail[0])
            .or_insert_with(|| {
                let mut ends = HashSet::new();
                let _ = ends.insert(curr_node);
                ends
            })
            .insert(curr_node);
    } else {
        let next_level = curr_level + 1;
        if let Some(nexts) = trails.get(&next_level) {
            // Check up
            if let Some(r) = row.checked_sub(1) {
                if nexts.contains(&(r, col)) {
                    let next_node = (r, col);
                    let mut next_trail = curr_trail.to_owned();
                    next_trail.push(next_node);
                    check_path(trails, next_node, next_level, &next_trail, scores);
                }
            }

            // Check down
            if nexts.contains(&(row + 1, col)) {
                let next_node = (row + 1, col);
                let mut next_trail = curr_trail.to_owned();
                next_trail.push(next_node);
                check_path(trails, next_node, next_level, &next_trail, scores);
            }

            // Check left
            if let Some(c) = col.checked_sub(1) {
                if nexts.contains(&(row, c)) {
                    let next_node = (row, c);
                    let mut next_trail = curr_trail.to_owned();
                    next_trail.push(next_node);
                    check_path(trails, next_node, next_level, &next_trail, scores);
                }
            }

            // Check right
            if nexts.contains(&(row, col + 1)) {
                let next_node = (row, col + 1);
                let mut next_trail = curr_trail.to_owned();
                next_trail.push(next_node);
                check_path(trails, next_node, next_level, &next_trail, scores);
            }
        }
    }
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<Vec<Vec<usize>>, usize>(AoCYear::AOC2024, AoCDay::AOCD10, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<Vec<usize>>, usize>(
        bench,
        AoCYear::AOC2024,
        AoCDay::AOCD10,
        setup,
        find2,
    )
    .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(matrix: Vec<Vec<usize>>) -> usize {
    let mut trails: BTreeMap<usize, Vec<(usize, usize)>> = BTreeMap::new();
    for (row_id, cols) in matrix.iter().enumerate() {
        for (col_id, col) in cols.iter().enumerate() {
            let _ = trails
                .entry(*col)
                .and_modify(|locs| locs.push((row_id, col_id)))
                .or_insert(vec![(row_id, col_id)]);
        }
    }
    // eprintln!("trailheads: {trails:?}");
    let mut scores = HashMap::new();
    if let Some(trailheads) = trails.get(&0) {
        for trailhead in trailheads {
            check_path2(&trails, *trailhead, 0, &[*trailhead], &mut scores);
        }
    }
    scores.values().map(Vec::len).sum()
}

type Scores = HashMap<(usize, usize), Vec<Vec<(usize, usize)>>>;

fn check_path2(
    trails: &BTreeMap<usize, Vec<(usize, usize)>>,
    curr_node: (usize, usize),
    curr_level: usize,
    curr_trail: &[(usize, usize)],
    scores: &mut Scores,
) {
    let (row, col) = curr_node;
    if curr_level == 9 {
        let _ = scores
            .entry(curr_trail[0])
            .and_modify(|trails| trails.push(curr_trail.to_vec()))
            .or_insert_with(|| vec![curr_trail.to_vec()]);
    } else {
        let next_level = curr_level + 1;
        if let Some(nexts) = trails.get(&next_level) {
            // Check up
            if let Some(r) = row.checked_sub(1) {
                if nexts.contains(&(r, col)) {
                    let next_node = (r, col);
                    let mut next_trail = curr_trail.to_owned();
                    next_trail.push(next_node);
                    check_path2(trails, next_node, next_level, &next_trail, scores);
                }
            }

            // Check down
            if nexts.contains(&(row + 1, col)) {
                let next_node = (row + 1, col);
                let mut next_trail = curr_trail.to_owned();
                next_trail.push(next_node);
                check_path2(trails, next_node, next_level, &next_trail, scores);
            }

            // Check left
            if let Some(c) = col.checked_sub(1) {
                if nexts.contains(&(row, c)) {
                    let next_node = (row, c);
                    let mut next_trail = curr_trail.to_owned();
                    next_trail.push(next_node);
                    check_path2(trails, next_node, next_level, &next_trail, scores);
                }
            }

            // Check right
            if nexts.contains(&(row, col + 1)) {
                let next_node = (row, col + 1);
                let mut next_trail = curr_trail.to_owned();
                next_trail.push(next_node);
                check_path2(trails, next_node, next_level, &next_trail, scores);
            }
        }
    }
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find(data), 36);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find2(data), 81);
        Ok(())
    }
}
