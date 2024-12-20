// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2019 ---**
//!
//! **--- Day 6: Universal Orbit Map ---**
//!
//! You've landed at the Universal Orbit Map facility on Mercury. Because navigation in space often involves transferring between orbits, the orbit maps here are useful for finding efficient routes between, for example, you and Santa. You download a map of the local orbits (your puzzle input).
//!
//! Except for the universal Center of Mass (COM), every object in space is in orbit around exactly one other object. An orbit looks roughly like this:
//!
//! ```text
//!                   \
//!                    \
//!                     |
//!                     |
//! AAA--> o            o <--BBB
//!                     |
//!                     |
//!                    /
//!                   /
//! ```
//!
//! In this diagram, the object BBB is in orbit around AAA. The path that BBB takes around AAA (drawn with lines) is only partly shown. In the map data, this orbital relationship is written AAA)BBB, which means "BBB is in orbit around AAA".
//!
//! Before you use your map data to plot a course, you need to make sure it wasn't corrupted during the download. To verify maps, the Universal Orbit Map facility uses orbit count checksums - the total number of direct orbits (like the one shown above) and indirect orbits.
//!
//! Whenever A orbits B and B orbits C, then A indirectly orbits C. This chain can be any number of objects long: if A orbits B, B orbits C, and C orbits D, then A indirectly orbits D.
//!
//! For example, suppose you have the following map:
//!
//! ```text
//! COM)B
//! B)C
//! C)D
//! D)E
//! E)F
//! B)G
//! G)H
//! D)I
//! E)J
//! J)K
//! K)L
//! ```
//!
//! Visually, the above map of orbits looks like this:
//!
//! ```text
//!         G - H       J - K - L
//!        /           /
//! COM - B - C - D - E - F
//!                \
//!                 I
//! ```
//!
//! In this visual representation, when two objects are connected by a line, the one on the right directly orbits the one on the left.
//!
//! Here, we can count the total number of orbits as follows:
//!
//! ```text
//!     D directly orbits C and indirectly orbits B and COM, a total of 3 orbits.
//!     L directly orbits K and indirectly orbits J, E, D, C, B, and COM, a total of 7 orbits.
//!     COM orbits nothing.
//! ```
//!
//! The total number of direct and indirect orbits in this example is 42.
//!
//! What is the total number of direct and indirect orbits in your map data?
//!
//! **--- Part Two ---**
//!
//! Now, you just need to figure out how many orbital transfers you (YOU) need to take to get to Santa (SAN).
//!
//! You start at the object YOU are orbiting; your destination is the object SAN is orbiting. An orbital transfer lets you move from any object to an object orbiting or orbited by that object.
//!
//! For example, suppose you have the following map:
//!
//! ```text
//! COM)B
//! B)C
//! C)D
//! D)E
//! E)F
//! B)G
//! G)H
//! D)I
//! E)J
//! J)K
//! K)L
//! K)YOU
//! I)SAN
//! ```
//!
//! Visually, the above map of orbits looks like this:
//!
//! ```text
//!                           YOU
//!                          /
//!         G - H       J - K - L
//!        /           /
//! COM - B - C - D - E - F
//!                \
//!                 I - SAN
//! ```
//!
//! In this example, YOU are in orbit around K, and SAN is in orbit around I. To move from K to I, a minimum of 4 orbital transfers are required:
//!
//! ```text
//!     K to J
//!     J to E
//!     E to D
//!     D to I
//! ```
//!
//! Afterward, the map of orbits looks like this:
//!
//! ```text
//!         G - H       J - K - L
//!        /           /
//! COM - B - C - D - E - F
//!                \
//!                 I - SAN
//!                  \
//!                   YOU
//! ```
//!
//! What is the minimum number of orbital transfers required to move from the object YOU are orbiting to the object SAN is orbiting? (Between the objects they are orbiting - not between YOU and SAN.)

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{get_cap, run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use petgraph::{algo::dijkstra, graph::UnGraph, Direction::Incoming};
use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

type OrbitData = HashMap<String, Vec<String>>;

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<OrbitData, usize>(AoCYear::AOC2019, AoCDay::AOCD06, setup, find).map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<OrbitData, usize>(bench, AoCYear::AOC2019, AoCDay::AOCD06, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> OrbitData {
    setup_br(reader).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T) -> Result<OrbitData>
where
    T: BufRead,
{
    let orbit_re = Regex::new(r"^([A-Z0-9]+)\)([A-Z0-9]+)$")?;
    let mut orbit_map = HashMap::new();
    for line in valid_lines(reader) {
        for caps in orbit_re.captures_iter(&line) {
            let parent = get_cap(1, &caps)?;
            let child = get_cap(2, &caps)?;
            let _ = orbit_map
                .entry(parent)
                .and_modify(|x: &mut Vec<String>| x.push(child.clone()))
                .or_insert(vec![child]);
        }
    }
    Ok(orbit_map)
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: OrbitData) -> usize {
    find_res(&data, false)
        .map_err(|e| {
            eprintln!("{e}");
            e
        })
        .unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(orbit_map: &OrbitData, second_star: bool) -> Result<usize> {
    let mut orbital_graph = UnGraph::<&str, ()>::new_undirected();
    let mut idx_map = HashMap::new();

    // add nodes
    for (parent, children) in orbit_map {
        let _p_idx = idx_map
            .entry(&parent[..])
            .or_insert(orbital_graph.add_node(&parent[..]));
        for child in children {
            let _c_idx = idx_map
                .entry(&child[..])
                .or_insert(orbital_graph.add_node(&child[..]));
        }
    }

    let com_idx = idx_map.get("COM").ok_or_else(|| anyhow!("No COM idx"))?;

    // add edges
    for (parent, children) in orbit_map {
        for child in children {
            if let Some((p_nidx, c_nidx)) = idx_map.get(&parent[..]).zip(idx_map.get(&child[..])) {
                let _ = orbital_graph.add_edge(*p_nidx, *c_nidx, ());
            } else {
                return Err(anyhow!("nidx for {parent} doesn't exist"));
            }
        }
    }

    if second_star {
        let you_idx = idx_map.get("YOU").ok_or_else(|| anyhow!("No YOU idx"))?;
        let san_idx = idx_map.get("SAN").ok_or_else(|| anyhow!("No SAN idx"))?;
        let you_parent = orbital_graph
            .neighbors_directed(*you_idx, Incoming)
            .collect::<Vec<_>>();
        let san_parent = orbital_graph
            .neighbors_directed(*san_idx, Incoming)
            .collect::<Vec<_>>();
        let you_to_san = dijkstra(&orbital_graph, you_parent[0], Some(san_parent[0]), |_| 1);
        let cost = you_to_san
            .get(&san_parent[0])
            .ok_or_else(|| anyhow!("no cost for you"))?;
        Ok(usize::try_from(*cost)?)
    } else {
        let mut direct = 0;
        let mut indirect = 0;
        for nidx in idx_map.values() {
            let back_to_root = dijkstra(&orbital_graph, *nidx, Some(*com_idx), |_| 1);
            if let Some(root_cost) = back_to_root.get(com_idx) {
                if *root_cost > 0 {
                    direct += 1;
                    indirect += *root_cost - 1;
                }
            }
        }
        Ok(direct + indirect)
    }
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<OrbitData, usize>(AoCYear::AOC2019, AoCDay::AOCD06, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<OrbitData, usize>(bench, AoCYear::AOC2019, AoCDay::AOCD06, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: OrbitData) -> usize {
    find_res(&data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find(data), 42);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find2(data), 4);
        Ok(())
    }
}
