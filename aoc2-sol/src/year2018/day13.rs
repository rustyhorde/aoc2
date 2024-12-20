// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2018 --**
//!
//! **--- Day 13: Mine Cart Madness ---**
//!
//! A crop of this size requires significant logistics to transport produce, soil, fertilizer, and so on. The Elves are very busy pushing things around in carts on some kind of rudimentary system of tracks they've come up with.
//!
//! Seeing as how cart-and-track systems don't appear in recorded history for another 1000 years, the Elves seem to be making this up as they go along. They haven't even figured out how to avoid collisions yet.
//!
//! You map out the tracks (your puzzle input) and see where you can help.
//!
//! Tracks consist of straight paths (| and -), curves (/ and \), and intersections (+). Curves connect exactly two perpendicular pieces of track; for example, this is a closed loop:
//!
//! ```text
//! /----\
//! |    |
//! |    |
//! \----/
//! ```
//!
//! Intersections occur when two perpendicular paths cross. At an intersection, a cart is capable of turning left, turning right, or continuing straight. Here are two loops connected by two intersections:
//!
//! ```text
//! /-----\
//! |     |
//! |  /--+--\
//! |  |  |  |
//! \--+--/  |
//!    |     |
//!    \-----/
//! ```
//!
//! Several carts are also on the tracks. Carts always face either up (^), down (v), left (<), or right (>). (On your initial map, the track under each cart is a straight path matching the direction the cart is facing.)
//!
//! Each time a cart has the option to turn (by arriving at any intersection), it turns left the first time, goes straight the second time, turns right the third time, and then repeats those directions starting again with left the fourth time, straight the fifth time, and so on. This process is independent of the particular intersection at which the cart has arrived - that is, the cart has no per-intersection memory.
//!
//! Carts all move at the same speed; they take turns moving a single step at a time. They do this based on their current location: carts on the top row move first (acting from left to right), then carts on the second row move (again from left to right), then carts on the third row, and so on. Once each cart has moved one step, the process repeats; each of these loops is called a tick.
//!
//! For example, suppose there are two carts on a straight track:
//!
//! ```text
//! |  |  |  |  |
//! v  |  |  |  |
//! |  v  v  |  |
//! |  |  |  v  X
//! |  |  ^  ^  |
//! ^  ^  |  |  |
//! |  |  |  |  |
//! ```
//!
//! First, the top cart moves. It is facing down (v), so it moves down one square. Second, the bottom cart moves. It is facing up (^), so it moves up one square. Because all carts have moved, the first tick ends. Then, the process repeats, starting with the first cart. The first cart moves down, then the second cart moves up - right into the first cart, colliding with it! (The location of the crash is marked with an X.) This ends the second and last tick.
//!
//! Here is a longer example:
//!
//! ```text
//! /->-\        
//! |   |  /----\
//! | /-+--+-\  |
//! | | |  | v  |
//! \-+-/  \-+--/
//!   \------/   
//!
//! /-->\        
//! |   |  /----\
//! | /-+--+-\  |
//! | | |  | |  |
//! \-+-/  \->--/
//!   \------/   
//!
//! /---v        
//! |   |  /----\
//! | /-+--+-\  |
//! | | |  | |  |
//! \-+-/  \-+>-/
//!   \------/   
//!
//! /---\        
//! |   v  /----\
//! | /-+--+-\  |
//! | | |  | |  |
//! \-+-/  \-+->/
//!   \------/   
//!
//! /---\        
//! |   |  /----\
//! | /->--+-\  |
//! | | |  | |  |
//! \-+-/  \-+--^
//!   \------/   
//!
//! /---\        
//! |   |  /----\
//! | /-+>-+-\  |
//! | | |  | |  ^
//! \-+-/  \-+--/
//!   \------/   
//!
//! /---\        
//! |   |  /----\
//! | /-+->+-\  ^
//! | | |  | |  |
//! \-+-/  \-+--/
//!   \------/   
//!
//! /---\        
//! |   |  /----<
//! | /-+-->-\  |
//! | | |  | |  |
//! \-+-/  \-+--/
//!   \------/   
//!
//! /---\        
//! |   |  /---<\
//! | /-+--+>\  |
//! | | |  | |  |
//! \-+-/  \-+--/
//!   \------/   
//!
//! /---\        
//! |   |  /--<-\
//! | /-+--+-v  |
//! | | |  | |  |
//! \-+-/  \-+--/
//!   \------/   
//!
//! /---\        
//! |   |  /-<--\
//! | /-+--+-\  |
//! | | |  | v  |
//! \-+-/  \-+--/
//!   \------/   
//!
//! /---\        
//! |   |  /<---\
//! | /-+--+-\  |
//! | | |  | |  |
//! \-+-/  \-<--/
//!   \------/   
//!
//! /---\        
//! |   |  v----\
//! | /-+--+-\  |
//! | | |  | |  |
//! \-+-/  \<+--/
//!   \------/   
//!
//! /---\        
//! |   |  /----\
//! | /-+--v-\  |
//! | | |  | |  |
//! \-+-/  ^-+--/
//!   \------/   
//!
//! /---\        
//! |   |  /----\
//! | /-+--+-\  |
//! | | |  X |  |
//! \-+-/  \-+--/
//!   \------/   
//! ```
//!
//! After following their respective paths for a while, the carts eventually crash. To help prevent crashes, you'd like to know the location of the first crash. Locations are given in X,Y coordinates, where the furthest left column is X=0 and the furthest top row is Y=0:
//!
//! ```text
//!            111
//!  0123456789012
//! 0/---\        
//! 1|   |  /----\
//! 2| /-+--+-\  |
//! 3| | |  X |  |
//! 4\-+-/  \-+--/
//! 5  \------/   
//! ```
//!
//! In this example, the location of the first crash is 7,3.
//!
//! **--- Part Two ---**
//!
//! There isn't much you can do to prevent crashes in this ridiculous system. However, by predicting the crashes, the Elves know where to be in advance and instantly remove the two crashing carts the moment any crash occurs.
//!
//! They can proceed like this for a while, but eventually, they're going to run out of carts. It could be useful to figure out where the last cart that hasn't crashed will end up.
//!
//! For example:
//!
//! ```text
//! />-<\  
//! |   |  
//! | /<+-\
//! | | | v
//! \>+</ |
//!   |   ^
//!   \<->/
//!
//! /---\  
//! |   |  
//! | v-+-\
//! | | | |
//! \-+-/ |
//!   |   |
//!   ^---^
//!
//! /---\  
//! |   |  
//! | /-+-\
//! | v | |
//! \-+-/ |
//!   ^   ^
//!   \---/
//!
//! /---\  
//! |   |  
//! | /-+-\
//! | | | |
//! \-+-/ ^
//!   |   |
//!   \---/
//! ```
//!
//! After four very expensive crashes, a tick ends with only one cart remaining; its final location is 6,4.
//!
//! What is the location of the last cart at the end of the first tick where it is the only cart left?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use getset::Getters;
use ndarray::{Array2, Axis};
use std::{
    cmp::Ordering,
    collections::BTreeMap,
    fmt,
    fs::File,
    io::{BufRead, BufReader},
    sync::Arc,
};

