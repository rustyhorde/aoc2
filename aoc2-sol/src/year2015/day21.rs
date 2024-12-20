// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 21 "RPG Simulator 20XX"
//!
//! **--- Day 21: RPG Simulator 20XX ---**
//!
//! **--- Part 1 ---**
//!
//! Little Henry Case got a new video game for Christmas. It's an RPG, and he's stuck on a boss.
//! He needs to know what equipment to buy at the shop. He hands you the controller.
//!
//! In this game, the player (you) and the enemy (the boss) take turns attacking. The player
//! always goes first. Each attack reduces the opponent's hit points by at least 1. The first
//! character at or below 0 hit points loses.
//!
//! Damage dealt by an attacker each turn is equal to the attacker's damage score minus the
//! defender's armor score. An attacker always does at least 1 damage. So, if the attacker
//! has a damage score of 8, and the defender has an armor score of 3, the defender loses 5 hit points.
//! If the defender had an armor score of 300, the defender would still lose 1 hit point.
//!
//! Your damage score and armor score both start at zero. They can be increased by buying items
//! in exchange for gold. You start with no items and have as much gold as you need. Your total damage
//!  or armor is equal to the sum of those stats from all of your items. You have `100` hit points.
//!
//! Here is what the item shop is selling:
//!
//! ```text
//! Weapons:    Cost  Damage  Armor
//! Dagger        8     4       0
//! Shortsword   10     5       0
//! Warhammer    25     6       0
//! Longsword    40     7       0
//! Greataxe     74     8       0
//!
//! Armor:      Cost  Damage  Armor
//! Leather      13     0       1
//! Chainmail    31     0       2
//! Splintmail   53     0       3
//! Bandedmail   75     0       4
//! Platemail   102     0       5
//!
//! Rings:      Cost  Damage  Armor
//! Damage +1    25     1       0
//! Damage +2    50     2       0
//! Damage +3   100     3       0
//! Defense +1   20     0       1
//! Defense +2   40     0       2
//! Defense +3   80     0       3
//! ```
//!
//! You must buy exactly one weapon; no dual-wielding. Armor is optional, but you can't use more than one.
//! You can buy 0-2 rings (at most one for each hand). You must use any items you buy. The shop only
//! has one of each item, so you can't buy, for example, two rings of Damage +3.
//!
//! For example, suppose you have 8 hit points, 5 damage, and 5 armor, and that the boss has
//! 12 hit points, 7 damage, and 2 armor:
//!
//! ```text
//! The player deals 5-2 = 3 damage; the boss goes down to 9 hit points.
//! The boss deals 7-5 = 2 damage; the player goes down to 6 hit points.
//! The player deals 5-2 = 3 damage; the boss goes down to 6 hit points.
//! The boss deals 7-5 = 2 damage; the player goes down to 4 hit points.
//! The player deals 5-2 = 3 damage; the boss goes down to 3 hit points.
//! The boss deals 7-5 = 2 damage; the player goes down to 2 hit points.
//! The player deals 5-2 = 3 damage; the boss goes down to 0 hit points.
//! In this scenario, the player wins! (Barely.)
//! ```
//!
//! You have `100` hit points. The boss's actual stats are in your puzzle input.
//! What is the least amount of gold you can spend and still win the fight?
//!
//! **--- Part Two ---**
//!
//! Turns out the shopkeeper is working with the boss, and can persuade you
//! to buy whatever items he wants. The other rules still apply, and he still
//! only has one of each item.
//!
//! What is the most amount of gold you can spend and still lose the fight?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{get_cap_x, run_solution, valid_lines},
};
use anyhow::{Context, Result};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fmt,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WeaponKind {
    Dagger,
    Shortsword,
    Warhammer,
    Longsword,
    Greataxe,
}
impl fmt::Display for WeaponKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Dagger => "Dagger",
                Self::Shortsword => "Short Sword",
                Self::Warhammer => "War Hammer",
                Self::Longsword => "Long Sword",
                Self::Greataxe => "Great Axe",
            }
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Weapon {
    kind: WeaponKind,
    cost: usize,
    damage: usize,
    armor: usize,
}

