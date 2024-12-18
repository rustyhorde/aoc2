// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2018 ---**
//!
//! **--- Day 18: Settlers of The North Pole ---**
//!
//! On the outskirts of the North Pole base construction project, many Elves are collecting lumber.
//!
//! The lumber collection area is 50 acres by 50 acres; each acre can be either open ground (.), trees (|), or a lumberyard (#). You take a scan of the area (your puzzle input).
//!
//! Strange magic is at work here: each minute, the landscape looks entirely different. In exactly one minute, an open acre can fill with trees, a wooded acre can be converted to a lumberyard, or a lumberyard can be cleared to open ground (the lumber having been sent to other projects).
//!
//! The change to each acre is based entirely on the contents of that acre as well as the number of open, wooded, or lumberyard acres adjacent to it at the start of each minute. Here, "adjacent" means any of the eight acres surrounding that acre. (Acres on the edges of the lumber collection area might have fewer than eight adjacent acres; the missing acres aren't counted.)
//!
//! In particular:
//!
//! ```text
//!     An open acre will become filled with trees if three or more adjacent acres contained trees. Otherwise, nothing happens.
//!     An acre filled with trees will become a lumberyard if three or more adjacent acres were lumberyards. Otherwise, nothing happens.
//!     An acre containing a lumberyard will remain a lumberyard if it was adjacent to at least one other lumberyard and at least one acre containing trees. Otherwise, it becomes open.
//! ```
//!
//! These changes happen across all acres simultaneously, each of them using the state of all acres at the beginning of the minute and changing to their new form by the end of that same minute. Changes that happen during the minute don't affect each other.
//!
//! For example, suppose the lumber collection area is instead only 10 by 10 acres with this initial configuration:
//!
//! ```text
//! Initial state:
//! .#.#...|#.
//! .....#|##|
//! .|..|...#.
//! ..|#.....#
//! #.#|||#|#|
//! ...#.||...
//! .|....|...
//! ||...#|.#|
//! |.||||..|.
//! ...#.|..|.
//!
//! After 1 minute:
//! .......##.
//! ......|###
//! .|..|...#.
//! ..|#||...#
//! ..##||.|#|
//! ...#||||..
//! ||...|||..
//! |||||.||.|
//! ||||||||||
//! ....||..|.
//!
//! After 2 minutes:
//! .......#..
//! ......|#..
//! .|.|||....
//! ..##|||..#
//! ..###|||#|
//! ...#|||||.
//! |||||||||.
//! ||||||||||
//! ||||||||||
//! .|||||||||
//!
//! After 3 minutes:
//! .......#..
//! ....|||#..
//! .|.||||...
//! ..###|||.#
//! ...##|||#|
//! .||##|||||
//! ||||||||||
//! ||||||||||
//! ||||||||||
//! ||||||||||
//!
//! After 4 minutes:
//! .....|.#..
//! ...||||#..
//! .|.#||||..
//! ..###||||#
//! ...###||#|
//! |||##|||||
//! ||||||||||
//! ||||||||||
//! ||||||||||
//! ||||||||||
//!
//! After 5 minutes:
//! ....|||#..
//! ...||||#..
//! .|.##||||.
//! ..####|||#
//! .|.###||#|
//! |||###||||
//! ||||||||||
//! ||||||||||
//! ||||||||||
//! ||||||||||
//!
//! After 6 minutes:
//! ...||||#..
//! ...||||#..
//! .|.###|||.
//! ..#.##|||#
//! |||#.##|#|
//! |||###||||
//! ||||#|||||
//! ||||||||||
//! ||||||||||
//! ||||||||||
//!
//! After 7 minutes:
//! ...||||#..
//! ..||#|##..
//! .|.####||.
//! ||#..##||#
//! ||##.##|#|
//! |||####|||
//! |||###||||
//! ||||||||||
//! ||||||||||
//! ||||||||||
//!
//! After 8 minutes:
//! ..||||##..
//! ..|#####..
//! |||#####|.
//! ||#...##|#
//! ||##..###|
//! ||##.###||
//! |||####|||
//! ||||#|||||
//! ||||||||||
//! ||||||||||
//!
//! After 9 minutes:
//! ..||###...
//! .||#####..
//! ||##...##.
//! ||#....###
//! |##....##|
//! ||##..###|
//! ||######||
//! |||###||||
//! ||||||||||
//! ||||||||||
//!
//! After 10 minutes:
//! .||##.....
//! ||###.....
//! ||##......
//! |##.....##
//! |##.....##
//! |##....##|
//! ||##.####|
//! ||#####|||
//! ||||#|||||
//! ||||||||||
//! ```
//!
//! After 10 minutes, there are 37 wooded acres and 31 lumberyards. Multiplying the number of wooded acres by the number of lumberyards gives the total resource value after ten minutes: 37 * 31 = 1147.
//!
//! What will the total resource value of the lumber collection area be after 10 minutes?
//!
//! **--- Part Two ---**
//!
//! This important natural resource will need to last for at least thousands of years. Are the Elves collecting this lumber sustainably?
//!
//! What will the total resource value of the lumber collection area be after 1000000000 minutes?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use console::style;
use core::fmt;
use crossterm::{
    cursor::{Hide, MoveToNextLine, RestorePosition, SavePosition, Show},
    style::Print,
    terminal::{Clear, ClearType},
    ExecutableCommand, QueueableCommand,
};
use itertools::Itertools;
use ndarray::{Array2, Axis};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use std::{
    collections::HashMap,
    fs::File,
    io::{stdout, BufRead, BufReader, Write},
    thread::sleep,
    time::Duration,
};

