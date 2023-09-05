// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Recursive Circus
//!
//! **--- Day 7: Recursive Circus ---**
//!
//! **--- Part 1 ---**
//!
//! Wandering further through the circuits of the computer, you come upon a tower of
//! programs that have gotten themselves into a bit of trouble. A recursive algorithm
//! has gotten out of hand, and now they're balanced precariously in a large tower.
//!
//! One program at the bottom supports the entire tower. It's holding a large disc,
//! and on the disc are balanced several more sub-towers. At the bottom of these
//! sub-towers, standing on the bottom disc, are other programs, each holding their
//! own disc, and so on. At the very tops of these sub-sub-sub-...-towers, many programs
//! stand simply keeping the disc below them balanced but with no disc of their own.
//!
//! You offer to help, but first you need to understand the structure of these towers.
//! You ask each program to yell out their name, their weight, and (if they're holding
//! a disc) the names of the programs immediately above them balancing on that disc.
//! You write this information down (your puzzle input). Unfortunately, in their panic,
//! they don't do this in an orderly fashion; by the time you're done, you're not sure
//! which program gave which information.
//!
//! For example, if your list is the following:
//!
//! ```text
//! pbga (66)
//! xhth (57)
//! ebii (61)
//! havc (66)
//! ktlj (57)
//! fwft (72) -> ktlj, cntj, xhth
//! qoyq (66)
//! padx (45) -> pbga, havc, qoyq
//! tknk (41) -> ugml, padx, fwft
//! jptl (61)
//! ugml (68) -> gyxo, ebii, jptl
//! gyxo (61)
//! cntj (57)
//! ```
//!
//! ...then you would be able to recreate the structure of the towers that looks like this:
//!
//! ```text
//!                 gyxo
//!               /
//!          ugml - ebii
//!        /      \
//!       |         jptl
//!       |
//!       |         pbga
//!      /        /
//! tknk --- padx - havc
//!      \        \
//!       |         qoyq
//!       |
//!       |         ktlj
//!        \      /
//!          fwft - cntj
//!               \
//!                 xhth
//! ```
//!
//! In this example, `tknk` is at the bottom of the tower (the bottom program), and is holding
//! up `ugml`, `padx`, and `fwft`. Those programs are, in turn, holding up other programs; in this
//! example, none of those programs are holding up any other programs, and are all the tops of
//! their own towers. (The actual tower balancing in front of you is much larger.)
//!
//! Before you're ready to help them, you need to make sure your information is correct.
//! What is the name of the bottom program?
//!
//! **--- Part Two ---**
//!
//! The programs explain the situation: they can't get down. Rather, they could get down, if they
//! weren't expending all of their energy trying to keep the tower balanced. Apparently, one program
//! has the wrong weight, and until it's fixed, they're stuck here.
//!
//! For any program holding a disc, each program standing on that disc forms a sub-tower. Each of
//! those sub-towers are supposed to be the same weight, or the disc itself isn't balanced. The
//! weight of a tower is the sum of the weights of the programs in that tower.
//!
//! In the example above, this means that for `ugml`'s disc to be balanced, `gyxo`, `ebii`, and
//! `jptl` must all have the same weight, and they do: `61`.
//!
//! However, for `tknk` to be balanced, each of the programs standing on its disc and all programs
//! above it must each match. This means that the following sums must all be the same:
//!
//! ```text
//! ugml + (gyxo + ebii + jptl) = 68 + (61 + 61 + 61) = 251
//! padx + (pbga + havc + qoyq) = 45 + (66 + 66 + 66) = 243
//! fwft + (ktlj + cntj + xhth) = 72 + (57 + 57 + 57) = 243
//! ```
//!
//! As you can see, `tknk`'s disc is unbalanced: `ugml`'s stack is heavier than the other two. Even
//! though the nodes above `ugml` are balanced, `ugml` itself is too heavy: it needs to be 8 units
//! lighter for its stack to weigh 243 and keep the towers balanced. If this change were made, its
//! weight would be 60.
//!
//! Given that exactly one program is the wrong weight, what would its weight need to be to balance
//! the entire tower?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{get_cap, get_cap_x, print_err, run_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use core::fmt;
use itertools::Itertools;
use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Program {
    id: usize,
    name: String,
    weight: usize,
    parent: Option<usize>,
    children: Option<Vec<usize>>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "id: {}, name: {}, weight: {}",
            self.id, self.name, self.weight
        )?;
        if let Some(parent) = self.parent {
            write!(f, ", parent: {parent}")?;
        }

        if let Some(children) = &self.children {
            write!(f, ", children => (")?;
            for child in children {
                write!(f, "{child},")?;
            }
            write!(f, ")")?;
        }
        Ok(())
    }
}

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_solution::<String>(AoCYear::AOC2017, AoCDay::AOCD07, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> String {
    find_br(reader).map_err(print_err).unwrap_or_default()
}

fn find_br<T>(reader: T) -> Result<String>
where
    T: BufRead,
{
    let (mut programs, children) = setup(reader)?;
    assign_parents(&mut programs, &children);
    Ok(find_root(&programs).ok_or_else(|| anyhow!(""))?.name)
}

type State = (Vec<Program>, HashMap<usize, Vec<String>>);

fn setup<T>(reader: T) -> Result<State>
where
    T: BufRead,
{
    let node_only_re = Regex::new(r"^([a-z]+) \((\d+)\)$")?;
    let node_w_child_re = Regex::new(r"^([a-z]+) \((\d+)\) -> (.*)$")?;
    let mut nodes = vec![];
    let mut children = HashMap::new();

    for (idx, line) in valid_lines(reader).enumerate() {
        if node_w_child_re.is_match(&line) {
            for caps in node_w_child_re.captures_iter(&line) {
                let name = get_cap(1, &caps)?;
                let weight = get_cap_x::<usize>(2, &caps)?;
                nodes.push(Program {
                    id: idx,
                    name,
                    weight,
                    ..Program::default()
                });
                let children_str = get_cap(3, &caps)?;
                let children_vec = children_str
                    .split(", ")
                    .map(str::to_string)
                    .collect::<Vec<String>>();
                *children.entry(idx).or_insert_with(Vec::new) = children_vec;
            }
        } else if node_only_re.is_match(&line) {
            for caps in node_only_re.captures_iter(&line) {
                let name = get_cap(1, &caps)?;
                let weight = get_cap_x::<usize>(2, &caps)?;
                nodes.push(Program {
                    id: idx,
                    name,
                    weight,
                    ..Program::default()
                });
            }
        } else {
            return Err(anyhow!(format!("invalid node: {line}")));
        }
    }

    Ok((nodes, children))
}

fn assign_parents(programs: &mut [Program], children: &HashMap<usize, Vec<String>>) {
    for (id, names) in children {
        for name in names {
            for node in &mut *programs {
                if &node.name == name {
                    node.parent = Some(*id);
                }
            }
        }
    }
}

fn find_root(nodes: &[Program]) -> Option<Program> {
    nodes
        .iter()
        .filter(|x| x.parent.is_none())
        .cloned()
        .at_most_one()
        .ok()
        .flatten()
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2017, AoCDay::AOCD07, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader).map_err(print_err).unwrap_or_default()
}

