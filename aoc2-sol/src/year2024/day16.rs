// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2024 ---**
//!
//! **--- Day 16: Reindeer Maze ---**
//!
//! It's time again for the Reindeer Olympics! This year, the big event is the Reindeer Maze, where the Reindeer compete for the lowest score.
//!
//! You and The Historians arrive to search for the Chief right as the event is about to start. It wouldn't hurt to watch a little, right?
//!
//! The Reindeer start on the Start Tile (marked S) facing East and need to reach the End Tile (marked E). They can move forward one tile at a time (increasing their score by 1 point), but never into a wall (#). They can also rotate clockwise or counterclockwise 90 degrees at a time (increasing their score by 1000 points).
//!
//! To figure out the best place to sit, you start by grabbing a map (your puzzle input) from a nearby kiosk. For example:
//!
//! ```text
//! ###############
//! #.......#....E#
//! #.#.###.#.###.#
//! #.....#.#...#.#
//! #.###.#####.#.#
//! #.#.#.......#.#
//! #.#.#####.###.#
//! #...........#.#
//! ###.#.#####.#.#
//! #...#.....#.#.#
//! #.#.#.###.#.#.#
//! #.....#...#.#.#
//! #.###.#.#.#.#.#
//! #S..#.....#...#
//! ###############
//! ```
//!
//! There are many paths through this maze, but taking any of the best paths would incur a score of only 7036. This can be achieved by taking a total of 36 steps forward and turning 90 degrees a total of 7 times:
//!
//! ```text
//! ###############
//! #.......#....E#
//! #.#.###.#.###^#
//! #.....#.#...#^#
//! #.###.#####.#^#
//! #.#.#.......#^#
//! #.#.#####.###^#
//! #..>>>>>>>>v#^#
//! ###^#.#####v#^#
//! #>>^#.....#v#^#
//! #^#.#.###.#v#^#
//! #^....#...#v#^#
//! #^###.#.#.#v#^#
//! #S..#.....#>>^#
//! ###############
//! ```
//!
//! Here's a second example:
//!
//! ```text
//! #################
//! #...#...#...#..E#
//! #.#.#.#.#.#.#.#.#
//! #.#.#.#...#...#.#
//! #.#.#.#.###.#.#.#
//! #...#.#.#.....#.#
//! #.#.#.#.#.#####.#
//! #.#...#.#.#.....#
//! #.#.#####.#.###.#
//! #.#.#.......#...#
//! #.#.###.#####.###
//! #.#.#...#.....#.#
//! #.#.#.#####.###.#
//! #.#.#.........#.#
//! #.#.#.#########.#
//! #S#.............#
//! #################
//! ```
//!
//! In this maze, the best paths cost 11048 points; following one such path would look like this:
//!
//! ```text
//! #################
//! #...#...#...#..E#
//! #.#.#.#.#.#.#.#^#
//! #.#.#.#...#...#^#
//! #.#.#.#.###.#.#^#
//! #>>v#.#.#.....#^#
//! #^#v#.#.#.#####^#
//! #^#v..#.#.#>>>>^#
//! #^#v#####.#^###.#
//! #^#v#..>>>>^#...#
//! #^#v###^#####.###
//! #^#v#>>^#.....#.#
//! #^#v#^#####.###.#
//! #^#v#^........#.#
//! #^#v#^#########.#
//! #S#>>^..........#
//! #################
//! ```
//!
//! Note that the path shown above includes one 90 degree turn as the very first move, rotating the Reindeer from facing East to facing North.
//!
//! Analyze your map carefully. What is the lowest score a Reindeer could possibly get?
//!
//! **--- Part Two ---**
//!
//! Now that you know what the best paths look like, you can figure out the best spot to sit.
//!
//! Every non-wall tile (S, ., or E) is equipped with places to sit along the edges of the tile. While determining which of these tiles would be the best spot to sit depends on a whole bunch of factors (how comfortable the seats are, how far away the bathrooms are, whether there's a pillar blocking your view, etc.), the most important factor is whether the tile is on one of the best paths through the maze. If you sit somewhere else, you'd miss all the action!
//!
//! So, you'll need to determine which tiles are part of any best path through the maze, including the S and E tiles.
//!
//! In the first example, there are 45 tiles (marked O) that are part of at least one of the various best paths through the maze:
//!
//! ###############
//! #.......#....O#
//! #.#.###.#.###O#
//! #.....#.#...#O#
//! #.###.#####.#O#
//! #.#.#.......#O#
//! #.#.#####.###O#
//! #..OOOOOOOOO#O#
//! ###O#O#####O#O#
//! #OOO#O....#O#O#
//! #O#O#O###.#O#O#
//! #OOOOO#...#O#O#
//! #O###.#.#.#O#O#
//! #O..#.....#OOO#
//! ###############
//!
//! In the second example, there are 64 tiles that are part of at least one of the best paths:
//!
//! #################
//! #...#...#...#..O#
//! #.#.#.#.#.#.#.#O#
//! #.#.#.#...#...#O#
//! #.#.#.#.###.#.#O#
//! #OOO#.#.#.....#O#
//! #O#O#.#.#.#####O#
//! #O#O..#.#.#OOOOO#
//! #O#O#####.#O###O#
//! #O#O#..OOOOO#OOO#
//! #O#O###O#####O###
//! #O#O#OOO#..OOO#.#
//! #O#O#O#####O###.#
//! #O#O#OOOOOOO..#.#
//! #O#O#O#########.#
//! #O#OOO..........#
//! #################
//!
//! Analyze your map further. How many tiles are part of at least one of the best paths through the maze?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use crossterm::{
    cursor::{Hide, MoveToNextLine, RestorePosition, SavePosition, Show},
    style::Print,
    ExecutableCommand, QueueableCommand,
};
use getset::Setters;
use ndarray::{Array2, Axis};
use pathfinding::prelude::{dijkstra, yen};
use std::{
    collections::HashSet,
    fs::File,
    io::{stdout, BufRead, BufReader, Write},
};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
enum Direction {
    West,
    #[default]
    East,
    North,
    South,
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Setters)]
struct Node {
    coord: (usize, usize),
    initial_dir: Direction,
}

