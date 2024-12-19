// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2017 ---**
//!
//! **--- Day 22: Sporifica Virus ---**
//!
//! Diagnostics indicate that the local grid computing cluster has been contaminated with the Sporifica Virus. The grid computing cluster is a seemingly-infinite two-dimensional grid of compute nodes. Each node is either clean or infected by the virus.
//!
//! To prevent overloading the nodes (which would render them useless to the virus) or detection by system administrators, exactly one virus carrier moves through the network, infecting or cleaning nodes as it moves. The virus carrier is always located on a single node in the network (the current node) and keeps track of the direction it is facing.
//!
//! To avoid detection, the virus carrier works in bursts; in each burst, it wakes up, does some work, and goes back to sleep. The following steps are all executed in order one time each burst:
//!
//! ```text
//!     If the current node is infected, it turns to its right. Otherwise, it turns to its left. (Turning is done in-place; the current node does not change.)
//!     If the current node is clean, it becomes infected. Otherwise, it becomes cleaned. (This is done after the node is considered for the purposes of changing direction.)
//!     The virus carrier moves forward one node in the direction it is facing.
//! ```
//!
//! Diagnostics have also provided a map of the node infection status (your puzzle input). Clean nodes are shown as .; infected nodes are shown as #. This map only shows the center of the grid; there are many more nodes beyond those shown, but none of them are currently infected.
//!
//! The virus carrier begins in the middle of the map facing up.
//!
//! For example, suppose you are given a map like this:
//!
//! ```text
//! ..#
//! #..
//! ...
//! ```
//!
//! Then, the middle of the infinite grid looks like this, with the virus carrier's position marked with [ ]:
//!
//! ```text
//! . . . . . . . . .
//! . . . . . . . . .
//! . . . . . . . . .
//! . . . . . # . . .
//! . . . #[.]. . . .
//! . . . . . . . . .
//! . . . . . . . . .
//! . . . . . . . . .
//! ```
//!
//! The virus carrier is on a clean node, so it turns left, infects the node, and moves left:
//!
//! ```text
//! . . . . . . . . .
//! . . . . . . . . .
//! . . . . . . . . .
//! . . . . . # . . .
//! . . .[#]# . . . .
//! . . . . . . . . .
//! . . . . . . . . .
//! . . . . . . . . .
//! ```
//!
//! The virus carrier is on an infected node, so it turns right, cleans the node, and moves up:
//!
//! ```text
//! . . . . . . . . .
//! . . . . . . . . .
//! . . . . . . . . .
//! . . .[.]. # . . .
//! . . . . # . . . .
//! . . . . . . . . .
//! . . . . . . . . .
//! . . . . . . . . .
//! ```
//!
//! Four times in a row, the virus carrier finds a clean, infects it, turns left, and moves forward, ending in the same place and still facing up:
//!
//! ```text
//! . . . . . . . . .
//! . . . . . . . . .
//! . . . . . . . . .
//! . . #[#]. # . . .
//! . . # # # . . . .
//! . . . . . . . . .
//! . . . . . . . . .
//! . . . . . . . . .
//! ```
//!
//! Now on the same node as before, it sees an infection, which causes it to turn right, clean the node, and move forward:
//!
//! ```text
//! . . . . . . . . .
//! . . . . . . . . .
//! . . . . . . . . .
//! . . # .[.]# . . .
//! . . # # # . . . .
//! . . . . . . . . .
//! . . . . . . . . .
//! . . . . . . . . .
//! ```
//!
//! After the above actions, a total of 7 bursts of activity had taken place. Of them, 5 bursts of activity caused an infection.
//!
//! After a total of 70, the grid looks like this, with the virus carrier facing up:
//!
//! ```text
//! . . . . . # # . .
//! . . . . # . . # .
//! . . . # . . . . #
//! . . # . #[.]. . #
//! . . # . # . . # .
//! . . . . . # # . .
//! . . . . . . . . .
//! . . . . . . . . .
//! ```
//!
//! By this time, 41 bursts of activity caused an infection (though most of those nodes have since been cleaned).
//!
//! After a total of 10000 bursts of activity, 5587 bursts will have caused an infection.
//!
//! Given your actual map, after 10000 bursts of activity, how many bursts cause a node to become infected? (Do not count nodes that begin infected.)
//!
//! **--- Part Two ---**
//!
//! As you go to remove the virus from the infected nodes, it evolves to resist your attempt.
//!
//! Now, before it infects a clean node, it will weaken it to disable your defenses. If it encounters an infected node, it will instead flag the node to be cleaned in the future. So:
//!
//! ```text
//!     Clean nodes become weakened.
//!     Weakened nodes become infected.
//!     Infected nodes become flagged.
//!     Flagged nodes become clean.
//! ```
//!
//! Every node is always in exactly one of the above states.
//!
//! The virus carrier still functions in a similar way, but now uses the following logic during its bursts of action:
//!
//! ```text
//!     Decide which way to turn based on the current node:
//!         If it is clean, it turns left.
//!         If it is weakened, it does not turn, and will continue moving in the same direction.
//!         If it is infected, it turns right.
//!         If it is flagged, it reverses direction, and will go back the way it came.
//!     Modify the state of the current node, as described above.
//!     The virus carrier moves forward one node in the direction it is facing.
//! ```
//!
//! Start with the same map (still using . for clean and # for infected) and still with the virus carrier starting in the middle and facing up.
//!
//! Using the same initial state as the previous example, and drawing weakened as W and flagged as F, the middle of the infinite grid looks like this, with the virus carrier's position again marked with [ ]:
//!
//! ```text
//! . . . . . . . . .
//! . . . . . . . . .
//! . . . . . . . . .
//! . . . . . # . . .
//! . . . #[.]. . . .
//! . . . . . . . . .
//! . . . . . . . . .
//! . . . . . . . . .
//! ```
//!
//! This is the same as before, since no initial nodes are weakened or flagged. The virus carrier is on a clean node, so it still turns left, instead weakens the node, and moves left:
//!
//! ```text
//! . . . . . . . . .
//! . . . . . . . . .
//! . . . . . . . . .
//! . . . . . # . . .
//! . . .[#]W . . . .
//! . . . . . . . . .
//! . . . . . . . . .
//! . . . . . . . . .
//! ```
//!
//! The virus carrier is on an infected node, so it still turns right, instead flags the node, and moves up:
//!
//! ```text
//! . . . . . . . . .
//! . . . . . . . . .
//! . . . . . . . . .
//! . . .[.]. # . . .
//! . . . F W . . . .
//! . . . . . . . . .
//! . . . . . . . . .
//! . . . . . . . . .
//! ```
//!
//! This process repeats three more times, ending on the previously-flagged node and facing right:
//!
//! ```text
//! . . . . . . . . .
//! . . . . . . . . .
//! . . . . . . . . .
//! . . W W . # . . .
//! . . W[F]W . . . .
//! . . . . . . . . .
//! . . . . . . . . .
//! . . . . . . . . .
//! ```
//!
//! Finding a flagged node, it reverses direction and cleans the node:
//!
//! ```text
//! . . . . . . . . .
//! . . . . . . . . .
//! . . . . . . . . .
//! . . W W . # . . .
//! . .[W]. W . . . .
//! . . . . . . . . .
//! . . . . . . . . .
//! . . . . . . . . .
//! ```
//!
//! The weakened node becomes infected, and it continues in the same direction:
//!
//! ```text
//! . . . . . . . . .
//! . . . . . . . . .
//! . . . . . . . . .
//! . . W W . # . . .
//! .[.]# . W . . . .
//! . . . . . . . . .
//! . . . . . . . . .
//! . . . . . . . . .
//! ```
//!
//! Of the first 100 bursts, 26 will result in infection. Unfortunately, another feature of this evolved virus is speed; of the first 10000000 bursts, 2511944 will result in infection.
//!
//! Given your actual map, after 10000000 bursts of activity, how many bursts cause a node to become infected? (Do not count nodes that begin infected.)

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use console::style;
use crossterm::{
    cursor::{Hide, MoveToNextLine, RestorePosition, SavePosition, Show},
    style::Print,
    terminal::{Clear, ClearType},
    ExecutableCommand, QueueableCommand,
};
use getset::{Getters, Setters};
use std::{
    collections::HashMap,
    fmt,
    fs::File,
    io::{stdout, BufRead, BufReader, Write},
    sync::mpsc::channel,
    thread::{sleep, spawn},
    time::Duration,
};

