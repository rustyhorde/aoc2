// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 7 "Internet Protocol Version 7"
//!
//! **--- Day 7: Internet Protocol Version 7 ---**
//!
//! **--- Part 1 ---**
//!
//! While snooping around the local network of EBHQ, you compile a list of
//! IP addresses (they're `IPv7`, of course; `IPv6` is much too limited). You'd like
//! to figure out which IPs support TLS (transport-layer snooping).
//!
//! An IP supports TLS if it has an Autonomous Bridge Bypass Annotation, or `ABBA`.
//! An `ABBA` is any four-character sequence which consists of a pair of two different
//! characters followed by the reverse of that pair, such as `xyyx` or `abba`.
//! However, the IP also must not have an ABBA within any hypernet sequences, which
//! are contained by square brackets.
//!
//! For example:
//!
//! ```text
//! abba[mnop]qrst supports TLS (abba outside square brackets).
//! abcd[bddb]xyyx does not support TLS (bddb is within square brackets, even though xyyx is outside square brackets).
//! aaaa[qwer]tyui does not support TLS (aaaa is invalid; the interior characters must be different).
//! ioxxoj[asdfgh]zxcvbn supports TLS (oxxo is outside square brackets, even though it's within a larger string).
//! ```
//!
//! How many IPs in your puzzle input support TLS?
//!
//! **--- Part Two ---**
//!
//! You would also like to know which IPs support SSL (super-secret listening).
//!
//! An IP supports SSL if it has an Area-Broadcast Accessor, or ABA, anywhere in the
//! supernet sequences (outside any square bracketed sections), and a corresponding
//! Byte Allocation Block, or BAB, anywhere in the hypernet sequences. An ABA is any
//! three-character sequence which consists of the same character twice with a different
//! character between them, such as `xyx` or `aba`. A corresponding BAB is the same
//! characters but in reversed positions: `yxy` and `bab`, respectively.
//!
//! For example:
//!
//! ```text
//! aba[bab]xyz supports SSL (aba outside square brackets with corresponding bab within square brackets).
//! xyx[xyx]xyx does not support SSL (xyx, but no corresponding yxy).
//! aaa[kek]eke supports SSL (eke in supernet with corresponding kek in hypernet; the aaa sequence is not related, because the interior character must be different).
//! zazbz[bzb]cdb supports SSL (zaz has no corresponding aza, but zbz has a corresponding bzb, even though zaz and zbz overlap).
//! ```
//!
//! How many IPs in your puzzle input support SSL?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_solution, valid_lines},
};
use anyhow::Result;
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
    run_solution::<usize>(AoCYear::AOC2016, AoCDay::AOCD07, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader)
}

fn find_br<T>(reader: T) -> usize
where
    T: BufRead,
{
    let mut ips = vec![];
    for line in valid_lines(reader) {
        let mut hypers = vec![];
        let mut supers = vec![];
        let mut hyper = String::new();
        let mut sup = String::new();
        let mut in_hyper = false;
        for ch in line.chars() {
            match ch {
                '[' => {
                    supers.push(sup.clone());
                    sup.clear();
                    in_hyper = true;
                }
                ']' => {
                    hypers.push(hyper.clone());
                    hyper.clear();
                    in_hyper = false;
                }
                _ => {
                    if in_hyper {
                        hyper.push(ch);
                    } else {
                        sup.push(ch);
                    }
                }
            }
        }
        if !sup.is_empty() {
            supers.push(sup);
        }
        ips.push((hypers, supers));
    }

    let mut valid = 0;

    'outer: for (hypers, supers) in ips {
        for hyper in hypers {
            for win in hyper.as_bytes().windows(4) {
                if win[0] == win[3] && win[1] == win[2] && win[0] != win[1] {
                    continue 'outer;
                }
            }
        }
        for sup in supers {
            for win in sup.as_bytes().windows(4) {
                if win[0] == win[3] && win[1] == win[2] && win[0] != win[1] {
                    valid += 1;
                    continue 'outer;
                }
            }
        }
    }
    valid
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2016, AoCDay::AOCD07, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader)
}

fn find2_br<T>(reader: T) -> usize
where
    T: BufRead,
{
    let mut ips = vec![];
    for line in valid_lines(reader) {
        let mut hypers = vec![];
        let mut supers = vec![];
        let mut hyper = String::new();
        let mut sup = String::new();
        let mut in_hyper = false;
        for ch in line.chars() {
            match ch {
                '[' => {
                    supers.push(sup.clone());
                    sup.clear();
                    in_hyper = true;
                }
                ']' => {
                    hypers.push(hyper.clone());
                    hyper.clear();
                    in_hyper = false;
                }
                _ => {
                    if in_hyper {
                        hyper.push(ch);
                    } else {
                        sup.push(ch);
                    }
                }
            }
        }
        if !sup.is_empty() {
            supers.push(sup);
        }
        ips.push((hypers, supers));
    }

    let mut valid = 0;

    'outer: for (hypers, supers) in ips {
        let mut abas = vec![];
        for hyper in hypers {
            for win in hyper.as_bytes().windows(3) {
                if win[0] == win[2] && win[0] != win[1] {
                    let opposite = vec![win[1], win[0], win[1]];
                    abas.push(opposite);
                }
            }
        }
        for sup in supers {
            for win in sup.as_bytes().windows(3) {
                if win[0] == win[2] && win[0] != win[1] {
                    let bab = vec![win[0], win[1], win[2]];
                    if abas.contains(&bab) {
                        valid += 1;
                        continue 'outer;
                    }
                }
            }
        }
    }

    valid
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use std::io::Cursor;

    const TEST_1: &str = r"abba[mnop]qrst
abcd[bddb]xyyx
aaaa[qwer]tyui
ioxxoj[asdfgh]zxcvbn";

    #[test]
    fn solution() {
        assert_eq!(find_br(Cursor::new(TEST_1)), 2);
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use std::io::Cursor;

    const TEST_1: &str = r"aba[bab]xyz
xyx[xyx]xyx
aaa[kek]eke
zazbz[bzb]cdb";

    #[test]
    fn solution() {
        assert_eq!(find2_br(Cursor::new(TEST_1)), 3);
    }
}
