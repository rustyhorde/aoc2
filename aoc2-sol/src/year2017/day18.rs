// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2017 ---**
//!
//! **--- Day 18: Duet ---**
//!
//! You discover a tablet containing some strange assembly code labeled simply "Duet". Rather than bother the sound card with it, you decide to run the code yourself. Unfortunately, you don't see any documentation, so you're left to figure out what the instructions mean on your own.
//!
//! It seems like the assembly is meant to operate on a set of registers that are each named with a single letter and that can each hold a single integer. You suppose each register should start with a value of 0.
//!
//! There aren't that many instructions, so it shouldn't be hard to figure out what they do. Here's what you determine:
//!
//! ```text
//!     snd X plays a sound with a frequency equal to the value of X.
//!     set X Y sets register X to the value of Y.
//!     add X Y increases register X by the value of Y.
//!     mul X Y sets register X to the result of multiplying the value contained in register X by the value of Y.
//!     mod X Y sets register X to the remainder of dividing the value contained in register X by the value of Y (that is, it sets X to the result of X modulo Y).
//!     rcv X recovers the frequency of the last sound played, but only when the value of X is not zero. (If it is zero, the command does nothing.)
//!     jgz X Y jumps with an offset of the value of Y, but only if the value of X is greater than zero. (An offset of 2 skips the next instruction, an offset of -1 jumps to the previous instruction, and so on.)
//! ```
//!
//! Many of the instructions can take either a register (a single letter) or a number. The value of a register is the integer it contains; the value of a number is that number.
//!
//! After each jump instruction, the program continues with the instruction to which the jump jumped. After any other instruction, the program continues with the next instruction. Continuing (or jumping) off either end of the program terminates it.
//!
//! For example:
//!
//! ```text
//! set a 1
//! add a 2
//! mul a a
//! mod a 5
//! snd a
//! set a 0
//! rcv a
//! jgz a -1
//! set a 1
//! jgz a -2
//! ```
//!
//! ```text
//!     The first four instructions set a to 1, add 2 to it, square it, and then set it to itself modulo 5, resulting in a value of 4.
//!     Then, a sound with frequency 4 (the value of a) is played.
//!     After that, a is set to 0, causing the subsequent rcv and jgz instructions to both be skipped (rcv because a is 0, and jgz because a is not greater than 0).
//!     Finally, a is set to 1, causing the next jgz instruction to activate, jumping back two instructions to another jump, which jumps again to the rcv, which ultimately triggers the recover operation.
//! ```
//!
//! At the time the recover operation is executed, the frequency of the last sound played is 4.
//!
//! What is the value of the recovered frequency (the value of the most recently played sound) the first time a rcv instruction is executed with a non-zero value?
//!
//! **--- Part Two ---**
//!
//! As you congratulate yourself for a job well done, you notice that the documentation has been on the back of the tablet this entire time. While you actually got most of the instructions correct, there are a few key differences. This assembly code isn't about sound at all - it's meant to be run twice at the same time.
//!
//! Each running copy of the program has its own set of registers and follows the code independently - in fact, the programs don't even necessarily run at the same speed. To coordinate, they use the send (snd) and receive (rcv) instructions:
//!
//! ```text
//!     snd X sends the value of X to the other program. These values wait in a queue until that program is ready to receive them. Each program has its own message queue, so a program can never receive a message it sent.
//!     rcv X receives the next value and stores it in register X. If no values are in the queue, the program waits for a value to be sent to it. Programs do not continue to the next instruction until they have received a value. Values are received in the order they are sent.
//! ```
//!
//! Each program also has its own program ID (one 0 and the other 1); the register p should begin with this value.
//!
//! ```text
//! For example:
//!
//! snd 1
//! snd 2
//! snd p
//! rcv a
//! rcv b
//! rcv c
//! rcv d
//! ```
//!
//! Both programs begin by sending three values to the other. Program 0 sends 1, 2, 0; program 1 sends 1, 2, 1. Then, each program receives a value (both 1) and stores it in a, receives another value (both 2) and stores it in b, and then each receives the program ID of the other program (program 0 receives 1; program 1 receives 0) and stores it in c. Each program now sees a different value in its own copy of register c.
//!
//! Finally, both programs try to rcv a fourth time, but no data is waiting for either of them, and they reach a deadlock. When this happens, both programs terminate.
//!
//! It should be noted that it would be equally valid for the programs to run at different speeds; for example, program 0 might have sent all three values and then stopped at the first rcv before program 1 executed even its first instruction.
//!
//! Once both of your programs have terminated (regardless of what caused them to do so), how many times did program 1 send a value?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    sync::mpsc::{channel, Receiver, Sender},
    thread,
    time::Duration,
};

