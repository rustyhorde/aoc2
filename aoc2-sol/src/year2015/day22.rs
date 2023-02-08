// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 22 " Wizard Simulator 20XX"
//!
//! **--- Day 22: Wizard Simulator 20XX ---**
//!
//! **--- Part 1 ---**
//!
//! Little Henry Case decides that defeating bosses with swords and stuff is boring.
//! Now he's playing the game with a wizard. Of course, he gets stuck on another boss
//! and needs your help again.
//!
//! In this version, combat still proceeds with the player and the boss taking
//! alternating turns. The player still goes first. Now, however, you don't get any equipment;
//! instead, you must choose one of your spells to cast. The first character at or below 0 hit points loses.
//!
//! Since you're a wizard, you don't get to wear armor, and you can't attack normally.
//! However, since you do magic damage, your opponent's armor is ignored, and so the boss
//! effectively has zero armor as well. As before, if armor (from a spell, in this case)
//! would reduce damage below 1, it becomes 1 instead - that is, the boss' attacks always deal at least 1 damage.
//!
//! On each of your turns, you must select one of your spells to cast. If you cannot afford
//! to cast any spell, you lose. Spells cost mana; you start with 500 mana, but have no maximum limit.
//! You must have enough mana to cast a spell, and its cost is immediately deducted when you cast it.
//! Your spells are Magic Missile, Drain, Shield, Poison, and Recharge.
//!
//! ```text
//! Magic Missile costs 53 mana. It instantly does 4 damage.
//! Drain costs 73 mana. It instantly does 2 damage and heals you for 2 hit points.
//! Shield costs 113 mana. It starts an effect that lasts for 6 turns. While it is active, your armor is increased by 7.
//! Poison costs 173 mana. It starts an effect that lasts for 6 turns. At the start of each turn while it is active, it deals the boss 3 damage.
//! Recharge costs 229 mana. It starts an effect that lasts for 5 turns. At the start of each turn while it is active, it gives you 101 new mana.
//! ```
//!
//! Effects all work the same way. Effects apply at the start of both the player's turns and
//! the boss' turns. Effects are created with a timer (the number of turns they last); at the start of
//! each turn, after they apply any effect they have, their timer is decreased by one.
//! If this decreases the timer to zero, the effect ends. You cannot cast a spell that would start an
//! effect which is already active. However, effects can be started on the same turn they end.
//!
//! For example, suppose the player has 10 hit points and 250 mana, and that the boss has 13 hit points and 8 damage:
//!
//! ```text
//! -- Player turn --
//! - Player has 10 hit points, 0 armor, 250 mana
//! - Boss has 13 hit points
//! Player casts Poison.
//!
//! -- Boss turn --
//! - Player has 10 hit points, 0 armor, 77 mana
//! - Boss has 13 hit points
//! Poison deals 3 damage; its timer is now 5.
//! Boss attacks for 8 damage.
//!
//! -- Player turn --
//! - Player has 2 hit points, 0 armor, 77 mana
//! - Boss has 10 hit points
//! Poison deals 3 damage; its timer is now 4.
//! Player casts Magic Missile, dealing 4 damage.
//!
//! -- Boss turn --
//! - Player has 2 hit points, 0 armor, 24 mana
//! - Boss has 3 hit points
//! Poison deals 3 damage. This kills the boss, and the player wins.
//! ```
//!
//! Now, suppose the same initial conditions, except that the boss has 14 hit points instead:
//!
//! ```text
//! -- Player turn --
//! - Player has 10 hit points, 0 armor, 250 mana
//! - Boss has 14 hit points
//! Player casts Recharge.
//!
//! -- Boss turn --
//! - Player has 10 hit points, 0 armor, 21 mana
//! - Boss has 14 hit points
//! Recharge provides 101 mana; its timer is now 4.
//! Boss attacks for 8 damage!
//!
//! -- Player turn --
//! - Player has 2 hit points, 0 armor, 122 mana
//! - Boss has 14 hit points
//! Recharge provides 101 mana; its timer is now 3.
//! Player casts Shield, increasing armor by 7.
//!
//! -- Boss turn --
//! - Player has 2 hit points, 7 armor, 110 mana
//! - Boss has 14 hit points
//! Shield's timer is now 5.
//! Recharge provides 101 mana; its timer is now 2.
//! Boss attacks for 8 - 7 = 1 damage!
//!
//! -- Player turn --
//! - Player has 1 hit point, 7 armor, 211 mana
//! - Boss has 14 hit points
//! Shield's timer is now 4.
//! Recharge provides 101 mana; its timer is now 1.
//! Player casts Drain, dealing 2 damage, and healing 2 hit points.
//!
//! -- Boss turn --
//! - Player has 3 hit points, 7 armor, 239 mana
//! - Boss has 12 hit points
//! Shield's timer is now 3.
//! Recharge provides 101 mana; its timer is now 0.
//! Recharge wears off.
//! Boss attacks for 8 - 7 = 1 damage!
//!
//! -- Player turn --
//! - Player has 2 hit points, 7 armor, 340 mana
//! - Boss has 12 hit points
//! Shield's timer is now 2.
//! Player casts Poison.
//!
//! -- Boss turn --
//! - Player has 2 hit points, 7 armor, 167 mana
//! - Boss has 12 hit points
//! Shield's timer is now 1.
//! Poison deals 3 damage; its timer is now 5.
//! Boss attacks for 8 - 7 = 1 damage!
//!
//! -- Player turn --
//! - Player has 1 hit point, 7 armor, 167 mana
//! - Boss has 9 hit points
//! Shield's timer is now 0.
//! Shield wears off, decreasing armor by 7.
//! Poison deals 3 damage; its timer is now 4.
//! Player casts Magic Missile, dealing 4 damage.
//!
//! -- Boss turn --
//! - Player has 1 hit point, 0 armor, 114 mana
//! - Boss has 2 hit points
//! Poison deals 3 damage. This kills the boss, and the player wins.
//! ```
//!
//! You start with 50 hit points and 500 mana points. The boss's actual stats
//! are in your puzzle input. What is the least amount of mana you can spend
//! and still win the fight? (Do not include mana recharge effects as "spending" negative mana.)
//!
//! **--- Part Two ---**
//!
//! On the next run through the game, you increase the difficulty to `hard`.
//!
//! At the start of each player turn (before any other effects apply), you lose `1` hit point.
//! If this brings you to or below 0 hit points, you lose.
//!
//! With the same starting stats for you and the boss, what is the least amount of mana
//! you can spend and still win the fight?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{get_cap_x, run_solution, valid_lines},
};
use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashMap,
    fmt,
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct Boss {
    hit_points: usize,
    damage: usize,
}

