// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2018 ---**
//!
//! **--- Day 17: Reservoir Research ---**
//!
//! You arrive in the year 18. If it weren't for the coat you got in 1018, you would be very cold: the North Pole base hasn't even been constructed.
//!
//! Rather, it hasn't been constructed yet. The Elves are making a little progress, but there's not a lot of liquid water in this climate, so they're getting very dehydrated. Maybe there's more underground?
//!
//! You scan a two-dimensional vertical slice of the ground nearby and discover that it is mostly sand with veins of clay. The scan only provides data with a granularity of square meters, but it should be good enough to determine how much water is trapped there. In the scan, x represents the distance to the right, and y represents the distance down. There is also a spring of water near the surface at x=500, y=0. The scan identifies which square meters are clay (your puzzle input).
//!
//! For example, suppose your scan shows the following veins of clay:
//!
//! ```text
//! x=495, y=2..7
//! y=7, x=495..501
//! x=501, y=3..7
//! x=498, y=2..4
//! x=506, y=1..2
//! x=498, y=10..13
//! x=504, y=10..13
//! y=13, x=498..504
//! ```
//!
//! Rendering clay as #, sand as ., and the water spring as +, and with x increasing to the right and y increasing downward, this becomes:
//!
//! ```text
//!    44444455555555
//!    99999900000000
//!    45678901234567
//!  0 ......+.......
//!  1 ............#.
//!  2 .#..#.......#.
//!  3 .#..#..#......
//!  4 .#..#..#......
//!  5 .#.....#......
//!  6 .#.....#......
//!  7 .#######......
//!  8 ..............
//!  9 ..............
//! 10 ....#.....#...
//! 11 ....#.....#...
//! 12 ....#.....#...
//! 13 ....#######...
//! ```
//!
//! The spring of water will produce water forever. Water can move through sand, but is blocked by clay. Water always moves down when possible, and spreads to the left and right otherwise, filling space that has clay on both sides and falling out otherwise.
//!
//! For example, if five squares of water are created, they will flow downward until they reach the clay and settle there. Water that has come to rest is shown here as ~, while sand through which water has passed (but which is now dry again) is shown as |:
//!
//! ```text
//! ......+.......
//! ......|.....#.
//! .#..#.|.....#.
//! .#..#.|#......
//! .#..#.|#......
//! .#....|#......
//! .#~~~~~#......
//! .#######......
//! ..............
//! ..............
//! ....#.....#...
//! ....#.....#...
//! ....#.....#...
//! ....#######...
//! ```
//!
//! Two squares of water can't occupy the same location. If another five squares of water are created, they will settle on the first five, filling the clay reservoir a little more:
//!
//! ```text
//! ......+.......
//! ......|.....#.
//! .#..#.|.....#.
//! .#..#.|#......
//! .#..#.|#......
//! .#~~~~~#......
//! .#~~~~~#......
//! .#######......
//! ..............
//! ..............
//! ....#.....#...
//! ....#.....#...
//! ....#.....#...
//! ....#######...
//! ```
//!
//! Water pressure does not apply in this scenario. If another four squares of water are created, they will stay on the right side of the barrier, and no water will reach the left side:
//!
//! ```text
//! ......+.......
//! ......|.....#.
//! .#..#.|.....#.
//! .#..#~~#......
//! .#..#~~#......
//! .#~~~~~#......
//! .#~~~~~#......
//! .#######......
//! ..............
//! ..............
//! ....#.....#...
//! ....#.....#...
//! ....#.....#...
//! ....#######...
//! ```
//!
//! At this point, the top reservoir overflows. While water can reach the tiles above the surface of the water, it cannot settle there, and so the next five squares of water settle like this:
//!
//! ```text
//! ......+.......
//! ......|.....#.
//! .#..#||||...#.
//! .#..#~~#|.....
//! .#..#~~#|.....
//! .#~~~~~#|.....
//! .#~~~~~#|.....
//! .#######|.....
//! ........|.....
//! ........|.....
//! ....#...|.#...
//! ....#...|.#...
//! ....#~~~~~#...
//! ....#######...
//! ```
//!
//! Note especially the leftmost |: the new squares of water can reach this tile, but cannot stop there. Instead, eventually, they all fall to the right and settle in the reservoir below.
//!
//! After 10 more squares of water, the bottom reservoir is also full:
//!
//! ```text
//! ......+.......
//! ......|.....#.
//! .#..#||||...#.
//! .#..#~~#|.....
//! .#..#~~#|.....
//! .#~~~~~#|.....
//! .#~~~~~#|.....
//! .#######|.....
//! ........|.....
//! ........|.....
//! ....#~~~~~#...
//! ....#~~~~~#...
//! ....#~~~~~#...
//! ....#######...
//! ```
//!
//! Finally, while there is nowhere left for the water to settle, it can reach a few more tiles before overflowing beyond the bottom of the scanned data:
//!
//! ```text
//! ......+.......    (line not counted: above minimum y value)
//! ......|.....#.
//! .#..#||||...#.
//! .#..#~~#|.....
//! .#..#~~#|.....
//! .#~~~~~#|.....
//! .#~~~~~#|.....
//! .#######|.....
//! ........|.....
//! ...|||||||||..
//! ...|#~~~~~#|..
//! ...|#~~~~~#|..
//! ...|#~~~~~#|..
//! ...|#######|..
//! ...|.......|..    (line not counted: below maximum y value)
//! ...|.......|..    (line not counted: below maximum y value)
//! ...|.......|..    (line not counted: below maximum y value)
//! ```
//!
//! How many tiles can be reached by the water? To prevent counting forever, ignore tiles with a y coordinate smaller than the smallest y coordinate in your scan data or larger than the largest one. Any x coordinate is valid. In this example, the lowest y coordinate given is 1, and the highest is 13, causing the water spring (in row 0) and the water falling off the bottom of the render (in rows 14 through infinity) to be ignored.
//!
//! So, in the example above, counting both water at rest (~) and other sand tiles the water can hypothetically reach (|), the total number of tiles the water can reach is 57.
//!
//! How many tiles can the water reach within the range of y values in your scan?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use console::style;
use crossterm::{
    cursor::{Hide, MoveToNextLine, RestorePosition, SavePosition, Show},
    style::{Attribute, Color, Stylize},
    terminal::{Clear, ClearType},
    ExecutableCommand, QueueableCommand,
};
use ndarray::{Array2, Axis};
use regex::Regex;
use std::{
    collections::HashMap,
    fmt,
    fs::File,
    io::{stdout, BufRead, BufReader, Write},
};

