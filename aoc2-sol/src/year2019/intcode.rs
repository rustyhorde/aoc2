// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::{
    fmt::{self, Formatter},
    sync::mpsc::{channel, Receiver, Sender},
};

use crate::error::Error;
use anyhow::{anyhow, Result};
use bnum::types::I256;
use getset::Setters;
#[cfg(feature = "intcode_debug")]
use serde::{Deserialize, Serialize};

pub(crate) type IntcodeData = Vec<I256>;

#[cfg(feature = "intcode_debug")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) enum DebugMessage {
    ParsedOpCode { op_code: u8, modes: (u8, u8, u8) },
    Input(I256),
    Output(I256),
    Allocate(usize),
    ReadMem(usize, Vec<I256>),
    WriteMem(usize, I256, Vec<I256>),
    Positional(I256),
    Immediate(I256),
    Relative(I256, I256),
    Arb(I256, I256),
    Equal(I256, I256),
    LessThan(I256, I256),
    JumpIfFalse(bool, usize),
    JumpIfTrue(bool, usize),
    Multiply(I256, I256),
    Add(I256, I256),
}

#[cfg(feature = "intcode_debug")]
impl DebugMessage {
    fn npo(op_code: u8, modes: (u8, u8, u8)) -> Self {
        Self::ParsedOpCode { op_code, modes }
    }
}

#[derive(Debug, Setters)]
pub(crate) struct Intcode {
    intcodes: Vec<I256>,
    receiver: Receiver<I256>,
    #[getset(set = "pub(crate)")]
    sender_opt: Option<Sender<I256>>,
    #[cfg(feature = "intcode_debug")]
    #[getset(set = "pub(crate)")]
    debug_opt: Option<Sender<Vec<u8>>>,
    relative_base: I256,
    #[cfg(feature = "intcode_debug")]
    #[getset(set = "pub(crate)")]
    debug: bool,
}

impl Intcode {
    pub(crate) fn new(intcodes: Vec<I256>) -> (Sender<I256>, Self) {
        let (sender, receiver) = channel::<I256>();
        (
            sender,
            Self {
                intcodes,
                receiver,
                sender_opt: None,
                #[cfg(feature = "intcode_debug")]
                debug_opt: None,
                relative_base: I256::ZERO,
                #[cfg(feature = "intcode_debug")]
                debug: false,
            },
        )
    }

    #[cfg(feature = "intcode_debug")]
    fn send_debug_message(&self, message: &DebugMessage) {
        if self.debug {
            if let Ok(encoded) = bincode::serialize(message) {
                if let Some(s) = &self.debug_opt {
                    s.send(encoded).unwrap_or_default();
                }
            }
        }
    }

    pub(crate) fn start(&mut self) -> Result<I256> {
        let mut op_code_idx = 0;
        let mut output = I256::ZERO;
        let (mut modes, mut op_code) = self.parse_op_code(op_code_idx)?;
        #[cfg(feature = "intcode_debug")]
        self.send_debug_message(&DebugMessage::npo(op_code, modes));

        while op_code != 99 {
            if op_code == 1 {
                self.handle_add(&mut op_code_idx, modes)?;
            } else if op_code == 2 {
                self.handle_mult(&mut op_code_idx, modes)?;
            } else if op_code == 3 {
                let input = self.receiver.recv()?;
                #[cfg(feature = "intcode_debug")]
                self.send_debug_message(&DebugMessage::Input(input));
                self.handle_input(&mut op_code_idx, modes, input)?;
            } else if op_code == 4 {
                self.handle_output(&mut op_code_idx, &mut output, modes)?;
                #[cfg(feature = "intcode_debug")]
                self.send_debug_message(&DebugMessage::Output(output));
                if let Some(sender) = &self.sender_opt {
                    match sender.send(output) {
                        Ok(()) => {}
                        Err(_e) => {
                            // Do nothing let the machine stop
                        }
                    }
                }
            } else if op_code == 5 {
                self.handle_jump_if_true(&mut op_code_idx, modes)?;
            } else if op_code == 6 {
                self.handle_jump_if_false(&mut op_code_idx, modes)?;
            } else if op_code == 7 {
                self.handle_less_than(&mut op_code_idx, modes)?;
            } else if op_code == 8 {
                self.handle_equals(&mut op_code_idx, modes)?;
            } else if op_code == 9 {
                self.handle_arb(&mut op_code_idx, modes)?;
            } else {
                break;
            }
            (modes, op_code) = self.parse_op_code(op_code_idx)?;
            #[cfg(feature = "intcode_debug")]
            self.send_debug_message(&DebugMessage::npo(op_code, modes));
        }
        Ok(output)
    }

    fn parse_op_code(&mut self, op_code_idx: usize) -> Result<((u8, u8, u8), u8)> {
        let op_code_val = self.read_memory(op_code_idx)?;
        let op_code_digits = to_op_code_vec(op_code_val)?;
        Ok((
            (op_code_digits[1], op_code_digits[2], op_code_digits[3]),
            op_code_digits[0],
        ))
    }