/// A value can either be a pointer to a register or a number.
#[derive(Clone, Debug, Eq, PartialEq)]
enum Value {
    /// A number value.
    Number(i64),
    /// A registe pointer value.
    Register(String),
}

type CommandData = HashMap<i64, (String, String, Option<Value>)>;

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<CommandData, usize>(AoCYear::AOC2017, AoCDay::AOCD18, setup, find)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<CommandData, usize>(bench, AoCYear::AOC2017, AoCDay::AOCD18, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> CommandData {
    setup_br(reader).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn setup_br<T>(reader: T) -> Result<CommandData>
where
    T: BufRead,
{
    let mut commands = HashMap::new();
    for (idx, line) in valid_lines(reader).enumerate() {
        let _res = commands.insert(i64::try_from(idx)?, parse_command(&line)?);
    }
    Ok(commands)
}

/// Parse a command into (command, register, value)
fn parse_command(command: &str) -> Result<(String, String, Option<Value>)> {
    let token_strs: Vec<&str> = command.split(' ').collect();

    if token_strs.len() == 3 {
        let value = if let Ok(number) = token_strs[2].parse::<i64>() {
            Value::Number(number)
        } else {
            Value::Register(token_strs[2].to_string())
        };
        Ok((
            token_strs[0].to_string(),
            token_strs[1].to_string(),
            Some(value),
        ))
    } else if token_strs.len() == 2 {
        Ok((token_strs[0].to_string(), token_strs[1].to_string(), None))
    } else {
        Err(anyhow!("Invalid command"))
    }
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: CommandData) -> usize {
    find_res(&data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(commands: &CommandData, second_star: bool) -> Result<usize> {
    if second_star {
        Ok(usize::try_from(thread_me()?)?)
    } else {
        let mut register_map: HashMap<String, i64> = HashMap::new();

        initialize_register_map(commands, &mut register_map);

        let mut id = 0;
        loop {
            if id == -1 {
                break;
            }
            let next_command = commands.get(&id).ok_or(anyhow!("invalid command"))?;
            id = run_command((id, next_command), &mut register_map)?;
        }

        let rcv = register_map.get("receive").ok_or(anyhow!("invalid rcv"))?;
        Ok(TryFrom::try_from(*rcv)?)
    }
}

/// Run the threaded version
fn thread_me() -> Result<u32> {
    use std::io::{self, Write};
    let (sender0, receiver0) = channel();
    let (sender1, receiver1) = channel();
    let (sender2, receiver2) = channel();

    let _hn = thread::spawn(move || {
        let mut commands: HashMap<i64, (String, String, Option<Value>)> = HashMap::new();
        let mut register_map: HashMap<String, i64> = HashMap::new();
        initialize(&mut commands, &mut register_map).expect("");
        *register_map.entry("p".to_string()).or_insert(0) = 0;
        run_solution_in_thread(0, &commands, &mut register_map, &sender0, &receiver1).expect("");
    });
    let _hn2 = thread::spawn(move || {
        let mut commands: HashMap<i64, (String, String, Option<Value>)> = HashMap::new();
        let mut register_map: HashMap<String, i64> = HashMap::new();
        initialize(&mut commands, &mut register_map).expect("");
        *register_map.entry("p".to_string()).or_insert(1) = 1;
        if run_solution_in_thread(1, &commands, &mut register_map, &sender1, &receiver0).is_ok() {
            let count = *register_map
                .get("prog1")
                .ok_or(anyhow!("invalid key"))
                .expect("");
            sender2.send(count).expect("");
        } else {
            sender2.send(-1).expect("");
        }
    });

    if receiver2.recv_timeout(Duration::from_millis(5_000)).is_ok() {
        Ok(0)
    } else {
        writeln!(io::stdout())?;
        Ok(1)
    }
}

/// Initialize
fn initialize(
    commands: &mut HashMap<i64, (String, String, Option<Value>)>,
    register_map: &mut HashMap<String, i64>,
) -> Result<()> {
    let reader = BufReader::new(File::open("data/2017/day18/data_file")?);

    for (idx, line_result) in reader.lines().enumerate() {
        let line = &line_result.unwrap_or_else(|_| String::new());
        let _res = commands.insert(i64::try_from(idx)?, parse_command(line)?);
    }
    initialize_register_map(commands, register_map);
    Ok(())
}

/// Run the solution in a thread.
fn run_solution_in_thread(
    prog_id: u8,
    commands: &HashMap<i64, (String, String, Option<Value>)>,
    register_map: &mut HashMap<String, i64>,
    sender: &Sender<i64>,
    receiver: &Receiver<i64>,
) -> Result<()> {
    let mut id = 0;
    loop {
        if id == -1 || id < 0 || id == i64::try_from(commands.len())? {
            break;
        }
        let next_command = commands.get(&id).ok_or(anyhow!("invalid command"))?;
        id = run_command_snd_rcv(prog_id, (id, next_command), register_map, sender, receiver)?;
    }
    Ok(())
}

/// Initialize the register map.
fn initialize_register_map(
    commands: &HashMap<i64, (String, String, Option<Value>)>,
    register_map: &mut HashMap<String, i64>,
) {
    for command in commands.values() {
        let _ = register_map.entry(command.1.clone()).or_insert(0);
    }
}

/// Run a command
#[allow(clippy::too_many_lines)]
fn run_command(
    (id, command): (i64, &(String, String, Option<Value>)),
    register_map: &mut HashMap<String, i64>,
) -> Result<i64> {
    let cmd = &command.0;
    let register = &command.1;
    let value = &command.2;
    let mut last_sound: Option<i64> = None;
    let mut receive: Option<i64> = None;

    match &cmd[..] {
        "set" => {
            let actual_value = match *value {
                Some(Value::Number(x)) => x,
                Some(Value::Register(ref x)) => {
                    *register_map.get(x).ok_or(anyhow!("invalid register"))?
                }
                _ => return Err(anyhow!("Invalid set command")),
            };
            *register_map
                .get_mut(register)
                .ok_or(anyhow!("invalid register"))? = actual_value;
        }
        "add" => {
            let actual_value = match *value {
                Some(Value::Number(x)) => x,
                Some(Value::Register(ref x)) => {
                    *register_map.get(x).ok_or(anyhow!("invalid register"))?
                }
                _ => return Err(anyhow!("Invalid set command")),
            };
            let x = register_map
                .get_mut(register)
                .ok_or(anyhow!("invalid register"))?;
            *x += actual_value;
        }
        "mul" => {
            let actual_value = match *value {
                Some(Value::Number(x)) => x,
                Some(Value::Register(ref x)) => {
                    *register_map.get(x).ok_or(anyhow!("invalid register"))?
                }
                _ => return Err(anyhow!("Invalid set command")),
            };
            let x = register_map
                .get_mut(register)
                .ok_or(anyhow!("invalid register"))?;
            *x *= actual_value;
        }
        "mod" => {
            let actual_value = match *value {
                Some(Value::Number(x)) => x,
                Some(Value::Register(ref x)) => {
                    *register_map.get(x).ok_or(anyhow!("invalid register"))?
                }
                _ => return Err(anyhow!("Invalid set command")),
            };
            let x = register_map
                .get_mut(register)
                .ok_or(anyhow!("invalid register"))?;
            *x %= actual_value;
        }
        "snd" => {
            let snd = register_map
                .get(register)
                .ok_or(anyhow!("invalid register"))?;
            last_sound = Some(*snd);
        }
        "rcv" => {
            let rcv = register_map
                .get(register)
                .ok_or(anyhow!("invalid register"))?;

            if *rcv != 0 {
                let last_sound = register_map
                    .get(&"last_sound".to_string())
                    .ok_or(anyhow!("invalid snd"))?;
                receive = Some(*last_sound);
            }
        }
        "jgz" => {
            let should_jump = if let Ok(val) = register.parse::<i64>() {
                val
            } else {
                *register_map
                    .get(register)
                    .ok_or(anyhow!("invalid register"))?
            };

            if should_jump > 0 {
                let actual_value = match *value {
                    Some(Value::Number(x)) => x,
                    Some(Value::Register(ref x)) => {
                        *register_map.get(x).ok_or(anyhow!("invalid register"))?
                    }
                    _ => return Err(anyhow!("Invalid set command")),
                };
                return Ok(id + actual_value);
            }
        }
        _ => {}
    }

    if let Some(sound) = last_sound {
        let _ = register_map.insert("last_sound".to_string(), sound);
    }

    if let Some(rcv) = receive {
        let _ = register_map.insert("receive".to_string(), rcv);
        Ok(-1)
    } else {
        Ok(id + 1)
    }
}

/// Run a command
fn run_command_snd_rcv(
    prog_id: u8,
    (id, command): (i64, &(String, String, Option<Value>)),
    register_map: &mut HashMap<String, i64>,
    sender: &Sender<i64>,
    receiver: &Receiver<i64>,
) -> Result<i64> {
    use std::io::{self, Write};
    let cmd = &command.0;
    let register = &command.1;
    let value = &command.2;

    match &cmd[..] {
        "set" => {
            let actual_value = match *value {
                Some(Value::Number(x)) => x,
                Some(Value::Register(ref x)) => {
                    *register_map.get(x).ok_or(anyhow!("invalid register"))?
                }
                _ => return Err(anyhow!("Invalid set command")),
            };
            *register_map
                .get_mut(register)
                .ok_or(anyhow!("invalid register"))? = actual_value;
        }
        "add" => {
            let actual_value = match *value {
                Some(Value::Number(x)) => x,
                Some(Value::Register(ref x)) => {
                    *register_map.get(x).ok_or(anyhow!("invalid register"))?
                }
                _ => return Err(anyhow!("Invalid set command")),
            };
            let x = register_map
                .get_mut(register)
                .ok_or(anyhow!("invalid register"))?;
            *x += actual_value;
        }
        "mul" => {
            let actual_value = match *value {
                Some(Value::Number(x)) => x,
                Some(Value::Register(ref x)) => {
                    *register_map.get(x).ok_or(anyhow!("invalid register"))?
                }
                _ => return Err(anyhow!("Invalid set command")),
            };
            let x = register_map
                .get_mut(register)
                .ok_or(anyhow!("invalid register"))?;
            *x *= actual_value;
        }
        "mod" => {
            let actual_value = match *value {
                Some(Value::Number(x)) => x,
                Some(Value::Register(ref x)) => {
                    *register_map.get(x).ok_or(anyhow!("invalid register"))?
                }
                _ => return Err(anyhow!("Invalid set command")),
            };
            let x = register_map
                .get_mut(register)
                .ok_or(anyhow!("invalid register"))?;
            *x %= actual_value;
        }
        "snd" => {
            if prog_id == 1 {
                let counter = register_map.entry("prog1".to_string()).or_insert(0);
                *counter += 1;
                write!(io::stdout(), "\rCount: {}", *counter)?;
            }
            let snd = register_map
                .get(register)
                .ok_or(anyhow!("invalid register"))?;
            sender.send(*snd)?;
        }
        "rcv" => {
            let val = receiver.recv()?;
            *register_map
                .get_mut(register)
                .ok_or(anyhow!("invalid register"))? = val;
        }
        "jgz" => {
            let should_jump = if let Ok(val) = register.parse::<i64>() {
                val
            } else {
                *register_map
                    .get(register)
                    .ok_or(anyhow!("invalid register"))?
            };

            if should_jump > 0 {
                let actual_value = match *value {
                    Some(Value::Number(x)) => x,
                    Some(Value::Register(ref x)) => {
                        *register_map.get(x).ok_or(anyhow!("invalid register"))?
                    }
                    _ => return Err(anyhow!("Invalid set command")),
                };
                return Ok(id + actual_value);
            }
        }
        _ => {}
    }

    Ok(id + 1)
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`] and
///   [`AoCDay`] cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<CommandData, usize>(AoCYear::AOC2017, AoCDay::AOCD18, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<CommandData, usize>(bench, AoCYear::AOC2017, AoCDay::AOCD18, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: CommandData) -> usize {
    find_res(&data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {
    use super::{find, setup_br};
    use anyhow::Result;
    use std::io::Cursor;

    const TEST_1: &str = r"set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";

    #[test]
    fn solution() -> Result<()> {
        let data = setup_br(Cursor::new(TEST_1))?;
        assert_eq!(find(data), 4);
        Ok(())
    }
}

#[cfg(test)]
mod two_star {}
