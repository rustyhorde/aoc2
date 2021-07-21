mod day01;

use crate::constants::AoCDay;
use anyhow::{anyhow, Result};

pub(crate) fn match_fn<T>(day: AoCDay, second_star: bool) -> Result<fn(T) -> Result<u32>>
where
    T: Iterator<Item = std::io::Result<String>> + Send,
{
    match day {
        AoCDay::AOCD01 => {
            if second_star {
                Ok(day01::part_2)
            } else {
                Ok(day01::part_1)
            }
        }
        _ => Err(anyhow!("not implemented")),
    }
}