/// The direction the virus is facing.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum Direction {
    /// Up
    #[default]
    Up,
    /// Down
    Down,
    /// Left
    Left,
    /// Right
    Right,
}

/// The current state of the coord.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(dead_code)]
enum State {
    /// Clean
    Clean,
    /// Weakened
    Weakened,
    /// Infected
    Infected,
    /// Flagged
    Flagged,
}

#[derive(Clone, Copy, Debug, Default, Eq, Getters, PartialEq, Setters)]
struct Virus {
    loc: (isize, isize),
    direction: Direction,
    #[getset(get)]
    infected_count: usize,
    #[getset(set)]
    second_star: bool,
}

impl Virus {
    fn new(loc: (isize, isize), direction: Direction) -> Self {
        Self {
            loc,
            direction,
            second_star: false,
            infected_count: 0,
        }
    }

    fn work(&mut self, v_map: &mut HashMap<(isize, isize), State>) -> Result<()> {
        let state = v_map
            .get_mut(&self.loc)
            .ok_or_else(|| anyhow!("Invalid location"))?;

        match state {
            State::Clean => {
                self.direction = match self.direction {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                };

                *state = if self.second_star {
                    State::Weakened
                } else {
                    self.infected_count += 1;
                    State::Infected
                };
            }
            State::Weakened => {
                self.infected_count += 1;
                *state = State::Infected;
            }
            State::Infected => {
                self.direction = match self.direction {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                };
                *state = if self.second_star {
                    State::Flagged
                } else {
                    State::Clean
                };
            }
            State::Flagged => {
                self.direction = match self.direction {
                    Direction::Up => Direction::Down,
                    Direction::Down => Direction::Up,
                    Direction::Left => Direction::Right,
                    Direction::Right => Direction::Left,
                };
                *state = State::Clean;
            }
        }

        let next_loc = match self.direction {
            Direction::Up => (self.loc.0, self.loc.1 - 1),
            Direction::Down => (self.loc.0, self.loc.1 + 1),
            Direction::Left => (self.loc.0 - 1, self.loc.1),
            Direction::Right => (self.loc.0 + 1, self.loc.1),
        };
        self.loc = next_loc;
        let _ = v_map.entry(next_loc).or_insert(State::Clean);
        Ok(())
    }
}

