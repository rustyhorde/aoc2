// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 13 "Knights of the Dinner Table"
//!
//! **--- Day 13: Knights of the Dinner Table ---**
//!
//! **--- Part 1 ---**
//!
//! In years past, the holiday feast with your family hasn't gone so well. Not everyone gets along!
//! This year, you resolve, will be different. You're going to find the optimal seating
//! arrangement and avoid all those awkward conversations.
//!
//! You start by writing up a list of everyone invited and the amount their happiness
//! would increase or decrease if they were to find themselves sitting next to each other person.
//! You have a circular table that will be just big enough to fit everyone comfortably,
//! and so each person will have exactly two neighbors.
//!
//! For example, suppose you have only four attendees planned, and you calculate their potential happiness as follows:
//!
//! ```text
//! Alice would gain 54 happiness units by sitting next to Bob.
//! Alice would lose 79 happiness units by sitting next to Carol.
//! Alice would lose 2 happiness units by sitting next to David.
//! Bob would gain 83 happiness units by sitting next to Alice.
//! Bob would lose 7 happiness units by sitting next to Carol.
//! Bob would lose 63 happiness units by sitting next to David.
//! Carol would lose 62 happiness units by sitting next to Alice.
//! Carol would gain 60 happiness units by sitting next to Bob.
//! Carol would gain 55 happiness units by sitting next to David.
//! David would gain 46 happiness units by sitting next to Alice.
//! David would lose 7 happiness units by sitting next to Bob.
//! David would gain 41 happiness units by sitting next to Carol.
//! ```
//!
//! Then, if you seat Alice next to David, Alice would lose 2 happiness units (because David talks so much),
//! but David would gain 46 happiness units (because Alice is such a good listener), for a total change of 44.
//!
//! If you continue around the table, you could then seat Bob next to Alice (Bob gains 83, Alice gains 54).
//! Finally, seat Carol, who sits next to Bob (Carol gains 60, Bob loses 7) and David (Carol gains 55, David gains 41).
//! The arrangement looks like this:
//!
//! ```text
//!      +41 +46
//! +55   David    -2
//! Carol       Alice
//! +60    Bob    +54
//!      -7  +83
//! ```
//!
//! After trying every other seating arrangement in this hypothetical scenario, you find that this one
//! is the most optimal, with a total change in happiness of `330`.
//!
//! What is the total change in happiness for the optimal seating arrangement of the actual guest list?
//!
//! **--- Part Two ---**
//!
//! In all the commotion, you realize that you forgot to seat yourself. At this point, you're
//! pretty apathetic toward the whole thing, and your happiness wouldn't really go up or down
//! regardless of who you sit next to. You assume everyone else would be just as ambivalent about
//! sitting next to you, too.
//!
//! So, add yourself to the list, and give all happiness relationships that involve you a score of 0.
//!
//! What is the total change in happiness for the optimal seating arrangement that actually includes yourself?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{get_cap, get_cap_x, run_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use itertools::Itertools;
use petgraph::{graph::NodeIndex, Graph};
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
    run_solution::<isize>(AoCYear::AOC2015, AoCDay::AOCD13, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> isize {
    find_br(reader).unwrap_or_default()
}

fn find_br<T>(reader: T) -> Result<isize>
where
    T: BufRead,
{
    let edge_re =
        Regex::new(r"^(.*) would (gain|lose) (\d+) happiness units by sitting next to (.*)\.$")?;
    let mut nodes = HashSet::new();
    let mut graph = Graph::new();

    for line in valid_lines(reader) {
        build_graph(&line, &edge_re, &mut graph, &mut nodes)?;
    }

    let (_min_dist, max_dist) = find_distances(&graph, &nodes)?;
    Ok(max_dist)
}

fn build_graph(
    line: &str,
    re: &Regex,
    graph: &mut Graph<String, isize>,
    nodes: &mut HashSet<NodeIndex>,
) -> Result<()> {
    for caps in re.captures_iter(line) {
        let n1 = get_cap(1, &caps)?;
        let op = get_cap(2, &caps)?;
        let w = get_cap_x::<isize>(3, &caps)?;
        let n2 = get_cap(4, &caps)?;

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
        _ = graph.add_edge(idx1, idx2, if op == "lose" { -w } else { w });
    }
    Ok(())
}

fn find_distances(
    graph: &Graph<String, isize>,
    nodes: &HashSet<NodeIndex>,
) -> Result<(isize, isize)> {
    let mut min_dist = isize::MAX;
    let mut max_dist = 0;

    for perm in nodes.iter().permutations(nodes.len()) {
        let mut dist = 0;
        for i in 0..perm.len() {
            let c1 = perm.get(i).ok_or_else(|| anyhow!("bad ni1"))?;
            let nidx = if i == (perm.len() - 1) { 0 } else { i + 1 };
            let c2 = perm.get(nidx).ok_or_else(|| anyhow!("bad ni2"))?;

            if let Some(eidx) = graph.find_edge(**c1, **c2) {
                let dist_b = graph.edge_weight(eidx).ok_or_else(|| anyhow!("bad dist"))?;
                dist += *dist_b;
            }
            if let Some(eidx) = graph.find_edge(**c2, **c1) {
                let dist_b = graph.edge_weight(eidx).ok_or_else(|| anyhow!("bad dist"))?;
                dist += *dist_b;
            }
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
    run_solution::<isize>(AoCYear::AOC2015, AoCDay::AOCD13, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> isize {
    find2_br(reader).unwrap_or_default()
}

fn find2_br<T>(reader: T) -> Result<isize>
where
    T: BufRead,
{
    let edge_re =
        Regex::new(r"^(.*) would (gain|lose) (\d+) happiness units by sitting next to (.*)\.$")?;
    let mut nodes = HashSet::new();
    let mut graph = Graph::new();

    for line in valid_lines(reader) {
        build_graph(&line, &edge_re, &mut graph, &mut nodes)?;
    }

    add_myself(&mut graph, &mut nodes);

    let (_min_dist, max_dist) = find_distances(&graph, &nodes)?;
    Ok(max_dist)
}

fn add_myself(graph: &mut Graph<String, isize>, nodes: &mut HashSet<NodeIndex>) {
    let my_idx = graph.add_node("Jason".to_string());

    for n_idx in &*nodes {
        _ = graph.add_edge(my_idx, *n_idx, 0);
        _ = graph.add_edge(*n_idx, my_idx, 0);
    }
    _ = nodes.insert(my_idx);
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1))?, 330);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find2_br(Cursor::new(TEST_1))?, 286);
        Ok(())
    }
}