    fn handle_add(&mut self, idx: &mut usize, modes: (u8, u8, u8)) -> Result<()> {
        let idx_1 = self.read_memory(*idx + 1)?;
        let idx_2 = self.read_memory(*idx + 2)?;
        let idx_3 = self.read_memory(*idx + 3)?;

        let addend_1 = self.handle_read(idx_1, modes.0)?;
        let addend_2 = self.handle_read(idx_2, modes.1)?;

        #[cfg(feature = "intcode_debug")]
        self.send_debug_message(&DebugMessage::Add(addend_1, addend_2));

        self.handle_write(idx_3, modes.2, addend_1 + addend_2)?;

        *idx += 4;
        Ok(())
    }

    fn handle_mult(&mut self, idx: &mut usize, modes: (u8, u8, u8)) -> Result<()> {
        let idx_1 = self.read_memory(*idx + 1)?;
        let idx_2 = self.read_memory(*idx + 2)?;
        let idx_3 = self.read_memory(*idx + 3)?;

        let factor_1 = self.handle_read(idx_1, modes.0)?;
        let factor_2 = self.handle_read(idx_2, modes.1)?;

        #[cfg(feature = "intcode_debug")]
        self.send_debug_message(&DebugMessage::Multiply(factor_1, factor_2));

        self.handle_write(idx_3, modes.2, factor_1 * factor_2)?;

        *idx += 4;
        Ok(())
    }

    fn handle_input(&mut self, idx: &mut usize, modes: (u8, u8, u8), input: I256) -> Result<()> {
        let idx_1 = self.read_memory(*idx + 1)?;
        self.handle_write(idx_1, modes.0, input)?;
        *idx += 2;
        Ok(())
    }

    fn handle_output(
        &mut self,
        idx: &mut usize,
        output: &mut I256,
        modes: (u8, u8, u8),
    ) -> Result<()> {
        let idx_1 = self.read_memory(*idx + 1)?;
        *output = self.handle_read(idx_1, modes.0)?;
        *idx += 2;
        Ok(())
    }

    fn handle_jump_if_true(&mut self, idx: &mut usize, modes: (u8, u8, u8)) -> Result<()> {
        let idx_1 = self.read_memory(*idx + 1)?;
        let check_val = self.handle_read(idx_1, modes.0)?;
        cfg_if::cfg_if! {
            if #[cfg(feature = "intcode_debug")] {
                let (jump, n_idx) = if check_val == I256::ZERO {
                    (false, *idx + 3)
                } else {
                    let next = self.read_memory(*idx + 2)?;
                    (true, to_u(self.handle_read(next, modes.1)?)?)
                };
            } else {
                let n_idx = if check_val == I256::ZERO {
                    *idx + 3
                } else {
                    let next = self.read_memory(*idx + 2)?;
                    to_u(self.handle_read(next, modes.1)?)?
                };
            }
        }

