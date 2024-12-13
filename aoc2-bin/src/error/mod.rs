// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `aoc2` error handling

use clap::error::ErrorKind;
use config::ConfigError;

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("There is no valid config directory")]
    ConfigDir,
    #[error("Unable to deserialize config")]
    ConfigDeserialize,
    #[error("Config build error")]
    ConfigBuild(#[from] ConfigError),
    #[error("Unable to load config")]
    ConfigLoad,
    #[error("Unable to initialize tracing")]
    TracingInit,
}

#[allow(clippy::needless_pass_by_value)]
pub(crate) fn clap_or_error(err: anyhow::Error) -> i32 {
    let disp_err = || {
        eprint!("{err:?}");
        1
    };
    match err.downcast_ref::<clap::Error>() {
        Some(e) => match e.kind() {
            ErrorKind::DisplayHelp => {
                eprint!("{e}");
                0
            }
            ErrorKind::DisplayVersion => 0,
            _ => disp_err(),
        },
        None => disp_err(),
    }
}

pub(crate) fn success((): ()) -> i32 {
    0
}

#[cfg(test)]
mod test {
    use super::{clap_or_error, success};
    use anyhow::{anyhow, Error};
    use clap::{
        error::ErrorKind::{DisplayHelp, DisplayVersion},
        Command,
    };

    #[test]
    fn success_works() {
        assert_eq!(0, success(()));
    }

    #[test]
    fn clap_or_error_is_error() {
        assert_eq!(1, clap_or_error(anyhow!("test")));
    }

    #[test]
    fn clap_or_error_is_help() {
        let mut cmd = Command::new("aoc2");
        let error = cmd.error(DisplayHelp, "help");
        let clap_error = Error::new(error);
        assert_eq!(0, clap_or_error(clap_error));
    }

    #[test]
    fn clap_or_error_is_version() {
        let mut cmd = Command::new("aoc2");
        let error = cmd.error(DisplayVersion, "1.0");
        let clap_error = Error::new(error);
        assert_eq!(0, clap_or_error(clap_error));
    }
}