#[allow(dead_code)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
enum SoilKind {
    Clay,
    FlowingWater,
    #[default]
    Sand,
    SettledWater,
    Spring,
}

impl fmt::Display for SoilKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SoilKind::Clay => "#",
                SoilKind::FlowingWater => "|",
                SoilKind::Sand => ".",
                SoilKind::SettledWater => "~",
                SoilKind::Spring => "+",
            }
        )
    }
}

type ClayData = (HashMap<usize, Vec<usize>>, HashMap<usize, Vec<usize>>);

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<ClayData, usize>(AoCYear::AOC2018, AoCDay::AOCD17, setup, find).map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<ClayData, usize>(bench, AoCYear::AOC2018, AoCDay::AOCD17, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> ClayData {
    setup_br(reader).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T) -> Result<ClayData>
where
    T: BufRead,
{
    let vein_re = Regex::new(r"(x|y)=(\d+), (x|y)=(\d+)\.\.(\d+)")?;
    let mut x_coord_map = HashMap::new();
    let mut y_coord_map = HashMap::new();

    for line in valid_lines(reader) {
        for caps in vein_re.captures_iter(&line) {
            let c1 = (caps[1]).to_string();
            let v1 = (caps[2]).parse::<usize>()?;
            let r1 = (caps[4]).parse::<usize>()?;
            let r2 = (caps[5]).parse::<usize>()?;

            match &c1[..] {
                "x" => {
                    let range_vec = x_coord_map.entry(v1).or_insert_with(Vec::new);

                    for i in r1..=r2 {
                        range_vec.push(i);
                    }
                }
                "y" => {
                    let range_vec = y_coord_map.entry(v1).or_insert_with(Vec::new);

                    for i in r1..=r2 {
                        range_vec.push(i);
                    }
                }
                _ => return Err(anyhow!("invalid coordinate")),
            }
        }
    }
    Ok((x_coord_map, y_coord_map))
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: ClayData) -> usize {
    find_res(data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(data: ClayData, _second_star: bool) -> Result<usize> {
    let (mut x_coord_map, mut y_coord_map) = data;
    let (min_x, max_x, min_y, max_y, shift_start) =
        calculate_mins_maxes(&x_coord_map, &y_coord_map)?;
    eprintln!("min_x: {min_x}, max_x: {max_x}, min_y: {min_y}, max_y: {max_y}");
    let spring = (500 - shift_start, 0_usize);
    eprintln!("spring: ({},{})", spring.0, spring.1);

    let final_x: HashMap<usize, Vec<usize>> = x_coord_map
        .drain()
        .map(|(k, v)| (k - shift_start, v))
        .collect();
    let final_y: HashMap<usize, Vec<usize>> = y_coord_map
        .drain()
        .map(|(k, v)| (k, v.iter().map(|x| x - shift_start).collect()))
        .collect();

    eprintln!("final_x: {final_x:?}");
    eprintln!("final_y: {final_y:?}");
    let mut spring_arr = Array2::<SoilKind>::default((max_x, max_y));
    spring_arr[[spring.0, spring.1]] = SoilKind::Spring;

    for (x, yv) in final_x {
        for y in yv {
            spring_arr[[x, y]] = SoilKind::Clay;
        }
    }
    for (y, xv) in final_y {
        for x in xv {
            spring_arr[[x, y]] = SoilKind::Clay;
        }
    }

    display_spring(&spring_arr, false, "Initial State:", true)?;
    Ok(0)
}
#[allow(clippy::similar_names)]
fn calculate_mins_maxes(
    x_coord_map: &HashMap<usize, Vec<usize>>,
    y_coord_map: &HashMap<usize, Vec<usize>>,
) -> Result<(usize, usize, usize, usize, usize)> {
    let mut min_x_key = *x_coord_map.keys().min().ok_or(anyhow!("no min x"))?;
    let mut max_x_key = *x_coord_map.keys().max().ok_or(anyhow!("no max x"))?;
    let mut min_y_key = 1;
    let mut max_y_key = *y_coord_map.keys().max().ok_or(anyhow!("no max y"))?;

    for yv in x_coord_map.values() {
        for y in yv {
            if *y > max_y_key {
                max_y_key = *y;
            }

            if *y < min_y_key {
                min_y_key = *y;
            }
        }
    }

    for xv in y_coord_map.values() {
        for x in xv {
            if *x > max_x_key {
                max_x_key = *x;
            }

            if *x < min_x_key {
                min_x_key = *x;
            }
        }
    }

    min_x_key = min_x_key.checked_sub(1).ok_or(anyhow!("underflow x"))?;
    max_x_key = max_x_key.checked_add(2).ok_or(anyhow!("overflow x"))?;
    max_y_key = max_y_key.checked_add(1).ok_or(anyhow!("overflow y"))?;

    Ok((0, max_x_key - min_x_key, min_y_key, max_y_key, min_x_key))
}

fn display_spring(
    data: &Array2<SoilKind>,
    restore: bool,
    header: &str,
    display: bool,
) -> Result<()> {
    if display {
        let mut stdout = stdout();

        let _ = stdout.execute(Hide)?;
        let _ = stdout.queue(SavePosition)?;
        let _ = stdout.queue(Clear(ClearType::CurrentLine))?;
        let _ = stdout.write(format!("{}", style(header).bold().yellow()).as_bytes())?;
        let _ = stdout.queue(MoveToNextLine(1))?;
        let _ = stdout.queue(MoveToNextLine(1))?;
        for row in data.axis_iter(Axis(1)) {
            for elem in row {
                match elem {
                    SoilKind::Clay => {
                        let _ = stdout.write(
                            format!(
                                "{}",
                                format!("{elem}")
                                    .with(Color::Rgb {
                                        r: 102,
                                        g: 76,
                                        b: 40
                                    })
                                    .attribute(Attribute::Bold)
                            )
                            .as_bytes(),
                        )?;
                    }
                    SoilKind::FlowingWater => {
                        let _ =
                            stdout.write(format!("{}", style(elem).bold().blue()).as_bytes())?;
                    }
                    SoilKind::Sand => {
                        let _ = stdout.write(
                            format!(
                                "{}",
                                format!("{elem}")
                                    .with(Color::Rgb {
                                        r: 255,
                                        g: 248,
                                        b: 231
                                    })
                                    .attribute(Attribute::Bold)
                            )
                            .as_bytes(),
                        )?;
                    }
                    SoilKind::SettledWater => {
                        let _ =
                            stdout.write(format!("{}", style(elem).bold().blue()).as_bytes())?;
                    }
                    SoilKind::Spring => {
                        let _ =
                            stdout.write(format!("{}", style(elem).bold().magenta()).as_bytes())?;
                    }
                }
            }
            let _ = stdout.queue(MoveToNextLine(1))?;
        }
        if restore {
            let _ = stdout.queue(RestorePosition)?;
        }
        let _ = stdout.execute(Show)?;
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
    run_setup_solution::<ClayData, usize>(AoCYear::AOC2018, AoCDay::AOCD17, setup, find2).map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<ClayData, usize>(bench, AoCYear::AOC2018, AoCDay::AOCD17, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: ClayData) -> usize {
    find_res(data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";

    const TEST_2: &str = r"x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..520";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find(data), 0);
        let data = setup_br(Cursor::new(TEST_2))?;
        assert_eq!(find(data), 0);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r">";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find2(data), 0);
        Ok(())
    }
}
