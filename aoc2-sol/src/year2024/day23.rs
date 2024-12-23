// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code --- **
//!
//! --- Day 23: LAN Party ---
//!  
//!  As The Historians wander around a secure area at Easter Bunny HQ, you come across posters for a LAN party scheduled for today! Maybe you can find it; you connect to a nearby datalink port and download a map of the local network (your puzzle input).
//!  
//!  The network map provides a list of every connection between two computers. For example:
//!  
//!  ```text
//!  kh-tc
//!  qp-kh
//!  de-cg
//!  ka-co
//!  yn-aq
//!  qp-ub
//!  cg-tb
//!  vc-aq
//!  tb-ka
//!  wh-tc
//!  yn-cg
//!  kh-ub
//!  ta-co
//!  de-co
//!  tc-td
//!  tb-wq
//!  wh-td
//!  ta-ka
//!  td-qp
//!  aq-cg
//!  wq-ub
//!  ub-vc
//!  de-ta
//!  wq-aq
//!  wq-vc
//!  wh-yn
//!  ka-de
//!  kh-ta
//!  co-tc
//!  wh-qp
//!  tb-vc
//!  td-yn
//!  ```
//!
//!  Each line of text in the network map represents a single connection; the line kh-tc represents a connection between the computer named kh and the computer named tc. Connections aren't directional; tc-kh would mean exactly the same thing.
//!  
//!  LAN parties typically involve multiplayer games, so maybe you can locate it by finding groups of connected computers. Start by looking for sets of three computers where each computer in the set is connected to the other two computers.
//!  
//!  In this example, there are 12 such sets of three inter-connected computers:
//!  
//!  ```text
//!  aq,cg,yn
//!  aq,vc,wq
//!  co,de,ka
//!  co,de,ta
//!  co,ka,ta
//!  de,ka,ta
//!  kh,qp,ub
//!  qp,td,wh
//!  tb,vc,wq
//!  tc,td,wh
//!  td,wh,yn
//!  ub,vc,wq
//!  ```
//!
//!  If the Chief Historian is here, and he's at the LAN party, it would be best to know that right away. You're pretty sure his computer's name starts with t, so consider only sets of three computers where at least one computer's name starts with t. That narrows the list down to 7 sets of three inter-connected computers:
//!
//!  ```text
//!  co,de,ta
//!  co,ka,ta
//!  de,ka,ta
//!  qp,td,wh
//!  tb,vc,wq
//!  tc,td,wh
//!  td,wh,yn
//!  ```
//!
//!  Find all the sets of three inter-connected computers. How many contain at least one computer with a name that starts with t?

use crate::utils::get_cap;
use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::Result;
use itertools::Itertools;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
};

type ConnData = Vec<(String, String)>;

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](AoCYear) and
///   [`AoCDay`](AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<ConnData, usize>(AoCYear::AOC2024, AoCDay::AOCD23, setup, find).map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<ConnData, usize>(bench, AoCYear::AOC2024, AoCDay::AOCD23, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> ConnData {
    setup_br(reader).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T) -> Result<ConnData>
where
    T: BufRead,
{
    let conn_re = Regex::new(r"^([a-z]{2})-([a-z]{2})$")?;
    let mut data = vec![];
    for line in valid_lines(reader) {
        for caps in conn_re.captures_iter(&line) {
            let l = get_cap(1, &caps)?;
            let r = get_cap(2, &caps)?;
            data.push((l, r));
        }
    }
    Ok(data)
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: ConnData) -> usize {
    find_res(&data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(conn_data: &ConnData, _second_star: bool) -> Result<usize> {
    let mut neighbors = HashSet::new();
    let mut comps = HashSet::new();

    for (a, b) in conn_data {
        let _ = comps.insert(a);
        let _ = comps.insert(b);
        let _ = neighbors.insert((a, b));
        let _ = neighbors.insert((b, a));
    }

    let mut trios = 0;
    for (a, b) in conn_data {
        for comp in &comps {
            if !a.starts_with('t') && !b.starts_with('t') && !comp.starts_with('t') {
                continue;
            }
            if neighbors.contains(&(comp, a)) && neighbors.contains(&(b, comp)) {
                trios += 1;
            }
        }
    }

    Ok(trios / 3)
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](AoCYear) and
///   [`AoCDay`](AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<ConnData, String>(AoCYear::AOC2024, AoCDay::AOCD23, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<ConnData, String>(bench, AoCYear::AOC2024, AoCDay::AOCD23, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: ConnData) -> String {
    find_res2(&data, true).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res2(conn_data: &ConnData, _second_star: bool) -> Result<String> {
    let mut neighbors = HashMap::new();
    let mut vs = HashSet::new();
    for (a, b) in conn_data {
        let v1 = Comp { name: a };
        let v2 = Comp { name: b };
        let _ = vs.insert(v1);
        let _ = vs.insert(v2);
        let _b = neighbors
            .entry(v1)
            .and_modify(|vs: &mut HashSet<Comp<'_>>| {
                let _ = vs.insert(v2);
            })
            .or_insert(HashSet::from([v2]));
        let _b = neighbors
            .entry(v2)
            .and_modify(|vs: &mut HashSet<Comp<'_>>| {
                let _ = vs.insert(v1);
            })
            .or_insert(HashSet::from([v1]));
    }

    let res = bron_kerbosch(vs, HashSet::new(), HashSet::new(), &neighbors)
        .iter()
        .max_by(|c1, c2| c1.len().cmp(&c2.len()))
        .map_or(String::new(), |vs| {
            vs.iter().map(|v| v.name).sorted().join(",")
        });
    Ok(res)
}

fn bron_kerbosch<'a>(
    mut p: HashSet<Comp<'a>>,
    r: HashSet<Comp<'a>>,
    mut x: HashSet<Comp<'a>>,
    n: &HashMap<Comp<'a>, HashSet<Comp<'a>>>,
) -> Vec<HashSet<Comp<'a>>> {
    if p.is_empty() && x.is_empty() {
        return vec![r];
    }
    let mut res = Vec::new();
    while let Some(v) = pop(&mut p) {
        let mut nr = r.clone();
        let _ = nr.insert(v);
        let np = p.clone().intersection(&n[&v]).copied().collect();
        let nx = x.clone().intersection(&n[&v]).copied().collect();
        res.extend(bron_kerbosch(np, nr, nx, n));
        let _ = x.insert(v);
    }
    res
}

fn pop<T>(s: &mut HashSet<T>) -> Option<T>
where
    T: Hash + Eq + Copy,
{
    let e = s.iter().next().copied()?;
    s.take(&e)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Comp<'a> {
    name: &'a str,
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find(data), 7);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find2(data), "co,de,ka,ta");
        Ok(())
    }
}
