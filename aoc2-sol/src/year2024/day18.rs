// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2024 ---**
//!
//! **--- Day 18: RAM Run ---**
//!
//! You and The Historians look a lot more pixelated than you remember. You're inside a computer at the North Pole!
//!
//! Just as you're about to check out your surroundings, a program runs up to you. "This region of memory isn't safe! The User misunderstood what a pushdown automaton is and their algorithm is pushing whole bytes down on top of us! Run!"
//!
//! The algorithm is fast - it's going to cause a byte to fall into your memory space once every nanosecond! Fortunately, you're faster, and by quickly scanning the algorithm, you create a list of which bytes will fall (your puzzle input) in the order they'll land in your memory space.
//!
//! Your memory space is a two-dimensional grid with coordinates that range from 0 to 70 both horizontally and vertically. However, for the sake of example, suppose you're on a smaller grid with coordinates that range from 0 to 6 and the following list of incoming byte positions:
//!
//! ```text
//! 5,4
//! 4,2
//! 4,5
//! 3,0
//! 2,1
//! 6,3
//! 2,4
//! 1,5
//! 0,6
//! 3,3
//! 2,6
//! 5,1
//! 1,2
//! 5,5
//! 2,5
//! 6,5
//! 1,4
//! 0,4
//! 6,4
//! 1,1
//! 6,1
//! 1,0
//! 0,5
//! 1,6
//! 2,0
//! ```
//!
//! Each byte position is given as an X,Y coordinate, where X is the distance from the left edge of your memory space and Y is the distance from the top edge of your memory space.
//!
//! You and The Historians are currently in the top left corner of the memory space (at 0,0) and need to reach the exit in the bottom right corner (at 70,70 in your memory space, but at 6,6 in this example). You'll need to simulate the falling bytes to plan out where it will be safe to run; for now, simulate just the first few bytes falling into your memory space.
//!
//! As bytes fall into your memory space, they make that coordinate corrupted. Corrupted memory coordinates cannot be entered by you or The Historians, so you'll need to plan your route carefully. You also cannot leave the boundaries of the memory space; your only hope is to reach the exit.
//!
//! In the above example, if you were to draw the memory space after the first 12 bytes have fallen (using . for safe and # for corrupted), it would look like this:
//!
//! ```text
//! ...#...
//! ..#..#.
//! ....#..
//! ...#..#
//! ..#..#.
//! .#..#..
//! #.#....
//! ```
//!
//! You can take steps up, down, left, or right. After just 12 bytes have corrupted locations in your memory space, the shortest path from the top left corner to the exit would take 22 steps. Here (marked with O) is one such path:
//!
//! ```text
//! OO.#OOO
//! .O#OO#O
//! .OOO#OO
//! ...#OO#
//! ..#OO#.
//! .#.O#..
//! #.#OOOO
//! ```
//!
//! Simulate the first kilobyte (1024 bytes) falling onto your memory space. Afterward, what is the minimum number of steps needed to reach the exit?
//!
//! **--- Part Two ---**
//!
//! The Historians aren't as used to moving around in this pixelated universe as you are. You're afraid they're not going to be fast enough to make it to the exit before the path is completely blocked.
//!
//! To determine how fast everyone needs to go, you need to determine the first byte that will cut off the path to the exit.
//!
//! In the above example, after the byte at 1,1 falls, there is still a path to the exit:
//!
//! ```text
//! O..#OOO
//! O##OO#O
//! O#OO#OO
//! OOO#OO#
//! ###OO##
//! .##O###
//! #.#OOOO
//! ```
//!
//! However, after adding the very next byte (at 6,1), there is no longer a path to the exit:
//!
//! ```text
//! ...#...
//! .##..##
//! .#..#..
//! ...#..#
//! ###..##
//! .##.###
//! #.#....
//! ```
//!
//! So, in this example, the coordinates of the first byte that prevents the exit from being reachable are 6,1.
//!
//! Simulate more of the bytes that are about to corrupt your memory space. What are the coordinates of the first byte that will prevent the exit from being reachable from your starting position? (Provide the answer as two integers separated by a comma with no other characters.)

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{get_cap_x, run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use console::style;
use crossterm::{
    cursor::{Hide, MoveToNextLine, RestorePosition, SavePosition, Show},
    style::Print,
    terminal::{Clear, ClearType},
    ExecutableCommand, QueueableCommand,
};
use getset::Setters;
use ndarray::{Array2, Axis};
use pathfinding::prelude::dijkstra;
use regex::Regex;
use std::{
    fs::File,
    io::{stdout, BufRead, BufReader, Write},
};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Setters)]
struct Node {
    coord: (usize, usize),
}

impl Node {
    fn new(coord: (usize, usize)) -> Self {
        Self { coord }
    }

