// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `aoc` Command Line Interface

use aoc2_sol::constants::{
    DAY_1, DAY_2, DAY_3, DAY_4, DAY_5, DAY_6, DAY_7, DAY_8, DAY_9, DAY_10, DAY_11, DAY_12, DAY_13,
    DAY_14, DAY_15, DAY_16, DAY_17, DAY_18, DAY_19, DAY_20, DAY_21, DAY_22, DAY_23, DAY_24, DAY_25,
};
use clap::{
    ArgAction::Count, Args as ClapArgs, Error, FromArgMatches, Parser, Subcommand, error::ErrorKind,
};
use config::{ConfigError, Map, Source, Value, ValueKind};
use getset::Getters;

use super::config::PathDefaults;

pub(crate) const DEFAULT_YEAR: &str = "2021";

#[derive(Clone, Debug, Getters, Parser)]
#[command(author, version, about = "Run Advent of Code daily problems", long_about = None)]
#[getset(get = "pub(crate)")]
pub(crate) struct Args {
    /// Set logging verbosity.  More v's, more verbose.
    #[clap(
        short,
        long,
        action = Count,
        help = "Turn up logging verbosity (multiple will turn it up more)",
        conflicts_with = "quiet",
    )]
    verbose: u8,
    /// Set logging quietness.  More q's, more quiet.
    #[clap(
        short,
        long,
        action = Count,
        help = "Turn down logging verbosity (multiple will turn it down more)",
        conflicts_with = "verbose",
    )]
    quiet: u8,
    #[clap(short, long, help = "Run benchmark over the given amount of runs")]
    bench: Option<u16>,
    #[arg(name = "year", short = 'y', long, help = "Specify the year you wish to work with", default_value_t = DEFAULT_YEAR.to_string())]
    year: String,
    #[arg(name = "time", short = 't', long, help = "Generate benchmark time")]
    time: bool,
    #[clap(short, long, help = "Specify a path to the config file")]
    config_path: Option<String>,
    #[command(subcommand)]
    command: Command,
}

impl Source for Args {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new((*self).clone())
    }

    fn collect(&self) -> Result<Map<String, Value>, ConfigError> {
        let mut map = Map::new();
        let origin = String::from("command line");
        let _old = map.insert(
            "verbose".to_string(),
            Value::new(Some(&origin), ValueKind::U64(u8::into(self.verbose))),
        );
        let _old = map.insert(
            "quiet".to_string(),
            Value::new(Some(&origin), ValueKind::U64(u8::into(self.quiet))),
        );
        if let Some(bench) = self.bench {
            let _old = map.insert(
                "bench".to_string(),
                Value::new(Some(&origin), ValueKind::U64(u16::into(bench))),
            );
        }
        let _old = map.insert(
            "year".to_string(),
            Value::new(Some(&origin), ValueKind::String(self.year.clone())),
        );
        let _old = map.insert(
            "time".to_string(),
            Value::new(Some(&origin), ValueKind::Boolean(self.time)),
        );
        if let Some(config_path) = &self.config_path() {
            let _old = map.insert(
                "config_path".to_string(),
                Value::new(Some(&origin), ValueKind::String(config_path.clone())),
            );
        }
        Ok(map)
    }
}

impl PathDefaults for Args {
    fn env_prefix(&self) -> String {
        env!("CARGO_PKG_NAME").to_ascii_uppercase()
    }

    fn config_file_path(&self) -> Option<String> {
        self.config_path.clone()
    }

    fn default_file_path(&self) -> String {
        env!("CARGO_PKG_NAME").to_string()
    }

    fn default_file_name(&self) -> String {
        env!("CARGO_PKG_NAME").to_string()
    }
}

#[derive(Clone, Debug)]
pub(crate) enum Command {
    Day01(AoC2Subcommand),
    Day02(AoC2Subcommand),
    Day03(AoC2Subcommand),
    Day04(AoC2Subcommand),
    Day05(AoC2Subcommand),
    Day06(AoC2Subcommand),
    Day07(AoC2Subcommand),
    Day08(AoC2Subcommand),
    Day09(AoC2Subcommand),
    Day10(AoC2Subcommand),
    Day11(AoC2Subcommand),
    Day12(AoC2Subcommand),
    Day13(AoC2Subcommand),
    Day14(AoC2Subcommand),
    Day15(AoC2Subcommand),
    Day16(AoC2Subcommand),
    Day17(AoC2Subcommand),
    Day18(AoC2Subcommand),
    Day19(AoC2Subcommand),
    Day20(AoC2Subcommand),
    Day21(AoC2Subcommand),
    Day22(AoC2Subcommand),
    Day23(AoC2Subcommand),
    Day24(AoC2Subcommand),
    Day25(AoC2Subcommand),
}