impl fmt::Display for Weapon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ArmorKind {
    Leather,
    Chainmail,
    Splintmail,
    Bandedmail,
    Platemail,
}

impl fmt::Display for ArmorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Leather => "Leather",
                Self::Chainmail => "Chain Mail",
                Self::Splintmail => "Splint Mail",
                Self::Bandedmail => "Banded Mail",
                Self::Platemail => "Plate Mail",
            }
        )
    }
}

#[allow(clippy::struct_field_names)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Armor {
    kind: ArmorKind,
    cost: usize,
    damage: usize,
    armor: usize,
}

impl fmt::Display for Armor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RingKind {
    Damage1,
    Damage2,
    Damage3,
    Defense1,
    Defense2,
    Defense3,
}

impl fmt::Display for RingKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Damage1 => "Damage +1",
                Self::Damage2 => "Damage +2",
                Self::Damage3 => "Damage +3",
                Self::Defense1 => "Defense +1",
                Self::Defense2 => "Defense +2",
                Self::Defense3 => "Defense +3",
            }
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Ring {
    kind: RingKind,
    cost: usize,
    damage: usize,
    armor: usize,
}

impl fmt::Display for Ring {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Player {
    weapon: Weapon,
    armor: Option<Armor>,
    ring_1: Option<Ring>,
    ring_2: Option<Ring>,
    total_cost: usize,
    hit_points: usize,
    damage: usize,
    total_armor: usize,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "w: {}, a: {}, r1: {}, r2: {}, hp: {}, d: {}, ta: {}, c: {}",
            self.weapon,
            self.armor
                .map_or_else(|| "None".to_string(), |armor| armor.to_string()),
            self.ring_1
                .map_or_else(|| "None".to_string(), |ring_1| ring_1.to_string()),
            self.ring_2
                .map_or_else(|| "None".to_string(), |ring_2| ring_2.to_string()),
            self.hit_points,
            self.damage,
            self.total_armor,
            self.total_cost,
        )
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct Boss {
    hit_points: usize,
    damage: usize,
    total_armor: usize,
}

impl fmt::Display for Boss {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "hp: {}, d: {}, a: {}",
            self.hit_points, self.damage, self.total_armor
        )
    }
}

lazy_static! {
    static ref WEAPONS: Vec<Weapon> = {
        vec![
            Weapon {
                kind: WeaponKind::Dagger,
                cost: 8,
                damage: 4,
                armor: 0,
            },
            Weapon {
                kind: WeaponKind::Shortsword,
                cost: 10,
                damage: 5,
                armor: 0,
            },
            Weapon {
                kind: WeaponKind::Warhammer,
                cost: 25,
                damage: 6,
                armor: 0,
            },
            Weapon {
                kind: WeaponKind::Longsword,
                cost: 40,
                damage: 7,
                armor: 0,
            },
            Weapon {
                kind: WeaponKind::Greataxe,
                cost: 74,
                damage: 8,
                armor: 0,
            },
        ]
    };
    static ref ARMORS: Vec<Option<Armor>> = {
        vec![
            None,
            Some(Armor {
                kind: ArmorKind::Leather,
                cost: 13,
                damage: 0,
                armor: 1,
            }),
            Some(Armor {
                kind: ArmorKind::Chainmail,
                cost: 31,
                damage: 0,
                armor: 2,
            }),
            Some(Armor {
                kind: ArmorKind::Splintmail,
                cost: 53,
                damage: 0,
                armor: 3,
            }),
            Some(Armor {
                kind: ArmorKind::Bandedmail,
                cost: 75,
                damage: 0,
                armor: 4,
            }),
            Some(Armor {
                kind: ArmorKind::Platemail,
                cost: 102,
                damage: 0,
                armor: 5,
            }),
        ]
    };
    static ref RINGS: Vec<Option<Ring>> = {
        vec![
            None,
            Some(Ring {
                kind: RingKind::Damage1,
                cost: 25,
                damage: 1,
                armor: 0,
            }),
            Some(Ring {
                kind: RingKind::Damage2,
                cost: 50,
                damage: 2,
                armor: 0,
            }),
            Some(Ring {
                kind: RingKind::Damage3,
                cost: 100,
                damage: 3,
                armor: 0,
            }),
            Some(Ring {
                kind: RingKind::Defense1,
                cost: 20,
                damage: 0,
                armor: 1,
            }),
            Some(Ring {
                kind: RingKind::Defense2,
                cost: 40,
                damage: 0,
                armor: 2,
            }),
            Some(Ring {
                kind: RingKind::Defense3,
                cost: 80,
                damage: 0,
                armor: 3,
            }),
        ]
    };
}

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](AoCYear) and
///   [`AoCDay`](AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD21, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader).unwrap_or_default()
}