impl fmt::Display for Virus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Virus: loc: {:?}, direction: {:?}, infected_count: {}",
            self.loc, self.direction, self.infected_count
        )
    }
}

type VirusData = (HashMap<(isize, isize), State>, Virus, usize, bool);

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<VirusData, usize>(AoCYear::AOC2017, AoCDay::AOCD22, setup, find).map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<VirusData, usize>(bench, AoCYear::AOC2017, AoCDay::AOCD22, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> VirusData {
    setup_br(reader, 10_000, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T, burst: usize, test: bool) -> Result<VirusData>
where
    T: BufRead,
{
    let mut data = HashMap::new();
    let mut max_x = usize::MIN;
    let mut max_y = usize::MIN;

    for (y, line) in valid_lines(reader).enumerate() {
        if y > max_y {
            max_y = y;
        }
        for (x, ch) in line.chars().enumerate() {
            if x > max_x {
                max_x = x;
            }
            let x_i = isize::try_from(x)?;
            let y_i = isize::try_from(y)?;
            let _ = match ch {
                '.' => data.insert((x_i, y_i), State::Clean),
                '#' => data.insert((x_i, y_i), State::Infected),
                _ => return Err(anyhow!("Invalid character in input")),
            };
        }
    }
    let start_loc = (isize::try_from(max_x / 2)?, isize::try_from(max_y / 2)?);
    let virus = Virus::new(start_loc, Direction::Up);

    Ok((data, virus, burst, test))
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: VirusData) -> usize {
    match find_res(data, false) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("{e}");
            0
        }
    }
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(data: VirusData, second_star: bool) -> Result<usize> {
    let (mut v_map, mut virus, bursts, test) = data;
    let _ = virus.set_second_star(second_star);

    let (sender, receiver) =
        channel::<(HashMap<(isize, isize), State>, Virus, String, bool, bool)>();

    let hdl = spawn(move || {
        while let Ok((v_map, virus, header, restore, done)) = receiver.recv() {
            display_virus(&v_map, &virus, &header, restore).unwrap_or_default();
            if done {
                break;
            }
            sleep(Duration::from_micros(900));
        }
    });
    let mut stdout = stdout();
    let _ = stdout.execute(Hide)?;

    for burst in 0..bursts {
        virus.work(&mut v_map)?;
        if test && burst == 0 {
            sender.send((v_map.clone(), virus, String::new(), false, true))?;
        } else if burst < 10000 && !test {
            sender
                .send((
                    v_map.clone(),
                    virus,
                    format!("Burst {}", burst + 1),
                    burst < 9999,
                    (burst == bursts - 1 || burst == 9999),
                ))
                .unwrap_or_default();
        }
    }

    let _ = stdout.execute(Show)?;
    stdout.flush()?;

    let _res = hdl.join();
    Ok(virus.infected_count)
}

fn display_virus(
    v_map: &HashMap<(isize, isize), State>,
    virus: &Virus,
    header: &str,
    restore: bool,
) -> Result<()> {
    let mut stdout = stdout();

    let _ = stdout.execute(Hide)?;
    let _ = stdout.queue(SavePosition)?;
    let _ = stdout.queue(Clear(ClearType::CurrentLine))?;
    let _ = stdout.queue(Print(format!("{}", style(header).bold().yellow())))?;
    let _ = stdout.queue(MoveToNextLine(1))?;
    let _ = stdout.queue(MoveToNextLine(1))?;

    let min_x = v_map.keys().map(|(x, _)| x).min().unwrap_or(&0);
    let max_x = v_map.keys().map(|(x, _)| x).max().unwrap_or(&0);
    let min_y = v_map.keys().map(|(_, y)| y).min().unwrap_or(&0);
    let max_y = v_map.keys().map(|(_, y)| y).max().unwrap_or(&0);

    let v_loc = virus.loc;

    for i in *min_y..=*max_y {
        let _ = stdout.queue(Clear(ClearType::CurrentLine))?;
        for j in *min_x..=*max_x {
            let ch = if j == v_loc.0 && i == v_loc.1 {
                match virus.direction {
                    Direction::Up => '^',
                    Direction::Down => 'v',
                    Direction::Left => '<',
                    Direction::Right => '>',
                }
            } else {
                match v_map.get(&(j, i)) {
                    Some(State::Clean) | None => '.',
                    Some(State::Weakened) => 'W',
                    Some(State::Infected) => '#',
                    Some(State::Flagged) => 'F',
                }
            };
            match ch {
                'V' | '^' | '<' | '>' => {
                    let _ = stdout.queue(Print(format!("{}", style(ch).bold().yellow())))?;
                }
                '#' => {
                    let _ = stdout.queue(Print(format!("{}", style(ch).green())))?;
                }
                'W' => {
                    let _ = stdout.queue(Print(format!("{}", style(ch).blue())))?;
                }
                'F' => {
                    let _ = stdout.queue(Print(format!("{}", style(ch).magenta())))?;
                }
                _ => {
                    let _ = stdout.queue(Print(ch))?;
                }
            }
        }
        let _ = stdout.queue(MoveToNextLine(1))?;
    }
    let _ = stdout.queue(MoveToNextLine(1))?;
    if restore {
        let _ = stdout.queue(RestorePosition)?;
    }
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
    run_setup_solution::<VirusData, usize>(AoCYear::AOC2017, AoCDay::AOCD22, setup2, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<VirusData, usize>(bench, AoCYear::AOC2017, AoCDay::AOCD22, setup2, find2)
        .map(|_| 0)
}

fn setup2(reader: BufReader<File>) -> VirusData {
    setup_br(reader, 10_000_000, false).unwrap_or_default()
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: VirusData) -> usize {
    find_res(data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"..#
#..
...";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1), 7, true)?;
        assert_eq!(find(data), 5);
        let data = setup_br(Cursor::new(TEST_1), 70, true)?;
        assert_eq!(find(data), 41);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"..#
#..
...";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1), 100, true)?;
        assert_eq!(find2(data), 26);
        // let data = setup_br(Cursor::new(TEST_1), 10_000_000, true)?;
        // assert_eq!(find2(data), 2511944);
        Ok(())
    }
}
