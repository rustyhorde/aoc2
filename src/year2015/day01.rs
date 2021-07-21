//! Advent of Code - Day 1 "Not Quite Lisp" Solution
//!
//! --- Day 1: Not Quite Lisp ---
//! Santa was hoping for a white Christmas, but his weather machine's "snow"
//! function is powered by stars, and he's fresh out! To save Christmas, he
//! needs you to collect fifty stars by December 25th.
//!
//! Collect stars by helping Santa solve puzzles. Two puzzles will be made
//! available on each day in the Advent calendar; the second puzzle is unlocked
//! when you complete the first. Each puzzle grants one star. Good luck!
//!
//! Here's an easy puzzle to warm you up.
//!
//! Santa is trying to deliver presents in a large apartment building, but
//! he can't find the right floor - the directions he got are a little confusing.
//! He starts on the ground floor (floor 0) and then follows the instructions
//! one character at a time.
//!
//! An opening parenthesis, (, means he should go up one floor, and a closing
//! parenthesis, ), means he should go down one floor.
//!
//! The apartment building is very tall, and the basement is very deep;
//! he will never find the top or bottom floors.
//!
//! For example:
//!
//! * (()) and ()() both result in floor 0.
//! * ((( and (()(()( both result in floor 3.
//! * ))((((( also results in floor 3.
//! * ()) and ))( both result in floor -1 (the first basement level).
//! * ))) and )())()) both result in floor -3.
//!
//! To what floor do the instructions take Santa?

use crate::utils::elapsed_parts;
use anyhow::Result;
use std::time::Instant;

#[allow(clippy::unnecessary_wraps)]
pub(crate) fn part_1(iter: impl Iterator<Item = std::io::Result<String>> + Send) -> Result<u32> {
    let now = Instant::now();
    let final_floor = find_floor(iter);
    let (whole, frac, units) = elapsed_parts(now.elapsed())?;
    println!("Answer:  {}", final_floor);
    println!("Elapsed: {}.{}{}", whole, frac, units);
    Ok(0)
}

fn find_floor(iter: impl Iterator<Item = std::io::Result<String>> + Send) -> isize {
    iter.filter_map(std::result::Result::ok)
        .fold(0, handle_line)
}

#[allow(clippy::needless_pass_by_value)]
#[inline]
fn handle_line(acc: isize, line: String) -> isize {
    line.chars().fold(acc, up_or_down)
}

#[inline]
fn up_or_down(acc: isize, ch: char) -> isize {
    match ch {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc,
    }
}

#[allow(clippy::unnecessary_wraps)]
pub(crate) fn part_2(iter: impl Iterator<Item = std::io::Result<String>> + Send) -> Result<u32> {
    let now = Instant::now();
    let found_at = find_basement(iter);
    let (whole, frac, units) = elapsed_parts(now.elapsed())?;
    println!("Answer:  {}", found_at);
    println!("Elapsed: {}.{}{}", whole, frac, units);
    Ok(0)
}

fn find_basement(iter: impl Iterator<Item = std::io::Result<String>> + Send) -> i32 {
    let mut state = (0, 0);

    iter.filter_map(std::result::Result::ok)
        .map(|line| line.chars().scan(&mut state, handle_ch).for_each(|_| ()))
        .for_each(|_| ());

    state.0
}

#[allow(clippy::mut_mut)]
fn handle_ch(state: &mut &mut (i32, i32), ch: char) -> Option<(i32, i32)> {
    state.0 += 1;

    if state.1 == 0 && ch == ')' {
        None
    } else {
        match ch {
            '(' => state.1 += 1,
            ')' => state.1 -= 1,
            _ => {}
        }
        Some(**state)
    }
}

#[cfg(test)]
mod one_star {
    use super::find_floor;
    use anyhow::Result;
    use std::io::{BufRead, Cursor};

    const TEST_CHAIN: &str = r"(())";
    const TEST_CHAIN_1: &str = r"()()";
    const TEST_CHAIN_2: &str = r"(((";
    const TEST_CHAIN_3: &str = r"(()(()(";
    const TEST_CHAIN_4: &str = r"))(((((";
    const TEST_CHAIN_5: &str = r"())";
    const TEST_CHAIN_6: &str = r"))(";
    const TEST_CHAIN_7: &str = r")))";
    const TEST_CHAIN_8: &str = r")())())";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_floor(Cursor::new(TEST_CHAIN).lines()), 0);
        assert_eq!(find_floor(Cursor::new(TEST_CHAIN_1).lines()), 0);
        assert_eq!(find_floor(Cursor::new(TEST_CHAIN_2).lines()), 3);
        assert_eq!(find_floor(Cursor::new(TEST_CHAIN_3).lines()), 3);
        assert_eq!(find_floor(Cursor::new(TEST_CHAIN_4).lines()), 3);
        assert_eq!(find_floor(Cursor::new(TEST_CHAIN_5).lines()), -1);
        assert_eq!(find_floor(Cursor::new(TEST_CHAIN_6).lines()), -1);
        assert_eq!(find_floor(Cursor::new(TEST_CHAIN_7).lines()), -3);
        assert_eq!(find_floor(Cursor::new(TEST_CHAIN_8).lines()), -3);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find_basement;
    use anyhow::Result;
    use std::io::{BufRead, Cursor};

    const TEST_CHAIN: &str = r")";
    const TEST_CHAIN_1: &str = r"()())";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(
            find_basement(Cursor::new(TEST_CHAIN).lines()),
            1,
            "they don't match"
        );
        assert_eq!(find_basement(Cursor::new(TEST_CHAIN_1).lines()), 5);
        Ok(())
    }
}
