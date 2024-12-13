// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2018 ---**
//!
//! **--- Day 3: No Matter How You Slice It ---**
//!
//! The Elves managed to locate the chimney-squeeze prototype fabric for Santa's suit (thanks to someone who helpfully wrote its box IDs on the wall of the warehouse in the middle of the night). Unfortunately, anomalies are still affecting them - nobody can even agree on how to cut the fabric.
//!
//! The whole piece of fabric they're working on is a very large square - at least 1000 inches on each side.
//!
//! Each Elf has made a claim about which area of fabric would be ideal for Santa's suit. All claims have an ID and consist of a single rectangle with edges parallel to the edges of the fabric. Each claim's rectangle is defined as follows:
//!
//! ```text
//!     The number of inches between the left edge of the fabric and the left edge of the rectangle.
//!     The number of inches between the top edge of the fabric and the top edge of the rectangle.
//!     The width of the rectangle in inches.
//!     The height of the rectangle in inches.
//! ```
//!
//! A claim like #123 @ 3,2: 5x4 means that claim ID 123 specifies a rectangle 3 inches from the left edge, 2 inches from the top edge, 5 inches wide, and 4 inches tall. Visually, it claims the square inches of fabric represented by # (and ignores the square inches of fabric represented by .) in the diagram below:
//!
//! ```text
//! ...........
//! ...........
//! ...#####...
//! ...#####...
//! ...#####...
//! ...#####...
//! ...........
//! ...........
//! ...........
//! ```
//!
//! The problem is that many of the claims overlap, causing two or more claims to cover part of the same areas. For example, consider the following claims:
//!
//! ```text
//! #1 @ 1,3: 4x4
//! #2 @ 3,1: 4x4
//! #3 @ 5,5: 2x2
//! ```
//!
//! Visually, these claim the following areas:
//!
//! ```text
//! ........
//! ...2222.
//! ...2222.
//! .11XX22.
//! .11XX22.
//! .111133.
//! .111133.
//! ........
//! ```
//!
//! The four square inches marked with X are claimed by both 1 and 2. (Claim 3, while adjacent to the others, does not overlap either of them.)
//!
//! If the Elves all proceed with their own plans, none of them will have enough fabric. How many square inches of fabric are within two or more claims?
//!
//! **--- Part Two ---**
//!
//! Amidst the chaos, you notice that exactly one claim doesn't overlap by even a single square inch of fabric with any other claim. If you can somehow draw attention to it, maybe the Elves will be able to make Santa's suit after all!
//!
//! For example, in the claims above, only claim 3 is intact after all claims are made.
//!
//! What is the ID of the only claim that doesn't overlap?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use ndarray::Array2;
use regex::Regex;
use std::{
    collections::BTreeMap,
    fmt,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[derive(Clone, Copy, Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.top_left, self.bottom_right)
    }
}

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<Vec<String>, u32>(AoCYear::AOC2018, AoCDay::AOCD03, setup, find).map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<String>, u32>(bench, AoCYear::AOC2018, AoCDay::AOCD03, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> Vec<String> {
    setup_br(reader).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T) -> Result<Vec<String>>
where
    T: BufRead,
{
    let mut data = vec![];
    for line in valid_lines(reader) {
        data.push(line);
    }
    Ok(data)
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: Vec<String>) -> u32 {
    find_res(data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(data: Vec<String>, second_star: bool) -> Result<u32> {
    let line_re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)")?;
    let mut rectangles = BTreeMap::new();

    for line in data {
        for cap in line_re.captures_iter(&line) {
            let id = (cap[1]).parse::<usize>()?;
            let l = (cap[2]).parse::<usize>()?;
            let t = (cap[3]).parse::<usize>()?;
            let w = (cap[4]).parse::<usize>()?;
            let h = (cap[5]).parse::<usize>()?;
            let top_left = Point { x: l, y: t };
            let bottom_right = Point {
                x: l + w - 1,
                y: t + h - 1,
            };
            let rectangle = Rectangle {
                top_left,
                bottom_right,
            };
            let _res = rectangles.insert(id, rectangle);
        }
    }

    if second_star {
        Ok(u32::try_from(find_non_overlaps(&rectangles)?)?)
    } else {
        Ok(check_points(&rectangles, 1000, 1000))
    }
}

fn check_points(all_claims: &BTreeMap<usize, Rectangle>, width: usize, height: usize) -> u32 {
    let mut cloth: Array2<u8> = Array2::zeros((width, height));

    for y in 0..height {
        for x in 0..width {
            let point = Point { x, y };

            for rectangle in all_claims.values() {
                if contains_point(*rectangle, point) {
                    if let Some(ps) = cloth.get_mut((x, y)) {
                        *ps += 1;
                    }
                }
            }
        }
    }

    let mut multi_count = 0;
    for ps in &cloth {
        match ps {
            0 | 1 => {}
            _ => multi_count += 1,
        }
    }

    multi_count
}

fn contains_point(rect: Rectangle, point: Point) -> bool {
    point.x >= rect.top_left.x
        && point.x <= rect.bottom_right.x
        && point.y >= rect.top_left.y
        && point.y <= rect.bottom_right.y
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<Vec<String>, u32>(AoCYear::AOC2018, AoCDay::AOCD03, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<String>, u32>(bench, AoCYear::AOC2018, AoCDay::AOCD03, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: Vec<String>) -> u32 {
    find_res(data, true).unwrap_or_default()
}

fn find_non_overlaps(rectangles: &BTreeMap<usize, Rectangle>) -> Result<usize> {
    for (id, rect1) in rectangles {
        let overlaps: Vec<Rectangle> = rectangles
            .iter()
            .filter(|(id2, _)| *id != **id2)
            .filter(|(_, rect2)| overlap(*rect1, **rect2))
            .map(|(_, v)| *v)
            .collect();

        if overlaps.is_empty() {
            return Ok(*id);
        }
    }

    Err(anyhow!("failed to find an non-overlapping rectangle"))
}

fn overlap(rect1: Rectangle, rect2: Rectangle) -> bool {
    // If one rectangle is on left side of other
    if rect1.top_left.x > rect2.bottom_right.x || rect2.top_left.x > rect1.bottom_right.x {
        return false;
    }

    // If one rectangle is above other
    if rect1.top_left.y > rect2.bottom_right.y || rect2.top_left.y > rect1.bottom_right.y {
        return false;
    }

    true
}

#[cfg(test)]
mod one_star {
    use crate::year2018::day03::{check_points, contains_point, Point, Rectangle};

    use std::collections::BTreeMap;

    #[test]
    fn solution() {
        let mut rectangles = BTreeMap::new();
        let rect1 = Rectangle {
            top_left: Point { x: 1, y: 3 },
            bottom_right: Point { x: 4, y: 6 },
        };
        let rect2 = Rectangle {
            top_left: Point { x: 3, y: 1 },
            bottom_right: Point { x: 6, y: 4 },
        };
        let rect3 = Rectangle {
            top_left: Point { x: 5, y: 5 },
            bottom_right: Point { x: 6, y: 6 },
        };
        let tl = Point { x: 3, y: 1 };
        let tr = Point { x: 6, y: 1 };
        let bl = Point { x: 3, y: 4 };
        let br = Point { x: 6, y: 4 };
        let inside = Point { x: 4, y: 3 };
        let outside = Point { x: 2, y: 2 };

        let _res = rectangles.insert(1, rect1);
        let _res = rectangles.insert(2, rect2);
        let _res = rectangles.insert(3, rect3);

        assert!(contains_point(rect2, tl));
        assert!(contains_point(rect2, tr));
        assert!(contains_point(rect2, bl));
        assert!(contains_point(rect2, br));
        assert!(contains_point(rect2, inside));
        assert!(!contains_point(rect2, outside));

        assert_eq!(check_points(&rectangles, 8, 8), 4);
    }
}

#[cfg(test)]
mod two_star {
    use crate::year2018::day03::{find_non_overlaps, Point, Rectangle};

    use anyhow::Result;
    use std::collections::BTreeMap;

    #[test]
    fn solution() -> Result<()> {
        let mut rectangles = BTreeMap::new();
        let rect1 = Rectangle {
            top_left: Point { x: 1, y: 3 },
            bottom_right: Point { x: 4, y: 6 },
        };
        let rect2 = Rectangle {
            top_left: Point { x: 3, y: 1 },
            bottom_right: Point { x: 6, y: 4 },
        };
        let rect3 = Rectangle {
            top_left: Point { x: 5, y: 5 },
            bottom_right: Point { x: 6, y: 6 },
        };

        let _res = rectangles.insert(1, rect1);
        let _res = rectangles.insert(2, rect2);
        let _res = rectangles.insert(3, rect3);

        assert_eq!(find_non_overlaps(&rectangles)?, 3);
        Ok(())
    }
}
