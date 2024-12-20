// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::{borrow::Cow, ops::RangeInclusive, path::PathBuf};

use self::defaults::ConfigDefaults;
use anyhow::{Context, Result};
use config::{Config, Environment, File, FileFormat, Source};
use getset::{CopyGetters, Getters};
use serde::{Deserialize, Serialize};
use tracing_subscriber_init::TracingConfig;

use crate::{
    constants::CONFIG_BUILD_ERROR,
    error::Error::{ConfigDeserialize, ConfigDir},
};

pub(crate) mod defaults;

/// Tracing configuration
#[allow(clippy::struct_excessive_bools, clippy::struct_field_names)]
#[derive(Clone, Copy, CopyGetters, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[getset(get_copy = "pub")]
pub(crate) struct Tracing {
    /// Should we trace the event target
    with_target: bool,
    /// Should we trace the thread id
    with_thread_ids: bool,
    /// Should we trace the thread names
    with_thread_names: bool,
    /// Should we trace the line numbers
    with_line_number: bool,
    /// Should we trace the level
    with_level: bool,
}

#[derive(Clone, CopyGetters, Debug, Default, Deserialize, Eq, Getters, PartialEq, Serialize)]
pub(crate) struct ConfigAoc2Di<'a> {
    #[serde(borrow)]
    env: Cow<'a, str>,
    #[getset(get_copy = "pub(crate)")]
    verbose: u8,
    #[getset(get_copy = "pub(crate)")]
    quiet: u8,
    #[serde(borrow)]
    year: Cow<'a, str>,
    start_day: Option<String>,
    end_day: Option<String>,
    #[getset(get_copy = "pub(crate)")]
    tracing: Tracing,
    #[serde(borrow)]
    session: Cow<'a, str>,
    #[serde(borrow)]
    aoc2_path: Cow<'a, str>,
}

impl ConfigAoc2Di<'_> {
    pub(crate) fn day_range(&self) -> Result<RangeInclusive<usize>> {
        let start_day = self
            .start_day
            .as_ref()
            .unwrap_or(&"1".to_string())
            .parse()?;
        let end_day = self.end_day.as_ref().unwrap_or(&"25".to_string()).parse()?;
        Ok(start_day..=end_day)
    }

    pub(crate) fn env(&self) -> &str {
        &self.env
    }

    pub(crate) fn year(&self) -> &str {
        &self.year
    }

    pub(crate) fn session(&self) -> &str {
        &self.session
    }

    pub(crate) fn aoc2_path(&self) -> &str {
        &self.aoc2_path
    }
}

impl TracingConfig for ConfigAoc2Di<'_> {
    fn quiet(&self) -> u8 {
        self.quiet
    }

    fn verbose(&self) -> u8 {
        self.verbose
    }

    fn with_target(&self) -> bool {
        self.tracing.with_target()
    }

    fn with_thread_ids(&self) -> bool {
        self.tracing.with_thread_ids()
    }

    fn with_thread_names(&self) -> bool {
        self.tracing.with_thread_names()
    }

    fn with_line_number(&self) -> bool {
        self.tracing.with_line_number()
    }

    fn with_level(&self) -> bool {
        self.tracing.with_level()
    }
}

/// Load the configuration
///
/// # Errors
///
pub(crate) fn load<'a, S, T, D>(cli: &S, defaults: &D) -> Result<T>
where
    T: Deserialize<'a>,
    S: Source + Clone + Send + Sync + 'static,
    D: ConfigDefaults,
{
    let config_file_path = config_file_path(defaults)?;
    let config = Config::builder()
        .set_default("env", "development")?
        .set_default("start_day", None::<String>)?
        .set_default("end_day", None::<String>)?
        .add_source(
            Environment::with_prefix(&defaults.env_prefix())
                .separator("_")
                .try_parsing(true),
        )
        .add_source(File::from(config_file_path).format(FileFormat::Toml))
        .add_source(cli.clone())
        .build()
        .with_context(|| CONFIG_BUILD_ERROR)?;
    config
        .try_deserialize::<T>()
        .with_context(|| ConfigDeserialize)
}

pub(crate) fn config_dir_path<D>(defaults: &D) -> Result<PathBuf>
where
    D: ConfigDefaults,
{
    let default_fn = || -> Result<PathBuf> { default_config_dir(defaults) };
    defaults
        .config_file_path()
        .as_ref()
        .map_or_else(default_fn, to_path_buf)
}

fn config_file_path<D>(defaults: &D) -> Result<PathBuf>
where
    D: ConfigDefaults,
{
    let default_fn = || -> Result<PathBuf> { default_config_file_path(defaults) };
    defaults
        .config_file_path()
        .as_ref()
        .map_or_else(default_fn, to_path_buf)
}

fn default_config_dir<D>(defaults: &D) -> Result<PathBuf>
where
    D: ConfigDefaults,
{
    let mut config_file_path = dirs2::config_dir().ok_or(ConfigDir)?;
    config_file_path.push(defaults.default_file_path());
    Ok(config_file_path)
}

fn default_config_file_path<D>(defaults: &D) -> Result<PathBuf>
where
    D: ConfigDefaults,
{
    let mut config_file_path = dirs2::config_dir().ok_or(ConfigDir)?;
    config_file_path.push(defaults.default_file_path());
    config_file_path.push(defaults.default_file_name());
    let _ = config_file_path.set_extension("toml");
    Ok(config_file_path)
}

#[allow(clippy::unnecessary_wraps)]
fn to_path_buf(path: &String) -> Result<PathBuf> {
    Ok(PathBuf::from(path))
}
