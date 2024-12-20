// Copyright (c) 2024 krocov developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use anyhow::Result;
use tracing::Level;
use tracing_appender::{non_blocking::WorkerGuard, rolling};
use tracing_subscriber::{EnvFilter, Layer, Registry};
use tracing_subscriber_init::{Iso8601, TracingConfig, UtcTime, compact, try_init};

use crate::config::{config_dir_path, defaults::ConfigDefaults};

/// Initialize tracing
///
/// # Errors
///
pub(crate) fn initialize<D, T>(
    defaults: &D,
    tracing_config: &T,
    layers_opt: Option<Vec<Box<dyn Layer<Registry> + Send + Sync>>>,
) -> Result<WorkerGuard>
where
    D: ConfigDefaults,
    T: TracingConfig,
{
    let mut layers = layers_opt.unwrap_or_default();
    let (layer, stdout_level_filter) = compact(tracing_config);
    let (mut file_layer, file_level_filter) = compact(tracing_config);
    let directives_base = match stdout_level_filter.into_level() {
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
        "{directives_base},actix_server=error,mio=error,rustls=error,h2=error,tokio_util=error,surf=error,isahc=error"
    );
    let stdout_filter = EnvFilter::builder()
        .with_default_directive(stdout_level_filter.into())
        .parse_lossy(directives);
    let stdout_layer = layer
        .with_timer(UtcTime::new(Iso8601::DEFAULT))
        .with_filter(stdout_filter);

    let directives = format!(
        "{directives_base},actix_server=error,mio=error,rustls=error,h2=error,tokio_util=error,surf=error,isahc=error,vergen_pretty=error"
    );
    let file_appender = rolling::daily(config_dir_path(defaults)?, "krocov.log");
    let file_filter = EnvFilter::builder()
        .with_default_directive(file_level_filter.into())
        .parse_lossy(directives);
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    file_layer.set_ansi(false);
    let file_layer = file_layer
        .without_time()
        .with_writer(non_blocking)
        .with_filter(file_filter);

    layers.push(stdout_layer.boxed());
    layers.push(file_layer.boxed());

    try_init(layers)?;
    Ok(guard)
}