fn find2_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let (mut programs, children) = setup(reader)?;
    assign_parents(&mut programs, &children);
    assign_children(&mut programs, children)?;
    let root = find_root(&programs).ok_or_else(|| anyhow!("blah"))?;
    let mut curr_weights = children_weight(&programs, &root)?;
    let mut curr_tuple = are_my_children_balanced(&curr_weights, 0);
    let mut is_balanced = curr_tuple.2;

    while !is_balanced {
        let program = programs.get(curr_tuple.0).ok_or_else(|| anyhow!(""))?;
        curr_weights = children_weight(&programs, program)?;
        curr_tuple = are_my_children_balanced(&curr_weights, curr_tuple.1);
        is_balanced = curr_tuple.2;
    }
    Ok(curr_tuple.1)
}

/// Traverse the vectors, assigning proper children vectors.
fn assign_children(nodes: &mut [Program], children: HashMap<usize, Vec<String>>) -> Result<()> {
    let children_ids: HashMap<usize, Vec<usize>> = children
        .into_iter()
        .map(|(k, v)| {
            (
                k,
                v.iter().filter_map(|x| get_id_by_name(nodes, x)).collect(),
            )
        })
        .collect();

    for (k, v) in children_ids {
        let node = nodes.get_mut(k).ok_or_else(|| anyhow!("Invalid node id"))?;
        node.children = Some(v);
    }

    Ok(())
}

fn get_id_by_name(nodes: &[Program], name: &str) -> Option<usize> {
    nodes
        .iter()
        .filter_map(|x| if x.name == name { Some(x.id) } else { None })
        .at_most_one()
        .ok()
        .flatten()
}

/// Traverse the tree accumulating weights
fn children_weight(nodes: &[Program], start_node: &Program) -> Result<Vec<(usize, usize)>> {
    let mut weights = Vec::new();
    if start_node.children.is_none() {
        weights.push((start_node.id, start_node.weight));
    } else {
        let node_clone = (*start_node).clone();
        let children = node_clone.children.ok_or_else(|| anyhow!("Bad children"))?;

        for node_id in children {
            let node = nodes
                .get(node_id)
                .ok_or_else(|| anyhow!("Cannot find child node"))?;
            let total_child_weight: usize = children_weight(nodes, node)?
                .iter()
                .fold(0, |acc, x| acc + x.1);
            weights.push((node_id, total_child_weight));
        }

        weights.push((node_clone.id, node_clone.weight));
    }
    Ok(weights)
}

/// Check weights
fn are_my_children_balanced(outer: &[(usize, usize)], diff: usize) -> (usize, usize, bool) {
    let len = outer.len() - 1;
    let mut inner = vec![(0, 0); outer.len()];
    inner.copy_from_slice(outer);

    for (i, x) in outer.iter().enumerate().take(len) {
        for y in inner.iter().take(len).skip(i + 1) {
            let outer_id = x.0;
            let inner_id = y.0;
            let ov = x.1;
            let iv = y.1;

            if ov == iv {
                continue;
            }
            if ov < iv {
                return (inner_id, (iv - ov), false);
            }
            return (outer_id, (ov - iv), false);
        }
    }
    (outer[len].0, outer[len].1 - diff, true)
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1))?, "tknk");
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find2_br(Cursor::new(TEST_1))?, 60);
        Ok(())
    }
}
