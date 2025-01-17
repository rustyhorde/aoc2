// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2018 ---**
//!
//! **--- Day 7: The Sum of Its Parts ---**
//!
//! You find yourself standing on a snow-covered coastline; apparently, you landed a little off course. The region is too hilly to see the North Pole from here, but you do spot some Elves that seem to be trying to unpack something that washed ashore. It's quite cold out, so you decide to risk creating a paradox by asking them for directions.
//!
//! "Oh, are you the search party?" Somehow, you can understand whatever Elves from the year 1018 speak; you assume it's Ancient Nordic Elvish. Could the device on your wrist also be a translator? "Those clothes don't look very warm; take this." They hand you a heavy coat.
//!
//! "We do need to find our way back to the North Pole, but we have higher priorities at the moment. You see, believe it or not, this box contains something that will solve all of Santa's transportation problems - at least, that's what it looks like from the pictures in the instructions." It doesn't seem like they can read whatever language it's in, but you can: "Sleigh kit. Some assembly required."
//!
//! "'Sleigh'? What a wonderful name! You must help us assemble this 'sleigh' at once!" They start excitedly pulling more parts out of the box.
//!
//! The instructions specify a series of steps and requirements about which steps must be finished before others can begin (your puzzle input). Each step is designated by a single letter. For example, suppose you have the following instructions:
//!
//! ```text
//! Step C must be finished before step A can begin.
//! Step C must be finished before step F can begin.
//! Step A must be finished before step B can begin.
//! Step A must be finished before step D can begin.
//! Step B must be finished before step E can begin.
//! Step D must be finished before step E can begin.
//! Step F must be finished before step E can begin.
//! ```
//!
//! Visually, these requirements look like this:
//!
//! ```text
//!   -->A--->B--
//!  /    \      \
//! C      -->D----->E
//!  \           /
//!   ---->F-----
//! ```
//!
//! Your first goal is to determine the order in which the steps should be completed. If more than one step is ready, choose the step which is first alphabetically. In this example, the steps would be completed as follows:
//!
//! ```text
//!     Only C is available, and so it is done first.
//!     Next, both A and F are available. A is first alphabetically, so it is done next.
//!     Then, even though F was available earlier, steps B and D are now also available, and B is the first alphabetically of the three.
//!     After that, only D and F are available. E is not available because only some of its prerequisites are complete. Therefore, D is completed next.
//!     F is the only choice, so it is done next.
//!     Finally, E is completed.
//! ```
//!
//! So, in this example, the correct order is CABDFE.
//!
//! In what order should the steps in your instructions be completed?
//!
//! **--- Part Two ---**
//!
//! As you're about to begin construction, four of the Elves offer to help. "The sun will set soon; it'll go faster if we work together." Now, you need to account for multiple people working on steps simultaneously. If multiple steps are available, workers should still begin them in alphabetical order.
//!
//! Each step takes 60 seconds plus an amount corresponding to its letter: A=1, B=2, C=3, and so on. So, step A takes 60+1=61 seconds, while step Z takes 60+26=86 seconds. No time is required between steps.
//!
//! To simplify things for the example, however, suppose you only have help from one Elf (a total of two workers) and that each step takes 60 fewer seconds (so that step A takes 1 second and step Z takes 26 seconds). Then, using the same instructions as above, this is how each second would be spent:
//!
//! ```text
//! Second   Worker 1   Worker 2   Done
//!    0        C          .        
//!    1        C          .        
//!    2        C          .        
//!    3        A          F       C
//!    4        B          F       CA
//!    5        B          F       CA
//!    6        D          F       CAB
//!    7        D          F       CAB
//!    8        D          F       CAB
//!    9        D          .       CABF
//!   10        E          .       CABFD
//!   11        E          .       CABFD
//!   12        E          .       CABFD
//!   13        E          .       CABFD
//!   14        E          .       CABFD
//!   15        .          .       CABFDE
//! ```
//!
//! Each row represents one second of time. The Second column identifies how many seconds have passed as of the beginning of that second. Each worker column shows the step that worker is currently doing (or . if they are idle). The Done column shows completed steps.
//!
//! Note that the order of the steps has changed; this is because steps now take time to finish and multiple workers can begin multiple steps simultaneously.
//!
//! In this example, it would take 15 seconds for two workers to complete these steps.
//!
//! With 5 workers and the 60+ second step durations described above, how long will it take to complete all of the steps?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use getset::{Getters, Setters};
use indexmap::IndexSet;
use regex::Regex;
use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::{BufRead, BufReader},
};

