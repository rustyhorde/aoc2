// Copyright (c) 2024 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

pub fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    nightly();
    beta();
    stable();
}

#[rustversion::nightly]
fn nightly() {
    println!("cargo:rustc-check-cfg=cfg(nightly)");
    println!("cargo:rustc-cfg=nightly");
}

#[rustversion::not(nightly)]
fn nightly() {
    println!("cargo:rustc-check-cfg=cfg(nightly)");
}

#[rustversion::beta]
fn beta() {
    println!("cargo:rustc-check-cfg=cfg(beta)");
    println!("cargo:rustc-cfg=beta");
}

#[rustversion::not(beta)]
fn beta() {
    println!("cargo:rustc-check-cfg=cfg(beta)");
}

#[rustversion::stable]
fn stable() {
    println!("cargo:rustc-check-cfg=cfg(stable)");
    println!("cargo:rustc-cfg=stable");
}

#[rustversion::not(stable)]
fn stable() {
    println!("cargo:rustc-check-cfg=cfg(stable)");
}
