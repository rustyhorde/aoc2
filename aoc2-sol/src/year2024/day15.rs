// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2024 ---**
//!
//! **--- Day 15: Warehouse Woes ---**
//!
//! You appear back inside your own mini submarine! Each Historian drives their mini submarine in a different direction; maybe the Chief has his own submarine down here somewhere as well?
//!
//! You look up to see a vast school of lanternfish swimming past you. On closer inspection, they seem quite anxious, so you drive your mini submarine over to see if you can help.
//!
//! Because lanternfish populations grow rapidly, they need a lot of food, and that food needs to be stored somewhere. That's why these lanternfish have built elaborate warehouse complexes operated by robots!
//!
//! These lanternfish seem so anxious because they have lost control of the robot that operates one of their most important warehouses! It is currently running amok, pushing around boxes in the warehouse with no regard for lanternfish logistics or lanternfish inventory management strategies.
//!
//! Right now, none of the lanternfish are brave enough to swim up to an unpredictable robot so they could shut it off. However, if you could anticipate the robot's movements, maybe they could find a safe option.
//!
//! The lanternfish already have a map of the warehouse and a list of movements the robot will attempt to make (your puzzle input). The problem is that the movements will sometimes fail as boxes are shifted around, making the actual movements of the robot difficult to predict.
//!
//! For example:
//!
//! ```text
//! ##########
//! #..O..O.O#
//! #......O.#
//! #.OO..O.O#
//! #..O@..O.#
//! #O#..O...#
//! #O..O..O.#
//! #.OO.O.OO#
//! #....O...#
//! ##########
//!
//! <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
//! vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
//! ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
//! <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
//! ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
//! ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
//! >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
//! <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
//! ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
//! v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
//! ```
//!
//! As the robot (@) attempts to move, if there are any boxes (O) in the way, the robot will also attempt to push those boxes. However, if this action would cause the robot or a box to move into a wall (#), nothing moves instead, including the robot. The initial positions of these are shown on the map at the top of the document the lanternfish gave you.
//!
//! The rest of the document describes the moves (^ for up, v for down, < for left, > for right) that the robot will attempt to make, in order. (The moves form a single giant sequence; they are broken into multiple lines just to make copy-pasting easier. Newlines within the move sequence should be ignored.)
//!
//! Here is a smaller example to get started:
//!
//! ```text
//! ########
//! #..O.O.#
//! ##@.O..#
//! #...O..#
//! #.#.O..#
//! #...O..#
//! #......#
//! ########
//!
//! <^^>>>vv<v>>v<<
//! ```
//!
//! Were the robot to attempt the given sequence of moves, it would push around the boxes as follows:
//!
//! ```text
//! Initial state:
//! ########
//! #..O.O.#
//! ##@.O..#
//! #...O..#
//! #.#.O..#
//! #...O..#
//! #......#
//! ########
//!
//! Move <:
//! ########
//! #..O.O.#
//! ##@.O..#
//! #...O..#
//! #.#.O..#
//! #...O..#
//! #......#
//! ########
//!
//! Move ^:
//! ########
//! #.@O.O.#
//! ##..O..#
//! #...O..#
//! #.#.O..#
//! #...O..#
//! #......#
//! ########
//!
//! Move ^:
//! ########
//! #.@O.O.#
//! ##..O..#
//! #...O..#
//! #.#.O..#
//! #...O..#
//! #......#
//! ########
//!
//! Move >:
//! ########
//! #..@OO.#
//! ##..O..#
//! #...O..#
//! #.#.O..#
//! #...O..#
//! #......#
//! ########
//!
//! Move >:
//! ########
//! #...@OO#
//! ##..O..#
//! #...O..#
//! #.#.O..#
//! #...O..#
//! #......#
//! ########
//!
//! Move >:
//! ########
//! #...@OO#
//! ##..O..#
//! #...O..#
//! #.#.O..#
//! #...O..#
//! #......#
//! ########
//!
//! Move v:
//! ########
//! #....OO#
//! ##..@..#
//! #...O..#
//! #.#.O..#
//! #...O..#
//! #...O..#
//! ########
//!
//! Move v:
//! ########
//! #....OO#
//! ##..@..#
//! #...O..#
//! #.#.O..#
//! #...O..#
//! #...O..#
//! ########
//!
//! Move <:
//! ########
//! #....OO#
//! ##.@...#
//! #...O..#
//! #.#.O..#
//! #...O..#
//! #...O..#
//! ########
//!
//! Move v:
//! ########
//! #....OO#
//! ##.....#
//! #..@O..#
//! #.#.O..#
//! #...O..#
//! #...O..#
//! ########
//!
//! Move >:
//! ########
//! #....OO#
//! ##.....#
//! #...@O.#
//! #.#.O..#
//! #...O..#
//! #...O..#
//! ########
//!
//! Move >:
//! ########
//! #....OO#
//! ##.....#
//! #....@O#
//! #.#.O..#
//! #...O..#
//! #...O..#
//! ########
//!
//! Move v:
//! ########
//! #....OO#
//! ##.....#
//! #.....O#
//! #.#.O@.#
//! #...O..#
//! #...O..#
//! ########
//!
//! Move <:
//! ########
//! #....OO#
//! ##.....#
//! #.....O#
//! #.#O@..#
//! #...O..#
//! #...O..#
//! ########
//!
//! Move <:
//! ########
//! #....OO#
//! ##.....#
//! #.....O#
//! #.#O@..#
//! #...O..#
//! #...O..#
//! ########
//! ```
//!
//! The larger example has many more moves; after the robot has finished those moves, the warehouse would look like this:
//!
//! ```text
//! ##########
//! #.O.O.OOO#
//! #........#
//! #OO......#
//! #OO@.....#
//! #O#.....O#
//! #O.....OO#
//! #O.....OO#
//! #OO....OO#
//! ##########
//! ```
//!
//! The lanternfish use their own custom Goods Positioning System (GPS for short) to track the locations of the boxes. The GPS coordinate of a box is equal to 100 times its distance from the top edge of the map plus its distance from the left edge of the map. (This process does not stop at wall tiles; measure all the way to the edges of the map.)
//!
//! So, the box shown below has a distance of 1 from the top edge of the map and 4 from the left edge of the map, resulting in a GPS coordinate of 100 * 1 + 4 = 104.
//!
//! ```text
//! #######
//! #...O..
//! #......
//! ```
//!
//! The lanternfish would like to know the sum of all boxes' GPS coordinates after the robot finishes moving. In the larger example, the sum of all boxes' GPS coordinates is 10092. In the smaller example, the sum is 2028.
//!
//! Predict the motion of the robot and boxes in the warehouse. After the robot is finished moving, what is the sum of all boxes' GPS coordinates?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::{anyhow, Error, Result};
use console::style;
use crossterm::{
    cursor::{Hide, MoveToNextLine, RestorePosition, SavePosition, Show},
    terminal::{Clear, ClearType},
    ExecutableCommand, QueueableCommand,
};
use itertools::Itertools;
use ndarray::{s, Array2, ArrayBase, Axis, Dim, ViewRepr};
use std::{
    fmt,
    fs::File,
    io::{stdout, BufRead, BufReader, Write},
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum ElementKind {
    Wall,
    Box,
    BigBoxLeft,
    BigBoxRight,
    Robot,
    #[default]
    Empty,
}

impl TryFrom<char> for ElementKind {
    type Error = Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        Ok(match value {
            '.' => ElementKind::Empty,
            '#' => ElementKind::Wall,
            '@' => ElementKind::Robot,
            'O' => ElementKind::Box,
            '[' => ElementKind::BigBoxLeft,
            ']' => ElementKind::BigBoxRight,
            _ => return Err(anyhow!("invalid element kind: '{value}'")),
        })
    }
}

