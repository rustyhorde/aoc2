// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 12 "JSAbacusFramework.io"
//!
//! **--- Day 12: JSAbacusFramework.io ---**
//!
//! **--- Part 1 ---**
//!
//! Santa's Accounting-Elves need help balancing the books after a recent order.
//! Unfortunately, their accounting software uses a peculiar storage format. That's where you come in.
//!
//! They have a JSON document which contains a variety of things: `arrays ([1,2,3])`,
//! `objects ({"a":1, "b":2})`, `numbers`, and `strings`. Your first job is to simply find all of
//! the numbers throughout the document and add them together.
//!
//! For example:
//!
//! ```text
//! [1,2,3] and {"a":2,"b":4} both have a sum of 6.
//! [[[3]]] and {"a":{"b":4},"c":-1} both have a sum of 3.
//! {"a":[-1,1]} and [-1,{"a":1}] both have a sum of 0.
//! [] and {} both have a sum of 0.
//! ```
//!
//! You will not encounter any strings containing numbers.
//!
//! What is the sum of all numbers in the document?
//!
//! **--- Part Two ---**
//!
//! Uh oh - the Accounting-Elves have realized that they double-counted everything red.
//!
//! Ignore any object (and all of its children) which has any property with the value "red".
//! Do this only for objects ({...}), not arrays ([...]).
//!
//! ```text
//! [1,2,3] still has a sum of 6.
//! [1,{"c":"red","b":2},3] now has a sum of 4, because the middle object is ignored.
//! {"d":"red","e":[1,2,3,4],"f":5} now has a sum of 0, because the entire structure is ignored.
//! [1,"red",5] has a sum of 6, because "red" in an array has no effect.
//! ```
//!

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_solution, valid_lines},
};
use anyhow::Result;
use serde_json::{Map, Value};
use std::{
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
    run_solution::<i64>(AoCYear::AOC2015, AoCDay::AOCD12, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> i64 {
    find_br(reader).unwrap_or_default()
}

fn find_br<T>(reader: T) -> Result<i64>
where
    T: BufRead,
{
    let mut result = 0;
    for line in valid_lines(reader) {
        let base: Value = serde_json::from_str(&line)?;
        result = sum_numbers(&base);
    }
    Ok(result)
}

fn sum_numbers(v: &Value) -> i64 {
    match v {
        Value::Object(map) => map.values().map(sum_numbers).sum(),
        Value::Array(arr) => arr.iter().map(sum_numbers).sum(),
        Value::Number(num) => num.as_i64().map_or(0, |val| val),
        _ => 0,
    }
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<i64>(AoCYear::AOC2015, AoCDay::AOCD12, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> i64 {
    find2_br(reader).unwrap_or_default()
}

fn find2_br<T>(reader: T) -> Result<i64>
where
    T: BufRead,
{
    let mut result = 0;
    for line in valid_lines(reader) {
        let base: Value = serde_json::from_str(&line)?;
        result = skip_red(&base);
    }
    Ok(result)
}

fn skip_red(v: &Value) -> i64 {
    match v {
        Value::Object(map) => {
            if no_red(map) {
                map.values().map(skip_red).sum()
            } else {
                0
            }
        }
        Value::Array(arr) => arr.iter().map(skip_red).sum(),
        Value::Number(num) => num.as_i64().map_or(0, |val| val),
        _ => 0,
    }
}

fn no_red(map: &Map<String, Value>) -> bool {
    map.values().all(|v| match v {
        Value::String(val) => val != "red",
        _ => true,
    })
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"[1,2,3]";
    const TEST_2: &str = r#"{"a":2,"b":4}"#;
    const TEST_3: &str = r"[[[3]]]";
    const TEST_4: &str = r#"{"a":{"b":4},"c":-1}"#;
    const TEST_5: &str = r#"{"a":[-1,1]}"#;
    const TEST_6: &str = r#"[-1,{"a":1}]"#;
    const TEST_7: &str = r"{}";
    const TEST_8: &str = r"[]";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1))?, 6);
        assert_eq!(find_br(Cursor::new(TEST_2))?, 6);
        assert_eq!(find_br(Cursor::new(TEST_3))?, 3);
        assert_eq!(find_br(Cursor::new(TEST_4))?, 3);
        assert_eq!(find_br(Cursor::new(TEST_5))?, 0);
        assert_eq!(find_br(Cursor::new(TEST_6))?, 0);
        assert_eq!(find_br(Cursor::new(TEST_7))?, 0);
        assert_eq!(find_br(Cursor::new(TEST_8))?, 0);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"[1,2,3]";
    const TEST_2: &str = r#"[1,{"c":"red","b":2},3]"#;
    const TEST_3: &str = r#"{"d":"red","e":[1,2,3,4],"f":5}"#;
    const TEST_4: &str = r#"[1,"red",5]"#;

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find2_br(Cursor::new(TEST_1))?, 6);
        assert_eq!(find2_br(Cursor::new(TEST_2))?, 4);
        assert_eq!(find2_br(Cursor::new(TEST_3))?, 0);
        assert_eq!(find2_br(Cursor::new(TEST_4))?, 6);
        Ok(())
    }
}
