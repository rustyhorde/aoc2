// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code ---**
//!
//! **--- Day 21: Keypad Conundrum ---**
//!
//! **NOTE** - Got this from someone elses solution.  This one stumped me badly.
//!
//! As you teleport onto Santa's Reindeer-class starship, The Historians begin to panic: someone from their search party is missing. A quick life-form scan by the ship's computer reveals that when the missing Historian teleported, he arrived in another part of the ship.
//!
//! The door to that area is locked, but the computer can't open it; it can only be opened by physically typing the door codes (your puzzle input) on the numeric keypad on the door.
//!
//! The numeric keypad has four rows of buttons: 789, 456, 123, and finally an empty gap followed by 0A. Visually, they are arranged like this:
//!
//! ```text
//! +---+---+---+
//! | 7 | 8 | 9 |
//! +---+---+---+
//! | 4 | 5 | 6 |
//! +---+---+---+
//! | 1 | 2 | 3 |
//! +---+---+---+
//!     | 0 | A |
//!     +---+---+
//! ```
//!
//! Unfortunately, the area outside the door is currently depressurized and nobody can go near the door. A robot needs to be sent instead.
//!
//! The robot has no problem navigating the ship and finding the numeric keypad, but it's not designed for button pushing: it can't be told to push a specific button directly. Instead, it has a robotic arm that can be controlled remotely via a directional keypad.
//!
//! The directional keypad has two rows of buttons: a gap / ^ (up) / A (activate) on the first row and < (left) / v (down) / > (right) on the second row. Visually, they are arranged like this:
//!
//! ```text
//!     +---+---+
//!     | ^ | A |
//! +---+---+---+
//! | < | v | > |
//! +---+---+---+
//! ```
//!
//! When the robot arrives at the numeric keypad, its robotic arm is pointed at the A button in the bottom right corner. After that, this directional keypad remote control must be used to maneuver the robotic arm: the up / down / left / right buttons cause it to move its arm one button in that direction, and the A button causes the robot to briefly move forward, pressing the button being aimed at by the robotic arm.
//!
//! For example, to make the robot type 029A on the numeric keypad, one sequence of inputs on the directional keypad you could use is:
//!
//! ```text
//!     < to move the arm from A (its initial position) to 0.
//!     A to push the 0 button.
//!     ^A to move the arm to the 2 button and push it.
//!     >^^A to move the arm to the 9 button and push it.
//!     vvvA to move the arm to the A button and push it.
//! ```
//!
//! In total, there are three shortest possible sequences of button presses on this directional keypad that would cause the robot to type 029A: ```<A^A>^^AvvvA```, ```<A^A^>^AvvvA```, and ```<A^A^^>AvvvA```.
//!
//! Unfortunately, the area containing this directional keypad remote control is currently experiencing high levels of radiation and nobody can go near it. A robot needs to be sent instead.
//!
//! When the robot arrives at the directional keypad, its robot arm is pointed at the A button in the upper right corner. After that, a second, different directional keypad remote control is used to control this robot (in the same way as the first robot, except that this one is typing on a directional keypad instead of a numeric keypad).
//!
//! There are multiple shortest possible sequences of directional keypad button presses that would cause this robot to tell the first robot to type 029A on the door. One such sequence is ```v<<A>>^A<A>AvA<^AA>A<vAAA>^A```.
//!
//! Unfortunately, the area containing this second directional keypad remote control is currently -40 degrees! Another robot will need to be sent to type on that directional keypad, too.
//!
//! There are many shortest possible sequences of directional keypad button presses that would cause this robot to tell the second robot to tell the first robot to eventually type 029A on the door. One such sequence is ```<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A```.
//!
//! Unfortunately, the area containing this third directional keypad remote control is currently full of Historians, so no robots can find a clear path there. Instead, you will have to type this sequence yourself.
//!
//! Were you to choose this sequence of button presses, here are all of the buttons that would be pressed on your directional keypad, the two robots' directional keypads, and the numeric keypad:
//!
//! ```text
//! <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
//! v<<A>>^A<A>AvA<^AA>A<vAAA>^A
//! <A^A>^^AvvvA
//! 029A
//! ```
//!
//! In summary, there are the following keypads:
//!
//! ```text
//!     One directional keypad that you are using.
//!     Two directional keypads that robots are using.
//!     One numeric keypad (on a door) that a robot is using.
//! ```
//!
//! It is important to remember that these robots are not designed for button pushing. In particular, if a robot arm is ever aimed at a gap where no button is present on the keypad, even for an instant, the robot will panic unrecoverably. So, don't do that. All robots will initially aim at the keypad's A key, wherever it is.
//!
//! To unlock the door, five codes will need to be typed on its numeric keypad. For example:
//!
//! ```text
//! 029A
//! 980A
//! 179A
//! 456A
//! 379A
//! ```
//!
//! For each of these, here is a shortest sequence of button presses you could type to cause the desired code to be typed on the numeric keypad:
//!
//! ```text
//! 029A: <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
//! 980A: <v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A
//! 179A: <v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
//! 456A: <v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A
//! 379A: <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
//! ```
//!
//! The Historians are getting nervous; the ship computer doesn't remember whether the missing Historian is trapped in the area containing a giant electromagnet or molten lava. You'll need to make sure that for each of the five codes, you find the shortest sequence of button presses necessary.
//!
//! The complexity of a single code (like 029A) is equal to the result of multiplying these two values:
//!
//! ```text
//!     The length of the shortest sequence of button presses you need to type on your directional keypad in order to cause the code to be typed on the numeric keypad; for 029A, this would be 68.
//!     The numeric part of the code (ignoring leading zeroes); for 029A, this would be 29.
//! ```
//!
//! In the above example, complexity of the five codes can be found by calculating 68 * 29, 60 * 980, 68 * 179, 64 * 456, and 64 * 379. Adding these together produces 126384.
//!
//! Find the fewest number of button presses you'll need to perform in order to cause the robot in front of the door to type each code. What is the sum of the complexities of the five codes on your list?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const NUM_CHARS: usize = 16;

