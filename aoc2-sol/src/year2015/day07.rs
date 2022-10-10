// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 7 "Some Assembly Required"
//!
//! **--- Day 7: Some Assembly Required ---**
//!
//! **--- Part 1 ---**
//!
//! This year, Santa brought little Bobby Tables a set of wires and bitwise logic gates!
//! Unfortunately, little Bobby is a little under the recommended age range, and he
//! needs help assembling the circuit.
//!
//! Each wire has an identifier (some lowercase letters) and can carry a 16-bit signal
//! (a number from 0 to 65535). A signal is provided to each wire by a gate, another wire,
//! or some specific value. Each wire can only get a signal from one source, but can
//! provide its signal to multiple destinations. A gate provides no signal until all of its
//! inputs have a signal.
//!
//! The included instructions booklet describes how to connect the parts together:
//! `x AND y -> z` means to connect wires `x` and `y` to an `AND` gate, and then connect
//! its output to wire `z`.
//!
//! For example:
//!
//! * `123 -> x` means that the signal `123` is provided to wire `x`.
//! * `x AND y -> z` means that the bitwise `AND` of wire `x` and wire `y` is provided to wire `z`.
//! * `p LSHIFT 2 -> q` means that the value from wire `p` is left-shifted by 2 and then provided to wire `q`.
//! * `NOT e -> f` means that the bitwise complement of the value from wire `e` is provided to wire `f`.
//!
//! Other possible gates include `OR` (bitwise OR) and `RSHIFT` (right-shift).
//! If, for some reason, you'd like to emulate the circuit instead, almost all programming languages
//! (for example, C, JavaScript, or Python) provide operators for these gates.
//!
//! For example, here is a simple circuit:
//!
//! ```text
//! 123 -> x
//! 456 -> y
//! x AND y -> d
//! x OR y -> e
//! x LSHIFT 2 -> f
//! y RSHIFT 2 -> g
//! NOT x -> h
//! NOT y -> i
//! ```
//!
//! After it is run, these are the signals on the wires:
//!
//! ```text
//! d: 72
//! e: 507
//! f: 492
//! g: 114
//! h: 65412
//! i: 65079
//! x: 123
//! y: 456
//! ```
//!
//! In little Bobby's kit's instructions booklet (provided as your puzzle input), what signal is ultimately provided to wire a?
//!
//! **--- Part Two ---**
//!
//! Now, take the signal you got on wire `a`, override wire `b` to that signal, and reset the
//! other wires (including wire `a`). What new signal is ultimately provided to wire `a`?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{get_cap, get_cap_u16, run_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use regex::Regex;
use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader, Cursor},
};

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD07, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    match find_br(reader) {
        Ok(map) => map
            .get(&"a".to_string())
            .copied()
            .unwrap_or_default()
            .into(),
        Err(e) => {
            eprintln!("{e}");
            1
        }
    }
}

fn find_br<T>(reader: T) -> Result<HashMap<String, u16>>
where
    T: BufRead,
{
    let line_re = Regex::new(r"(.*) -> (.*)")?;
    let val_re = Regex::new(r"(^\d+$)")?;
    let and_re = Regex::new(r"(.*) AND (.*)")?;
    let or_re = Regex::new(r"(.*) OR (.*)")?;
    let l_shift_re = Regex::new(r"(.*) LSHIFT (.*)")?;
    let r_shift_re = Regex::new(r"(.*) RSHIFT (.*)")?;
    let not_re = Regex::new(r"NOT (.*)")?;
    let mut circuit_map = HashMap::new();
    let mut no_input = VecDeque::new();

    for line in valid_lines(reader) {
        for cap in line_re.captures_iter(&line) {
            let input = get_cap(1, &cap)?;
            let wire = get_cap(2, &cap)?;
            no_input.push_back((input, wire));
        }
    }

    while let Some((action, wire)) = no_input.pop_front() {
        if val_re.is_match(&action) {
            val_on_input(&mut circuit_map, &action, &wire, &val_re)?;
        } else if and_re.is_match(&action) {
            process_and(&mut circuit_map, &mut no_input, &action, &wire, &and_re)?;
        } else if or_re.is_match(&action) {
            process_or(&mut circuit_map, &mut no_input, &action, &wire, &or_re)?;
        } else if l_shift_re.is_match(&action) {
            process_lshift(&mut circuit_map, &mut no_input, &action, &wire, &l_shift_re)?;
        } else if r_shift_re.is_match(&action) {
            process_rshift(&mut circuit_map, &mut no_input, &action, &wire, &r_shift_re)?;
        } else if not_re.is_match(&action) {
            process_not(&mut circuit_map, &mut no_input, &action, &wire, &not_re)?;
        } else if let Some(val) = circuit_map.get(&action) {
            let val = *val;
            *circuit_map.entry(wire.clone()).or_insert(val) = val;
        } else {
            push_back(&mut no_input, &action, &wire);
        }
    }

    Ok(circuit_map)
}