impl Node {
    fn new(coord: (usize, usize), initial_dir: Direction) -> Self {
        Self { coord, initial_dir }
    }

    fn successors(&self, maze_data: &Array2<bool>) -> Vec<(Node, usize)> {
        let mut succ = vec![];
        let (x, y) = self.coord;
        if y > 0 {
            if let Some(has_north) = maze_data.get((x, y - 1)) {
                if *has_north {
                    match self.initial_dir {
                        Direction::East | Direction::West => {
                            succ.push((Node::new(self.coord, Direction::North), 1000));
                        }
                        Direction::North => succ.push((Node::new((x, y - 1), Direction::North), 1)),
                        Direction::South => {}
                    }
                }
            }
        }

        if let Some(has_east) = maze_data.get((x + 1, y)) {
            if *has_east {
                match self.initial_dir {
                    Direction::West => {}
                    Direction::East => succ.push((Node::new((x + 1, y), Direction::East), 1)),
                    Direction::North | Direction::South => {
                        succ.push((Node::new(self.coord, Direction::East), 1000));
                    }
                }
            }
        }

        if x > 0 {
            if let Some(has_west) = maze_data.get((x - 1, y)) {
                if *has_west {
                    match self.initial_dir {
                        Direction::West => succ.push((Node::new((x - 1, y), Direction::West), 1)),
                        Direction::East => {}
                        Direction::North | Direction::South => {
                            succ.push((Node::new(self.coord, Direction::West), 1000));
                        }
                    }
                }
            }
        }

        if let Some(has_south) = maze_data.get((x, y + 1)) {
            if *has_south {
                match self.initial_dir {
                    Direction::West | Direction::East => {
                        succ.push((Node::new(self.coord, Direction::South), 1000));
                    }
                    Direction::North => {}
                    Direction::South => succ.push((Node::new((x, y + 1), Direction::South), 1)),
                }
            }
        }
        succ
    }
}