impl fmt::Display for ElementKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match self {
            ElementKind::Wall => '#',
            ElementKind::Box => 'O',
            ElementKind::Robot => '@',
            ElementKind::Empty => '.',
            ElementKind::BigBoxLeft => '[',
            ElementKind::BigBoxRight => ']',
        };
        write!(f, "{ch}")
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Movement {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for Movement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match self {
            Movement::Up => '^',
            Movement::Down => 'v',
            Movement::Left => '<',
            Movement::Right => '>',
        };
        write!(f, "{ch}")
    }
}

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<(Vec<String>, Vec<String>), usize>(
        AoCYear::AOC2024,
        AoCDay::AOCD15,
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
    run_bench_solution::<(Vec<String>, Vec<String>), usize>(
        bench,
        AoCYear::AOC2024,
        AoCDay::AOCD15,
        setup,
        find,
    )
    .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> (Vec<String>, Vec<String>) {
    setup_br(reader).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T) -> Result<(Vec<String>, Vec<String>)>
where
    T: BufRead,
{
    let mut warehouse = vec![];
    let mut robot_moves = vec![];

    for line in valid_lines(reader) {
        if line.starts_with('#') {
            warehouse.push(line);
        } else if !line.is_empty() {
            robot_moves.push(line);
        }
    }
    Ok((warehouse, robot_moves))
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: (Vec<String>, Vec<String>)) -> usize {
    find_res(data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(data: (Vec<String>, Vec<String>), second_star: bool) -> Result<usize> {
    let (warehouse_data, robot_moves_data) = data;
    let max_x = warehouse_data[0].len();
    let max_y = warehouse_data.len();

    let robot_moves = robot_moves_data
        .iter()
        .flat_map(|x| x.chars())
        .filter_map(|x| match x {
            '>' => Some(Movement::Right),
            '<' => Some(Movement::Left),
            '^' => Some(Movement::Up),
            'v' => Some(Movement::Down),
            _ => None,
        })
        .collect::<Vec<Movement>>();
    let mut warehouse: Array2<ElementKind> = if second_star {
        Array2::default((max_x * 2, max_y))
    } else {
        Array2::default((max_x, max_y))
    };

    let mut curr_x = 0;
    let mut curr_y = 0;
    for (y, warehouse_row) in warehouse_data.iter().enumerate() {
        for (x, ch) in warehouse_row.chars().enumerate() {
            let elem: ElementKind = ch.try_into()?;
            if second_star {
                let first_x = 2 * x;
                let second_x = 2 * x + 1;
                match elem {
                    ElementKind::Wall | ElementKind::Empty => {
                        warehouse[[first_x, y]] = elem;
                        warehouse[[second_x, y]] = elem;
                    }
                    ElementKind::Box => {
                        warehouse[[first_x, y]] = ElementKind::BigBoxLeft;
                        warehouse[[second_x, y]] = ElementKind::BigBoxRight;
                    }
                    ElementKind::Robot => {
                        warehouse[[first_x, y]] = ElementKind::Robot;
                        warehouse[[second_x, y]] = ElementKind::Empty;
                        curr_x = first_x;
                        curr_y = y;
                    }
                    _ => {}
                }
            } else {
                if elem == ElementKind::Robot {
                    curr_x = x;
                    curr_y = y;
                }
                warehouse[[x, y]] = elem;
            }
        }
    }

    // disp_warehouse(&warehouse, "Initial State:");
    display_warehouse(&warehouse, true, "Initial State:")?;
    let len = robot_moves.len();
    for (idx, robot_move) in robot_moves.iter().enumerate() {
        if second_star {
            if let Some((next_x, next_y)) =
                can_move_robot(&mut warehouse, curr_x, curr_y, *robot_move)?
            {
                warehouse[[curr_x, curr_y]] = ElementKind::Empty;
                curr_x = next_x;
                curr_y = next_y;
                warehouse[[curr_x, curr_y]] = ElementKind::Robot;
            }
            disp_warehouse(&warehouse, &format!("Move '{robot_move}'"));
            // if idx > 0 {
            //     break;
            // }
        } else {
            try_move_robot(&mut warehouse, &mut curr_x, &mut curr_y, *robot_move)?;
            display_warehouse(&warehouse, idx != len - 1, &format!("Move '{robot_move}'"))?;
            // disp_warehouse(&warehouse, &format!("Move '{robot_move}'"));
        }
    }

    let mut gps_sum = 0;
    for ((x, y), elem) in warehouse.indexed_iter() {
        if *elem == ElementKind::Box {
            gps_sum += (100 * y) + x;
        }
    }
    Ok(gps_sum)
}

fn can_move_robot(
    warehouse: &mut Array2<ElementKind>,
    curr_x: usize,
    curr_y: usize,
    movement: Movement,
) -> Result<Option<(usize, usize)>> {
    let mut next_loc = None;
    let (check_x, check_y) = match movement {
        Movement::Up => (curr_x, curr_y - 1),
        Movement::Down => (curr_x, curr_y + 1),
        Movement::Left => (curr_x - 1, curr_y),
        Movement::Right => (curr_x + 1, curr_y),
    };
    let next_elem = warehouse[[check_x, check_y]];

    match next_elem {
        ElementKind::Wall => {}
        ElementKind::Box => return Err(anyhow!("encountered a box in star two!")),
        ElementKind::BigBoxLeft => {
            if movement == Movement::Left {
                return Err(anyhow!("trying to move left into a left box half"));
            }
            eprintln!("moving '{movement}' into {next_elem}");
        }
        ElementKind::BigBoxRight => {
            if movement == Movement::Right {
                return Err(anyhow!("trying to move right into a right box half"));
            }
            eprintln!("moving '{movement}' into {next_elem}");
        }
        ElementKind::Robot => return Err(anyhow!("encountered another robot!")),
        ElementKind::Empty => {
            next_loc = Some((check_x, check_y));
        }
    }
    Ok(next_loc)
}

fn try_move_robot(
    warehouse: &mut Array2<ElementKind>,
    curr_x: &mut usize,
    curr_y: &mut usize,
    movement: Movement,
) -> Result<()> {
    let (next_x, next_y) = match movement {
        Movement::Up => (*curr_x, *curr_y - 1),
        Movement::Down => (*curr_x, *curr_y + 1),
        Movement::Left => (*curr_x - 1, *curr_y),
        Movement::Right => (*curr_x + 1, *curr_y),
    };
    if let Some(next) = warehouse.get((next_x, next_y)) {
        match next {
            ElementKind::Empty => {
                warehouse[[*curr_x, *curr_y]] = ElementKind::Empty;
                warehouse[[next_x, next_y]] = ElementKind::Robot;
                *curr_x = next_x;
                *curr_y = next_y;
            }
            ElementKind::Wall => {
                // Do Nothing
            }
            ElementKind::Box => {
                if try_push_boxes(warehouse, next_x, next_y, movement) {
                    warehouse[[*curr_x, *curr_y]] = ElementKind::Empty;
                    warehouse[[next_x, next_y]] = ElementKind::Robot;
                    *curr_x = next_x;
                    *curr_y = next_y;
                }
            }
            ElementKind::Robot => return Err(anyhow!("i've encountered another robot. Error!!!")),
            ElementKind::BigBoxLeft | ElementKind::BigBoxRight => {
                return Err(anyhow!("i've encountered a big box on one star. Error!!!"))
            }
        }
    }
    Ok(())
}

fn try_push_boxes(
    warehouse: &mut Array2<ElementKind>,
    box_x: usize,
    box_y: usize,
    movement: Movement,
) -> bool {
    let mut can_move = false;
    let mut slice = match movement {
        Movement::Up => warehouse.slice_mut(s![box_x, ..box_y;-1]),
        Movement::Down => warehouse.slice_mut(s![box_x, box_y..]),
        Movement::Left => warehouse.slice_mut(s![..box_x;-1, box_y]),
        Movement::Right => warehouse.slice_mut(s![box_x.., box_y]),
    };
    if can_move_boxes(&slice) {
        can_move = true;
        if let Some((x, _)) = slice.iter().find_position(|x| **x == ElementKind::Empty) {
            slice[x] = ElementKind::Box;
        }
    }

    can_move
}

fn can_move_boxes(slice: &ArrayBase<ViewRepr<&mut ElementKind>, Dim<[usize; 1]>>) -> bool {
    let mut can_move_boxes = false;
    let mut blah = slice.iter().skip_while(|x| **x == ElementKind::Box);

    if let Some(next) = blah.next() {
        if *next == ElementKind::Empty {
            can_move_boxes = true;
        }
    }
    can_move_boxes
}

fn disp_warehouse(warehouse: &Array2<ElementKind>, header: &str) {
    eprintln!("{header}");
    for row in warehouse.axis_iter(Axis(1)) {
        for elem in row {
            eprint!("{elem}");
        }
        eprintln!();
    }
    eprintln!();
}

fn display_warehouse(warehouse: &Array2<ElementKind>, restore: bool, header: &str) -> Result<()> {
    let mut stdout = stdout();

    let _ = stdout.execute(Hide)?;
    let _ = stdout.queue(SavePosition)?;
    let _ = stdout.queue(Clear(ClearType::CurrentLine))?;
    let _ = stdout.write(header.as_bytes())?;
    let _ = stdout.queue(MoveToNextLine(1))?;
    let _ = stdout.queue(MoveToNextLine(1))?;
    for row in warehouse.axis_iter(Axis(1)) {
        for elem in row {
            if *elem == ElementKind::Robot {
                let _ = stdout.write(format!("{}", style(elem).bold().magenta()).as_bytes())?;
            } else if *elem == ElementKind::Box {
                let _ = stdout.write(format!("{}", style(elem).green()).as_bytes())?;
            } else if *elem == ElementKind::Wall {
                let _ = stdout.write(format!("{}", style(elem).red()).as_bytes())?;
            } else {
                let _ = stdout.write(format!("{elem}").as_bytes())?;
            }
        }
        let _ = stdout.queue(MoveToNextLine(1))?;
    }
    if restore {
        let _ = stdout.queue(RestorePosition)?;
    }
    let _ = stdout.execute(Show)?;
    Ok(())
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<(Vec<String>, Vec<String>), usize>(
        AoCYear::AOC2024,
        AoCDay::AOCD15,
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
    run_bench_solution::<(Vec<String>, Vec<String>), usize>(
        bench,
        AoCYear::AOC2024,
        AoCDay::AOCD15,
        setup,
        find2,
    )
    .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: (Vec<String>, Vec<String>)) -> usize {
    find_res(data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const TEST_2: &str = r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find(data), 2028);
        let data = setup_br(Cursor::new(TEST_2))?;
        assert_eq!(find(data), 10092);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    //     const TEST_1: &str = r"#######
    // #...#.#
    // #.....#
    // #..OO@#
    // #..O..#
    // #.....#
    // #######

    // <vv<<^^<<^^";

    const TEST_2: &str = r"
#######
#...#.#
#.....#
#....@#
#.....#
#.....#
#######

^^^^vv<<<<<^>>>>>";

    #[test]
    fn solution() -> Result<()> {
        // let data = setup_br(Cursor::new(TEST_1))?;
        // assert_eq!(find2(data), 0);
        let data = setup_br(Cursor::new(TEST_2))?;
        assert_eq!(find2(data), 0);
        Ok(())
    }
}
