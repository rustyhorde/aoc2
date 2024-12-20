// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 9 "All in a Single Night"
//!
//! **--- Day 9: All in a Single Night ---**
//!
//! **--- Part 1 ---**
//!
//! Every year, Santa manages to deliver all of his presents in a single night.
//!
//! This year, however, he has some new locations to visit; his elves have provided
//! him the distances between every pair of locations. He can start and end at any two
//! (different) locations he wants, but he must visit each location exactly once.
//! What is the shortest distance he can travel to achieve this?
//!
//! For example, given the following distances:
//!
//! ```text
//! London to Dublin = 464
//! London to Belfast = 518
//! Dublin to Belfast = 141
//! ```
//!
//! The possible routes are therefore:
//!
//! ```text
//! Dublin -> London -> Belfast = 982
//! London -> Dublin -> Belfast = 605
//! London -> Belfast -> Dublin = 659
//! Dublin -> Belfast -> London = 659
//! Belfast -> Dublin -> London = 605
//! Belfast -> London -> Dublin = 982
//! ```
//!
//! The shortest of these is `London -> Dublin -> Belfast = 605`, and so the
//! answer is `605` in this example.
//!
//! What is the distance of the shortest route?
//!
//! **--- Part Two ---**
//!
//! The next year, just to show off, Santa decides to take the route with the longest distance instead.
//!
//! He can still start and end at any two (different) locations he wants, and he still must visit each location exactly once.
//!
//! For example, given the distances above, the longest route would be `982` via (for example) `Dublin -> London -> Belfast`.
//!
//! What is the distance of the longest route?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{get_cap, get_cap_x, run_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use itertools::Itertools;
use petgraph::{graph::NodeIndex, Graph, Undirected};
use regex::Regex;
use std::{
    collections::HashSet,
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
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD09, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader).unwrap_or_default()
}

fn find_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let edge_re = Regex::new(r"(.*) to (.*) = (\d+)")?;
    let mut nodes = HashSet::new();
    let mut graph = Graph::new_undirected();

    for line in valid_lines(reader) {
        build_graph(&line, &edge_re, &mut graph, &mut nodes)?;
    }

    let (min_dist, _) = find_distances(&graph, &nodes)?;
    Ok(min_dist)
}

fn build_graph(
    line: &str,
    re: &Regex,
    graph: &mut Graph<String, usize, Undirected>,
    nodes: &mut HashSet<NodeIndex>,
) -> Result<()> {
    for caps in re.captures_iter(line) {
        let n1 = get_cap(1, &caps)?;
        let n2 = get_cap(2, &caps)?;
        let w = get_cap_x::<usize>(3, &caps)?;

        let idx1 = graph
            .node_indices()
            .find(|i| graph[*i] == n1)
            .unwrap_or_else(|| {
                let idx = graph.add_node(n1.clone());
                _ = nodes.insert(idx);
                idx
            });
        let idx2 = graph
            .node_indices()
            .find(|i| graph[*i] == n2)
            .unwrap_or_else(|| {
                let idx = graph.add_node(n2.clone());
                _ = nodes.insert(idx);
                idx
            });
        _ = graph.add_edge(idx1, idx2, w);
    }
    Ok(())
}

fn find_distances(
    graph: &Graph<String, usize, Undirected>,
    nodes: &HashSet<NodeIndex>,
) -> Result<(usize, usize)> {
    let mut min_dist = usize::MAX;
    let mut max_dist = 0;

    for perm in nodes.iter().permutations(nodes.len()) {
        let mut dist = 0;
        for i in 0..perm.len() - 1 {
            let c1 = perm.get(i).ok_or_else(|| anyhow!("bad ni"))?;
            let c2 = perm.get(i + 1).ok_or_else(|| anyhow!("bad ni"))?;
            let eidx = graph
                .find_edge(**c1, **c2)
                .ok_or_else(|| anyhow!("bad ni"))?;
            let dist_b = graph.edge_weight(eidx).ok_or_else(|| anyhow!("bad ni"))?;
            dist += *dist_b;
        }
        if dist < min_dist {
            min_dist = dist;
        }
        if dist > max_dist {
            max_dist = dist;
        }
    }

    Ok((min_dist, max_dist))
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD09, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader).unwrap_or_default()
}

fn find2_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let edge_re = Regex::new(r"(.*) to (.*) = (\d+)")?;
    let mut nodes = HashSet::new();
    let mut graph = Graph::new_undirected();

    for line in valid_lines(reader) {
        build_graph(&line, &edge_re, &mut graph, &mut nodes)?;
    }

    let (_, max_dist) = find_distances(&graph, &nodes)?;
    Ok(max_dist)
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1))?, 605);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find2_br(Cursor::new(TEST_1))?, 982);
        Ok(())
    }
}