const LUT3: [u128; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 21, 26, 16, 18, 18, 26, 21, 12, 27, 22,
    13, 28, 23, 14, 0, 19, 1, 18, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 10, 1, 0, 19, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 18, 0, 1, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 25, 17, 1,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 1, 25, 12, 19, 26, 13, 20, 27, 14, 21, 0, 22,
    0, 0, 0, 0, 21, 1, 10, 11, 12, 19, 20, 13, 20, 21, 0, 17, 0, 0, 0, 0, 16, 18, 1, 10, 21, 12,
    19, 22, 13, 20, 0, 16, 0, 0, 0, 0, 21, 19, 18, 1, 22, 21, 12, 23, 22, 13, 0, 23, 0, 0, 0, 0,
    22, 16, 17, 18, 1, 10, 11, 12, 19, 20, 0, 18, 0, 0, 0, 0, 17, 21, 16, 17, 18, 1, 10, 21, 12,
    19, 0, 17, 0, 0, 0, 0, 22, 22, 21, 16, 19, 18, 1, 22, 21, 12, 0, 24, 0, 0, 0, 0, 23, 17, 18,
    19, 16, 17, 18, 1, 10, 11, 0, 19, 0, 0, 0, 0, 18, 22, 17, 18, 21, 16, 17, 18, 1, 10, 0, 18, 0,
    0, 0, 0, 23, 23, 22, 17, 22, 21, 16, 19, 18, 1,
];