type MazeData = (Vec<String>, bool);

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<MazeData, usize>(AoCYear::AOC2024, AoCDay::AOCD16, setup, find).map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<MazeData, usize>(bench, AoCYear::AOC2024, AoCDay::AOCD16, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> MazeData {
    setup_br(reader, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T, test: bool) -> Result<MazeData>
where
    T: BufRead,
{
    let mut data = vec![];

    for line in valid_lines(reader) {
        data.push(line);
    }
    Ok((data, test))
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: MazeData) -> usize {
    find_res(&data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(data: &MazeData, second_star: bool) -> Result<usize> {
    let (data, _test) = data;
    let max_x = data[0].len();
    let max_y = data.len();
    let mut maze_data = Array2::<bool>::default((max_x, max_y));
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (y, line) in data.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '#' => {}
                '.' => maze_data[[x, y]] = true,
                'S' => {
                    start = (x, y);
                    maze_data[[x, y]] = true;
                }
                'E' => {
                    end = (x, y);
                    maze_data[[x, y]] = true;
                }
                _ => return Err(anyhow!("invalid maze data: '{ch}'")),
            }
        }
    }

    let start_node = Node::new(start, Direction::East);
    let min_cost = dijkstra(
        &start_node,
        |node| node.successors(&maze_data),
        |node| node.coord == end,
    )
    .map(|(_, cost)| cost)
    .ok_or_else(|| anyhow!("no cost for you"))?;

    if second_star {
        let top_fifty_paths = yen(
            &start_node,
            |node| node.successors(&maze_data),
            |node| node.coord == end,
            50,
        );
        let mut nodes_set = HashSet::new();
        for nodes in
            top_fifty_paths
                .iter()
                .filter_map(|(x, cost)| if *cost == min_cost { Some(x) } else { None })
        {
            for node in nodes {
                let _ = nodes_set.insert(node.coord);
            }
        }
        Ok(nodes_set.len())
    } else {
        Ok(min_cost)
    }
}

#[allow(dead_code)]
fn print_maze(maze_data: &Array2<bool>, restore: bool, curr_loc: (usize, usize)) -> Result<()> {
    let mut stdout = stdout();

    let _ = stdout.execute(Hide)?;
    let _ = stdout.queue(SavePosition)?;
    let _ = stdout.queue(MoveToNextLine(1))?;
    for (y, axis) in maze_data.axis_iter(Axis(1)).enumerate() {
        for (x, node) in axis.indexed_iter() {
            if *node {
                if curr_loc.0 == x && curr_loc.1 == y {
                    let _ = stdout.queue(Print('S'))?;
                } else {
                    let _ = stdout.queue(Print('.'))?;
                }
            } else {
                let _ = stdout.queue(Print('#'))?;
            }
        }
        let _ = stdout.queue(MoveToNextLine(1))?;
    }
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
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<MazeData, usize>(AoCYear::AOC2024, AoCDay::AOCD16, setup, find2).map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<MazeData, usize>(bench, AoCYear::AOC2024, AoCDay::AOCD16, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: MazeData) -> usize {
    find_res(&data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    //     const TEST_2: &str = r"#################
    // #...#...#...#..E#
    // #.#.#.#.#.#.#.#.#
    // #.#.#.#...#...#.#
    // #.#.#.#.###.#.#.#
    // #...#.#.#.....#.#
    // #.#.#.#.#.#####.#
    // #.#...#.#.#.....#
    // #.#.#####.#.###.#
    // #.#.#.......#...#
    // #.#.###.#####.###
    // #.#.#...#.....#.#
    // #.#.#.#####.###.#
    // #.#.#.........#.#
    // #.#.#.#########.#
    // #S#.............#
    // #################";

    //     const TEST_3: &str = r"#######
    // #....E#
    // #.#####
    // #.#...#
    // #...#.#
    // #S#...#
    // #######";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1), true)?;
        assert_eq!(find(data), 7036);
        // let data = setup_br(Cursor::new(TEST_2))?;
        // assert_eq!(find(data), 11048);
        // let data = setup_br(Cursor::new(TEST_3))?;
        // assert_eq!(find(data), 2008);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const TEST_2: &str = r"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1), true)?;
        assert_eq!(find2(data), 45);
        let data = setup_br(Cursor::new(TEST_2), true)?;
        assert_eq!(find2(data), 64);
        Ok(())
    }
}