        #[cfg(feature = "intcode_debug")]
        self.send_debug_message(&DebugMessage::JumpIfTrue(jump, n_idx));
        *idx = n_idx;
        Ok(())
    }

    fn handle_jump_if_false(&mut self, idx: &mut usize, modes: (u8, u8, u8)) -> Result<()> {
        let idx_1 = self.read_memory(*idx + 1)?;
        let check_val = self.handle_read(idx_1, modes.0)?;
        cfg_if::cfg_if! {
            if #[cfg(feature = "intcode_debug")] {
                let (jump, n_idx) = if check_val == I256::ZERO {
                    let next = self.read_memory(*idx + 2)?;
                    (true, to_u(self.handle_read(next, modes.1)?)?)
                } else {
                    (false, *idx + 3)
                };
            } else {
                let n_idx = if check_val == I256::ZERO {
                    let next = self.read_memory(*idx + 2)?;
                    to_u(self.handle_read(next, modes.1)?)?
                } else {
                    *idx + 3
                };
            }
        }
        #[cfg(feature = "intcode_debug")]
        self.send_debug_message(&DebugMessage::JumpIfFalse(jump, n_idx));
        *idx = n_idx;
        Ok(())
    }

    fn handle_less_than(&mut self, idx: &mut usize, modes: (u8, u8, u8)) -> Result<()> {
        let idx_1 = self.read_memory(*idx + 1)?;
        let idx_2 = self.read_memory(*idx + 2)?;
        let idx_3 = self.read_memory(*idx + 3)?;

        let val_1 = self.handle_read(idx_1, modes.0)?;
        let val_2 = self.handle_read(idx_2, modes.1)?;

        let res = if val_1 < val_2 { I256::ONE } else { I256::ZERO };
        #[cfg(feature = "intcode_debug")]
        self.send_debug_message(&DebugMessage::LessThan(val_1, val_2));
        self.handle_write(idx_3, modes.2, res)?;
        *idx += 4;
        Ok(())
    }

    fn handle_equals(&mut self, idx: &mut usize, modes: (u8, u8, u8)) -> Result<()> {
        let idx_1 = self.read_memory(*idx + 1)?;
        let idx_2 = self.read_memory(*idx + 2)?;
        let idx_3 = self.read_memory(*idx + 3)?;

        let val_1 = self.handle_read(idx_1, modes.0)?;
        let val_2 = self.handle_read(idx_2, modes.1)?;

        let res = if val_1 == val_2 {
            I256::ONE
        } else {
            I256::ZERO
        };
        #[cfg(feature = "intcode_debug")]
        self.send_debug_message(&DebugMessage::Equal(val_1, val_2));
        self.handle_write(idx_3, modes.2, res)?;
        *idx += 4;
        Ok(())
    }

    fn handle_arb(&mut self, idx: &mut usize, modes: (u8, u8, u8)) -> Result<()> {
        let idx_1 = self.read_memory(*idx + 1)?;
        let val_1 = self.handle_read(idx_1, modes.0)?;
        #[cfg(feature = "intcode_debug")]
        self.send_debug_message(&DebugMessage::Arb(
            self.relative_base,
            self.relative_base + val_1,
        ));
        self.relative_base += val_1;
        *idx += 2;
        Ok(())
    }

    fn handle_read(&mut self, idx: I256, mode: u8) -> Result<I256> {
        Ok(if mode == 0 {
            #[cfg(feature = "intcode_debug")]
            self.send_debug_message(&DebugMessage::Positional(idx));
            self.read_memory(to_u(idx)?)?
        } else if mode == 1 {
            #[cfg(feature = "intcode_debug")]
            self.send_debug_message(&DebugMessage::Immediate(idx));
            idx
        } else {
            #[cfg(feature = "intcode_debug")]
            self.send_debug_message(&DebugMessage::Relative(self.relative_base, idx));
            self.read_memory(to_u(self.relative_base + idx)?)?
        })
    }

    fn handle_write(&mut self, idx: I256, mode: u8, input: I256) -> Result<()> {
        if mode == 0 {
            #[cfg(feature = "intcode_debug")]
            self.send_debug_message(&DebugMessage::Positional(idx));
            self.write_memory(to_u(idx)?, input)?;
        } else if mode == 1 {
            return Err(anyhow!("immediate mode invalid for a write!"));
        } else {
            #[cfg(feature = "intcode_debug")]
            self.send_debug_message(&DebugMessage::Relative(self.relative_base, idx));
            self.write_memory(to_u(self.relative_base + idx)?, input)?;
        }
        Ok(())
    }

    fn read_memory(&mut self, idx: usize) -> Result<I256> {
        self.allocate_memory(idx)?;
        #[cfg(feature = "intcode_debug")]
        self.send_debug_message(&DebugMessage::ReadMem(idx, self.intcodes.clone()));
        Ok(self.intcodes[idx])
    }

    fn write_memory(&mut self, idx: usize, val: I256) -> Result<()> {
        self.allocate_memory(idx)?;
        #[cfg(feature = "intcode_debug")]
        self.send_debug_message(&DebugMessage::WriteMem(idx, val, self.intcodes.clone()));
        self.intcodes[idx] = val;
        Ok(())
    }

    fn allocate_memory(&mut self, idx: usize) -> Result<()> {
        let len = self.intcodes.len();
        if idx >= len {
            // allocate more memory
            let new_size = idx - len + 1;
            #[cfg(feature = "intcode_debug")]
            self.send_debug_message(&DebugMessage::Allocate(new_size));
            self.intcodes.try_reserve(new_size)?;
            self.intcodes.extend((len..=idx).map(|_| I256::ZERO));
        }
        Ok(())
    }
}

impl fmt::Display for Intcode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let len = self.intcodes.len();
        for (idx, opcode) in self.intcodes.iter().enumerate() {
            write!(f, "{opcode}")?;

            if idx < len - 1 {
                write!(f, ",")?;
            }
        }
        Ok(())
    }
}

fn to_u(v: I256) -> Result<usize> {
    usize::try_from(v).map_err(|e| anyhow!("{e}"))
}

pub(crate) fn as_isize(x: &str) -> Option<I256> {
    x.parse::<I256>().ok()
}

fn to_op_code_vec(mut v: I256) -> Result<[u8; 4]> {
    let mut digits = [0; 4];

    // 21108 becomes [8, 1, 1, 2], 20110 becomes [10, 1, 0, 2]
    let mut i = 0;
    while v > I256::ZERO {
        let factor = if i == 0 { I256::from(100) } else { I256::TEN };
        let n = v % factor;
        v /= factor;
        digits[i] = u8::try_from(n).map_err(|_| Error::ParseInt)?;
        i += 1;
    }
    Ok(digits)
}

#[cfg(test)]
mod test {
    use super::to_op_code_vec;
    use anyhow::Result;
    use bnum::types::I256;

    #[test]
    fn to_five_digits_works() -> Result<()> {
        assert_eq!(to_op_code_vec(I256::from(4))?, [4, 0, 0, 0]);
        assert_eq!(to_op_code_vec(I256::from(99))?, [99, 0, 0, 0]);
        assert_eq!(to_op_code_vec(I256::from(21108))?, [8, 1, 1, 2]);
        assert_eq!(to_op_code_vec(I256::from(20110))?, [10, 1, 0, 2]);
        Ok(())
    }
}