#[derive(Clone, Default)]
enum TrackKind {
    UpDown,
    LeftRight,
    CurveRight,
    CurveLeft,
    Junction,
    #[default]
    Empty,
    Collision,
}

impl fmt::Display for TrackKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match self {
            TrackKind::UpDown => '|',
            TrackKind::LeftRight => '-',
            TrackKind::CurveRight => '/',
            TrackKind::CurveLeft => '\\',
            TrackKind::Junction => '+',
            TrackKind::Empty => ' ',
            TrackKind::Collision => 'X',
        };
        write!(f, "{ch}")
    }
}

#[derive(Clone, Default, Getters)]
struct Track {
    kind: TrackKind,
    #[getset(get)]
    cart: Option<Cart>,
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref c) = self.cart {
            write!(f, "{c}")
        } else {
            write!(f, "{}", self.kind)
        }
    }
}

#[derive(Clone, Debug)]
struct Cart {
    direction: CartDirection,
    turn_state: CartTurnState,
}

impl fmt::Display for Cart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.direction)
    }
}

#[derive(Clone, Copy, Debug)]
enum CartTurnState {
    Left,
    Straight,
    Right,
}

#[derive(Clone, Copy, Debug)]
enum CartDirection {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for CartDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match self {
            CartDirection::Up => '^',
            CartDirection::Down => 'v',
            CartDirection::Left => '<',
            CartDirection::Right => '>',
        };
        write!(f, "{ch}")
    }
}

#[derive(Clone, Copy, Debug, Eq)]
struct CartPoint {
    i: usize,
    j: usize,
}