const LUT26: [u128; 256] = [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    1,
    27_622_800_565,
    30_331_287_706,
    20_790_420_654,
    22_411_052_532,
    22_411_052_532,
    31_420_065_370,
    28_154_654_777,
    14_752_615_084,
    31_420_065_371,
    28_154_654_778,
    14_752_615_085,
    31_420_065_372,
    28_154_654_779,
    14_752_615_086,
    0,
    24_095_973_437,
    1,
    22_411_052_532,
    14_287_938_116,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    25_419_021_194,
    14_287_938_116,
    1,
    0,
    25_419_021_193,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    14_752_615_084,
    22_411_052_532,
    0,
    1,
    28_154_654_777,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    14_287_938_116,
    0,
    30_331_287_705,
    22_778_092_491,
    1,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    14_287_938_116,
    0,
    0,
    0,
    0,
    1,
    31_420_065_369,
    14_752_615_084,
    24_095_973_437,
    31_420_065_370,
    14_752_615_085,
    24_095_973_438,
    31_420_065_371,
    14_752_615_086,
    24_095_973_439,
    0,
    27_052_881_364,
    0,
    0,
    0,
    0,
    27_052_881_363,
    1,
    14_287_938_116,
    14_287_938_117,
    14_752_615_084,
    24_095_973_437,
    24_095_973_438,
    14_752_615_085,
    24_095_973_438,
    24_095_973_439,
    0,
    22_778_092_491,
    0,
    0,
    0,
    0,
    20_790_420_654,
    22_411_052_532,
    1,
    14_287_938_116,
    28_154_654_777,
    14_752_615_084,
    24_095_973_437,
    28_154_654_778,
    14_752_615_085,
    24_095_973_438,
    0,
    20_790_420_654,
    0,
    0,
    0,
    0,
    27_622_800_565,
    22_411_052_533,
    22_411_052_532,
    1,
    28_154_654_778,
    28_154_654_777,
    14_752_615_084,
    28_154_654_779,
    28_154_654_778,
    14_752_615_085,
    0,
    27_052_881_365,
    0,
    0,
    0,
    0,
    27_052_881_364,
    20_790_420_654,
    22_778_092_491,
    22_778_092_492,
    1,
    14_287_938_116,
    14_287_938_117,
    14_752_615_084,
    24_095_973_437,
    24_095_973_438,
    0,
    22_778_092_492,
    0,
    0,
    0,
    0,
    20_790_420_655,
    27_622_800_565,
    20_790_420_654,
    22_778_092_491,
    22_411_052_532,
    1,
    14_287_938_116,
    28_154_654_777,
    14_752_615_084,
    24_095_973_437,
    0,
    20_790_420_655,
    0,
    0,
    0,
    0,
    27_622_800_566,
    27_622_800_566,
    27_622_800_565,
    20_790_420_654,
    22_411_052_533,
    22_411_052_532,
    1,
    28_154_654_778,
    28_154_654_777,
    14_752_615_084,
    0,
    27_052_881_366,
    0,
    0,
    0,
    0,
    27_052_881_365,
    20_790_420_655,
    22_778_092_492,
    22_778_092_493,
    20_790_420_654,
    22_778_092_491,
    22_778_092_492,
    1,
    14_287_938_116,
    14_287_938_117,
    0,
    22_778_092_493,
    0,
    0,
    0,
    0,
    20_790_420_656,
    27_622_800_566,
    20_790_420_655,
    22_778_092_492,
    27_622_800_565,
    20_790_420_654,
    22_778_092_491,
    22_411_052_532,
    1,
    14_287_938_116,
    0,
    20_790_420_656,
    0,
    0,
    0,
    0,
    27_622_800_567,
    27_622_800_567,
    27_622_800_566,
    20_790_420_655,
    27_622_800_566,
    27_622_800_565,
    20_790_420_654,
    22_411_052_533,
    22_411_052_532,
    1,
];

type KeypadData = Vec<u128>;

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<KeypadData, u128>(AoCYear::AOC2024, AoCDay::AOCD21, setup, find).map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<KeypadData, u128>(bench, AoCYear::AOC2024, AoCDay::AOCD21, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> KeypadData {
    setup_br(reader).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T) -> Result<KeypadData>
where
    T: BufRead,
{
    let mut counts = vec![0; NUM_CHARS * NUM_CHARS];
    for line in valid_lines(reader) {
        let numval = line[..line.len() - 1].parse::<u128>()?;

        let mut old_code = encode('A');

        for ch in line.chars() {
            let curr_code = encode(ch);
            counts[old_code * NUM_CHARS + curr_code] += numval;
            old_code = curr_code;
        }
    }
    Ok(counts)
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: KeypadData) -> u128 {
    find_res(&data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(counts: &KeypadData, second_star: bool) -> Result<u128> {
    Ok(counts
        .iter()
        .zip(if second_star {
            LUT26.iter()
        } else {
            LUT3.iter()
        })
        .map(|(x, y)| x * y)
        .sum::<u128>())
}

fn encode(c: char) -> usize {
    match c {
        'A' => 1,
        'v' => 2,
        '<' => 3,
        '>' => 4,
        '^' => 5,
        x @ '0'..='9' => ((x as u8) - b'0' + 6) as usize,
        _ => unreachable!(),
    }
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<KeypadData, u128>(AoCYear::AOC2024, AoCDay::AOCD21, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<KeypadData, u128>(bench, AoCYear::AOC2024, AoCDay::AOCD21, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: KeypadData) -> u128 {
    find_res(&data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {}

#[cfg(test)]
mod two_star {}