impl fmt::Display for Boss {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "hp: {}, d: {}", self.hit_points, self.damage)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Player {
    mana: usize,
    hit_points: usize,
    armor: usize,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "m: {}, hp: {}, a: {}",
            self.mana, self.hit_points, self.armor,
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum SpellKind {
    MagicMissle,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl fmt::Display for SpellKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::MagicMissle => "Magic Missle",
                Self::Drain => "Drain",
                Self::Shield => "Shield",
                Self::Poison => "Poison",
                Self::Recharge => "Recharge",
            }
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Spell {
    kind: SpellKind,
    mana_cost: usize,
    damage: usize,
    healing: usize,
    effect_dur: usize,
}

lazy_static! {
    static ref SPELLS: Vec<Spell> = vec![
        Spell {
            kind: SpellKind::MagicMissle,
            mana_cost: 53,
            damage: 4,
            healing: 0,
            effect_dur: 0,
        },
        Spell {
            kind: SpellKind::Drain,
            mana_cost: 73,
            damage: 2,
            healing: 2,
            effect_dur: 0,
        },
        Spell {
            kind: SpellKind::Shield,
            mana_cost: 113,
            damage: 0,
            healing: 0,
            effect_dur: 6,
        },
        Spell {
            kind: SpellKind::Poison,
            mana_cost: 173,
            damage: 0,
            healing: 0,
            effect_dur: 6,
        },
        Spell {
            kind: SpellKind::Recharge,
            mana_cost: 229,
            damage: 0,
            healing: 0,
            effect_dur: 5,
        },
    ];
}

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD22, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> usize {
    find_br(reader).unwrap_or_default()
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct GameState {
    player: Player,
    boss: Boss,
    effects: HashMap<SpellKind, usize>,
    mana_spent: usize,
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use std::fmt::Write;

        let effects = self.effects.iter().fold(String::new(), |acc, (kind, dur)| {
            let mut acc = acc;
            write!(acc, "{kind} => {dur},").expect("Unable to write string");
            acc
        });
        write!(
            f,
            "player: {}, boss: {}, ms: {}, e: {effects}",
            self.player, self.boss, self.mana_spent,
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EndTurn {
    PlayerKilled,
    BossKilled,
    Continue,
}

fn find_br<T>(reader: T) -> Result<usize>
where
    T: BufRead,
{
    let hp_re = Regex::new(r"^Hit Points: (\d+)$")?;
    let d_re = Regex::new(r"^Damage: (\d+)$")?;
    let mut boss = Boss::default();

    for line in valid_lines(reader) {
        if hp_re.is_match(&line) {
            let caps = hp_re.captures(&line).context("bad captures")?;
            boss.hit_points = get_cap_x::<usize>(1, &caps)?;
        } else if d_re.is_match(&line) {
            let caps = d_re.captures(&line).context("bad captures")?;
            boss.damage = get_cap_x::<usize>(1, &caps)?;
        }
    }

    let player = Player {
        mana: 500,
        hit_points: 50,
        armor: 0,
    };
    let mut initial_state = GameState {
        player,
        boss,
        effects: HashMap::new(),
        mana_spent: 0,
    };

    let mut ms = usize::MAX;
    play(&mut initial_state, &mut ms, false);
    Ok(ms)
}

fn play(state: &mut GameState, ms: &mut usize, hard_mode: bool) {
    for spell in &*SPELLS {
        let mut my_gs = state.clone();
        if spell.mana_cost > my_gs.player.mana {
            continue;
        }

        if *ms < (my_gs.mana_spent + spell.mana_cost) {
            continue;
        }

        match player_turn(&mut my_gs, spell, hard_mode) {
            EndTurn::BossKilled => {
                if my_gs.mana_spent < *ms {
                    *ms = my_gs.mana_spent;
                }
                continue;
            }
            EndTurn::Continue => {}
            EndTurn::PlayerKilled => {
                continue;
            }
        }

        match boss_turn(&mut my_gs) {
            EndTurn::PlayerKilled => {
                continue;
            }
            EndTurn::BossKilled => {
                if my_gs.mana_spent < *ms {
                    *ms = my_gs.mana_spent;
                }
                continue;
            }
            EndTurn::Continue => play(&mut my_gs, ms, hard_mode),
        }
    }
}

fn player_turn(state: &mut GameState, spell_to_cast: &Spell, hard_mode: bool) -> EndTurn {
    state.mana_spent += spell_to_cast.mana_cost;
    state.player.mana -= spell_to_cast.mana_cost;

    if hard_mode {
        if let Some(hp) = state.player.hit_points.checked_sub(1) {
            state.player.hit_points = hp;

            if hp == 0 {
                return EndTurn::PlayerKilled;
            }
        } else {
            return EndTurn::PlayerKilled;
        }
    }

    if apply_effects(state) {
        EndTurn::BossKilled
    } else {
        match spell_to_cast.kind {
            SpellKind::Drain => {
                state.player.hit_points += spell_to_cast.healing;
                check_boss_hp(state, spell_to_cast.damage)
            }
            SpellKind::MagicMissle => check_boss_hp(state, spell_to_cast.damage),
            SpellKind::Poison => {
                let _ = state
                    .effects
                    .insert(SpellKind::Poison, spell_to_cast.effect_dur);
                EndTurn::Continue
            }
            SpellKind::Recharge => {
                let _ = state
                    .effects
                    .insert(SpellKind::Recharge, spell_to_cast.effect_dur);
                EndTurn::Continue
            }
            SpellKind::Shield => {
                let _ = state
                    .effects
                    .insert(SpellKind::Shield, spell_to_cast.effect_dur);
                EndTurn::Continue
            }
        }
    }
}

fn boss_turn(state: &mut GameState) -> EndTurn {
    if apply_effects(state) {
        EndTurn::BossKilled
    } else {
        let damage = state
            .boss
            .damage
            .checked_sub(state.player.armor)
            .map_or(1, |damage| damage);
        state
            .player
            .hit_points
            .checked_sub(damage)
            .map_or(EndTurn::PlayerKilled, |hp| {
                state.player.hit_points = hp;
                if hp == 0 {
                    EndTurn::PlayerKilled
                } else {
                    EndTurn::Continue
                }
            })
    }
}

fn apply_effects(state: &mut GameState) -> bool {
    // Check effects, update stats
    let mut remove_recharge = false;
    let mut remove_poison = false;
    let mut remove_shield = false;

    for (effect, dur) in &mut state.effects {
        match effect {
            SpellKind::Poison => {
                if let Some(hp) = state.boss.hit_points.checked_sub(3) {
                    state.boss.hit_points = hp;
                    if hp == 0 {
                        return true;
                    }
                } else {
                    return true;
                }
                *dur -= 1;
                if *dur == 0 {
                    remove_poison = true;
                }
            }
            SpellKind::Recharge => {
                state.player.mana += 101;
                *dur -= 1;

                if *dur == 0 {
                    remove_recharge = true;
                }
            }
            SpellKind::Shield => {
                state.player.armor = 7;
                *dur -= 1;
                if *dur == 0 {
                    remove_shield = true;
                    state.player.armor = 0;
                }
            }
            _ => {}
        }
    }

    if remove_poison {
        let _ = state.effects.remove(&SpellKind::Poison);
    }
    if remove_recharge {
        let _ = state.effects.remove(&SpellKind::Recharge);
    }
    if remove_shield {
        let _ = state.effects.remove(&SpellKind::Shield);
    }

    false
}

fn check_boss_hp(state: &mut GameState, damage: usize) -> EndTurn {
    state
        .boss
        .hit_points
        .checked_sub(damage)
        .map_or(EndTurn::BossKilled, |hp| {
            state.boss.hit_points = hp;
            if hp == 0 {
                EndTurn::BossKilled
            } else {
                EndTurn::Continue
            }
        })
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
/// [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<usize>(AoCYear::AOC2015, AoCDay::AOCD22, find2).map(|_| 0)
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
    let mut boss = Boss::default();

    for line in valid_lines(reader) {
        if hp_re.is_match(&line) {
            let caps = hp_re.captures(&line).context("bad captures")?;
            boss.hit_points = get_cap_x::<usize>(1, &caps)?;
        } else if d_re.is_match(&line) {
            let caps = d_re.captures(&line).context("bad captures")?;
            boss.damage = get_cap_x::<usize>(1, &caps)?;
        }
    }

    let player = Player {
        mana: 500,
        hit_points: 50,
        armor: 0,
    };
    let mut initial_state = GameState {
        player,
        boss,
        effects: HashMap::new(),
        mana_spent: 0,
    };

    let mut ms = usize::MAX;
    play(&mut initial_state, &mut ms, true);
    Ok(ms)
}

#[cfg(test)]
mod one_star {
    use super::{boss_turn, player_turn, Boss, EndTurn, GameState, Player, SpellKind, SPELLS};
    use std::collections::HashMap;
    // use std::io::Cursor;

    #[test]
    fn solution() {
        let mut state = GameState {
            player: Player {
                hit_points: 10,
                armor: 0,
                mana: 250,
            },
            boss: Boss {
                hit_points: 14,
                damage: 8,
            },
            effects: HashMap::new(),
            mana_spent: 0,
        };
        println!();
        assert_eq!(10, state.player.hit_points);
        assert_eq!(0, state.player.armor);
        assert_eq!(250, state.player.mana);
        assert_eq!(14, state.boss.hit_points);
        let result = player_turn(&mut state, &(*SPELLS)[4], false);
        assert_eq!(result, EndTurn::Continue);
        assert_eq!(10, state.player.hit_points);
        assert_eq!(0, state.player.armor);
        assert_eq!(21, state.player.mana);
        assert_eq!(14, state.boss.hit_points);
        assert!(state.effects.contains_key(&SpellKind::Recharge));
        println!("Player Turn: {state}");
        let result = boss_turn(&mut state);
        assert_eq!(result, EndTurn::Continue);
        assert_eq!(2, state.player.hit_points);
        assert_eq!(0, state.player.armor);
        assert_eq!(122, state.player.mana);
        assert_eq!(14, state.boss.hit_points);
        println!("Boss Turn:   {state}");
        let result = player_turn(&mut state, &(*SPELLS)[2], false);
        assert_eq!(result, EndTurn::Continue);
        println!("Player Turn: {state}");
        let result = boss_turn(&mut state);
        assert_eq!(result, EndTurn::Continue);
        println!("Boss Turn:   {state}");
        let result = player_turn(&mut state, &(*SPELLS)[1], false);
        assert_eq!(result, EndTurn::Continue);
        println!("Player Turn: {state}");
        let result = boss_turn(&mut state);
        assert_eq!(result, EndTurn::Continue);
        println!("Boss Turn:   {state}");
        let result = player_turn(&mut state, &(*SPELLS)[3], false);
        assert_eq!(result, EndTurn::Continue);
        println!("Player Turn: {state}");
        let result = boss_turn(&mut state);
        assert_eq!(result, EndTurn::Continue);
        println!("Boss Turn:   {state}");
        let result = player_turn(&mut state, &(*SPELLS)[0], false);
        assert_eq!(result, EndTurn::Continue);
        println!("Player Turn: {state}");
        let result = boss_turn(&mut state);
        assert_eq!(result, EndTurn::BossKilled);
        println!("Boss Turn:   {state}");
        // assert_eq!(find_br(Cursor::new(TEST_1))?, 1_000_000);
        // assert_eq!(find_br(Cursor::new(TEST_2))?, 1_000);
        // assert_eq!(find_br(Cursor::new(TEST_3))?, 999_996);
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
