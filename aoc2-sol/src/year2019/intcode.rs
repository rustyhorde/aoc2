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

use anyhow::{anyhow, Result};
use bnum::types::I256;
use getset::Setters;
use itertools::Itertools;

pub(crate) type IntcodeData = Vec<I256>;

#[derive(Debug, Setters)]
pub(crate) struct Intcode {
    intcodes: Vec<I256>,
    receiver: Receiver<I256>,
    #[getset(set = "pub(crate)")]
    sender_opt: Option<Sender<I256>>,
    relative_base: I256,
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
                relative_base: I256::ZERO,
            },
        )
    }

    pub(crate) fn start(&mut self) -> Result<I256> {
        let mut op_code_idx = 0;
        let mut output = I256::ZERO;
        let (mut modes, mut op_code) = self.parse_op_code(op_code_idx)?;

        while op_code != 99 {
            if op_code == 1 {
                self.handle_add(&mut op_code_idx, modes)?;
            } else if op_code == 2 {
                self.handle_mult(&mut op_code_idx, modes)?;
            } else if op_code == 3 {
                let input = self.receiver.recv()?;
                self.handle_input(&mut op_code_idx, input)?;
            } else if op_code == 4 {
                self.handle_output(&mut op_code_idx, &mut output, modes)?;
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
        }
        Ok(output)
    }

    fn parse_op_code(&mut self, op_code_idx: usize) -> Result<((u8, u8, u8), usize)> {
        let op_code_val = self.read_memory(op_code_idx)?;
        let op_code_digits = self.to_five_digits(op_code_val)?;
        let (modes, op_code_sl) = op_code_digits.split_at(3);
        let op_code_str = op_code_sl.iter().map(ToString::to_string).join("");
        let op_code = op_code_str.parse::<usize>()?;
        Ok(((modes[2], modes[1], modes[0]), op_code))
    }

    #[allow(clippy::unused_self, clippy::same_item_push)]
    fn to_five_digits(&self, mut v: I256) -> Result<Vec<u8>> {
        let mut digits: Vec<u8> = Vec::with_capacity(20);

        while v > I256::ZERO {
            let n = u8::try_from(v % I256::TEN).map_err(|e| anyhow!("{e}"))?;
            v /= I256::TEN;
            digits.push(n);
        }
        for _ in digits.len()..5 {
            digits.push(0);
        }
        digits.reverse();
        Ok(digits)
    }

    fn handle_add(&mut self, idx: &mut usize, modes: (u8, u8, u8)) -> Result<()> {
        let idx_1 = self.read_memory(*idx + 1)?;
        let idx_2 = self.read_memory(*idx + 2)?;
        let idx_3 = self.read_memory(*idx + 3)?;

        let addend_1 = self.handle_mode_0(idx_1, modes)?;
        let addend_2 = self.handle_mode_1(idx_2, modes)?;

        self.write_memory(to_u(idx_3)?, addend_1 + addend_2)?;

        *idx += 4;
        Ok(())
    }

    fn handle_mult(&mut self, idx: &mut usize, modes: (u8, u8, u8)) -> Result<()> {
        let idx_1 = self.read_memory(*idx + 1)?;
        let idx_2 = self.read_memory(*idx + 2)?;
        let idx_3 = self.read_memory(*idx + 3)?;

        let factor_1 = self.handle_mode_0(idx_1, modes)?;
        let factor_2 = self.handle_mode_1(idx_2, modes)?;

        self.write_memory(to_u(idx_3)?, factor_1 * factor_2)?;
        *idx += 4;
        Ok(())
    }

    fn handle_input(&mut self, idx: &mut usize, input: I256) -> Result<()> {
        let idx_1 = self.read_memory(*idx + 1)?;
        self.write_memory(to_u(idx_1)?, input)?;
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
        *output = if modes.0 == 0 {
            self.read_memory(to_u(idx_1)?)?
        } else if modes.0 == 1 {
            self.read_memory(*idx + 1)?
        } else {
            self.read_memory(to_u(self.relative_base + idx_1)?)?
        };
        *idx += 2;
        Ok(())
    }

    fn handle_jump_if_true(&mut self, idx: &mut usize, modes: (u8, u8, u8)) -> Result<()> {
        let idx_1 = self.read_memory(*idx + 1)?;
        let check_val = self.handle_mode_0(idx_1, modes)?;
        if check_val == I256::ZERO {
            *idx += 3;
        } else {
            let next = self.read_memory(*idx + 2)?;
            let idx_2 = to_u(next)?;
            *idx = if modes.1 == 0 {
                to_u(self.read_memory(idx_2)?)?
            } else if modes.1 == 1 {
                idx_2
            } else {
                to_u(self.read_memory(to_u(self.relative_base + next)?)?)?
            };
        }
        Ok(())
    }

    fn handle_jump_if_false(&mut self, idx: &mut usize, modes: (u8, u8, u8)) -> Result<()> {
        let idx_1 = self.read_memory(*idx + 1)?;
        let check_val = self.handle_mode_0(idx_1, modes)?;
        if check_val == I256::ZERO {
            let next = self.read_memory(*idx + 2)?;
            let idx_2 = to_u(next)?;
            *idx = if modes.1 == 0 {
                to_u(self.read_memory(idx_2)?)?
            } else if modes.1 == 1 {
                idx_2
            } else {
                to_u(self.read_memory(to_u(self.relative_base + next)?)?)?
            };
        } else {
            *idx += 3;
        }
        Ok(())
    }

    fn handle_less_than(&mut self, idx: &mut usize, modes: (u8, u8, u8)) -> Result<()> {
        let idx_1 = self.read_memory(*idx + 1)?;
        let idx_2 = self.read_memory(*idx + 2)?;
        let idx_3 = to_u(self.read_memory(*idx + 3)?)?;

        let val_1 = self.handle_mode_0(idx_1, modes)?;
        let val_2 = self.handle_mode_1(idx_2, modes)?;

        if val_1 < val_2 {
            self.write_memory(idx_3, I256::ONE)?;
        } else {
            self.write_memory(idx_3, I256::ZERO)?;
        }
        *idx += 4;
        Ok(())
    }

    fn handle_equals(&mut self, idx: &mut usize, modes: (u8, u8, u8)) -> Result<()> {
        let idx_1 = self.read_memory(*idx + 1)?;
        let idx_2 = self.read_memory(*idx + 2)?;
        let idx_3 = to_u(self.read_memory(*idx + 3)?)?;

        let val_1 = self.handle_mode_0(idx_1, modes)?;
        let val_2 = self.handle_mode_1(idx_2, modes)?;

        if val_1 == val_2 {
            self.write_memory(idx_3, I256::ONE)?;
        } else {
            self.write_memory(idx_3, I256::ZERO)?;
        }
        *idx += 4;
        Ok(())
    }

    fn handle_arb(&mut self, idx: &mut usize, modes: (u8, u8, u8)) -> Result<()> {
        let idx_1 = self.read_memory(*idx + 1)?;
        let val_1 = self.handle_mode_0(idx_1, modes)?;
        self.relative_base += val_1;
        *idx += 2;
        Ok(())
    }

    fn handle_mode_0(&mut self, idx: I256, modes: (u8, u8, u8)) -> Result<I256> {
        Ok(if modes.0 == 0 {
            self.read_memory(to_u(idx)?)?
        } else if modes.0 == 1 {
            idx
        } else {
            self.read_memory(to_u(self.relative_base + idx)?)?
        })
    }

    fn handle_mode_1(&mut self, idx: I256, modes: (u8, u8, u8)) -> Result<I256> {
        Ok(if modes.1 == 0 {
            self.read_memory(to_u(idx)?)?
        } else if modes.1 == 1 {
            idx
        } else {
            self.read_memory(to_u(self.relative_base + idx)?)?
        })
    }

    fn read_memory(&mut self, idx: usize) -> Result<I256> {
        self.allocate_memory(idx)?;
        Ok(self.intcodes[idx])
    }

    fn write_memory(&mut self, idx: usize, val: I256) -> Result<()> {
        self.allocate_memory(idx)?;
        self.intcodes[idx] = val;
        Ok(())
    }

    fn allocate_memory(&mut self, idx: usize) -> Result<()> {
        let len = self.intcodes.len();
        if idx >= len {
            // allocate more memory
            let new_size = idx - len + 1;
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