    fn successors(&self, maze_data: &Array2<bool>) -> Vec<(Node, usize)> {
        let mut succ = vec![];
        let (x, y) = self.coord;
        if y > 0 {
            if let Some(north_blocked) = maze_data.get((x, y - 1)) {
                if !*north_blocked {
                    succ.push((Node::new((x, y - 1)), 1));
                }
            }
        }

        if let Some(east_blocked) = maze_data.get((x + 1, y)) {
            if !*east_blocked {
                succ.push((Node::new((x + 1, y)), 1));
            }
        }

        if x > 0 {
            if let Some(west_blocked) = maze_data.get((x - 1, y)) {
                if !*west_blocked {
                    succ.push((Node::new((x - 1, y)), 1));
                }
            }
        }

        if let Some(south_blocked) = maze_data.get((x, y + 1)) {
            if !*south_blocked {
                succ.push((Node::new((x, y + 1)), 1));
            }
        }
        succ
    }
}

type ByteData = (Vec<(usize, usize)>, (usize, usize), usize);

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<ByteData, usize>(AoCYear::AOC2024, AoCDay::AOCD18, setup, find).map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<ByteData, usize>(bench, AoCYear::AOC2024, AoCDay::AOCD18, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> ByteData {
    setup_br(reader, (70, 70), 1024).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T, dim: (usize, usize), byte_count: usize) -> Result<ByteData>
where
    T: BufRead,
{
    let byte_dim_re = Regex::new(r"^(\d+),(\d+)$")?;
    let mut data = vec![];
    for line in valid_lines(reader) {
        for caps in byte_dim_re.captures_iter(&line) {
            let x = get_cap_x::<usize>(1, &caps)?;
            let y = get_cap_x::<usize>(2, &caps)?;
            data.push((x, y));
        }
    }
    Ok((data, dim, byte_count))
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: ByteData) -> usize {
    find_res(data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(data: ByteData, _second_star: bool) -> Result<usize> {
    let (falling_bytes, (max_x, max_y), byte_count) = data;
    let mut mem_space = Array2::<bool>::default((max_x + 1, max_y + 1));
    let start = Node::new((0, 0));
    let end = Node::new((max_x, max_y));

    for i in 0..byte_count {
        if let Some((x, y)) = falling_bytes.get(i) {
            mem_space[[*x, *y]] = true;
        }
    }

    disp_mem_space(&mem_space, false, "State After 12")?;
    let min_cost = dijkstra(
        &start,
        |node| node.successors(&mem_space),
        |node| node.coord == end.coord,
    )
    .map(|(_, cost)| cost)
    .ok_or_else(|| anyhow!("no cost for you"))?;
    Ok(min_cost)
}

fn disp_mem_space(mem_space: &Array2<bool>, restore: bool, header: &str) -> Result<()> {
    let mut stdout = stdout();

    let _ = stdout.execute(Hide)?;
    let _ = stdout.queue(SavePosition)?;
    let _ = stdout.queue(Clear(ClearType::CurrentLine))?;
    let _ = stdout.queue(Print(format!("{}", style(header).bold().yellow())))?;
    let _ = stdout.queue(MoveToNextLine(1))?;
    let _ = stdout.queue(MoveToNextLine(1))?;
    for row in mem_space.axis_iter(Axis(1)) {
        for elem in row {
            if *elem {
                let _ = stdout.queue(Print(format!("{}", style("#").bold().red())))?;
            } else {
                let _ = stdout.queue(Print("."))?;
            }
        }
        let _ = stdout.queue(MoveToNextLine(1))?;
    }
    let _ = stdout.queue(MoveToNextLine(1))?;
    if restore {
        let _ = stdout.queue(RestorePosition)?;
    }
    let _ = stdout.execute(Show)?;
    stdout.flush()?;
    Ok(())
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<ByteData, String>(AoCYear::AOC2024, AoCDay::AOCD18, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<ByteData, String>(bench, AoCYear::AOC2024, AoCDay::AOCD18, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: ByteData) -> String {
    find_res2(data, true).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res2(data: ByteData, _second_star: bool) -> Result<String> {
    let (falling_bytes, (max_x, max_y), _byte_count) = data;
    let mut mem_space = Array2::<bool>::default((max_x + 1, max_y + 1));
    let start = Node::new((0, 0));
    let end = Node::new((max_x, max_y));

    let mut final_x = String::new();
    let mut final_y = String::new();

    for i in 0.. {
        if let Some((x, y)) = falling_bytes.get(i) {
            mem_space[[*x, *y]] = true;
            if dijkstra(
                &start,
                |node| node.successors(&mem_space),
                |node| node.coord == end.coord,
            )
            .is_none()
            {
                final_x = x.to_string();
                final_y = y.to_string();
                break;
            }
        }
    }
    Ok(format!("{final_x},{final_y}"))
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1), (6, 6), 12)?;
        assert_eq!(find(data), 22);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1), (6, 6), 12)?;
        assert_eq!(find2(data), "6,1");
        Ok(())
    }
}
