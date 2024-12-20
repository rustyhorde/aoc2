// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `aoc2` config

use std::{borrow::Cow, path::PathBuf};

use anyhow::{Context, Result};
use config::{Config, Environment, File, FileFormat, Source};
use getset::{CopyGetters, Getters};
use serde::{Deserialize, Serialize};
use tracing_subscriber_init::TracingConfig;

use crate::{
    error::Error::{ConfigDeserialize, ConfigDir},
    tracing::Tracing,
};

pub(crate) trait PathDefaults {
    /// Environment variable prefix
    fn env_prefix(&self) -> String;
    /// The full path to use
    fn config_file_path(&self) -> Option<String>;
    /// The default file path to use
    fn default_file_path(&self) -> String;
    /// The default file name to use
    fn default_file_name(&self) -> String;
}

#[derive(Clone, CopyGetters, Debug, Default, Deserialize, Eq, Getters, PartialEq, Serialize)]
#[allow(clippy::struct_excessive_bools)]
pub(crate) struct ConfigAoc2<'a> {
    #[getset(get_copy = "pub(crate)")]
    verbose: u8,
    #[getset(get_copy = "pub(crate)")]
    quiet: u8,
    #[getset(get = "pub(crate)")]
    bench: Option<u16>,
    #[getset(get = "pub(crate)")]
    #[serde(borrow)]
    env: Cow<'a, str>,
    #[getset(get_copy = "pub(crate)")]
    tracing: Tracing,
    #[getset(get = "pub(crate)")]
    post_rebase_cmd: Option<String>,
    #[getset(get = "pub(crate)")]
    post_build_cmd: Option<String>,
}

impl TracingConfig for ConfigAoc2<'_> {
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

pub(crate) fn load<'a, S, T, D>(cli: &S, defaults: &D) -> Result<T>
where
    T: Deserialize<'a>,
    S: Source + Clone + Send + Sync + 'static,
    D: PathDefaults,
{
    let config_file_path = config_file_path(defaults)?;
    let config = Config::builder()
        .set_default("env", "development")?
        .add_source(
            Environment::with_prefix(&defaults.env_prefix())
                .separator("_")
                .try_parsing(true),
        )
        .add_source(File::from(config_file_path).format(FileFormat::Toml))
        .add_source(cli.clone())
        .build()?;
    config
        .try_deserialize::<T>()
        .with_context(|| ConfigDeserialize)
}

fn config_file_path<D>(defaults: &D) -> Result<PathBuf>
where
    D: PathDefaults,
{
    let default_fn = || -> Result<PathBuf> { default_config_file_path(defaults) };
    defaults
        .config_file_path()
        .as_ref()
        .map_or_else(default_fn, to_path_buf)
}

fn default_config_file_path<D>(defaults: &D) -> Result<PathBuf>
where
    D: PathDefaults,
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
