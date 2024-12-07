// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Advent of Code - Day 5
//!
//! --- Day 5: Sunny with a Chance of Asteroids ---
//!
//! You're starting to sweat as the ship makes its way toward Mercury. The Elves suggest that you get the air conditioner working by upgrading your ship computer to support the Thermal Environment Supervision Terminal.
//!
//! The Thermal Environment Supervision Terminal (TEST) starts by running a diagnostic program (your puzzle input). The TEST diagnostic program will run on your existing Intcode computer after a few modifications:
//!
//! First, you'll need to add two new instructions:
//!
//! > Opcode 3 takes a single integer as input and saves it to the position given by its only parameter. For example, the instruction 3,50 would take an input value and store it at address 50.
//! > Opcode 4 outputs the value of its only parameter. For example, the instruction 4,50 would output the value at address 50.
//!
//! Programs that use these instructions will come with documentation that explains what should be connected to the input and output. The program 3,0,4,0,99 outputs whatever it gets as input, then halts.
//!
//! Second, you'll need to add support for parameter modes:
//!
//! Each parameter of an instruction is handled based on its parameter mode. Right now, your ship computer already understands parameter mode 0, position mode, which causes the parameter to be interpreted as a position - if the parameter is 50, its value is the value stored at address 50 in memory. Until now, all parameters have been in position mode.
//!
//! Now, your ship computer will also need to handle parameters in mode 1, immediate mode. In immediate mode, a parameter is interpreted as a value - if the parameter is 50, its value is simply 50.
//!
//! Parameter modes are stored in the same value as the instruction's opcode. The opcode is a two-digit number based only on the ones and tens digit of the value, that is, the opcode is the rightmost two digits of the first value in an instruction. Parameter modes are single digits, one per parameter, read right-to-left from the opcode: the first parameter's mode is in the hundreds digit, the second parameter's mode is in the thousands digit, the third parameter's mode is in the ten-thousands digit, and so on. Any missing modes are 0.
//!
//! For example, consider the program 1002,4,3,4,33.
//!
//! The first instruction, 1002,4,3,4, is a multiply instruction - the rightmost two digits of the first value, 02, indicate opcode 2, multiplication. Then, going right to left, the parameter modes are 0 (hundreds digit), 1 (thousands digit), and 0 (ten-thousands digit, not present and therefore zero):
//!
//! > ABCDE
//! >  1002
//! >
//! > DE - two-digit opcode,      02 == opcode 2
//! >  C - mode of 1st parameter,  0 == position mode
//! >  B - mode of 2nd parameter,  1 == immediate mode
//! >  A - mode of 3rd parameter,  0 == position mode,
//! >                                   omitted due to being a leading zero
//!
//! This instruction multiplies its first two parameters. The first parameter, 4 in position mode, works like it did before - its value is the value stored at address 4 (33). The second parameter, 3 in immediate mode, simply has value 3. The result of this operation, 33 * 3 = 99, is written according to the third parameter, 4 in position mode, which also works like it did before - 99 is written to address 4.
//!
//! Parameters that an instruction writes to will never be in immediate mode.
//!
//! Finally, some notes:
//!
//! > It is important to remember that the instruction pointer should increase by the number of values in the instruction after the instruction finishes. Because of the new instructions, this amount is no longer always 4.
//! > Integers can be negative: 1101,100,-1,4,0 is a valid program (find 100 + -1, store the result in position 4).
//!
//! The TEST diagnostic program will start by requesting from the user the ID of the system to test by running an input instruction - provide it 1, the ID for the ship's air conditioner unit.
//!
//! It will then perform a series of diagnostic tests confirming that various parts of the Intcode computer, like parameter modes, function correctly. For each test, it will run an output instruction indicating how far the result of the test was from the expected value, where 0 means the test was successful. Non-zero outputs mean that a function is not working correctly; check the instructions that were run before the output instruction to see which one failed.
//!
//! Finally, the program will output a diagnostic code and immediately halt. This final output isn't an error; an output followed immediately by a halt means the program finished. If all outputs were zero except the diagnostic code, the diagnostic program ran successfully.
//!
//! After providing 1 to the only input instruction and passing all the tests, what diagnostic code does the program produce?
//!
//! --- Part Two ---
//!
//! The air conditioner comes online! Its cold air feels good for a while, but then the TEST alarms start to go off. Since the air conditioner can't vent its heat anywhere but back into the spacecraft, it's actually making the air inside the ship warmer.
//!
//! Instead, you'll need to use the TEST to extend the thermal radiators. Fortunately, the diagnostic program (your puzzle input) is already equipped for this. Unfortunately, your Intcode computer is not.
//!
//! Your computer is only missing a few opcodes:
//!
//! > Opcode 5 is jump-if-true: if the first parameter is non-zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
//! > Opcode 6 is jump-if-false: if the first parameter is zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
//! > Opcode 7 is less than: if the first parameter is less than the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
//! > Opcode 8 is equals: if the first parameter is equal to the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
//!
//! Like all instructions, these instructions need to support parameter modes as described above.
//!
//! Normally, after an instruction is finished, the instruction pointer increases by the number of values in that instruction. However, if the instruction modifies the instruction pointer, that value is used and the instruction pointer is not automatically increased.
//!
//! For example, here are several programs that take one input, compare it to the value 8, and then produce one output:
//!
//! > 3,9,8,9,10,9,4,9,99,-1,8 - Using position mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
//! > 3,9,7,9,10,9,4,9,99,-1,8 - Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
//! > 3,3,1108,-1,8,3,4,3,99 - Using immediate mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
//! > 3,3,1107,-1,8,3,4,3,99 - Using immediate mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
//!
//! Here are some jump tests that take an input, then output 0 if the input was zero or 1 if the input was non-zero:
//!
//! > 3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9 (using position mode)
//! > 3,3,1105,-1,9,1101,0,0,12,4,12,99,1 (using immediate mode)
//!
//! Here's a larger example:
//!
//! > 3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
//! > 1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
//! > 999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99
//!
//! The above example program uses an input instruction to ask for a single number. The program will then output 999 if the input value is below 8, output 1000 if the input value is equal to 8, or output 1001 if the input value is greater than 8.
//!
//! This time, when the TEST diagnostic program runs its input instruction to get the ID of the system to test, provide it 5, the ID for the ship's thermal radiator controller. This diagnostic test suite only outputs one number, the diagnostic code.
//!
//! What is the diagnostic code for system ID 5?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_solution, valid_lines},
};
use anyhow::Result;
use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
use tracing::error;

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_solution::<isize>(AoCYear::AOC2019, AoCDay::AOCD05, find).map(|_| 0)
}

