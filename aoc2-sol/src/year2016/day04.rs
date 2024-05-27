// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 4 "Security Through Obscurity"
//!
//! **--- Day 4: Security Through Obscurity ---**
//!
//! **--- Part 1 ---**
//!
//! Finally, you come across an information kiosk with a list of rooms. Of
//! course, the list is encrypted and full of decoy data, but the instructions
//! to decode the list are barely hidden nearby. Better remove the decoy data first.
//!
//! Each room consists of an encrypted name (lowercase letters separated by dashes)
//! followed by a dash, a sector ID, and a checksum in square brackets.
//!
//! A room is real (not a decoy) if the checksum is the five most common letters in
//! the encrypted name, in order, with ties broken by alphabetization. For example:
//!
//! ```text
//! aaaaa-bbb-z-y-x-123[abxyz] is a real room because the most common letters are a (5), b (3), and then a tie between x, y, and z, which are listed alphabetically.
//! a-b-c-d-e-f-g-h-987[abcde] is a real room because although the letters are all tied (1 of each), the first five are listed alphabetically.
//! not-a-real-room-404[oarel] is a real room.
//! totally-real-room-200[decoy] is not.
//! ```
//!
//! Of the real rooms from the list above, the sum of their sector IDs is 1514.
//!
//! What is the sum of the sector IDs of the real rooms?
//!
//! **--- Part Two ---**
//!
//! With all the decoy data out of the way, it's time to decrypt this list and get moving.
//!
//! The room names are encrypted by a state-of-the-art shift cipher, which
//! is nearly unbreakable without the right software. However, the information
//! kiosk designers at Easter Bunny HQ were not expecting to deal with a master
//! cryptographer like yourself.
//!
//! To decrypt a room name, rotate each letter forward through the alphabet a number of
//! times equal to the room's sector ID. A becomes B, B becomes C, Z becomes A, and so on.
//! Dashes become spaces.
//!
//! For example, the real name for `qzmt-zixmtkozy-ivhz-343` is very encrypted name.
//!
//! What is the sector ID of the room where North Pole objects are stored?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{get_cap, get_cap_x, run_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use itertools::Itertools;
use regex::Regex;
use std::{
    cmp::Ordering,
    collections::BTreeMap,
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
    run_solution::<usize>(AoCYear::AOC2016, AoCDay::AOCD04, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader).unwrap_or_default()
}

fn find_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let enc_re = Regex::new(r"^(.*)-(\d+)\[([a-z]{5})\]$")?;
    let mut rooms = vec![];

    for line in valid_lines(reader) {
        for caps in enc_re.captures_iter(&line) {
            let enc = get_cap(1, &caps)?.replace('-', "");
            let section_id = get_cap_x::<usize>(2, &caps)?;
            let checksum = get_cap(3, &caps)?;
            rooms.push((enc, section_id, checksum));
        }
    }

    let mut total = 0;
    for (enc, s_id, checksum) in rooms {
        let mut f_map = BTreeMap::new();
        let mut tuples = vec![];
        for ch in enc.chars() {
            *f_map.entry(ch).or_insert(0) += 1;
        }

        for (k, v) in f_map {
            tuples.push((k, v));
        }

        let sorted = tuples
            .into_iter()
            .sorted_by(|(ch1, c1), (ch2, c2)| {
                if c2.cmp(c1) == Ordering::Equal {
                    ch1.cmp(ch2)
                } else {
                    c2.cmp(c1)
                }
            })
            .collect::<Vec<(char, i32)>>();

        let mut calc_all = String::new();
        for tuple in sorted {
            calc_all.push(tuple.0);
        }
        let calc_chk = &calc_all[0..5];

        if checksum == calc_chk {
            total += s_id;
        }
    }

    Ok(total)
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2016, AoCDay::AOCD04, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader, "pole").unwrap_or_default()
}

fn find2_br<T>(reader: T, contains: &str) -> Result<usize>
where
    T: BufRead,
{
    let enc_re = Regex::new(r"^(.*)-(\d+)\[([a-z]{5})\]$")?;
    let mut rooms = vec![];

    for line in valid_lines(reader) {
        for caps in enc_re.captures_iter(&line) {
            let enc = get_cap(1, &caps)?.replace('-', "");
            let full_enc = get_cap(1, &caps)?;
            let section_id = get_cap_x::<usize>(2, &caps)?;
            let checksum = get_cap(3, &caps)?;
            rooms.push((enc, full_enc, section_id, checksum));
        }
    }

    let mut names = vec![];
    for (enc, full_enc, s_id, checksum) in rooms {
        let mut f_map = BTreeMap::new();
        let mut tuples = vec![];
        for ch in enc.chars() {
            *f_map.entry(ch).or_insert(0) += 1;
        }

        for (k, v) in f_map {
            tuples.push((k, v));
        }

        let sorted = tuples
            .into_iter()
            .sorted_by(|(ch1, c1), (ch2, c2)| {
                if c2.cmp(c1) == Ordering::Equal {
                    ch1.cmp(ch2)
                } else {
                    c2.cmp(c1)
                }
            })
            .collect::<Vec<(char, i32)>>();

        let mut calc_all = String::new();
        for tuple in sorted {
            calc_all.push(tuple.0);
        }
        let calc_chk = &calc_all[0..5];

        if checksum == calc_chk {
            let mut as_nums = convert_to_nums(&full_enc);
            for _i in 0..s_id {
                for val in &mut as_nums {
                    if *val != 100 {
                        *val = (*val + 1) % 26;
                    }
                }
            }
            let name = convert_to_string(&as_nums);
            names.push((name, s_id));
        }
    }

    let mut f_s_id = 0;
    for (_name, s_id) in names.iter().filter(|(name, _)| name.contains(contains)) {
        f_s_id = *s_id;
    }
    Ok(f_s_id)
}

fn convert_to_nums(full_enc: &str) -> Vec<usize> {
    full_enc
        .chars()
        .map(|ch| {
            Ok(match ch {
                'a' => 0,
                'b' => 1,
                'c' => 2,
                'd' => 3,
                'e' => 4,
                'f' => 5,
                'g' => 6,
                'h' => 7,
                'i' => 8,
                'j' => 9,
                'k' => 10,
                'l' => 11,
                'm' => 12,
                'n' => 13,
                'o' => 14,
                'p' => 15,
                'q' => 16,
                'r' => 17,
                's' => 18,
                't' => 19,
                'u' => 20,
                'v' => 21,
                'w' => 22,
                'x' => 23,
                'y' => 24,
                'z' => 25,
                '-' => 100,
                _ => return Err(anyhow!("invalid name")),
            })
        })
        .filter_map(std::result::Result::ok)
        .collect()
}

fn convert_to_string(nums: &[usize]) -> String {
    nums.iter().fold(String::new(), |acc, num| {
        let mut buf = acc;
        buf.push(match num {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            8 => 'i',
            9 => 'j',
            10 => 'k',
            11 => 'l',
            12 => 'm',
            13 => 'n',
            14 => 'o',
            15 => 'p',
            16 => 'q',
            17 => 'r',
            18 => 's',
            19 => 't',
            20 => 'u',
            21 => 'v',
            22 => 'w',
            23 => 'x',
            24 => 'y',
            25 => 'z',
            100 => ' ',
            _ => 'X',
        });
        buf
    })
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1))?, 1514);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"qzmt-zixmtkozy-ivhz-343[zimth]";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find2_br(Cursor::new(TEST_1), "encrypted")?, 343);
        Ok(())
    }
}