fn find_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let hp_re = Regex::new(r"^Hit Points: (\d+)$")?;
    let d_re = Regex::new(r"^Damage: (\d+)$")?;
    let a_re = Regex::new(r"^Armor: (\d+)$")?;
    let mut boss = Boss::default();

    for line in valid_lines(reader) {
        if hp_re.is_match(&line) {
            let caps = hp_re.captures(&line).context("bad captures")?;
            boss.hit_points = get_cap_x::<usize>(1, &caps)?;
        } else if d_re.is_match(&line) {
            let caps = d_re.captures(&line).context("bad captures")?;
            boss.damage = get_cap_x::<usize>(1, &caps)?;
        } else if a_re.is_match(&line) {
            let caps = a_re.captures(&line).context("bad captures")?;
            boss.total_armor = get_cap_x::<usize>(1, &caps)?;
        }
    }

    let mut players = vec![];
    for weapon in &(*WEAPONS) {
        for armor in &(*ARMORS) {
            let damage = weapon.damage + armor.map_or_else(|| 0, |a| a.damage);
            let total_armor = weapon.armor + armor.map_or_else(|| 0, |a| a.armor);
            let total_cost = weapon.cost + armor.map_or_else(|| 0, |a| a.cost);
            players.push(Player {
                weapon: *weapon,
                armor: *armor,
                ring_1: None,
                ring_2: None,
                hit_points: 100,
                damage,
                total_armor,
                total_cost,
            });
            for rings in (*RINGS).iter().permutations(2) {
                let damage = weapon.damage
                    + armor.map_or_else(|| 0, |a| a.damage)
                    + rings[0].map_or_else(|| 0, |r| r.damage)
                    + rings[1].map_or_else(|| 0, |r| r.damage);
                let total_armor = weapon.armor
                    + armor.map_or_else(|| 0, |a| a.armor)
                    + rings[0].map_or_else(|| 0, |r| r.armor)
                    + rings[1].map_or_else(|| 0, |r| r.armor);
                let total_cost = weapon.cost
                    + armor.map_or_else(|| 0, |a| a.cost)
                    + rings[0].map_or_else(|| 0, |r| r.cost)
                    + rings[1].map_or_else(|| 0, |r| r.cost);
                players.push(Player {
                    weapon: *weapon,
                    armor: *armor,
                    ring_1: *rings[0],
                    ring_2: *rings[1],
                    hit_points: 100,
                    damage,
                    total_armor,
                    total_cost,
                });
            }
        }
    }

    let mut wins = vec![];
    for player in &players {
        if fight(player, &boss) {
            wins.push(*player);
        }
    }

    let min = wins.iter().min_by(|x, y| x.total_cost.cmp(&y.total_cost));
    Ok(min.map_or_else(|| 0, |p| p.total_cost))
}

fn fight(player: &Player, boss: &Boss) -> bool {
    let player_damage = player.damage;
    let player_armor = player.total_armor;
    let mut player_hp = player.hit_points;
    let boss_damage = boss.damage;
    let boss_armor = boss.total_armor;
    let mut boss_hp = boss.hit_points;

    loop {
        let final_boss_hp_opt = if boss_armor >= player_damage {
            boss_hp.checked_sub(1)
        } else {
            boss_hp.checked_sub(player_damage - boss_armor)
        };

        if let Some(boss_hp_now) = final_boss_hp_opt {
            if boss_hp_now == 0 {
                return true;
            }
            boss_hp = boss_hp_now;
        } else {
            return true;
        }

        let final_player_hp_opt = if player_armor >= boss_damage {
            player_hp.checked_sub(1)
        } else {
            player_hp.checked_sub(boss_damage - player_armor)
        };

        if let Some(player_hp_now) = final_player_hp_opt {
            if player_hp_now == 0 {
                return false;
            }
            player_hp = player_hp_now;
        } else {
            return false;
        }
    }
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](AoCYear) and
///   [`AoCDay`](AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD21, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> usize {
    find2_br(reader).unwrap_or_default()
}