fn find(reader: BufReader<File>) -> isize {
    find_br(reader, 1)
        .map_err(|e| {
            error!("{e}");
            e
        })
        .unwrap_or_default()
}

fn find_br<T>(reader: T, input: isize) -> Result<isize>
where
    T: BufRead,
{
    let mut intcodes = vec![];
    let mut output: isize = 0;
    for line in valid_lines(reader) {
        intcodes = line.split(',').filter_map(as_isize).collect();
    }

    let mut op_code_idx = 0;
    let (mut modes, mut op_code) = parse_op_code(intcodes[op_code_idx])?;

    while op_code != 99 {
        if op_code == 1 {
            handle_add(&mut intcodes, &mut op_code_idx, modes)?;
        } else if op_code == 2 {
            handle_mult(&mut intcodes, &mut op_code_idx, modes)?;
        } else if op_code == 3 {
            handle_input(&mut intcodes, &mut op_code_idx, input)?;
        } else if op_code == 4 {
            handle_output(&mut intcodes, &mut op_code_idx, &mut output, modes)?;
        } else {
            break;
        }
        (modes, op_code) = parse_op_code(intcodes[op_code_idx])?;
    }
    Ok(output)
}

fn as_isize(x: &str) -> Option<isize> {
    x.parse::<isize>().ok()
}

fn parse_op_code(op_code_val: isize) -> Result<((u8, u8, u8), usize)> {
    let op_code_digits = to_digits(usize::try_from(op_code_val)?)?;
    let (modes, op_code_sl) = op_code_digits.split_at(3);
    let op_code_str = op_code_sl.iter().map(ToString::to_string).join("");
    let op_code = op_code_str.parse::<usize>()?;
    Ok(((modes[2], modes[1], modes[0]), op_code))
}

#[allow(clippy::same_item_push)]
fn to_digits(mut v: usize) -> Result<Vec<u8>> {
    let mut digits: Vec<u8> = Vec::with_capacity(20);

    while v > 0 {
        let n = u8::try_from(v % 10)?;
        v /= 10;
        digits.push(n);
    }
    for _ in digits.len()..5 {
        digits.push(0);
    }
    digits.reverse();
    Ok(digits)
}

fn handle_add(intcodes: &mut [isize], idx: &mut usize, modes: (u8, u8, u8)) -> Result<()> {
    let idx_1 = intcodes[*idx + 1];
    let idx_2 = intcodes[*idx + 2];
    let idx_3 = usize::try_from(intcodes[*idx + 3])?;

    let addend_1 = if modes.0 == 0 {
        intcodes[usize::try_from(idx_1)?]
    } else {
        idx_1
    };
    let addend_2 = if modes.1 == 0 {
        intcodes[usize::try_from(idx_2)?]
    } else {
        idx_2
    };
    intcodes[idx_3] = addend_1 + addend_2;
    *idx += 4;
    Ok(())
}