type ForestData = (Vec<String>, usize, bool);

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
enum AcreType {
    #[default]
    Empty,
    Trees,
    LumberYard,
}

impl fmt::Display for AcreType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AcreType::Empty => ".",
                AcreType::Trees => "|",
                AcreType::LumberYard => "#",
            }
        )
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
struct Acre {
    kind: AcreType,
    neighbors: Vec<AcreType>,
}

impl Acre {
    fn new(kind: AcreType) -> Self {
        Self {
            kind,
            neighbors: vec![],
        }
    }
}

impl fmt::Display for Acre {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len = self.neighbors.len();
        let few = self
            .neighbors
            .iter()
            .take(3)
            .map(ToString::to_string)
            .join(",");
        write!(
            f,
            "{} => {}{}",
            self.kind,
            few,
            if len > 3 { "..." } else { "" }
        )
    }
}

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<ForestData, usize>(AoCYear::AOC2018, AoCDay::AOCD18, setup, find)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<ForestData, usize>(bench, AoCYear::AOC2018, AoCDay::AOCD18, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> ForestData {
    setup_br(reader, 10, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T, iterations: usize, test: bool) -> Result<ForestData>
where
    T: BufRead,
{
    let mut data = vec![];
    for line in valid_lines(reader) {
        data.push(line);
    }
    Ok((data, iterations, test))
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: ForestData) -> usize {
    find_res(data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(data: ForestData, second_star: bool) -> Result<usize> {
    let (data, iterations, test) = data;
    let mut forest = HashMap::new();

    for (y, line) in data.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let _ = match ch {
                '.' => forest.entry((x, y)).or_insert(Acre::new(AcreType::Empty)),
                '|' => forest.entry((x, y)).or_insert(Acre::new(AcreType::Trees)),
                '#' => forest
                    .entry((x, y))
                    .or_insert(Acre::new(AcreType::LumberYard)),
                _ => return Err(anyhow!("invalid forest character: {ch}")),
            };
        }
    }

    disp_forest(&forest, true, "Initial State:", !test, 0)?;
    let mut trees = 0;
    let mut ly = 0;
    for i in 1..=iterations {
        update_all_neighbors(&mut forest);
        update_forest(&mut forest);
        if !test {
            sleep(Duration::from_millis(25));
        }
        if i > 600 {
            // 1_000_000_000 mod 28 is 20.  The pattern repeats every 28 steps.
            if i % 28 == 20 {
                let (trees_x, ly_x) = count_tress_ly(&forest);
                trees = trees_x;
                ly = ly_x;
            }
        }
        disp_forest(
            &forest,
            i != iterations,
            &format!("After {i} minute(s): ({})", trees * ly),
            !test,
            0,
        )?;
        if second_star && i > 999 {
            disp_forest(
                &forest,
                false,
                &format!("After {i} minute(s): ({})", trees * ly),
                !test,
                0,
            )?;
            break;
        }
    }
    let (trees, ly) = count_tress_ly(&forest);
    Ok(trees * ly)
}

fn count_tress_ly(forest: &HashMap<(usize, usize), Acre>) -> (usize, usize) {
    let trees = forest
        .values()
        .filter(|x| x.kind == AcreType::Trees)
        .count();
    let ly = forest
        .values()
        .filter(|x| x.kind == AcreType::LumberYard)
        .count();
    (trees, ly)
}
fn update_forest(forest: &mut HashMap<(usize, usize), Acre>) {
    forest
        .par_iter_mut()
        .for_each(|((_x, _y), acre)| check_acre(acre));
}

fn check_acre(acre: &mut Acre) {
    match acre.kind {
        AcreType::Empty => {
            if acre
                .neighbors
                .iter()
                .filter(|x| **x == AcreType::Trees)
                .count()
                >= 3
            {
                acre.kind = AcreType::Trees;
            }
        }
        AcreType::Trees => {
            if acre
                .neighbors
                .iter()
                .filter(|x| **x == AcreType::LumberYard)
                .count()
                >= 3
            {
                acre.kind = AcreType::LumberYard;
            }
        }
        AcreType::LumberYard => {
            let adj_ly = acre
                .neighbors
                .iter()
                .filter(|x| **x == AcreType::LumberYard)
                .count();
            let adj_t = acre
                .neighbors
                .iter()
                .filter(|x| **x == AcreType::Trees)
                .count();

            if adj_ly < 1 || adj_t < 1 {
                acre.kind = AcreType::Empty;
            }
        }
    }
}

fn directions(x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut dirs = vec![];
    if x > 0 && y > 0 {
        // up left
        dirs.push((x - 1, y - 1));
    }

    if y > 0 {
        // up
        dirs.push((x, y - 1));
        // up right
        dirs.push((x + 1, y - 1));
    }

    if x > 0 {
        // left
        dirs.push((x - 1, y));
        // down left
        dirs.push((x - 1, y + 1));
    }

    // down
    dirs.push((x, y + 1));
    // down right
    dirs.push((x + 1, y + 1));
    // right
    dirs.push((x + 1, y));

    dirs
}

fn update_all_neighbors(forest: &mut HashMap<(usize, usize), Acre>) {
    let fc = forest.clone();
    forest
        .par_iter_mut()
        .for_each(|((x, y), acre)| add_neighbors(&fc, *x, *y, acre));
}

fn add_neighbors(forest: &HashMap<(usize, usize), Acre>, x: usize, y: usize, acre: &mut Acre) {
    acre.neighbors.clear();
    for dir in directions(x, y) {
        if let Some(neighbor) = forest.get(&dir) {
            acre.neighbors.push(neighbor.kind);
        }
    }
}

fn disp_forest(
    forest: &HashMap<(usize, usize), Acre>,
    restore: bool,
    header: &str,
    display: bool,
    _i: usize,
) -> Result<()> {
    if display {
        let max_x = forest
            .keys()
            .map(|(x, _y)| x)
            .max()
            .ok_or_else(|| anyhow!("no max x"))?;
        let max_y = forest
            .keys()
            .map(|(_x, y)| y)
            .max()
            .ok_or_else(|| anyhow!("no max y"))?;
        let mut arr = Array2::<Acre>::default((*max_x + 1, *max_y + 1));
        for ((x, y), acre) in forest {
            arr[[*x, *y]] = acre.clone();
        }

        let mut stdout = stdout();

        let _ = stdout.execute(Hide)?;
        let _ = stdout.queue(SavePosition)?;
        let _ = stdout.queue(MoveToNextLine(1))?;
        for row in arr.axis_iter(Axis(1)) {
            for elem in row {
                match elem.kind {
                    AcreType::Empty => { let _ = stdout.queue(Print(elem.kind))?; },
                    AcreType::Trees => { let _ = stdout.queue(Print(format!("{}", style(elem.kind).bold().red())))?; },
                    AcreType::LumberYard => { let _ = stdout.queue(Print(format!("{}", style(elem.kind).bold().green())))?; },
                }
            }
            let _ = stdout.queue(MoveToNextLine(1))?;
        }
        let _ = stdout.queue(MoveToNextLine(1))?;
        let _ = stdout.queue(Clear(ClearType::CurrentLine))?;
        let _ = stdout.queue(Print(format!("{}", style(header).bold().yellow())))?;
        let _ = stdout.queue(MoveToNextLine(1))?;
        if restore {
            let _ = stdout.queue(RestorePosition)?;
        }
        let _ = stdout.execute(Show)?;
        stdout.flush()?;
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
    run_setup_solution::<ForestData, usize>(AoCYear::AOC2018, AoCDay::AOCD18, setup2, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<ForestData, usize>(bench, AoCYear::AOC2018, AoCDay::AOCD18, setup2, find2)
        .map(|_| 0)
}

fn setup2(reader: BufReader<File>) -> ForestData {
    setup_br(reader, 1_000_000_000, false).unwrap_or_default()
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: ForestData) -> usize {
    find_res(data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1), 10, true)?;
        assert_eq!(find(data), 1147);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {}