fn val_on_input(
    circuit_map: &mut HashMap<String, u16>,
    action: &str,
    wire: &str,
    re: &Regex,
) -> Result<()> {
    for cap in re.captures_iter(action) {
        let val = get_cap_u16(1, &cap)?;
        *circuit_map.entry(wire.to_string()).or_insert(val) = val;
    }
    Ok(())
}

fn process_and(
    circuit_map: &mut HashMap<String, u16>,
    no_input: &mut VecDeque<(String, String)>,
    action: &str,
    wire: &str,
    re: &Regex,
) -> Result<()> {
    for cap in re.captures_iter(action) {
        let left = get_cap(1, &cap)?;
        let right = get_cap(2, &cap)?;

        if let Some(left_val) = circuit_map.get(&left) {
            if let Some(right_val) = circuit_map.get(&right) {
                let val = left_val & right_val;
                *circuit_map.entry(wire.to_string()).or_insert(val) = val;
            } else {
                push_back(no_input, action, wire);
            }
        } else if let Ok(left_val) = left.parse::<u16>() {
            if let Some(right_val) = circuit_map.get(&right) {
                let val = left_val & right_val;
                *circuit_map.entry(wire.to_string()).or_insert(val) = val;
            } else {
                push_back(no_input, action, wire);
            }
        } else {
            push_back(no_input, action, wire);
        }
    }
    Ok(())
}

fn process_or(
    circuit_map: &mut HashMap<String, u16>,
    no_input: &mut VecDeque<(String, String)>,
    action: &str,
    wire: &str,
    re: &Regex,
) -> Result<()> {
    for cap in re.captures_iter(action) {
        let left = get_cap(1, &cap)?;
        let right = get_cap(2, &cap)?;

        if let Some(left_val) = circuit_map.get(&left) {
            if let Some(right_val) = circuit_map.get(&right) {
                let val = left_val | right_val;
                *circuit_map.entry(wire.to_string()).or_insert(val) = val;
            } else {
                push_back(no_input, action, wire);
            }
        } else {
            push_back(no_input, action, wire);
        }
    }
    Ok(())
}

fn process_lshift(
    circuit_map: &mut HashMap<String, u16>,
    no_input: &mut VecDeque<(String, String)>,
    action: &str,
    wire: &str,
    re: &Regex,
) -> Result<()> {
    for cap in re.captures_iter(action) {
        let left = get_cap(1, &cap)?;
        let right = get_cap_u16(2, &cap)?;

        if let Some(left_val) = circuit_map.get(&left) {
            let val = left_val << right;
            *circuit_map.entry(wire.to_string()).or_insert(val) = val;
        } else {
            push_back(no_input, action, wire);
        }
    }
    Ok(())
}

fn process_rshift(
    circuit_map: &mut HashMap<String, u16>,
    no_input: &mut VecDeque<(String, String)>,
    action: &str,
    wire: &str,
    re: &Regex,
) -> Result<()> {
    for cap in re.captures_iter(action) {
        let left = get_cap(1, &cap)?;
        let right = get_cap_u16(2, &cap)?;

        if let Some(left_val) = circuit_map.get(&left) {
            let val = left_val >> right;
            *circuit_map.entry(wire.to_string()).or_insert(val) = val;
        } else {
            push_back(no_input, action, wire);
        }
    }
    Ok(())
}

fn process_not(
    circuit_map: &mut HashMap<String, u16>,
    no_input: &mut VecDeque<(String, String)>,
    action: &str,
    wire: &str,
    re: &Regex,
) -> Result<()> {
    for cap in re.captures_iter(action) {
        let right = get_cap(1, &cap)?;

        if let Some(right_val) = circuit_map.get(&right) {
            let val = !right_val;
            *circuit_map.entry(wire.to_string()).or_insert(val) = val;
        } else {
            push_back(no_input, action, wire);
        }
    }
    Ok(())
}

fn push_back(no_input: &mut VecDeque<(String, String)>, action: &str, wire: &str) {
    no_input.push_back((action.to_string(), wire.to_string()));
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD07, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    match find2_br(reader) {
        Ok(map) => map
            .get(&"a".to_string())
            .copied()
            .unwrap_or_default()
            .into(),
        Err(e) => {
            eprintln!("{e}");
            1
        }
    }
}