impl FromArgMatches for Command {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, Error> {
        match matches.subcommand() {
            Some((DAY_1, args)) => Ok(Self::Day01(AoC2Subcommand::from_arg_matches(args)?)),
            Some((DAY_2, args)) => Ok(Self::Day02(AoC2Subcommand::from_arg_matches(args)?)),
            Some((DAY_3, args)) => Ok(Self::Day03(AoC2Subcommand::from_arg_matches(args)?)),
            Some((DAY_4, args)) => Ok(Self::Day04(AoC2Subcommand::from_arg_matches(args)?)),
            Some((DAY_5, args)) => Ok(Self::Day05(AoC2Subcommand::from_arg_matches(args)?)),
            Some((DAY_6, args)) => Ok(Self::Day06(AoC2Subcommand::from_arg_matches(args)?)),
            Some((DAY_7, args)) => Ok(Self::Day07(AoC2Subcommand::from_arg_matches(args)?)),
            Some((DAY_8, args)) => Ok(Self::Day08(AoC2Subcommand::from_arg_matches(args)?)),
            Some((DAY_9, args)) => Ok(Self::Day09(AoC2Subcommand::from_arg_matches(args)?)),
            Some((DAY_10, args)) => Ok(Self::Day10(AoC2Subcommand::from_arg_matches(args)?)),
            Some((DAY_11, args)) => Ok(Self::Day11(AoC2Subcommand::from_arg_matches(args)?)),
            Some((DAY_12, args)) => Ok(Self::Day12(AoC2Subcommand::from_arg_matches(args)?)),
            Some((DAY_13, args)) => Ok(Self::Day13(AoC2Subcommand::from_arg_matches(args)?)),
            Some((DAY_14, args)) => Ok(Self::Day14(AoC2Subcommand::from_arg_matches(args)?)),
            Some((DAY_15, args)) => Ok(Self::Day15(AoC2Subcommand::from_arg_matches(args)?)),
            Some((DAY_16, args)) => Ok(Self::Day16(AoC2Subcommand::from_arg_matches(args)?)),
            Some((DAY_17, args)) => Ok(Self::Day17(AoC2Subcommand::from_arg_matches(args)?)),
            Some((DAY_18, args)) => Ok(Self::Day18(AoC2Subcommand::from_arg_matches(args)?)),
            Some((DAY_19, args)) => Ok(Self::Day19(AoC2Subcommand::from_arg_matches(args)?)),
            Some((DAY_20, args)) => Ok(Self::Day20(AoC2Subcommand::from_arg_matches(args)?)),
            Some((DAY_21, args)) => Ok(Self::Day21(AoC2Subcommand::from_arg_matches(args)?)),
            Some((DAY_22, args)) => Ok(Self::Day22(AoC2Subcommand::from_arg_matches(args)?)),
            Some((DAY_23, args)) => Ok(Self::Day23(AoC2Subcommand::from_arg_matches(args)?)),
            Some((DAY_24, args)) => Ok(Self::Day24(AoC2Subcommand::from_arg_matches(args)?)),
            Some((DAY_25, args)) => Ok(Self::Day25(AoC2Subcommand::from_arg_matches(args)?)),
            Some((_, _)) => Err(Error::raw(
                ErrorKind::InvalidSubcommand,
                "Valid subcommands are `day01` through `day25`",
            )),
            None => Err(Error::raw(
                ErrorKind::MissingSubcommand,
                "Valid subcommands are `day01` through `day25`",
            )),
        }
    }

    fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) -> Result<(), Error> {
        match matches.subcommand() {
            Some((DAY_1, args)) => *self = Self::Day01(AoC2Subcommand::from_arg_matches(args)?),
            Some((DAY_2, args)) => *self = Self::Day02(AoC2Subcommand::from_arg_matches(args)?),
            Some((DAY_3, args)) => *self = Self::Day03(AoC2Subcommand::from_arg_matches(args)?),
            Some((DAY_4, args)) => *self = Self::Day04(AoC2Subcommand::from_arg_matches(args)?),
            Some((DAY_5, args)) => *self = Self::Day05(AoC2Subcommand::from_arg_matches(args)?),
            Some((DAY_6, args)) => *self = Self::Day06(AoC2Subcommand::from_arg_matches(args)?),
            Some((DAY_7, args)) => *self = Self::Day07(AoC2Subcommand::from_arg_matches(args)?),
            Some((DAY_8, args)) => *self = Self::Day08(AoC2Subcommand::from_arg_matches(args)?),
            Some((DAY_9, args)) => *self = Self::Day09(AoC2Subcommand::from_arg_matches(args)?),
            Some((DAY_10, args)) => *self = Self::Day10(AoC2Subcommand::from_arg_matches(args)?),
            Some((DAY_11, args)) => *self = Self::Day11(AoC2Subcommand::from_arg_matches(args)?),
            Some((DAY_12, args)) => *self = Self::Day12(AoC2Subcommand::from_arg_matches(args)?),
            Some((DAY_13, args)) => *self = Self::Day13(AoC2Subcommand::from_arg_matches(args)?),
            Some((DAY_14, args)) => *self = Self::Day14(AoC2Subcommand::from_arg_matches(args)?),
            Some((DAY_15, args)) => *self = Self::Day15(AoC2Subcommand::from_arg_matches(args)?),
            Some((DAY_16, args)) => *self = Self::Day16(AoC2Subcommand::from_arg_matches(args)?),
            Some((DAY_17, args)) => *self = Self::Day17(AoC2Subcommand::from_arg_matches(args)?),
            Some((DAY_18, args)) => *self = Self::Day18(AoC2Subcommand::from_arg_matches(args)?),
            Some((DAY_19, args)) => *self = Self::Day19(AoC2Subcommand::from_arg_matches(args)?),
            Some((DAY_20, args)) => *self = Self::Day20(AoC2Subcommand::from_arg_matches(args)?),
            Some((DAY_21, args)) => *self = Self::Day21(AoC2Subcommand::from_arg_matches(args)?),
            Some((DAY_22, args)) => *self = Self::Day22(AoC2Subcommand::from_arg_matches(args)?),
            Some((DAY_23, args)) => *self = Self::Day23(AoC2Subcommand::from_arg_matches(args)?),
            Some((DAY_24, args)) => *self = Self::Day24(AoC2Subcommand::from_arg_matches(args)?),
            Some((DAY_25, args)) => *self = Self::Day25(AoC2Subcommand::from_arg_matches(args)?),
            Some((_, _)) => {
                return Err(Error::raw(
                    ErrorKind::InvalidSubcommand,
                    "Valid subcommands are `add` and `remove`",
                ));
            }
            None => (),
        };
        Ok(())
    }
}

impl Subcommand for Command {
    fn augment_subcommands(cmd: clap::Command) -> clap::Command {
        cmd.subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_1)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_2)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_3)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_4)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_5)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_6)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_7)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_8)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_9)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_10)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_11)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_12)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_13)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_14)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_15)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_16)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_17)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_18)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_19)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_20)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_21)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_22)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_23)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_24)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_25)))
            .subcommand_required(true)
    }

    fn augment_subcommands_for_update(cmd: clap::Command) -> clap::Command {
        cmd.subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_1)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_2)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_3)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_4)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_5)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_6)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_7)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_8)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_9)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_10)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_11)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_12)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_13)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_14)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_15)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_16)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_17)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_18)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_19)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_20)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_21)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_22)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_23)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_24)))
            .subcommand(AoC2Subcommand::augment_args(clap::Command::new(DAY_25)))
            .subcommand_required(true)
    }

    fn has_subcommand(name: &str) -> bool {
        matches!(
            name,
            DAY_1
                | DAY_2
                | DAY_3
                | DAY_4
                | DAY_5
                | DAY_6
                | DAY_7
                | DAY_8
                | DAY_9
                | DAY_10
                | DAY_11
                | DAY_12
                | DAY_13
                | DAY_14
                | DAY_15
                | DAY_16
                | DAY_17
                | DAY_18
                | DAY_19
                | DAY_20
                | DAY_21
                | DAY_22
                | DAY_23
                | DAY_24
                | DAY_25
        )
    }
}

#[derive(Clone, Debug, Getters, Parser)]
#[getset(get = "pub(crate)")]
pub(crate) struct AoC2Subcommand {
    #[arg(name = "file", short, long, default_value_t = String::from("data_file"))]
    file: String,
    #[arg(
        name = "second",
        short,
        long,
        help = "Run the algorithm to calculate the value for the 2nd star",
        conflicts_with = "both"
    )]
    second: bool,
    #[arg(
        name = "both",
        short,
        long,
        help = "Run the algorithm to calculate the value for both stars",
        conflicts_with = "second"
    )]
    both: bool,
}
