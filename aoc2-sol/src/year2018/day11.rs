// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2018 ---**
//!
//! **--- Day 11: Chronal Charge ---**
//!
//! You watch the Elves and their sleigh fade into the distance as they head toward the North Pole.
//!
//! Actually, you're the one fading. The falling sensation returns.
//!
//! The low fuel warning light is illuminated on your wrist-mounted device. Tapping it once causes it to project a hologram of the situation: a 300x300 grid of fuel cells and their current power levels, some negative. You're not sure what negative power means in the context of time travel, but it can't be good.
//!
//! Each fuel cell has a coordinate ranging from 1 to 300 in both the X (horizontal) and Y (vertical) direction. In X,Y notation, the top-left cell is 1,1, and the top-right cell is 300,1.
//!
//! The interface lets you select any 3x3 square of fuel cells. To increase your chances of getting to your destination, you decide to choose the 3x3 square with the largest total power.
//!
//! The power level in a given fuel cell can be found through the following process:
//!
//! ```text
//!     Find the fuel cell's rack ID, which is its X coordinate plus 10.
//!     Begin with a power level of the rack ID times the Y coordinate.
//!     Increase the power level by the value of the grid serial number (your puzzle input).
//!     Set the power level to itself multiplied by the rack ID.
//!     Keep only the hundreds digit of the power level (so 12345 becomes 3; numbers with no hundreds digit become 0).
//!     Subtract 5 from the power level.
//! ```
//!
//! For example, to find the power level of the fuel cell at 3,5 in a grid with serial number 8:
//!
//! ```text
//!     The rack ID is 3 + 10 = 13.
//!     The power level starts at 13 * 5 = 65.
//!     Adding the serial number produces 65 + 8 = 73.
//!     Multiplying by the rack ID produces 73 * 13 = 949.
//!     The hundreds digit of 949 is 9.
//!     Subtracting 5 produces 9 - 5 = 4.
//! ```
//!
//! So, the power level of this fuel cell is 4.
//!
//! Here are some more example power levels:
//!
//! ```text
//!     Fuel cell at  122,79, grid serial number 57: power level -5.
//!     Fuel cell at 217,196, grid serial number 39: power level  0.
//!     Fuel cell at 101,153, grid serial number 71: power level  4.
//! ```
//!
//! Your goal is to find the 3x3 square which has the largest total power. The square must be entirely within the 300x300 grid. Identify this square using the X,Y coordinate of its top-left fuel cell. For example:
//!
//! For grid serial number 18, the largest total 3x3 square has a top-left corner of 33,45 (with a total power of 29); these fuel cells appear in the middle of this 5x5 region:
//!
//! ```text
//! -2  -4   4   4   4
//! -4   4   4   4  -5
//!  4   3   3   4  -4
//!  1   1   2   4  -3
//! -1   0   2  -5  -2
//! ```
//!
//! For grid serial number 42, the largest 3x3 square's top-left is 21,61 (with a total power of 30); they are in the middle of this region:
//!
//! ```text
//! -3   4   2   2   2
//! -4   4   3   3   4
//! -5   3   3   4  -4
//!  4   3   3   4  -3
//!  3   3   3  -5  -1
//! ```
//!
//! What is the X,Y coordinate of the top-left fuel cell of the 3x3 square with the largest total power?
//!
//! **--- Part Two ---**
//!
//! You discover a dial on the side of the device; it seems to let you select a square of any size, not just 3x3. Sizes from 1x1 to 300x300 are supported.
//!
//! Realizing this, you now must find the square of any size with the largest total power. Identify this square by including its size as a third parameter after the top-left coordinate: a 9x9 square with a top-left corner of 3,5 is identified as 3,5,9.
//!
//! For example:
//!
//! ```text
//!     For grid serial number 18, the largest total square (with a total power of 113) is 16x16 and has a top-left corner of 90,269, so its identifier is 90,269,16.
//!     For grid serial number 42, the largest total square (with a total power of 119) is 12x12 and has a top-left corner of 232,251, so its identifier is 232,251,12.
//! ```
//!
//! What is the X,Y,size identifier of the square with the largest total power?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::Result;
use ndarray::Array2;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::{DivAssign, Rem},
};

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<usize, String>(AoCYear::AOC2018, AoCDay::AOCD11, setup, find).map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<usize, String>(bench, AoCYear::AOC2018, AoCDay::AOCD11, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> usize {
    setup_br(reader).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let mut serial_number = 0;
    for line in valid_lines(reader) {
        serial_number = line.parse::<usize>()?;
    }
    Ok(serial_number)
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: usize) -> String {
    find_res(data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(serial_number: usize, second_star: bool) -> Result<String> {
    let (x, y, size, power_level) = submatrix_sum_queries(serial_number, second_star)?;
    let res = if second_star {
        format!("{x},{y},{size} : {power_level}")
    } else {
        format!("{x},{y} : {power_level}")
    };
    Ok(res)
}

fn submatrix_sum_queries(
    serial_number: usize,
    second_star: bool,
) -> Result<(usize, usize, usize, isize)> {
    let mut power_levels: Array2<isize> = Array2::zeros((300, 300));

    for x in 0..300 {
        for y in 0..300 {
            power_levels[[x, y]] = find_cell_power(x + 1, y + 1, serial_number)?;
        }
    }

    let mut max_power_level = isize::MIN;
    let mut max_cell = (0, 0, 0, 0);

    if second_star {
        for size in 1..300 {
            submatrix_sum_query(&power_levels, size, &mut max_power_level, &mut max_cell);
        }
    } else {
        submatrix_sum_query(&power_levels, 3, &mut max_power_level, &mut max_cell);
    }

    Ok(max_cell)
}

fn find_cell_power(x: usize, y: usize, serial_number: usize) -> Result<isize> {
    let rack_id = x + 10;
    let mut power_level = rack_id * y;
    power_level += serial_number;
    power_level *= rack_id;

    let new_pl = if power_level > 99 {
        let digits_iter = digits(power_level);
        let mut hundreds = 0;
        let mut count = 3;
        for digit in digits_iter {
            hundreds = digit;
            count -= 1;
            if count == 0 {
                break;
            }
        }
        hundreds
    } else {
        0
    };
    let signed_pl = isize::try_from(new_pl)? - 5;
    Ok(signed_pl)
}

fn digits<T>(mut x: T) -> impl Iterator<Item = T>
where
    T: From<u8> + Copy + PartialOrd + DivAssign + Rem<Output = T>,
{
    let zero = T::from(0);
    let ten = T::from(10);

    let iter = std::iter::from_fn(move || {
        if x > zero {
            let digit = x % ten;
            x /= ten;
            Some(digit)
        } else {
            None
        }
    });

    (x == zero).then_some(zero).into_iter().chain(iter)
}

fn submatrix_sum_query(
    power_levels: &Array2<isize>,
    size: usize,
    max_power_level: &mut isize,
    max_cell: &mut (usize, usize, usize, isize),
) {
    for i in 0..=(300 - size) {
        for j in 0..=(300 - size) {
            let result = sum_query(power_levels, i, j, size);

            if result > *max_power_level {
                *max_power_level = result;
                max_cell.0 = i + 1;
                max_cell.1 = j + 1;
                max_cell.2 = size;
                max_cell.3 = result;
            }
        }
    }
}

fn sum_query(aux: &Array2<isize>, i: usize, j: usize, size: usize) -> isize {
    let mut res = 0;
    for x in i..i + size {
        for y in j..j + size {
            res += aux[[x, y]];
        }
    }
    res
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<usize, String>(AoCYear::AOC2018, AoCDay::AOCD11, setup, find2).map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<usize, String>(bench, AoCYear::AOC2018, AoCDay::AOCD11, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: usize) -> String {
    find_res(data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {
    use super::{find_cell_power, submatrix_sum_queries};
    use anyhow::Result;

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_cell_power(3, 5, 8)?, 4);
        assert_eq!(find_cell_power(122, 79, 57)?, -5);
        assert_eq!(find_cell_power(217, 196, 39)?, 0);
        assert_eq!(find_cell_power(101, 153, 71)?, 4);
        assert_eq!(submatrix_sum_queries(18, false)?, (33, 45, 3, 29));
        assert_eq!(submatrix_sum_queries(42, false)?, (21, 61, 3, 30));
        assert_eq!(submatrix_sum_queries(7511, false)?, (21, 22, 3, 34));
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::submatrix_sum_queries;
    use anyhow::Result;

    #[test]
    #[ignore = "This test takes a long time to run"]
    fn solution() -> Result<()> {
        assert_eq!(submatrix_sum_queries(18, true)?, (90, 269, 16, 113));
        assert_eq!(submatrix_sum_queries(42, true)?, (232, 251, 12, 119));
        Ok(())
    }
}
