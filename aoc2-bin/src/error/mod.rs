// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `aoc2` error handling

use clap::ErrorKind;

#[allow(clippy::needless_pass_by_value)]
pub(crate) fn clap_or_error(err: anyhow::Error) -> i32 {
    let disp_err = || {
        eprint!("{:?}", err);
        1
    };
    match err.downcast_ref::<clap::Error>() {
        Some(e) => match e.kind {
            ErrorKind::HelpDisplayed => {
                eprint!("{}", e.message);
                0
            }
            ErrorKind::VersionDisplayed => 0,
            _ => disp_err(),
        },
        None => disp_err(),
    }
}

pub(crate) fn success(_: ()) -> i32 {
    0
}

#[cfg(test)]
mod test {
    use super::{clap_or_error, success};
    use anyhow::{anyhow, Error};
    use clap::ErrorKind::{HelpDisplayed, VersionDisplayed};

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
        let clap_error = Error::new(clap::Error::with_description("help", HelpDisplayed));
        assert_eq!(0, clap_or_error(clap_error));
    }

    #[test]
    fn clap_or_error_is_version() {
        let clap_error = Error::new(clap::Error::with_description("version", VersionDisplayed));
        assert_eq!(0, clap_or_error(clap_error));
    }
}