fn handle_mult(intcodes: &mut [isize], idx: &mut usize, modes: (u8, u8, u8)) -> Result<()> {
    let idx_1 = intcodes[*idx + 1];
    let idx_2 = intcodes[*idx + 2];
    let idx_3 = usize::try_from(intcodes[*idx + 3])?;

    let factor_1 = if modes.0 == 0 {
        intcodes[usize::try_from(idx_1)?]
    } else {
        idx_1
    };
    let factor_2 = if modes.1 == 0 {
        intcodes[usize::try_from(idx_2)?]
    } else {
        idx_2
    };
    intcodes[idx_3] = factor_1 * factor_2;
    *idx += 4;
    Ok(())
}

fn handle_input(intcodes: &mut [isize], idx: &mut usize, input: isize) -> Result<()> {
    let idx_1 = usize::try_from(intcodes[*idx + 1])?;
    intcodes[idx_1] = input;
    *idx += 2;
    Ok(())
}

fn handle_output(
    intcodes: &mut [isize],
    idx: &mut usize,
    output: &mut isize,
    modes: (u8, u8, u8),
) -> Result<()> {
    let idx_1 = usize::try_from(intcodes[*idx + 1])?;
    *output = if modes.0 == 0 {
        intcodes[idx_1]
    } else {
        intcodes[*idx + 1]
    };
    *idx += 2;
    Ok(())
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_solution::<isize>(AoCYear::AOC2019, AoCDay::AOCD05, find2).map(|_| 0)
}

fn find2(reader: BufReader<File>) -> isize {
    find2_br(reader, 5)
        .map_err(|e| {
            error!("{e}");
            e
        })
        .unwrap_or_default()
}

fn find2_br<T>(reader: T, input: isize) -> Result<isize>
where
    T: BufRead,
{
    let mut intcodes = vec![];
    let mut output: isize = 0;
    for line in valid_lines(reader) {
        intcodes = line.split(',').filter_map(as_isize).collect();
    }

    let mut op_code_idx = 0;
    let (mut modes, mut op_code) = parse_op_code(intcodes[op_code_idx])?;

    while op_code != 99 {
        if op_code == 1 {
            handle_add(&mut intcodes, &mut op_code_idx, modes)?;
        } else if op_code == 2 {
            handle_mult(&mut intcodes, &mut op_code_idx, modes)?;
        } else if op_code == 3 {
            handle_input(&mut intcodes, &mut op_code_idx, input)?;
        } else if op_code == 4 {
            handle_output(&mut intcodes, &mut op_code_idx, &mut output, modes)?;
        } else if op_code == 5 {
            handle_jump_if_true(&mut intcodes, &mut op_code_idx, modes)?;
        } else if op_code == 6 {
            handle_jump_if_false(&mut intcodes, &mut op_code_idx, modes)?;
        } else if op_code == 7 {
            handle_less_than(&mut intcodes, &mut op_code_idx, modes)?;
        } else if op_code == 8 {
            handle_equals(&mut intcodes, &mut op_code_idx, modes)?;
        } else {
            break;
        }
        (modes, op_code) = parse_op_code(intcodes[op_code_idx])?;
    }
    Ok(output)
}

fn handle_jump_if_true(intcodes: &mut [isize], idx: &mut usize, modes: (u8, u8, u8)) -> Result<()> {
    let idx_1 = intcodes[*idx + 1];
    let check_val = if modes.0 == 0 {
        intcodes[usize::try_from(idx_1)?]
    } else {
        idx_1
    };
    if check_val != 0 {
        let idx_2 = usize::try_from(intcodes[*idx + 2])?;
        *idx = if modes.1 == 0 {
            usize::try_from(intcodes[idx_2])?
        } else {
            idx_2
        };
    } else {
        *idx += 3;
    }
    Ok(())
}

fn handle_jump_if_false(
    intcodes: &mut [isize],
    idx: &mut usize,
    modes: (u8, u8, u8),
) -> Result<()> {
    let idx_1 = intcodes[*idx + 1];
    let check_val = if modes.0 == 0 {
        intcodes[usize::try_from(idx_1)?]
    } else {
        idx_1
    };
    if check_val == 0 {
        let idx_2 = usize::try_from(intcodes[*idx + 2])?;
        *idx = if modes.1 == 0 {
            usize::try_from(intcodes[idx_2])?
        } else {
            idx_2
        };
    } else {
        *idx += 3;
    }
    Ok(())
}

