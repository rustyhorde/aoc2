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
use getset::{CopyGetters, Setters};
use itertools::Itertools;
use ndarray::{s, Array2, ArrayBase, Axis, Dim, ViewRepr};
use std::{
    collections::HashSet,
    fmt,
    fs::File,
    hash::{Hash, Hasher},
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

type WarehouseData = (Vec<String>, Vec<String>, bool, bool);

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<WarehouseData, usize>(AoCYear::AOC2024, AoCDay::AOCD15, setup, find)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<WarehouseData, usize>(bench, AoCYear::AOC2024, AoCDay::AOCD15, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> WarehouseData {
    setup_br(reader, true, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T, display: bool, test: bool) -> Result<WarehouseData>
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
    Ok((warehouse, robot_moves, display, test))
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: WarehouseData) -> usize {
    find_res(data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps, clippy::too_many_lines)]
fn find_res(data: WarehouseData, second_star: bool) -> Result<usize> {
    let (warehouse_data, robot_moves_data, display, test) = data;
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

    if test {
        disp_warehouse(&warehouse, "Initial State:");
    }
    display_warehouse(&warehouse, true, "Initial State:", display)?;
    let len = robot_moves.len();
    for (idx, robot_move) in robot_moves.iter().enumerate() {
        if second_star {
            if let Some((next_x, next_y, move_boxes, box_tree)) =
                can_move_robot(&mut warehouse, curr_x, curr_y, *robot_move)?
            {
                warehouse[[curr_x, curr_y]] = ElementKind::Empty;
                if move_boxes && box_tree.is_empty() {
                    move_boxes_lr(&mut warehouse, curr_x, curr_y, *robot_move)?;
                } else if move_boxes {
                    // Move tree
                    // Find min y
                    if let Some(min_y) = box_tree
                        .iter()
                        .flat_map(|x| vec![x.left().1, x.right().1])
                        .min()
                    {
                        eprintln!("min_y: {min_y:?}");
                        let moving = box_tree
                            .iter()
                            .copied()
                            .filter(|x| x.left().1 == min_y || x.right().1 == min_y)
                            .collect::<Vec<Box>>();
                        for bxx in moving {
                            bxx.move_up(&mut warehouse);
                        }
                    }
                }
                curr_x = next_x;
                curr_y = next_y;
                warehouse[[curr_x, curr_y]] = ElementKind::Robot;
            }
            if test {
                disp_warehouse(&warehouse, &format!("Move '{robot_move}'"));
            }
        } else {
            try_move_robot(&mut warehouse, &mut curr_x, &mut curr_y, *robot_move)?;
            display_warehouse(
                &warehouse,
                idx != len - 1,
                &format!("Move '{robot_move}'"),
                display,
            )?;
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

#[derive(Clone, Copy, CopyGetters, Debug, Default, Setters)]
#[getset(get_copy = "pub(crate)", set = "pub(crate)")]
struct Box {
    left: (usize, usize),
    right: (usize, usize),
    can_move: bool,
}

impl Hash for Box {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.left.hash(state);
        self.right.hash(state);
    }
}

impl PartialEq for Box {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.right == other.right
    }
}

impl Eq for Box {}

impl fmt::Display for Box {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Box {{ ({},{}),({},{}) }} => can_move = {}",
            self.left.0, self.left.1, self.right.0, self.right.1, self.can_move
        )
    }
}

impl Box {
    fn move_up(&self, warehouse: &mut Array2<ElementKind>) {
        let new_left_y = self.left.1 - 1;
        let new_right_y = self.right.1 - 1;
        warehouse[[self.left.0, self.left.1]] = ElementKind::Empty;
        warehouse[[self.right.0, self.right.1]] = ElementKind::Empty;
        warehouse[[self.left.0, new_left_y]] = ElementKind::BigBoxLeft;
        warehouse[[self.right.0, new_right_y]] = ElementKind::BigBoxRight;
    }

    #[allow(clippy::nonminimal_bool)]
    fn can_move_up(&self, warehouse: &Array2<ElementKind>, boxes: &HashSet<Box>) -> Option<Box> {
        let up_left = (self.left.0, self.left.1 - 1);
        let up_right = (self.right.0, self.right.1 - 1);
        let boxes_has_up_left = boxes
            .iter()
            .flat_map(|bxx| vec![bxx.left(), bxx.right()])
            .contains(&up_left);
        let boxes_has_up_right = boxes
            .iter()
            .flat_map(|bxx| vec![bxx.left(), bxx.right()])
            .contains(&up_right);
        let mut updated_box = None;
        let mut can_move = false;

        if boxes_has_up_right && boxes_has_up_left {
            can_move = true;
        } else if let Some((above_left, above_right)) =
            warehouse.get(up_left).zip(warehouse.get(up_right))
        {
            if (*above_left == ElementKind::Empty && *above_right == ElementKind::Empty)
                || (*above_left == ElementKind::Empty && boxes_has_up_right)
                || (*above_right == ElementKind::Empty && boxes_has_up_left)
            {
                can_move = true;
            }
        }

        if can_move {
            let mut new_box = *self;
            let _ = new_box.set_can_move(true);
            updated_box = Some(new_box);
        }
        updated_box
    }
}

type MoveData = Option<(usize, usize, bool, HashSet<Box>)>;

fn can_move_robot(
    warehouse: &mut Array2<ElementKind>,
    curr_x: usize,
    curr_y: usize,
    movement: Movement,
) -> Result<MoveData> {
    let mut next_loc = None;
    let (check_x, check_y) = match movement {
        Movement::Up => (curr_x, curr_y - 1),
        Movement::Down => (curr_x, curr_y + 1),
        Movement::Left => (curr_x - 1, curr_y),
        Movement::Right => (curr_x + 1, curr_y),
    };
    let next_elem = warehouse[[check_x, check_y]];
    let box_tree = HashSet::new();

    match next_elem {
        ElementKind::Wall => {}
        ElementKind::Box => return Err(anyhow!("encountered a box in star two!")),
        ElementKind::BigBoxLeft => {
            if movement == Movement::Left {
                return Err(anyhow!("trying to move left into a left box half"));
            }
            match movement {
                Movement::Up | Movement::Down => {
                    // Move Down or Up
                }
                Movement::Right => {
                    let slice = warehouse.slice(s![curr_x + 1.., curr_y]);
                    if let Some(next) = slice.iter().find(|x| {
                        !(**x == ElementKind::BigBoxLeft || **x == ElementKind::BigBoxRight)
                    }) {
                        if *next == ElementKind::Empty {
                            next_loc = Some((check_x, check_y, true, box_tree));
                        }
                    }
                }
                Movement::Left => unreachable!(),
            }
        }
        ElementKind::BigBoxRight => {
            if movement == Movement::Right {
                return Err(anyhow!("trying to move right into a right box half"));
            }
            match movement {
                Movement::Up | Movement::Down => {
                    let mut boxes = HashSet::new();
                    let mut initial_box = Box::default();
                    let _ = initial_box.set_right((check_x, check_y));
                    let _ = initial_box.set_left((check_x - 1, check_y));
                    let _ = boxes.insert(initial_box);
                    traverse_box_tree(warehouse, &mut boxes, initial_box, movement);
                    if can_box_tree_move(warehouse, &mut boxes, movement) {
                        next_loc = Some((check_x, check_y, true, boxes.clone()));
                    }
                    for bxx in boxes {
                        eprintln!("{bxx}");
                    }
                }
                Movement::Left => {
                    let slice = warehouse.slice(s![..curr_x;-1, curr_y]);
                    if let Some(next) = slice.iter().find(|x| {
                        !(**x == ElementKind::BigBoxLeft || **x == ElementKind::BigBoxRight)
                    }) {
                        if *next == ElementKind::Empty {
                            next_loc = Some((check_x, check_y, true, box_tree));
                        }
                    }
                }
                Movement::Right => unreachable!(),
            }
        }
        ElementKind::Robot => return Err(anyhow!("encountered another robot!")),
        ElementKind::Empty => {
            next_loc = Some((check_x, check_y, false, box_tree));
        }
    }
    Ok(next_loc)
}

fn can_box_tree_move(
    warehouse: &Array2<ElementKind>,
    boxes: &mut HashSet<Box>,
    _movement: Movement,
) -> bool {
    let mut replace = vec![];
    for bxx in boxes.iter() {
        if let Some(bxx) = bxx.can_move_up(warehouse, boxes) {
            replace.push(bxx);
        }
    }

    for bxx in replace {
        let _ = boxes.replace(bxx);
    }

    boxes.iter().all(Box::can_move)
}

fn traverse_box_tree(
    warehouse: &Array2<ElementKind>,
    boxes: &mut HashSet<Box>,
    bxx: Box,
    movement: Movement,
) {
    if movement == Movement::Up {
        if let Some(bxx) = has_up_left(warehouse, &bxx) {
            let _ = boxes.insert(bxx);
            traverse_box_tree(warehouse, boxes, bxx, movement);
        }
        if let Some(bxx) = has_up_right(warehouse, &bxx) {
            let _ = boxes.insert(bxx);
            traverse_box_tree(warehouse, boxes, bxx, movement);
        }
    }
}

fn has_up_left(warehouse: &Array2<ElementKind>, bxx: &Box) -> Option<Box> {
    let check_x = bxx.left().0 - 1;
    let check_y = bxx.left().1 - 1;
    let mut box_opt = None;
    if let Some(elem) = warehouse.get([check_x, check_y]) {
        if *elem == ElementKind::BigBoxLeft {
            let mut bxx = Box::default();
            let _ = bxx.set_left((check_x, check_y));
            let _ = bxx.set_right((check_x + 1, check_y));
            box_opt = Some(bxx);
        }
    }
    box_opt
}

fn has_up_right(warehouse: &Array2<ElementKind>, bxx: &Box) -> Option<Box> {
    let check_x = bxx.right().0 + 1;
    let check_y = bxx.right().1 - 1;
    let mut box_opt = None;
    if let Some(elem) = warehouse.get([check_x, check_y]) {
        if *elem == ElementKind::BigBoxRight {
            let mut bxx = Box::default();
            let _ = bxx.set_left((check_x - 1, check_y));
            let _ = bxx.set_right((check_x, check_y));
            box_opt = Some(bxx);
        }
    }
    box_opt
}

fn move_boxes_lr(
    warehouse: &mut Array2<ElementKind>,
    from_x: usize,
    from_y: usize,
    movement: Movement,
) -> Result<()> {
    let mut slice = match movement {
        Movement::Left => warehouse.slice_mut(s![..from_x;-1, from_y]),
        Movement::Right => warehouse.slice_mut(s![from_x + 1.., from_y]),
        _ => return Err(anyhow!("invalid movement type for left or right")),
    };

    let mut prev = ElementKind::Empty;

    for next in &mut slice {
        if *next == ElementKind::BigBoxLeft {
            prev = ElementKind::BigBoxRight;
            *next = ElementKind::BigBoxRight;
        } else if *next == ElementKind::BigBoxRight {
            prev = ElementKind::BigBoxLeft;
            *next = ElementKind::BigBoxLeft;
        } else if *next == ElementKind::Empty {
            if prev == ElementKind::BigBoxRight {
                *next = ElementKind::BigBoxLeft;
            } else if prev == ElementKind::BigBoxLeft {
                *next = ElementKind::BigBoxRight;
            }
            break;
        }
    }

    Ok(())
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

fn display_warehouse(
    warehouse: &Array2<ElementKind>,
    restore: bool,
    header: &str,
    display: bool,
) -> Result<()> {
    if display {
        let mut stdout = stdout();

        let _ = stdout.execute(Hide)?;
        let _ = stdout.queue(SavePosition)?;
        let _ = stdout.queue(Clear(ClearType::CurrentLine))?;
        let _ = stdout.write(format!("{}", style(header).bold().yellow()).as_bytes())?;
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
    }
    Ok(())
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<WarehouseData, usize>(AoCYear::AOC2024, AoCDay::AOCD15, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<WarehouseData, usize>(
        bench,
        AoCYear::AOC2024,
        AoCDay::AOCD15,
        setup,
        find2,
    )
    .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: WarehouseData) -> usize {
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
        let data = setup_br(Cursor::new(TEST_1), false, false)?;
        assert_eq!(find(data), 2028);
        let data = setup_br(Cursor::new(TEST_2), false, false)?;
        assert_eq!(find(data), 10092);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::{find2, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"
########
#....#.#
#......#
#...OO.#
#...OO@#
#..OO..#
#......#
########

<vv<<^";
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
        let data = setup_br(Cursor::new(TEST_1), false, true)?;
        assert_eq!(find2(data), 0);
        let data = setup_br(Cursor::new(TEST_2), false, false)?;
        assert_eq!(find2(data), 0);
        Ok(())
    }
}
