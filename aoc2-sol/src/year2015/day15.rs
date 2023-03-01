// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 15 "Science for Hungry People"
//!
//! **--- Day 15: Science for Hungry People ---**
//!
//! **--- Part 1 ---*
//!
//! Today, you set out on the task of perfecting your milk-dunking cookie recipe.
//! All you have to do is find the right balance of ingredients.
//!
//! Your recipe leaves room for exactly 100 teaspoons of ingredients.
//! You make a list of the remaining ingredients you could use to finish the recipe
//! (your puzzle input) and their properties per teaspoon:
//!
//! ```text
//! capacity (how well it helps the cookie absorb milk)
//! durability (how well it keeps the cookie intact when full of milk)
//! flavor (how tasty it makes the cookie)
//! texture (how it improves the feel of the cookie)
//! calories (how many calories it adds to the cookie)
//! ```
//!
//! You can only measure ingredients in whole-teaspoon amounts accurately, and you
//! have to be accurate so you can reproduce your results in the future. The total
//! score of a cookie can be found by adding up each of the properties
//! (negative totals become 0) and then multiplying together everything except calories.
//!
//! For instance, suppose you have these two ingredients:
//!
//! ```text
//! Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
//! Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
//! ```
//!
//! Then, choosing to use `44` teaspoons of butterscotch and `56` teaspoons of cinnamon
//! (because the amounts of each ingredient must add up to 100) would result in a cookie
//! with the following properties:
//!
//! ```text
//! A capacity of 44*-1 + 56*2 = 68
//! A durability of 44*-2 + 56*3 = 80
//! A flavor of 44*6 + 56*-2 = 152
//! A texture of 44*3 + 56*-1 = 76
//! ```
//!
//! Multiplying these together (`68 * 80 * 152 * 76`, ignoring calories for now) results in
//! a total score of `62842880`, which happens to be the best score possible given these
//! ingredients. If any properties had produced a negative total, it would have instead
//! become zero, causing the whole score to multiply to zero.
//!
//! Given the ingredients in your kitchen and their properties, what is the total score
//! of the highest-scoring cookie you can make?
//!
//! **--- Part Two ---**
//!
//! Your cookie recipe becomes wildly popular! Someone asks if you can make another recipe
//! that has exactly `500` calories per cookie (so they can use it as a meal replacement).
//! Keep the rest of your award-winning process the same (100 teaspoons, same ingredients,
//! same scoring system).
//!
//! For example, given the ingredients above, if you had instead selected `40` teaspoons of
//! butterscotch and `60` teaspoons of cinnamon (which still adds to 100), the total calorie
//! count would be `40*8 + 60*3 = 500`. The total score would go down, though: only `57600000`,
//! the best you can do in such trying circumstances.
//!
//! Given the ingredients in your kitchen and their properties, what is the total score of the
//! highest-scoring cookie you can make with a calorie total of `500`?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{get_cap, get_cap_x, run_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use itertools::Itertools;
use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Ingredient {
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize,
}

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_solution::<isize>(AoCYear::AOC2015, AoCDay::AOCD15, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> isize {
    find_br(reader).unwrap_or_default()
}

fn find_br<T>(reader: T) -> Result<isize>
where
    T: BufRead,
{
    let recipe_re = Regex::new(
        r"^(.*): capacity (-?\d), durability (-?\d), flavor (-?\d), texture (-?\d), calories (-?\d)$",
    )?;
    let mut i_map = HashMap::new();

    for line in valid_lines(reader) {
        for caps in recipe_re.captures_iter(&line) {
            let name = get_cap(1, &caps)?;
            let capacity = get_cap_x::<isize>(2, &caps)?;
            let durability = get_cap_x::<isize>(3, &caps)?;
            let flavor = get_cap_x::<isize>(4, &caps)?;
            let texture = get_cap_x::<isize>(5, &caps)?;
            let calories = get_cap_x::<isize>(6, &caps)?;

            _ = i_map.insert(
                name,
                Ingredient {
                    capacity,
                    durability,
                    flavor,
                    texture,
                    calories,
                },
            );
        }
    }

    let permuts: Vec<Vec<isize>> = (0..100)
        .permutations(i_map.len())
        .filter(|a| a.iter().sum::<isize>() == 100)
        .collect();

    let mut scores = vec![];

    for hun in permuts {
        let mut t_cap = 0;
        let mut t_dur = 0;
        let mut t_fla = 0;
        let mut t_text = 0;
        for (i, (_, ing)) in i_map.iter().enumerate() {
            let factor = hun[i];
            t_cap += ing.capacity * factor;
            t_dur += ing.durability * factor;
            t_fla += ing.flavor * factor;
            t_text += ing.texture * factor;
        }
        if t_cap < 0 || t_dur < 0 || t_fla < 0 || t_text < 0 {
            scores.push(0);
        } else {
            scores.push(t_cap * t_dur * t_fla * t_text);
        }
    }

    Ok(*scores
        .iter()
        .max()
        .ok_or_else(|| anyhow!("failure to score!"))?)
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<isize>(AoCYear::AOC2015, AoCDay::AOCD15, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> isize {
    find2_br(reader).unwrap_or_default()
}

fn find2_br<T>(reader: T) -> Result<isize>
where
    T: BufRead,
{
    let recipe_re = Regex::new(
        r"^(.*): capacity (-?\d), durability (-?\d), flavor (-?\d), texture (-?\d), calories (-?\d)$",
    )?;
    let mut i_map = HashMap::new();

    for line in valid_lines(reader) {
        for caps in recipe_re.captures_iter(&line) {
            let name = get_cap(1, &caps)?;
            let capacity = get_cap_x::<isize>(2, &caps)?;
            let durability = get_cap_x::<isize>(3, &caps)?;
            let flavor = get_cap_x::<isize>(4, &caps)?;
            let texture = get_cap_x::<isize>(5, &caps)?;
            let calories = get_cap_x::<isize>(6, &caps)?;

            _ = i_map.insert(
                name,
                Ingredient {
                    capacity,
                    durability,
                    flavor,
                    texture,
                    calories,
                },
            );
        }
    }

    let permuts: Vec<Vec<isize>> = (0..100)
        .permutations(i_map.len())
        .filter(|a| a.iter().sum::<isize>() == 100)
        .collect();

    let mut scores = vec![];

    for hun in permuts {
        let mut t_capacity = 0;
        let mut t_dur = 0;
        let mut t_fla = 0;
        let mut t_text = 0;
        let mut t_calories = 0;
        for (i, (_, ing)) in i_map.iter().enumerate() {
            let factor = hun[i];
            t_capacity += ing.capacity * factor;
            t_dur += ing.durability * factor;
            t_fla += ing.flavor * factor;
            t_text += ing.texture * factor;
            t_calories += ing.calories * factor;
        }
        if t_capacity < 0 || t_dur < 0 || t_fla < 0 || t_text < 0 || t_calories != 500 {
            scores.push(0);
        } else {
            scores.push(t_capacity * t_dur * t_fla * t_text);
        }
    }

    Ok(*scores
        .iter()
        .max()
        .ok_or_else(|| anyhow!("failure to score!"))?)
}

#[cfg(test)]
mod one_star {
    use super::find_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1))?, 62_842_880);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find2_br(Cursor::new(TEST_1))?, 57_600_000);
        Ok(())
    }
}
