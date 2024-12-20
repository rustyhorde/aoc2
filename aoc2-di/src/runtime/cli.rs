// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use clap::{ArgAction::Count, Parser};
use config::{ConfigError, Map, Source, Value, ValueKind};
use getset::Getters;

use crate::config::defaults::ConfigDefaults;

const DEFAULT_YEAR: &str = "2020";

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
    #[arg(name = "wait", short = 'w', long, help = "Wait for the next day to start a download", conflicts_with_all = &["start_day", "end_day", "year"])]
    wait: bool,
    #[arg(name = "year", short = 'y', long, help = "Specify the year you wish to work with", default_value_t = DEFAULT_YEAR.to_string(), conflicts_with = "wait")]
    year: String,
    #[arg(
        name = "start_day",
        short = 's',
        long,
        help = "Specify the start day",
        conflicts_with = "wait"
    )]
    start_day: Option<String>,
    #[arg(
        name = "end_day",
        short = 'e',
        long,
        help = "Specify the end day",
        conflicts_with = "wait"
    )]
    end_day: Option<String>,
    /// Config file path
    #[clap(short, long, help = "Specify a path to the config file")]
    config_path: Option<String>,
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
        let _old = map.insert(
            "wait".to_string(),
            Value::new(Some(&origin), ValueKind::Boolean(self.wait)),
        );
        let _old = map.insert(
            "year".to_string(),
            Value::new(Some(&origin), ValueKind::String(self.year.clone())),
        );
        if let Some(start_day) = &self.start_day {
            let _old = map.insert(
                "start_day".to_string(),
                Value::new(Some(&origin), ValueKind::String(start_day.clone())),
            );
        }
        if let Some(end_day) = &self.end_day {
            let _old = map.insert(
                "end_day".to_string(),
                Value::new(Some(&origin), ValueKind::String(end_day.clone())),
            );
        }
        if let Some(config_path) = &self.config_path() {
            let _old = map.insert(
                "config_path".to_string(),
                Value::new(Some(&origin), ValueKind::String(config_path.clone())),
            );
        }
        Ok(map)
    }
}

impl ConfigDefaults for Args {
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