fn find2_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let hp_re = Regex::new(r"^Hit Points: (\d+)$")?;
    let d_re = Regex::new(r"^Damage: (\d+)$")?;
    let a_re = Regex::new(r"^Armor: (\d+)$")?;
    let mut boss = Boss::default();

    for line in valid_lines(reader) {
        if hp_re.is_match(&line) {
            let caps = hp_re.captures(&line).context("bad captures")?;
            boss.hit_points = get_cap_x::<usize>(1, &caps)?;
        } else if d_re.is_match(&line) {
            let caps = d_re.captures(&line).context("bad captures")?;
            boss.damage = get_cap_x::<usize>(1, &caps)?;
        } else if a_re.is_match(&line) {
            let caps = a_re.captures(&line).context("bad captures")?;
            boss.total_armor = get_cap_x::<usize>(1, &caps)?;
        }
    }

    let mut players = vec![];
    for weapon in &(*WEAPONS) {
        for armor in &(*ARMORS) {
            let damage = weapon.damage + armor.map_or_else(|| 0, |a| a.damage);
            let total_armor = weapon.armor + armor.map_or_else(|| 0, |a| a.armor);
            let total_cost = weapon.cost + armor.map_or_else(|| 0, |a| a.cost);
            players.push(Player {
                weapon: *weapon,
                armor: *armor,
                ring_1: None,
                ring_2: None,
                hit_points: 100,
                damage,
                total_armor,
                total_cost,
            });
            for rings in (*RINGS).iter().permutations(2) {
                let damage = weapon.damage
                    + armor.map_or_else(|| 0, |a| a.damage)
                    + rings[0].map_or_else(|| 0, |r| r.damage)
                    + rings[1].map_or_else(|| 0, |r| r.damage);
                let total_armor = weapon.armor
                    + armor.map_or_else(|| 0, |a| a.armor)
                    + rings[0].map_or_else(|| 0, |r| r.armor)
                    + rings[1].map_or_else(|| 0, |r| r.armor);
                let total_cost = weapon.cost
                    + armor.map_or_else(|| 0, |a| a.cost)
                    + rings[0].map_or_else(|| 0, |r| r.cost)
                    + rings[1].map_or_else(|| 0, |r| r.cost);
                players.push(Player {
                    weapon: *weapon,
                    armor: *armor,
                    ring_1: *rings[0],
                    ring_2: *rings[1],
                    hit_points: 100,
                    damage,
                    total_armor,
                    total_cost,
                });
            }
        }
    }

    let mut losses = vec![];
    for player in &players {
        if !fight(player, &boss) {
            losses.push(*player);
        }
    }

    let max = losses.iter().max_by(|x, y| x.total_cost.cmp(&y.total_cost));
    Ok(max.map_or_else(|| 0, |p| p.total_cost))
}

#[cfg(test)]
mod one_star {
    // use super::find_br;
    // use std::io::Cursor;

    //     const TEST_1: &str = r"Hit Points: 12
    // Damage: 7
    // Armor: 2";

    #[test]
    fn solution() {
        // assert_eq!(find_br(Cursor::new(TEST_1))?, 1_000);
    }
}

#[cfg(test)]
mod two_star {
    // use super::find2_br;
    // use std::io::Cursor;

    // const TEST_1: &str = r"turn on 0,0 through 0,0";
    // const TEST_2: &str = r"toggle 0,0 through 999,999";

    #[test]
    fn solution() {
        // assert_eq!(find2_br(Cursor::new(TEST_1))?, 1);
        // assert_eq!(find2_br(Cursor::new(TEST_2))?, 2_000_000);
    }
}