fn find2_br<T>(reader: T) -> Result<HashMap<String, u16>>
where
    T: BufRead,
{
    let mut first_pass = vec![];
    let mut second_pass = vec![];

    for line in valid_lines(reader) {
        first_pass.push(line.clone());
        second_pass.push(line.clone());
    }

    if let Some(buf) = first_pass.into_iter().reduce(|a, b| format!("{a}\n{b}")) {
        let cursor = Cursor::new(buf);
        let mut circuit_map = find_br(cursor)?;
        let val_at_a = circuit_map
            .get(&"a".to_string())
            .copied()
            .ok_or_else(|| anyhow!("invalid a value"))?;

        circuit_map.clear();
        *circuit_map.entry("b".to_string()).or_default() = val_at_a;

        let line_re = Regex::new(r"(.*) -> (.*)")?;
        let val_re = Regex::new(r"(^\d+$)")?;
        let and_re = Regex::new(r"(.*) AND (.*)")?;
        let or_re = Regex::new(r"(.*) OR (.*)")?;
        let l_shift_re = Regex::new(r"(.*) LSHIFT (.*)")?;
        let r_shift_re = Regex::new(r"(.*) RSHIFT (.*)")?;
        let not_re = Regex::new(r"NOT (.*)")?;
        let mut no_input = VecDeque::new();

        if let Some(buf) = second_pass.into_iter().reduce(|a, b| format!("{a}\n{b}")) {
            let cursor = Cursor::new(buf);
            for line in valid_lines(cursor) {
                for cap in line_re.captures_iter(&line) {
                    let input = get_cap(1, &cap)?;
                    let wire = get_cap(2, &cap)?;
                    no_input.push_back((input, wire));
                }
            }

            while let Some((action, wire)) = no_input.pop_front() {
                if val_re.is_match(&action) {
                    val_on_input_p2(&mut circuit_map, &action, &wire, &val_re)?;
                } else if and_re.is_match(&action) {
                    process_and(&mut circuit_map, &mut no_input, &action, &wire, &and_re)?;
                } else if or_re.is_match(&action) {
                    process_or(&mut circuit_map, &mut no_input, &action, &wire, &or_re)?;
                } else if l_shift_re.is_match(&action) {
                    process_lshift(&mut circuit_map, &mut no_input, &action, &wire, &l_shift_re)?;
                } else if r_shift_re.is_match(&action) {
                    process_rshift(&mut circuit_map, &mut no_input, &action, &wire, &r_shift_re)?;
                } else if not_re.is_match(&action) {
                    process_not(&mut circuit_map, &mut no_input, &action, &wire, &not_re)?;
                } else if let Some(val) = circuit_map.get(&action) {
                    let val = *val;
                    *circuit_map.entry(wire.clone()).or_insert(val) = val;
                } else {
                    push_back(&mut no_input, &action, &wire);
                }
            }
            Ok(circuit_map)
        } else {
            Err(anyhow!("Bad Bad BAd"))
        }
    } else {
        Err(anyhow!("Bad Bad BAd"))
    }
}

fn val_on_input_p2(
    circuit_map: &mut HashMap<String, u16>,
    action: &str,
    wire: &str,
    re: &Regex,
) -> Result<()> {
    for cap in re.captures_iter(action) {
        let val = get_cap_u16(1, &cap)?;
        if wire != "b" {
            *circuit_map.entry(wire.to_string()).or_insert(val) = val;
        }
    }
    Ok(())
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";

    #[test]
    fn solution() -> Result<()> {
        let map = find_br(Cursor::new(TEST_1))?;
        assert_eq!(Some(&72), map.get(&"d".to_string()));
        assert_eq!(Some(&507), map.get(&"e".to_string()));
        assert_eq!(Some(&492), map.get(&"f".to_string()));
        assert_eq!(Some(&114), map.get(&"g".to_string()));
        assert_eq!(Some(&65412), map.get(&"h".to_string()));
        assert_eq!(Some(&65079), map.get(&"i".to_string()));
        assert_eq!(Some(&123), map.get(&"x".to_string()));
        assert_eq!(Some(&456), map.get(&"y".to_string()));
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    // use super::find2_br;
    use anyhow::Result;
    // use std::io::Cursor;

    // const TEST_1: &str = r"turn on 0,0 through 0,0";
    // const TEST_2: &str = r"toggle 0,0 through 999,999";

    #[test]
    fn solution() -> Result<()> {
        // assert_eq!(find2_br(Cursor::new(TEST_1))?, 1);
        // assert_eq!(find2_br(Cursor::new(TEST_2))?, 2_000_000);
        Ok(())
    }
}