fn handle_less_than(intcodes: &mut [isize], idx: &mut usize, modes: (u8, u8, u8)) -> Result<()> {
    let idx_1 = intcodes[*idx + 1];
    let idx_2 = intcodes[*idx + 2];
    let idx_3 = usize::try_from(intcodes[*idx + 3])?;

    let val_1 = if modes.0 == 0 {
        intcodes[usize::try_from(idx_1)?]
    } else {
        idx_1
    };
    let val_2 = if modes.1 == 0 {
        intcodes[usize::try_from(idx_2)?]
    } else {
        idx_2
    };
    if val_1 < val_2 {
        intcodes[idx_3] = 1;
    } else {
        intcodes[idx_3] = 0;
    }
    *idx += 4;
    Ok(())
}

fn handle_equals(intcodes: &mut [isize], idx: &mut usize, modes: (u8, u8, u8)) -> Result<()> {
    let idx_1 = intcodes[*idx + 1];
    let idx_2 = intcodes[*idx + 2];
    let idx_3 = usize::try_from(intcodes[*idx + 3])?;

    let val_1 = if modes.0 == 0 {
        intcodes[usize::try_from(idx_1)?]
    } else {
        idx_1
    };
    let val_2 = if modes.1 == 0 {
        intcodes[usize::try_from(idx_2)?]
    } else {
        idx_2
    };
    if val_1 == val_2 {
        intcodes[idx_3] = 1;
    } else {
        intcodes[idx_3] = 0;
    }
    *idx += 4;
    Ok(())
}

#[cfg(test)]
mod one_star {
    use super::{find_br, parse_op_code, to_digits};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"3,0,4,0,99";

    #[test]
    fn parse_op_code_works() -> Result<()> {
        assert_eq!(parse_op_code(3)?, ((0, 0, 0), 3));
        Ok(())
    }

    #[test]
    fn to_digits_works() -> Result<()> {
        let mut op_code: usize = 1;
        assert_eq!(to_digits(op_code)?, vec![0, 0, 0, 0, 1]);
        op_code = 1002;
        assert_eq!(to_digits(op_code)?, vec![0, 1, 0, 0, 2]);
        Ok(())
    }

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find_br(Cursor::new(TEST_1), 1)?, 1);
        assert_eq!(find_br(Cursor::new(TEST_1), 2)?, 2);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {
    use super::find2_br;
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"3,9,8,9,10,9,4,9,99,-1,8";
    const TEST_2: &str = r"3,9,7,9,10,9,4,9,99,-1,8";
    const TEST_3: &str = r"3,3,1108,-1,8,3,4,3,99";
    const TEST_4: &str = r"3,3,1107,-1,8,3,4,3,99";
    const TEST_5: &str = r"3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
    const TEST_6: &str = r"3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
    const TEST_7: &str = r"3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";

    #[test]
    fn solution() -> Result<()> {
        assert_eq!(find2_br(Cursor::new(TEST_1), 7)?, 0);
        assert_eq!(find2_br(Cursor::new(TEST_1), 8)?, 1);
        assert_eq!(find2_br(Cursor::new(TEST_1), 9)?, 0);
        assert_eq!(find2_br(Cursor::new(TEST_2), 7)?, 1);
        assert_eq!(find2_br(Cursor::new(TEST_2), 8)?, 0);
        assert_eq!(find2_br(Cursor::new(TEST_2), 9)?, 0);
        assert_eq!(find2_br(Cursor::new(TEST_3), 7)?, 0);
        assert_eq!(find2_br(Cursor::new(TEST_3), 8)?, 1);
        assert_eq!(find2_br(Cursor::new(TEST_3), 9)?, 0);
        assert_eq!(find2_br(Cursor::new(TEST_4), 7)?, 1);
        assert_eq!(find2_br(Cursor::new(TEST_4), 8)?, 0);
        assert_eq!(find2_br(Cursor::new(TEST_4), 9)?, 0);
        assert_eq!(find2_br(Cursor::new(TEST_5), 0)?, 0);
        assert_eq!(find2_br(Cursor::new(TEST_5), 1)?, 1);
        assert_eq!(find2_br(Cursor::new(TEST_6), 0)?, 0);
        assert_eq!(find2_br(Cursor::new(TEST_6), 1)?, 1);
        assert_eq!(find2_br(Cursor::new(TEST_7), 7)?, 999);
        assert_eq!(find2_br(Cursor::new(TEST_7), 8)?, 1000);
        assert_eq!(find2_br(Cursor::new(TEST_7), 9)?, 1001);
        Ok(())
    }
}