impl PartialOrd for CartPoint {
    fn partial_cmp(&self, other: &CartPoint) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CartPoint {
    fn cmp(&self, other: &CartPoint) -> Ordering {
        if self.j < other.j {
            Ordering::Less
        } else if self.j > other.j {
            Ordering::Greater
        } else if self.i < other.i {
            Ordering::Less
        } else if self.i > other.i {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialEq for CartPoint {
    fn eq(&self, other: &CartPoint) -> bool {
        self.i == other.i && self.j == other.j
    }
}

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<(bool, Arc<Array2<Track>>), String>(
        AoCYear::AOC2018,
        AoCDay::AOCD13,
        setup,
        find,
    )
    .map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<(bool, Arc<Array2<Track>>), String>(
        bench,
        AoCYear::AOC2018,
        AoCDay::AOCD13,
        setup,
        find,
    )
    .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> (bool, Arc<Array2<Track>>) {
    setup_br(reader, 150, 150, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T, x: usize, y: usize, test: bool) -> Result<(bool, Arc<Array2<Track>>)>
where
    T: BufRead,
{
    let mut mine_arr: Array2<Track> = Array2::default((x, y));
    for (j, line) in valid_lines(reader).enumerate() {
        for (i, ch) in line.chars().enumerate() {
            let (kind, cart) = match ch {
                '/' => (TrackKind::CurveRight, None),
                '\\' => (TrackKind::CurveLeft, None),
                '|' => (TrackKind::UpDown, None),
                '-' => (TrackKind::LeftRight, None),
                '+' => (TrackKind::Junction, None),
                '^' => (
                    TrackKind::UpDown,
                    Some(Cart {
                        direction: CartDirection::Up,
                        turn_state: CartTurnState::Left,
                    }),
                ),
                'v' => (
                    TrackKind::UpDown,
                    Some(Cart {
                        direction: CartDirection::Down,
                        turn_state: CartTurnState::Left,
                    }),
                ),
                '<' => (
                    TrackKind::LeftRight,
                    Some(Cart {
                        direction: CartDirection::Left,
                        turn_state: CartTurnState::Left,
                    }),
                ),
                '>' => (
                    TrackKind::LeftRight,
                    Some(Cart {
                        direction: CartDirection::Right,
                        turn_state: CartTurnState::Left,
                    }),
                ),
                _ => (TrackKind::Empty, None),
            };

            mine_arr[[i, j]] = Track { kind, cart };
        }
    }
    Ok((test, Arc::new(mine_arr)))
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: (bool, Arc<Array2<Track>>)) -> String {
    find_res(data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(data: (bool, Arc<Array2<Track>>), second_star: bool) -> Result<String> {
    let (test, mut mine_arr) = data;
    if test {
        println!();
    }
    if test {
        print_mine_arr(&mine_arr);
    }

    let my_mine_arr = Arc::make_mut(&mut mine_arr);
    if let Some((i, j)) = run_carts(my_mine_arr, second_star, test)? {
        Ok(format!("{i},{j}"))
    } else {
        Err(anyhow!("no coordinates"))
    }
}

fn run_carts(
    mine_arr: &mut Array2<Track>,
    second_star: bool,
    test: bool,
) -> Result<Option<(usize, usize)>> {
    let mut res;
    loop {
        let cart_map = find_carts(mine_arr);
        res = move_carts(&cart_map, mine_arr, second_star)?;

        if test {
            print_mine_arr(mine_arr);
        }

        if res.is_some() {
            break;
        }
    }

    Ok(res)
}

fn find_carts(mine_arr: &Array2<Track>) -> BTreeMap<CartPoint, CartDirection> {
    let mut cart_map = BTreeMap::new();
    for (idx, t) in mine_arr.indexed_iter() {
        if let Some(ref c) = t.cart {
            let _ = cart_map.insert(CartPoint { i: idx.0, j: idx.1 }, c.direction);
        }
    }
    cart_map
}

#[allow(clippy::too_many_lines)]
fn move_carts(
    cart_map: &BTreeMap<CartPoint, CartDirection>,
    mine_arr: &mut Array2<Track>,
    second_star: bool,
) -> Result<Option<(usize, usize)>> {
    for (cart_point, direction) in cart_map {
        let i = cart_point.i;
        let j = cart_point.j;

        // Save off the turn state in case of junction.
        let turn_state = if let Some(curr_cart) = mine_arr[[i, j]].cart() {
            curr_cart.turn_state
        } else {
            continue;
        };

        // Remove the cart from the old track.
        {
            let track = &mut mine_arr[[i, j]];
            track.cart = None;
        }

        // Generate the next index
        let nidx = match direction {
            CartDirection::Down => [i, j + 1],
            CartDirection::Up => [i, j - 1],
            CartDirection::Right => [i + 1, j],
            CartDirection::Left => [i - 1, j],
        };

        let mut collision = false;
        if second_star {
            // Get the track at the next index
            let track = &mine_arr[nidx];

            // If there is already a cart there, COLLISION!
            if track.cart().is_some() {
                collision = true;
            }
        } else {
            // Get the track at the next index
            let track = &mut mine_arr[nidx];

            // If there is already a cart there, COLLISION!
            // Set the track appropriately for display
            // and return the index where the collision happened.
            if track.cart().is_some() {
                track.kind = TrackKind::Collision;
                track.cart = None;
                return Ok(Some(nidx.into()));
            }
        }

        if collision {
            {
                let track = &mut mine_arr[nidx];
                if track.cart().is_some() {
                    track.cart = None;
                }
            }

            let carts = find_carts(mine_arr);
            if carts.len() == 1 {
                let (cart_point, direction) = carts.iter().next().ok_or(anyhow!(""))?;
                let i = cart_point.i;
                let j = cart_point.j;

                let fidx = match direction {
                    CartDirection::Up => [i, j - 1],
                    CartDirection::Down => [i, j + 1],
                    CartDirection::Left => [i - 1, j],
                    CartDirection::Right => [i + 1, j],
                };
                return Ok(Some(fidx.into()));
            }
        } else {
            let track = &mut mine_arr[nidx];

            // Otherwise, setup the new track position.
            match track.kind {
                TrackKind::Junction => {
                    let new_direction = match turn_state {
                        CartTurnState::Left => match direction {
                            CartDirection::Down => CartDirection::Right,
                            CartDirection::Up => CartDirection::Left,
                            CartDirection::Right => CartDirection::Up,
                            CartDirection::Left => CartDirection::Down,
                        },
                        CartTurnState::Straight => *direction,
                        CartTurnState::Right => match direction {
                            CartDirection::Down => CartDirection::Left,
                            CartDirection::Up => CartDirection::Right,
                            CartDirection::Right => CartDirection::Down,
                            CartDirection::Left => CartDirection::Up,
                        },
                    };

                    track.cart = Some(Cart {
                        direction: new_direction,
                        turn_state: next_turn_state(turn_state),
                    });
                }
                TrackKind::UpDown | TrackKind::LeftRight => {
                    track.cart = Some(Cart {
                        direction: *direction,
                        turn_state,
                    });
                }
                TrackKind::CurveLeft => {
                    let new_direction = match direction {
                        CartDirection::Down => CartDirection::Right,
                        CartDirection::Up => CartDirection::Left,
                        CartDirection::Right => CartDirection::Down,
                        CartDirection::Left => CartDirection::Up,
                    };
                    track.cart = Some(Cart {
                        direction: new_direction,
                        turn_state,
                    });
                }
                TrackKind::CurveRight => {
                    let new_direction = match direction {
                        CartDirection::Down => CartDirection::Left,
                        CartDirection::Up => CartDirection::Right,
                        CartDirection::Right => CartDirection::Up,
                        CartDirection::Left => CartDirection::Down,
                    };
                    track.cart = Some(Cart {
                        direction: new_direction,
                        turn_state,
                    });
                }
                TrackKind::Collision => return Err(anyhow!("Can't move into a collision area!")),
                TrackKind::Empty => return Err(anyhow!("Can't move into an empty area")),
            }
        }
    }
    Ok(None)
}

/// Cycle the turn state machine
fn next_turn_state(turn_state: CartTurnState) -> CartTurnState {
    match turn_state {
        CartTurnState::Left => CartTurnState::Straight,
        CartTurnState::Straight => CartTurnState::Right,
        CartTurnState::Right => CartTurnState::Left,
    }
}

fn print_mine_arr(mine_arr: &Array2<Track>) {
    for row in mine_arr.axis_iter(Axis(1)) {
        for cell in row {
            print!("{cell}");
        }
        println!();
    }
    println!();
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<(bool, Arc<Array2<Track>>), String>(
        AoCYear::AOC2018,
        AoCDay::AOCD13,
        setup,
        find2,
    )
    .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<(bool, Arc<Array2<Track>>), String>(
        bench,
        AoCYear::AOC2018,
        AoCDay::AOCD13,
        setup,
        find2,
    )
    .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: (bool, Arc<Array2<Track>>)) -> String {
    find_res(data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"|
v
|
|
|
^
|";
    const TEST_2: &str = r"/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   ";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1), 1, 7, true)?;
        assert_eq!(find(data), "0,3");
        let data = setup_br(Cursor::new(TEST_2), 13, 6, true)?;
        assert_eq!(find(data), "7,3");
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"/>-<\
|   |
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1), 7, 7, true)?;
        assert_eq!(find2(data), "6,4");
        Ok(())
    }
}
