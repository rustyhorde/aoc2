// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **-- Advent of Code - Day 12 --**
//!
//! **--- Day 12: Garden Groups ---**
//!
//! Why not search for the Chief Historian near the gardener and his massive farm? There's plenty of food, so The Historians grab something to eat while they search.
//!
//! You're about to settle near a complex arrangement of garden plots when some Elves ask if you can lend a hand. They'd like to set up fences around each region of garden plots, but they can't figure out how much fence they need to order or how much it will cost. They hand you a map (your puzzle input) of the garden plots.
//!
//! Each garden plot grows only a single type of plant and is indicated by a single letter on your map. When multiple garden plots are growing the same type of plant and are touching (horizontally or vertically), they form a region. For example:
//!
//! ```text
//! AAAA
//! BBCD
//! BBCC
//! EEEC
//! ```
//!
//! This 4x4 arrangement includes garden plots growing five different types of plants (labeled A, B, C, D, and E), each grouped into their own region.
//!
//! In order to accurately calculate the cost of the fence around a single region, you need to know that region's area and perimeter.
//!
//! The area of a region is simply the number of garden plots the region contains. The above map's type A, B, and C plants are each in a region of area 4. The type E plants are in a region of area 3; the type D plants are in a region of area 1.
//!
//! Each garden plot is a square and so has four sides. The perimeter of a region is the number of sides of garden plots in the region that do not touch another garden plot in the same region. The type A and C plants are each in a region with perimeter 10. The type B and E plants are each in a region with perimeter 8. The lone D plot forms its own region with perimeter 4.
//!
//! Visually indicating the sides of plots in each region that contribute to the perimeter using - and |, the above map's regions' perimeters are measured as follows:
//!
//! ```text
//! +-+-+-+-+
//! |A A A A|
//! +-+-+-+-+     +-+
//!               |D|
//! +-+-+   +-+   +-+
//! |B B|   |C|
//! +   +   + +-+
//! |B B|   |C C|
//! +-+-+   +-+ +
//!           |C|
//! +-+-+-+   +-+
//! |E E E|
//! +-+-+-+
//! ```
//!
//! Plants of the same type can appear in multiple separate regions, and regions can even appear within other regions. For example:
//!
//! ```text
//! OOOOO
//! OXOXO
//! OOOOO
//! OXOXO
//! OOOOO
//! ```
//!
//! The above map contains five regions, one containing all of the O garden plots, and the other four each containing a single X plot.
//!
//! The four X regions each have area 1 and perimeter 4. The region containing 21 type O plants is more complicated; in addition to its outer edge contributing a perimeter of 20, its boundary with each X region contributes an additional 4 to its perimeter, for a total perimeter of 36.
//!
//! Due to "modern" business practices, the price of fence required for a region is found by multiplying that region's area by its perimeter. The total price of fencing all regions on a map is found by adding together the price of fence for every region on the map.
//!
//! In the first example, region A has price 4 * 10 = 40, region B has price 4 * 8 = 32, region C has price 4 * 10 = 40, region D has price 1 * 4 = 4, and region E has price 3 * 8 = 24. So, the total price for the first example is 140.
//!
//! In the second example, the region with all of the O plants has price 21 * 36 = 756, and each of the four smaller X regions has price 1 * 4 = 4, for a total price of 772 (756 + 4 + 4 + 4 + 4).
//!
//! Here's a larger example:
//!
//! ```text
//! RRRRIICCFF
//! RRRRIICCCF
//! VVRRRCCFFF
//! VVRCCCJFFF
//! VVVVCJJCFE
//! VVIVCCJJEE
//! VVIIICJJEE
//! MIIIIIJJEE
//! MIIISIJEEE
//! MMMISSJEEE
//! ```
//!
//! It contains:
//!
//! ```text
//!     A region of R plants with price 12 * 18 = 216.
//!     A region of I plants with price 4 * 8 = 32.
//!     A region of C plants with price 14 * 28 = 392.
//!     A region of F plants with price 10 * 18 = 180.
//!     A region of V plants with price 13 * 20 = 260.
//!     A region of J plants with price 11 * 20 = 220.
//!     A region of C plants with price 1 * 4 = 4.
//!     A region of E plants with price 13 * 18 = 234.
//!     A region of I plants with price 14 * 22 = 308.
//!     A region of M plants with price 5 * 12 = 60.
//!     A region of S plants with price 3 * 8 = 24.
//! ```
//!
//! So, it has a total price of 1930.
//!
//! What is the total price of fencing all regions on your map?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::Result;
use std::{
    collections::{BTreeMap, HashMap, VecDeque},
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
    run_setup_solution::<Vec<Vec<char>>, usize>(AoCYear::AOC2024, AoCDay::AOCD12, setup, find)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<Vec<char>>, usize>(
        bench,
        AoCYear::AOC2024,
        AoCDay::AOCD12,
        setup,
        find,
    )
    .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> Vec<Vec<char>> {
    setup_br(reader).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T) -> Result<Vec<Vec<char>>>
where
    T: BufRead,
{
    let mut matrix = vec![];
    for row in valid_lines(reader) {
        matrix.push(row.chars().collect());
    }
    Ok(matrix)
}

// fn within_one(plots: &[(isize, isize)], plot_to_check: &(isize, isize)) -> bool {
//     let mut close = false;
//     let (ox, oy) = plot_to_check;
//     for (x, y) in plots {
//         if (x - ox).abs() + (y - oy).abs() == 1 {
//             close = true;
//             break;
//         }
//     }
//     close
// }

// fn combine_touching_lots(left: &[(isize, isize)], right: &[(isize, isize)]) -> bool {
//     let mut close = false;
//     for ((x, y), (ox, oy)) in left.iter().cartesian_product(right) {
//         if (x - ox).abs() + (y - oy).abs() == 1 {
//             close = true;
//             break;
//         }
//     }
//     close
// }

fn search_neighbors(
    queue: &mut VecDeque<(isize, isize)>,
    plots: &mut Vec<(isize, isize)>,
    initial_plot: (isize, isize),
) {
    let (x, y) = initial_plot;
    plots.push(initial_plot);

    if let Ok(idx) = queue.binary_search(&(x - 1, y)) {
        if let Some(blah) = queue.remove(idx) {
            search_neighbors(queue, plots, blah);
        }
    }
    if let Ok(idx) = queue.binary_search(&(x + 1, y)) {
        if let Some(blah) = queue.remove(idx) {
            search_neighbors(queue, plots, blah);
        }
    }
    if let Ok(idx) = queue.binary_search(&(x, y - 1)) {
        if let Some(blah) = queue.remove(idx) {
            search_neighbors(queue, plots, blah);
        }
    }
    if let Ok(idx) = queue.binary_search(&(x, y + 1)) {
        if let Some(blah) = queue.remove(idx) {
            search_neighbors(queue, plots, blah);
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn find(matrix: Vec<Vec<char>>) -> usize {
    let mut garden_map = BTreeMap::new();
    for (x, cols) in matrix.iter().enumerate() {
        for (y, col) in cols.iter().enumerate() {
            if let Some((ix, iy)) = isize::try_from(x).ok().zip(isize::try_from(y).ok()) {
                let _ = garden_map
                    .entry(*col)
                    .and_modify(|locs: &mut Vec<(isize, isize)>| locs.push((ix, iy)))
                    .or_insert(vec![(ix, iy)]);
            }
        }
    }

    let mut plots_map: HashMap<char, Vec<Vec<(isize, isize)>>> = HashMap::new();
    for (plot_name, plots) in garden_map {
        let mut sorted_plots = plots;
        sorted_plots.sort_unstable();

        let mut queue: VecDeque<(isize, isize)> = VecDeque::new();
        queue.extend(sorted_plots);

        let mut plots_vec = vec![];

        while let Some(initial_plot) = queue.pop_front() {
            let mut plots = vec![];
            search_neighbors(&mut queue, &mut plots, initial_plot);
            plots_vec.push(plots);
        }

        let _res = plots_map.insert(plot_name, plots_vec);
    }
    // eprintln!("plots_map: {plots_map:?}");

    let mut total_cost = 0;
    for (_k, plots) in plots_map {
        for plot in plots {
            let mut area = plot.len();
            let mut perimeter = 0;

            for (row, col) in &plot {
                let mut my_partners = 0;
                // Check up
                if let Some(nrow) = row.checked_sub(1) {
                    if plot.contains(&(nrow, *col)) {
                        my_partners += 1;
                    }
                }

                // Check down
                if plot.contains(&(row + 1, *col)) {
                    my_partners += 1;
                }

                // Check left
                if let Some(ncol) = col.checked_sub(1) {
                    if plot.contains(&(*row, ncol)) {
                        my_partners += 1;
                    }
                }

                // Check right
                if plot.contains(&(*row, col + 1)) {
                    my_partners += 1;
                }

                // if 1 partner perimeter += 3
                // if 2 partners perimeter += 2
                // if 3 partners perimeter += 1
                // if 4 partners perimeter += 0
                if my_partners == 0 {
                    area = 1;
                    perimeter += 4;
                } else if my_partners == 1 {
                    perimeter += 3;
                } else if my_partners == 2 {
                    perimeter += 2;
                } else if my_partners == 3 {
                    perimeter += 1;
                }
            }
            // eprintln!("{k}: Area {area}, Perimiter {perimeter}");
            total_cost += area * perimeter;
        }
    }
    total_cost
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<Vec<Vec<char>>, usize>(AoCYear::AOC2024, AoCDay::AOCD12, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<Vec<Vec<char>>, usize>(
        bench,
        AoCYear::AOC2024,
        AoCDay::AOCD12,
        setup,
        find2,
    )
    .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(_data: Vec<Vec<char>>) -> usize {
    0
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"AAAA
BBCD
BBCC
EEEC";
    const TEST_2: &str = r"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
    const TEST_3: &str = r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find(data), 140);
        let data = setup_br(Cursor::new(TEST_2))?;
        assert_eq!(find(data), 772);
        let data = setup_br(Cursor::new(TEST_3))?;
        assert_eq!(find(data), 1930);
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
