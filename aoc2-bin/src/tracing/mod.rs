// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `aoc2` tracing
//!

use anyhow::Result;
use getset::CopyGetters;
use serde::{Deserialize, Serialize};
use tracing::Level;
use tracing_subscriber::{EnvFilter, Registry};
use tracing_subscriber_init::{compact, try_init, Iso8601, Layer, TracingConfig, UtcTime};

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

pub(crate) fn initialize<T>(
    config: &T,
    layers_opt: Option<Vec<Box<dyn Layer<Registry> + Send + Sync>>>,
) -> Result<()>
where
    T: TracingConfig,
{
    let mut layers = layers_opt.unwrap_or_default();
    let (layer, level_filter) = compact(config);
    let directives_base = match level_filter.into_level() {
        Some(level) => match level {
            Level::TRACE => "trace",
            Level::DEBUG => "debug",
            Level::INFO => "info",
            Level::WARN => "warn",
            Level::ERROR => "error",
        },
        None => "info",
    };
    let directives = format!(
        "{directives_base},actix_server=error,mio=error,rustls=error,h2=error,tokio_util=error"
    );
    let filter = EnvFilter::builder()
        .with_default_directive(level_filter.into())
        .parse_lossy(directives);
    let stdout_layer = layer
        .with_timer(UtcTime::new(Iso8601::DEFAULT))
        .with_filter(filter);
    layers.push(stdout_layer.boxed());
    try_init(layers)?;
    Ok(())
}
