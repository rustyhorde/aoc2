// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2018 ---**
//!
//! **--- Day 15: Beverage Bandits ---**
//!
//! Having perfected their hot chocolate, the Elves have a new problem: the Goblins that live in these caves will do anything to steal it. Looks like they're here for a fight.
//!
//! You scan the area, generating a map of the walls (#), open cavern (.), and starting position of every Goblin (G) and Elf (E) (your puzzle input).
//!
//! Combat proceeds in rounds; in each round, each unit that is still alive takes a turn, resolving all of its actions before the next unit's turn begins. On each unit's turn, it tries to move into range of an enemy (if it isn't already) and then attack (if it is in range).
//!
//! All units are very disciplined and always follow very strict combat rules. Units never move or attack diagonally, as doing so would be dishonorable. When multiple choices are equally valid, ties are broken in reading order: top-to-bottom, then left-to-right. For instance, the order in which units take their turns within a round is the reading order of their starting positions in that round, regardless of the type of unit or whether other units have moved after the round started. For example:
//!
//! ```text
//!                  would take their
//! These units:   turns in this order:
//!   #######           #######
//!   #.G.E.#           #.1.2.#
//!   #E.G.E#           #3.4.5#
//!   #.G.E.#           #.6.7.#
//!   #######           #######
//! ```
//!
//! Each unit begins its turn by identifying all possible targets (enemy units). If no targets remain, combat ends.
//!
//! Then, the unit identifies all of the open squares (.) that are in range of each target; these are the squares which are adjacent (immediately up, down, left, or right) to any target and which aren't already occupied by a wall or another unit. Alternatively, the unit might already be in range of a target. If the unit is not already in range of a target, and there are no open squares which are in range of a target, the unit ends its turn.
//!
//! If the unit is already in range of a target, it does not move, but continues its turn with an attack. Otherwise, since it is not in range of a target, it moves.
//!
//! To move, the unit first considers the squares that are in range and determines which of those squares it could reach in the fewest steps. A step is a single movement to any adjacent (immediately up, down, left, or right) open (.) square. Units cannot move into walls or other units. The unit does this while considering the current positions of units and does not do any prediction about where units will be later. If the unit cannot reach (find an open path to) any of the squares that are in range, it ends its turn. If multiple squares are in range and tied for being reachable in the fewest steps, the square which is first in reading order is chosen. For example:
//!
//! ```text
//! Targets:      In range:     Reachable:    Nearest:      Chosen:
//! #######       #######       #######       #######       #######
//! #E..G.#       #E.?G?#       #E.@G.#       #E.!G.#       #E.+G.#
//! #...#.#  -->  #.?.#?#  -->  #.@.#.#  -->  #.!.#.#  -->  #...#.#
//! #.G.#G#       #?G?#G#       #@G@#G#       #!G.#G#       #.G.#G#
//! #######       #######       #######       #######       #######
//! ```
//!
//! In the above scenario, the Elf has three targets (the three Goblins):
//!
//! ```text
//!     Each of the Goblins has open, adjacent squares which are in range (marked with a ? on the map).
//!     Of those squares, four are reachable (marked @); the other two (on the right) would require moving through a wall or unit to reach.
//!     Three of these reachable squares are nearest, requiring the fewest steps (only 2) to reach (marked !).
//!     Of those, the square which is first in reading order is chosen (+).
//! ```
//!
//! The unit then takes a single step toward the chosen square along the shortest path to that square. If multiple steps would put the unit equally closer to its destination, the unit chooses the step which is first in reading order. (This requires knowing when there is more than one shortest path so that you can consider the first step of each such path.) For example:
//!
//! ```text
//! In range:     Nearest:      Chosen:       Distance:     Step:
//! #######       #######       #######       #######       #######
//! #.E...#       #.E...#       #.E...#       #4E212#       #..E..#
//! #...?.#  -->  #...!.#  -->  #...+.#  -->  #32101#  -->  #.....#
//! #..?G?#       #..!G.#       #...G.#       #432G2#       #...G.#
//! #######       #######       #######       #######       #######
//! ```
//!
//! The Elf sees three squares in range of a target (?), two of which are nearest (!), and so the first in reading order is chosen (+). Under "Distance", each open square is marked with its distance from the destination square; the two squares to which the Elf could move on this turn (down and to the right) are both equally good moves and would leave the Elf 2 steps from being in range of the Goblin. Because the step which is first in reading order is chosen, the Elf moves right one square.
//!
//! Here's a larger example of movement:
//!
//! ```text
//! Initially:
//! #########
//! #G..G..G#
//! #.......#
//! #.......#
//! #G..E..G#
//! #.......#
//! #.......#
//! #G..G..G#
//! #########
//!
//! After 1 round:
//! #########
//! #.G...G.#
//! #...G...#
//! #...E..G#
//! #.G.....#
//! #.......#
//! #G..G..G#
//! #.......#
//! #########
//!
//! After 2 rounds:
//! #########
//! #..G.G..#
//! #...G...#
//! #.G.E.G.#
//! #.......#
//! #G..G..G#
//! #.......#
//! #.......#
//! #########
//!
//! After 3 rounds:
//! #########
//! #.......#
//! #..GGG..#
//! #..GEG..#
//! #G..G...#
//! #......G#
//! #.......#
//! #.......#
//! #########
//! ```
//!
//! Once the Goblins and Elf reach the positions above, they all are either in range of a target or cannot find any square in range of a target, and so none of the units can move until a unit dies.
//!
//! After moving (or if the unit began its turn in range of a target), the unit attacks.
//!
//! To attack, the unit first determines all of the targets that are in range of it by being immediately adjacent to it. If there are no such targets, the unit ends its turn. Otherwise, the adjacent target with the fewest hit points is selected; in a tie, the adjacent target with the fewest hit points which is first in reading order is selected.
//!
//! The unit deals damage equal to its attack power to the selected target, reducing its hit points by that amount. If this reduces its hit points to 0 or fewer, the selected target dies: its square becomes . and it takes no further turns.
//!
//! Each unit, either Goblin or Elf, has 3 attack power and starts with 200 hit points.
//!
//! For example, suppose the only Elf is about to attack:
//!
//! ```text
//!        HP:            HP:
//! G....  9       G....  9  
//! ..G..  4       ..G..  4  
//! ..EG.  2  -->  ..E..     
//! ..G..  2       ..G..  2  
//! ...G.  1       ...G.  1  
//! ```
//!
//! The "HP" column shows the hit points of the Goblin to the left in the corresponding row. The Elf is in range of three targets: the Goblin above it (with 4 hit points), the Goblin to its right (with 2 hit points), and the Goblin below it (also with 2 hit points). Because three targets are in range, the ones with the lowest hit points are selected: the two Goblins with 2 hit points each (one to the right of the Elf and one below the Elf). Of those, the Goblin first in reading order (the one to the right of the Elf) is selected. The selected Goblin's hit points (2) are reduced by the Elf's attack power (3), reducing its hit points to -1, killing it.
//!
//! After attacking, the unit's turn ends. Regardless of how the unit's turn ends, the next unit in the round takes its turn. If all units have taken turns in this round, the round ends, and a new round begins.
//!
//! The Elves look quite outnumbered. You need to determine the outcome of the battle: the number of full rounds that were completed (not counting the round in which combat ends) multiplied by the sum of the hit points of all remaining units at the moment combat ends. (Combat only ends when a unit finds no targets during its turn.)
//!
//! Below is an entire sample combat. Next to each map, each row's units' hit points are listed from left to right.
//!
//! ```text
//! Initially:
//! #######   
//! #.G...#   G(200)
//! #...EG#   E(200), G(200)
//! #.#.#G#   G(200)
//! #..G#E#   G(200), E(200)
//! #.....#   
//! #######   
//!
//! After 1 round:
//! #######   
//! #..G..#   G(200)
//! #...EG#   E(197), G(197)
//! #.#G#G#   G(200), G(197)
//! #...#E#   E(197)
//! #.....#   
//! #######   
//!
//! After 2 rounds:
//! #######   
//! #...G.#   G(200)
//! #..GEG#   G(200), E(188), G(194)
//! #.#.#G#   G(194)
//! #...#E#   E(194)
//! #.....#   
//! #######   
//! ```
//!
//! Combat ensues; eventually, the top Elf dies:
//!
//! ```text
//! After 23 rounds:
//! #######   
//! #...G.#   G(200)
//! #..G.G#   G(200), G(131)
//! #.#.#G#   G(131)
//! #...#E#   E(131)
//! #.....#   
//! #######   
//!
//! After 24 rounds:
//! #######   
//! #..G..#   G(200)
//! #...G.#   G(131)
//! #.#G#G#   G(200), G(128)
//! #...#E#   E(128)
//! #.....#   
//! #######   
//!
//! After 25 rounds:
//! #######   
//! #.G...#   G(200)
//! #..G..#   G(131)
//! #.#.#G#   G(125)
//! #..G#E#   G(200), E(125)
//! #.....#   
//! #######   
//!
//! After 26 rounds:
//! #######   
//! #G....#   G(200)
//! #.G...#   G(131)
//! #.#.#G#   G(122)
//! #...#E#   E(122)
//! #..G..#   G(200)
//! #######   
//!
//! After 27 rounds:
//! #######   
//! #G....#   G(200)
//! #.G...#   G(131)
//! #.#.#G#   G(119)
//! #...#E#   E(119)
//! #...G.#   G(200)
//! #######   
//!
//! After 28 rounds:
//! #######   
//! #G....#   G(200)
//! #.G...#   G(131)
//! #.#.#G#   G(116)
//! #...#E#   E(113)
//! #....G#   G(200)
//! #######   
//! ```
//!
//! More combat ensues; eventually, the bottom Elf dies:
//!
//! ```text
//! After 47 rounds:
//! #######   
//! #G....#   G(200)
//! #.G...#   G(131)
//! #.#.#G#   G(59)
//! #...#.#   
//! #....G#   G(200)
//! #######   
//! ```
//!
//! Before the 48th round can finish, the top-left Goblin finds that there are no targets remaining, and so combat ends. So, the number of full rounds that were completed is 47, and the sum of the hit points of all remaining units is 200+131+59+200 = 590. From these, the outcome of the battle is 47 * 590 = 27730.
//!
//! Here are a few example summarized combats:
//!
//! ```text
//! #######       #######
//! #G..#E#       #...#E#   E(200)
//! #E#E.E#       #E#...#   E(197)
//! #G.##.#  -->  #.E##.#   E(185)
//! #...#E#       #E..#E#   E(200), E(200)
//! #...E.#       #.....#
//! #######       #######
//!
//! Combat ends after 37 full rounds
//! Elves win with 982 total hit points left
//! Outcome: 37 * 982 = 36334
//!
//! #######       #######   
//! #E..EG#       #.E.E.#   E(164), E(197)
//! #.#G.E#       #.#E..#   E(200)
//! #E.##E#  -->  #E.##.#   E(98)
//! #G..#.#       #.E.#.#   E(200)
//! #..E#.#       #...#.#   
//! #######       #######   
//!
//! Combat ends after 46 full rounds
//! Elves win with 859 total hit points left
//! Outcome: 46 * 859 = 39514
//!
//! #######       #######   
//! #E.G#.#       #G.G#.#   G(200), G(98)
//! #.#G..#       #.#G..#   G(200)
//! #G.#.G#  -->  #..#..#   
//! #G..#.#       #...#G#   G(95)
//! #...E.#       #...G.#   G(200)
//! #######       #######   
//!
//! Combat ends after 35 full rounds
//! Goblins win with 793 total hit points left
//! Outcome: 35 * 793 = 27755
//!
//! #######       #######   
//! #.E...#       #.....#   
//! #.#..G#       #.#G..#   G(200)
//! #.###.#  -->  #.###.#   
//! #E#G#G#       #.#.#.#   
//! #...#G#       #G.G#G#   G(98), G(38), G(200)
//! #######       #######   
//!
//! Combat ends after 54 full rounds
//! Goblins win with 536 total hit points left
//! Outcome: 54 * 536 = 28944
//!
//! #########       #########   
//! #G......#       #.G.....#   G(137)
//! #.E.#...#       #G.G#...#   G(200), G(200)
//! #..##..G#       #.G##...#   G(200)
//! #...##..#  -->  #...##..#   
//! #...#...#       #.G.#...#   G(200)
//! #.G...G.#       #.......#   
//! #.....G.#       #.......#   
//! #########       #########   
//!
//! Combat ends after 20 full rounds
//! Goblins win with 937 total hit points left
//! Outcome: 20 * 937 = 18740
//! ```
//!
//! What is the outcome of the combat described in your puzzle input?
//!
//! **--- Part Two ---**
//!
//! According to your calculations, the Elves are going to lose badly. Surely, you won't mess up the timeline too much if you give them just a little advanced technology, right?
//!
//! You need to make sure the Elves not only win, but also suffer no losses: even the death of a single Elf is unacceptable.
//!
//! However, you can't go too far: larger changes will be more likely to permanently alter spacetime.
//!
//! So, you need to find the outcome of the battle in which the Elves have the lowest integer attack power (at least 4) that allows them to win without a single death. The Goblins always have an attack power of 3.
//!
//! In the first summarized example above, the lowest attack power the Elves need to win without losses is 15:
//!
//! ```text
//! #######       #######
//! #.G...#       #..E..#   E(158)
//! #...EG#       #...E.#   E(14)
//! #.#.#G#  -->  #.#.#.#
//! #..G#E#       #...#.#
//! #.....#       #.....#
//! #######       #######
//!
//! Combat ends after 29 full rounds
//! Elves win with 172 total hit points left
//! Outcome: 29 * 172 = 4988
//! ```
//!
//! In the second example above, the Elves need only 4 attack power:
//!
//! ```text
//! #######       #######
//! #E..EG#       #.E.E.#   E(200), E(23)
//! #.#G.E#       #.#E..#   E(200)
//! #E.##E#  -->  #E.##E#   E(125), E(200)
//! #G..#.#       #.E.#.#   E(200)
//! #..E#.#       #...#.#
//! #######       #######
//!
//! Combat ends after 33 full rounds
//! Elves win with 948 total hit points left
//! Outcome: 33 * 948 = 31284
//! ```
//!
//! In the third example above, the Elves need 15 attack power:
//!
//! ```text
//! #######       #######
//! #E.G#.#       #.E.#.#   E(8)
//! #.#G..#       #.#E..#   E(86)
//! #G.#.G#  -->  #..#..#
//! #G..#.#       #...#.#
//! #...E.#       #.....#
//! #######       #######
//!
//! Combat ends after 37 full rounds
//! Elves win with 94 total hit points left
//! Outcome: 37 * 94 = 3478
//! ```
//!
//! In the fourth example above, the Elves need 12 attack power:
//!
//! ```text
//! #######       #######
//! #.E...#       #...E.#   E(14)
//! #.#..G#       #.#..E#   E(152)
//! #.###.#  -->  #.###.#
//! #E#G#G#       #.#.#.#
//! #...#G#       #...#.#
//! #######       #######
//!
//! Combat ends after 39 full rounds
//! Elves win with 166 total hit points left
//! Outcome: 39 * 166 = 6474
//! ```
//!
//! In the last example above, the lone Elf needs 34 attack power:
//!
//! ```text
//! #########       #########   
//! #G......#       #.......#   
//! #.E.#...#       #.E.#...#   E(38)
//! #..##..G#       #..##...#   
//! #...##..#  -->  #...##..#   
//! #...#...#       #...#...#   
//! #.G...G.#       #.......#   
//! #.....G.#       #.......#   
//! #########       #########   
//!
//! Combat ends after 30 full rounds
//! Elves win with 38 total hit points left
//! Outcome: 30 * 38 = 1140
//! ```
//!
//! After increasing the Elves' attack power until it is just barely enough for them to win without any Elves dying, what is the outcome of the combat described in your puzzle input?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use ndarray::{Array2, Axis, Zip};
use std::{
    collections::{HashMap, VecDeque},
    fmt,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum UnitKind {
    Elf,
    Goblin,
}

impl fmt::Display for UnitKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match self {
            UnitKind::Elf => 'E',
            UnitKind::Goblin => 'G',
        };
        write!(f, "{ch}")
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Unit {
    kind: UnitKind,
    attack_power: usize,
    hit_points: usize,
    has_moved: bool,
    has_attacked: bool,
}

impl Unit {
    fn new_elf(attack_power: usize) -> Self {
        Self {
            kind: UnitKind::Elf,
            attack_power,
            hit_points: 200,
            has_moved: false,
            has_attacked: false,
        }
    }

    fn new_goblin() -> Self {
        Self {
            kind: UnitKind::Goblin,
            attack_power: 3,
            hit_points: 200,
            has_moved: false,
            has_attacked: false,
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum ElementKind {
    #[default]
    Cavern,
    Wall,
    Unit,
}

impl fmt::Display for ElementKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match self {
            ElementKind::Cavern => '.',
            ElementKind::Wall => '#',
            ElementKind::Unit => 'U',
        };
        write!(f, "{ch}")
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Element {
    kind: ElementKind,
    unit: Option<Unit>,
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref u) = self.unit {
            write!(f, "{u}")
        } else {
            write!(f, "{}", self.kind)
        }
    }
}

enum Outcome {
    NoMoreEnemies,
    DeadElf,
    BattleOn,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Action {
    Attack([usize; 2]),
    Move([usize; 2]),
    No,
}

type BattleData = (usize, usize, bool, Vec<String>);

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<BattleData, usize>(AoCYear::AOC2018, AoCDay::AOCD15, setup, find)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<BattleData, usize>(bench, AoCYear::AOC2018, AoCDay::AOCD15, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> BattleData {
    setup_br(reader, 32, 32, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T, max_i: usize, max_j: usize, test: bool) -> Result<BattleData>
where
    T: BufRead,
{
    let mut data = vec![];

    for line in valid_lines(reader) {
        data.push(line);
    }
    Ok((max_i, max_j, test, data))
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: BattleData) -> usize {
    find_res(data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(data: BattleData, second_star: bool) -> Result<usize> {
    let (max_i, max_j, test, data) = data;
    let mut board = Array2::default((0, 0));
    let mut dead_elf = true;
    let mut round_count = 0;
    let mut elf_attack_power = 3;

    while dead_elf {
        round_count = 0;
        board = generate_map(&data, max_i, max_j, elf_attack_power)?;

        let mut done = false;
        while !done {
            match round(&mut board, max_i, max_j, second_star)? {
                Outcome::NoMoreEnemies => {
                    dead_elf = false;
                    done = true;
                }
                Outcome::DeadElf => {
                    if second_star {
                        done = true;
                    }
                }
                Outcome::BattleOn => {}
            }

            if !done {
                round_count += 1;
            } else if done && test {
                print_board(&board, round_count);
            }
        }
        elf_attack_power += 1;
    }

    let hps: usize = board
        .iter()
        .filter_map(|x| {
            if x.kind == ElementKind::Unit {
                x.unit.clone()
            } else {
                None
            }
        })
        .map(|u| u.hit_points)
        .sum();
    Ok(round_count * hps)
}

fn generate_map(
    data: &[String],
    max_i: usize,
    max_j: usize,
    elf_attack_power: usize,
) -> Result<Array2<Element>> {
    let mut board: Array2<Element> = Array2::default((max_i, max_j));

    for (j, line) in data.iter().enumerate() {
        for (i, ch) in line.chars().enumerate() {
            match ch {
                '#' => {
                    board[[i, j]] = Element {
                        kind: ElementKind::Wall,
                        unit: None,
                    }
                }
                '.' => {
                    board[[i, j]] = Element {
                        kind: ElementKind::Cavern,
                        unit: None,
                    }
                }
                'E' => {
                    board[[i, j]] = Element {
                        kind: ElementKind::Unit,
                        unit: Some(Unit::new_elf(elf_attack_power)),
                    }
                }
                'G' => {
                    board[[i, j]] = Element {
                        kind: ElementKind::Unit,
                        unit: Some(Unit::new_goblin()),
                    }
                }
                _ => return Err(anyhow!("invalid game element type!")),
            }
        }
    }
    Ok(board)
}

fn round(board: &mut Array2<Element>, i: usize, j: usize, second_star: bool) -> Result<Outcome> {
    let mut done = false;
    let mut dead_elf = false;

    'outer: for row in 0..j {
        for col in 0..i {
            let result = take_turn(board, col, row, i, j, second_star)?;

            if result == 0 {
                done = true;
                break 'outer;
            } else if result == 2 {
                done = true;
                dead_elf = true;
                break 'outer;
            }
        }
    }

    if !done {
        for row in 0..j {
            for col in 0..i {
                reset_units(board, col, row);
            }
        }
    }

    if !done {
        Ok(Outcome::BattleOn)
    } else if !second_star && done {
        Ok(Outcome::NoMoreEnemies)
    } else if second_star && dead_elf {
        Ok(Outcome::DeadElf)
    } else {
        Ok(Outcome::NoMoreEnemies)
    }
}

fn take_turn(
    board: &mut Array2<Element>,
    i: usize,
    j: usize,
    max_i: usize,
    max_j: usize,
    second_star: bool,
) -> Result<usize> {
    let mut move_vec = Vec::new();

    // Scope for mutable board change below.
    {
        let curr_cell = &board[[i, j]];

        match curr_cell.kind {
            ElementKind::Wall | ElementKind::Cavern => {}
            ElementKind::Unit => {
                if let Some(ref unit) = curr_cell.unit {
                    let targets = find_enemy_targets(board, unit.kind);

                    if targets.is_empty() {
                        return Ok(0);
                    }

                    if let Ok(Some(target)) =
                        move_if_not_adjacent(board, &targets, unit, i, j, max_i, max_j)
                    {
                        move_vec.push((Action::Move([i, j]), target));
                    } else {
                        move_vec.push((Action::No, [0, 0]));
                    }
                }
            }
        }
    }

    let mut next_coord = [0, 0];
    let mut moved = false;

    for (action, coord) in move_vec {
        match action {
            Action::Attack([_, _]) => return Err(anyhow!("Attack in Move Phase")),
            Action::Move([i, j]) => {
                board[coord] = board[[i, j]].clone();
                moved = true;
                next_coord = coord;

                if let Some(ref mut unit) = board[coord].unit {
                    unit.has_moved = true;
                }
                board[[i, j]] = Element {
                    kind: ElementKind::Cavern,
                    unit: None,
                };
            }
            Action::No => {}
        }
    }

    let mut attack_vec = Vec::new();

    {
        let curr_cell = if moved {
            &board[next_coord]
        } else {
            &board[[i, j]]
        };
        let i = if moved { next_coord[0] } else { i };
        let j = if moved { next_coord[1] } else { j };

        match curr_cell.kind {
            ElementKind::Wall | ElementKind::Cavern => {}
            ElementKind::Unit => {
                if let Some(ref unit) = curr_cell.unit {
                    if !unit.has_attacked {
                        if let Some(target) = attack_adjacent(board, unit, i, j, max_i, max_j) {
                            attack_vec.push((
                                Action::Attack([i, j]),
                                target,
                                Some(unit.attack_power),
                            ));
                        } else {
                            attack_vec.push((Action::No, [0, 0], None));
                        }
                    }
                }
            }
        }
    }

    for (action, coord, atk_pwr_opt) in attack_vec {
        match action {
            Action::Attack([i, j]) => {
                let mut dead = false;
                if let Some(ref mut unit) = board[[i, j]].unit {
                    unit.has_attacked = true;
                }
                if let Some(ref mut unit) = board[coord].unit {
                    if let Some(atk_pwr) = atk_pwr_opt {
                        unit.hit_points = unit.hit_points.saturating_sub(atk_pwr);

                        if unit.hit_points == 0 {
                            dead = true;

                            if second_star && unit.kind == UnitKind::Elf {
                                return Ok(2);
                            }
                        }
                    }
                }

                if dead {
                    board[coord] = Element {
                        kind: ElementKind::Cavern,
                        unit: None,
                    };
                }
            }
            Action::Move(_) | Action::No => {}
        }
    }

    Ok(1)
}

fn reset_units(board: &mut Array2<Element>, i: usize, j: usize) {
    let element = &mut board[[i, j]];

    if element.kind == ElementKind::Unit {
        if let Some(ref mut unit) = element.unit {
            unit.has_moved = false;
            unit.has_attacked = false;
        }
    }
}

fn find_enemy_targets(board: &Array2<Element>, unit_kind: UnitKind) -> Vec<[usize; 2]> {
    let units: HashMap<(usize, usize), &Element> = board
        .indexed_iter()
        .filter(|(_idx, element)| element.kind == ElementKind::Unit)
        .filter(|(_idx, element)| {
            if let Some(ref unit) = element.unit {
                unit.kind != unit_kind
            } else {
                false
            }
        })
        .collect();
    units.keys().map(|(i, j)| [*i, *j]).collect()
}

fn attack_adjacent(
    board: &Array2<Element>,
    curr_unit: &Unit,
    i: usize,
    j: usize,
    max_i: usize,
    max_j: usize,
) -> Option<[usize; 2]> {
    let mut target = None;
    let mut min_hit_points = usize::MAX;

    let above = [i, j - 1];
    let left = [i - 1, j];
    let right = [i + 1, j];
    let down = [i, j + 1];

    // Check up first (reading order and all)
    if j > 0 {
        calculate_attack(board, curr_unit, above, &mut target, &mut min_hit_points);
    }

    if i > 0 {
        calculate_attack(board, curr_unit, left, &mut target, &mut min_hit_points);
    }

    if i < max_i - 1 {
        calculate_attack(board, curr_unit, right, &mut target, &mut min_hit_points);
    }

    if j < max_j - 1 {
        calculate_attack(board, curr_unit, down, &mut target, &mut min_hit_points);
    }
    target
}

fn calculate_attack(
    board: &Array2<Element>,
    curr_unit: &Unit,
    coord: [usize; 2],
    target: &mut Option<[usize; 2]>,
    min_hit_points: &mut usize,
) {
    let element = &board[coord];

    if element.kind == ElementKind::Unit {
        if let Some(ref unit) = element.unit {
            if unit.kind != curr_unit.kind {
                let hit_points = unit.hit_points;

                if hit_points < *min_hit_points {
                    *target = Some(coord);
                    *min_hit_points = hit_points;
                }
            }
        }
    }
}

#[allow(clippy::cognitive_complexity, clippy::too_many_lines)]
fn move_if_not_adjacent(
    board: &Array2<Element>,
    targets: &[[usize; 2]],
    curr_unit: &Unit,
    i: usize,
    j: usize,
    max_i: usize,
    max_j: usize,
) -> Result<Option<[usize; 2]>> {
    // If the unit has already moved, don't move again.
    if curr_unit.has_moved {
        return Ok(None);
    }

    // Check for adjacent units, and return if there are any.
    if j > 0 {
        let above = &board[[i, j - 1]];

        if above.kind == ElementKind::Unit {
            if let Some(ref adj_unit) = above.unit {
                if adj_unit.kind != curr_unit.kind {
                    return Ok(None);
                }
            }
        }
    }

    if i > 0 {
        let left = &board[[i - 1, j]];

        if left.kind == ElementKind::Unit {
            if let Some(ref adj_unit) = left.unit {
                if adj_unit.kind != curr_unit.kind {
                    return Ok(None);
                }
            }
        }
    }

    if i < max_i - 1 {
        let right = &board[[i + 1, j]];

        if right.kind == ElementKind::Unit {
            if let Some(ref adj_unit) = right.unit {
                if adj_unit.kind != curr_unit.kind {
                    return Ok(None);
                }
            }
        }
    }

    if j < max_j - 1 {
        let down = &board[[i, j + 1]];

        if down.kind == ElementKind::Unit {
            if let Some(ref adj_unit) = down.unit {
                if adj_unit.kind != curr_unit.kind {
                    return Ok(None);
                }
            }
        }
    }

    // Find the empty spots nearest the targets.
    let actual_locs: Vec<[usize; 2]> = targets
        .iter()
        .flat_map(|x| {
            let i = x[0];
            let j = x[1];
            let mut caverns = Vec::new();
            if j > 0 {
                let above = &board[[i, j - 1]];

                if above.kind == ElementKind::Cavern {
                    caverns.push([i, j - 1]);
                }
            }

            if i > 0 {
                let left = &board[[i - 1, j]];

                if left.kind == ElementKind::Cavern {
                    caverns.push([i - 1, j]);
                }
            }

            if i < max_i - 1 {
                let right = &board[[i + 1, j]];

                if right.kind == ElementKind::Cavern {
                    caverns.push([i + 1, j]);
                }
            }

            if j < max_j - 1 {
                let down = &board[[i, j + 1]];

                if down.kind == ElementKind::Cavern {
                    caverns.push([i, j + 1]);
                }
            }

            caverns
        })
        .collect();

    let mut min_dist = usize::MAX;

    if actual_locs.is_empty() {
        return Ok(None);
    }

    let mut first_step_vec = Vec::new();

    for target in actual_locs {
        let mut visited: Array2<bool> = Array2::default((max_i, max_j));

        Zip::from(&mut visited)
            .and(board)
            .for_each(|visited, element| {
                *visited = matches!(element.kind, ElementKind::Wall | ElementKind::Unit);
            });

        visited[target] = false;

        let mut queue = VecDeque::new();
        let move_queue = VecDeque::new();
        queue.push_back(([i, j], move_queue, 0));

        while !queue.is_empty() {
            let (coord, mut path, dist) = queue.pop_front().ok_or(anyhow!(""))?;

            if coord == target {
                #[allow(clippy::comparison_chain)]
                if dist < min_dist {
                    min_dist = dist;
                    let first_step = path.pop_front().ok_or(anyhow!(""))?;
                    first_step_vec.clear();
                    first_step_vec.push(first_step);
                } else if dist == min_dist {
                    let first_step = path.pop_front().ok_or(anyhow!(""))?;
                    first_step_vec.push(first_step);
                }

                break;
            }

            // Moving up
            let up_coord = [coord[0], coord[1] - 1];
            if j > 0 && !visited[up_coord] {
                let mut new_path = path.clone();
                new_path.push_back(up_coord);
                queue.push_back((up_coord, new_path, dist + 1));
                visited[up_coord] = true;
            }

            // Moving left
            let left_coord = [coord[0] - 1, coord[1]];
            if i > 0 && !visited[left_coord] {
                let mut new_path = path.clone();
                new_path.push_back(left_coord);
                queue.push_back((left_coord, new_path, dist + 1));
                visited[left_coord] = true;
            }

            // Moving right
            let right_coord = [coord[0] + 1, coord[1]];

            if i + 1 < max_i && !visited[right_coord] {
                let mut new_path = path.clone();
                new_path.push_back(right_coord);
                queue.push_back((right_coord, new_path, dist + 1));
                visited[right_coord] = true;
            }

            // Moving down
            let down_coord = [coord[0], coord[1] + 1];

            if j + 1 < max_j && !visited[down_coord] {
                let mut new_path = path.clone();
                new_path.push_back(down_coord);
                queue.push_back((down_coord, new_path, dist + 1));
                visited[down_coord] = true;
            }
        }
    }

    first_step_vec.dedup();

    if first_step_vec.is_empty() {
        Ok(None)
    } else {
        let above = [i, j - 1];
        let left = [i - 1, j];
        let right = [i + 1, j];
        let down = [i, j + 1];

        if first_step_vec.contains(&above) {
            Ok(Some(above))
        } else if first_step_vec.contains(&left) {
            Ok(Some(left))
        } else if first_step_vec.contains(&right) {
            Ok(Some(right))
        } else {
            Ok(Some(down))
        }
    }
}

fn print_board(board: &Array2<Element>, round: usize) {
    use std::fmt::Write;

    if round == 0 {
        println!("Initially:");
    } else if round == 1 {
        println!("After 1 round:");
    } else {
        println!("After {round} rounds:");
    }
    for row in board.axis_iter(Axis(1)) {
        let mut unit_vec = Vec::new();
        for cell in row {
            if let Some(ref unit) = cell.unit {
                unit_vec.push((unit.kind, unit.hit_points));
            }
            print!("{cell}");
        }

        let mut buffer = String::new();
        if !unit_vec.is_empty() {
            buffer.push_str("  ");

            for (kind, hitpoints) in unit_vec {
                write!(buffer, "{kind}({hitpoints}), ").expect("");
            }
        }
        let x: &[_] = &[',', ' '];
        print!("{}", buffer.trim_end_matches(x));
        println!();
    }
    println!();
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<BattleData, usize>(AoCYear::AOC2018, AoCDay::AOCD15, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<BattleData, usize>(bench, AoCYear::AOC2018, AoCDay::AOCD15, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: BattleData) -> usize {
    find_res(data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";

    const TEST_2: &str = r"#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######";

    const TEST_3: &str = r"#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######";

    const TEST_4: &str = r"#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######";

    const TEST_5: &str = r"#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######";

    const TEST_6: &str = r"#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1), 7, 7, true)?;
        assert_eq!(find(data), 27730);
        let data = setup_br(Cursor::new(TEST_2), 7, 7, true)?;
        assert_eq!(find(data), 36334);
        let data = setup_br(Cursor::new(TEST_3), 7, 7, true)?;
        assert_eq!(find(data), 39514);
        let data = setup_br(Cursor::new(TEST_4), 7, 7, true)?;
        assert_eq!(find(data), 27755);
        let data = setup_br(Cursor::new(TEST_5), 7, 7, true)?;
        assert_eq!(find(data), 28944);
        let data = setup_br(Cursor::new(TEST_6), 9, 9, true)?;
        assert_eq!(find(data), 18740);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";

    const TEST_2: &str = r"#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######";

    const TEST_3: &str = r"#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######";

    const TEST_4: &str = r"#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######";

    const TEST_5: &str = r"#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######";

    const TEST_6: &str = r"#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1), 7, 7, true)?;
        assert_eq!(find2(data), 4988);
        let data = setup_br(Cursor::new(TEST_2), 7, 7, true)?;
        assert_eq!(find2(data), 29064);
        let data = setup_br(Cursor::new(TEST_3), 7, 7, true)?;
        assert_eq!(find2(data), 31284);
        let data = setup_br(Cursor::new(TEST_4), 7, 7, true)?;
        assert_eq!(find2(data), 3478);
        let data = setup_br(Cursor::new(TEST_5), 7, 7, true)?;
        assert_eq!(find2(data), 6474);
        let data = setup_br(Cursor::new(TEST_6), 9, 9, true)?;
        assert_eq!(find2(data), 1140);
        Ok(())
    }
}
