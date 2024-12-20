// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use anyhow::Result;
use cookie::CookieBuilder;
use reqwest::{Url, cookie::Jar};

use crate::constants::AOC_URL;

use super::{ConfigAoc2Di, DOMAIN, SESSION};

pub(crate) fn cookie_jar(config: &ConfigAoc2Di<'_>) -> Result<Jar> {
    let session = config.session();
    let cookie = CookieBuilder::new(SESSION, session)
        .domain(DOMAIN)
        .path("/")
        .secure(true)
        .http_only(true)
        .build();
    let jar = Jar::default();
    jar.add_cookie_str(&cookie.to_string(), &AOC_URL.parse::<Url>()?);
    Ok(jar)
}