#[allow(dead_code)]
#[derive(Clone, Debug, Default, Getters, Setters)]
struct Worker {
    id: u32,
    #[getset(get, set)]
    work: Option<(String, u32)>,
    #[getset(get)]
    remaining: u32,
}

type SoiPData = (
    HashMap<(String, u32), Vec<(String, u32)>>,
    HashMap<(String, u32), Vec<(String, u32)>>,
    IndexSet<(String, u32)>,
    Vec<Worker>,
);

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<BTreeMap<char, Vec<char>>, String>(
        AoCYear::AOC2018,
        AoCDay::AOCD07,
        setup,
        find,
    )
    .map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<BTreeMap<char, Vec<char>>, String>(
        bench,
        AoCYear::AOC2018,
        AoCDay::AOCD07,
        setup,
        find,
    )
    .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> BTreeMap<char, Vec<char>> {
    setup_br(reader).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T) -> Result<BTreeMap<char, Vec<char>>>
where
    T: BufRead,
{
    let mut node_map: BTreeMap<char, Vec<char>> = BTreeMap::new();
    let line_re = Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z])")?;

    for line in valid_lines(reader) {
        for cap in line_re.captures_iter(&line) {
            let first = (cap[1]).chars().next().ok_or(anyhow!("invalid char"))?;
            let second = (cap[2]).chars().next().ok_or(anyhow!("invalid char"))?;

            {
                let _ = node_map
                    .entry(first)
                    .or_insert_with(|| Vec::with_capacity(25));
            }
            {
                let snode = node_map
                    .entry(second)
                    .or_insert_with(|| Vec::with_capacity(25));
                snode.push(first);
            }
        }
    }

    Ok(node_map)
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: BTreeMap<char, Vec<char>>) -> String {
    find_res(data).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(node_map: BTreeMap<char, Vec<char>>) -> Result<String> {
    // The final order
    let mut node_map = node_map;
    let mut result = String::new();

    // Loop through the map, completing one character, removing that character from the remaining
    // parent vectors, removing the completed character from the map and pushing onto the result.
    while !node_map.is_empty() {
        let completed = complete(&node_map)?;
        remove_from_parents(&mut node_map, completed);
        let _res = node_map.remove(&completed).ok_or(anyhow!("blah"))?;
        result.push(completed);
    }

    Ok(result)
}

fn complete(node_map: &BTreeMap<char, Vec<char>>) -> Result<char> {
    let ready: Vec<char> = node_map
        .iter()
        .filter_map(|(x, y)| if y.is_empty() { Some(*x) } else { None })
        .collect();
    ready.first().copied().ok_or(anyhow!("blah"))
}

fn remove_from_parents(node_map: &mut BTreeMap<char, Vec<char>>, key: char) {
    for node in node_map.values_mut() {
        node.retain(|x| *x != key);
    }
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<SoiPData, usize>(AoCYear::AOC2018, AoCDay::AOCD07, setup2, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<SoiPData, usize>(bench, AoCYear::AOC2018, AoCDay::AOCD07, setup2, find2)
        .map(|_| 0)
}

fn setup2(reader: BufReader<File>) -> SoiPData {
    setup_br2(reader, 60, 5).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br2<T>(reader: T, base: u32, workers_count: u32) -> Result<SoiPData>
where
    T: BufRead,
{
    let line_re = Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z])")?;
    let mut pending = IndexSet::new();
    let mut child_map = HashMap::new();
    let mut parents_map = HashMap::new();
    let mut workers = Vec::new();

    for i in 0..workers_count {
        let worker = Worker {
            id: i,
            ..Default::default()
        };
        workers.push(worker);
    }

    for line in valid_lines(reader) {
        for cap in line_re.captures_iter(&line) {
            let first = (cap[1]).to_string();
            let first_duration = duration_of(&first, base)?;
            let first_tuple = (first, first_duration);
            let second = (cap[2]).to_string();
            let second_duration = duration_of(&second, base)?;
            let second_tuple = (second, second_duration);
            let _ = pending.insert(first_tuple.clone());
            let _ = pending.insert(second_tuple.clone());
            let children = child_map
                .entry(first_tuple.clone())
                .or_insert_with(Vec::new);
            children.push(second_tuple.clone());
            let parents = parents_map.entry(second_tuple).or_insert_with(Vec::new);
            parents.push(first_tuple);
        }
    }
    Ok((child_map, parents_map, pending, workers))
}

fn duration_of(val: &str, base: u32) -> Result<u32> {
    Ok(match val {
        "A" => base + 1,
        "B" => base + 2,
        "C" => base + 3,
        "D" => base + 4,
        "E" => base + 5,
        "F" => base + 6,
        "G" => base + 7,
        "H" => base + 8,
        "I" => base + 9,
        "J" => base + 10,
        "K" => base + 11,
        "L" => base + 12,
        "M" => base + 13,
        "N" => base + 14,
        "O" => base + 15,
        "P" => base + 16,
        "Q" => base + 17,
        "R" => base + 18,
        "S" => base + 19,
        "T" => base + 20,
        "U" => base + 21,
        "V" => base + 22,
        "W" => base + 23,
        "X" => base + 24,
        "Y" => base + 25,
        "Z" => base + 26,
        _ => return Err(anyhow!("invalid instructions")),
    })
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: SoiPData) -> usize {
    find_res2(data).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res2(data: SoiPData) -> Result<usize> {
    let (child_map, parents_map, pending, mut workers) = data;
    let all_children: IndexSet<(String, u32)> =
        child_map.iter().flat_map(|(_, c)| c).cloned().collect();
    let mut ready: IndexSet<(String, u32)> = pending.difference(&all_children).cloned().collect();
    let mut complete: IndexSet<(String, u32)> = IndexSet::new();

    ready.sort_by(|x, y| x.cmp(y).reverse());

    let mut total_ticks = 0;
    for tick in 0.. {
        // Check for completed work and adjust the ready and complete queues as necessary
        let complete_work = complete_work(&mut workers);

        for work in complete_work {
            if let Some(children) = child_map.get(&work) {
                for child in children {
                    let _ = ready.insert(child.clone());
                }

                // Sort the ready work as new children have been added to the queue.
                ready.sort_by(|x, y| x.cmp(y).reverse());
            }

            let _ = complete.insert(work);
        }

        // Assign work to idle workers if conditions are favorable.
        'outer: for worker in &mut workers {
            if let Some(nx) = ready.pop() {
                // Check that all the parent steps have completed.  If not, move on.
                if let Some(parents) = parents_map.get(&nx) {
                    for parent in parents {
                        if !complete.contains(parent) {
                            continue 'outer;
                        }
                    }
                }

                // Cosume ready work if the worker is idle.  Otherwise, push the ready
                // back onto the queue, sort,  and check the next worker.
                if !consume_work(worker, &nx) {
                    let _ = ready.insert(nx);
                    ready.sort_by(|x, y| x.cmp(y).reverse());
                }
            }
        }

        // Complete 1 second of work on each busy worker
        adjust_workers(&mut workers);

        // println!();
        // println!("Tick: {}", tick);
        // println!("Ready: {:?}", ready);
        // println!("Complete: {:?}", complete);

        // for worker in &workers {
        //     println!("{}", worker);
        // }

        // Are we done?
        if ready.is_empty() && all_idle(&workers) {
            total_ticks = tick;
            break;
        }
    }

    Ok(total_ticks)
}

fn complete_work(workers: &mut Vec<Worker>) -> Vec<(String, u32)> {
    let mut result = Vec::new();

    for worker in workers {
        let mut clear = false;
        if let Some(work) = worker.work() {
            if worker.remaining == 0 {
                result.push(work.clone());
                clear = true;
            }
        }

        if clear {
            let _ = worker.set_work(None);
        }
    }

    result
}

fn consume_work(worker: &mut Worker, work: &(String, u32)) -> bool {
    if worker.work.is_none() {
        worker.remaining = work.1;
        worker.work = Some(work.clone());
        true
    } else {
        false
    }
}

fn adjust_workers(workers: &mut Vec<Worker>) {
    for worker in workers {
        if worker.work.is_some() {
            worker.remaining -= 1;
        }
    }
}

fn all_idle(workers: &[Worker]) -> bool {
    workers.iter().all(|worker| worker.work().is_none())
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find(data), "CABDFE");
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br2};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br2(Cursor::new(TEST_1), 0, 2)?;
        assert_eq!(find2(data), 15);
        Ok(())
    }
}
