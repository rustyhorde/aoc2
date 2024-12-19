// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! **--- Advent of Code 2017 ---**
//!
//! **--- Day 20: Particle Swarm ---**
//!
//! Suddenly, the GPU contacts you, asking for help. Someone has asked it to simulate too many particles, and it won't be able to finish them all in time to render the next frame at this rate.
//!
//! It transmits to you a buffer (your puzzle input) listing each particle in order (starting with particle 0, then particle 1, particle 2, and so on). For each particle, it provides the X, Y, and Z coordinates for the particle's position (p), velocity (v), and acceleration (a), each in the format <X,Y,Z>.
//!
//! Each tick, all particles are updated simultaneously. A particle's properties are updated in the following order:
//!
//! ```text
//!     Increase the X velocity by the X acceleration.
//!     Increase the Y velocity by the Y acceleration.
//!     Increase the Z velocity by the Z acceleration.
//!     Increase the X position by the X velocity.
//!     Increase the Y position by the Y velocity.
//!     Increase the Z position by the Z velocity.
//! ```
//!
//! Because of seemingly tenuous rationale involving z-buffering, the GPU would like to know which particle will stay closest to position <0,0,0> in the long term. Measure this using the Manhattan distance, which in this situation is simply the sum of the absolute values of a particle's X, Y, and Z position.
//!
//! For example, suppose you are only given two particles, both of which stay entirely on the X-axis (for simplicity). Drawing the current states of particles 0 and 1 (in that order) with an adjacent a number line and diagram of current X positions (marked in parentheses), the following would take place:
//!
//! ```text
//! p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>    -4 -3 -2 -1  0  1  2  3  4
//! p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>                         (0)(1)
//!
//! p=< 4,0,0>, v=< 1,0,0>, a=<-1,0,0>    -4 -3 -2 -1  0  1  2  3  4
//! p=< 2,0,0>, v=<-2,0,0>, a=<-2,0,0>                      (1)   (0)
//!
//! p=< 4,0,0>, v=< 0,0,0>, a=<-1,0,0>    -4 -3 -2 -1  0  1  2  3  4
//! p=<-2,0,0>, v=<-4,0,0>, a=<-2,0,0>          (1)               (0)
//!
//! p=< 3,0,0>, v=<-1,0,0>, a=<-1,0,0>    -4 -3 -2 -1  0  1  2  3  4
//! p=<-8,0,0>, v=<-6,0,0>, a=<-2,0,0>                         (0)   
//! ```
//!
//! At this point, particle 1 will never be closer to <0,0,0> than particle 0, and so, in the long run, particle 0 will stay closest.
//!
//! Which particle will stay closest to position <0,0,0> in the long term?
//!
//! **--- Part Two ---**
//!
//! To simplify the problem further, the GPU would like to remove any particles that collide. Particles collide if their positions ever exactly match. Because particles are updated simultaneously, more than two particles can collide at the same time and place. Once particles collide, they are removed and cannot collide with anything else after that tick.
//!
//! For example:
//!
//! ```text
//! p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>    
//! p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>    -6 -5 -4 -3 -2 -1  0  1  2  3
//! p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>    (0)   (1)   (2)            (3)
//! p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>
//!
//! p=<-3,0,0>, v=< 3,0,0>, a=< 0,0,0>    
//! p=<-2,0,0>, v=< 2,0,0>, a=< 0,0,0>    -6 -5 -4 -3 -2 -1  0  1  2  3
//! p=<-1,0,0>, v=< 1,0,0>, a=< 0,0,0>             (0)(1)(2)      (3)   
//! p=< 2,0,0>, v=<-1,0,0>, a=< 0,0,0>
//!
//! p=< 0,0,0>, v=< 3,0,0>, a=< 0,0,0>    
//! p=< 0,0,0>, v=< 2,0,0>, a=< 0,0,0>    -6 -5 -4 -3 -2 -1  0  1  2  3
//! p=< 0,0,0>, v=< 1,0,0>, a=< 0,0,0>                       X (3)      
//! p=< 1,0,0>, v=<-1,0,0>, a=< 0,0,0>
//!
//! ------destroyed by collision------    
//! ------destroyed by collision------    -6 -5 -4 -3 -2 -1  0  1  2  3
//! ------destroyed by collision------                      (3)         
//! p=< 0,0,0>, v=<-1,0,0>, a=< 0,0,0>
//! ```
//!
//! In this example, particles 0, 1, and 2 are simultaneously destroyed at the time and place marked X. On the next tick, particle 3 passes through unharmed.
//!
//! How many particles are left after all collisions are resolved?

use crate::{
    constants::{AoCDay, AoCYear},
    utils::{run_bench_solution, run_setup_solution, valid_lines},
};
use anyhow::{anyhow, Result};
use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
/// A particle has x,y,z coords, a velocity, an acceleration,
/// and a Manhattan Distance from the origin.
struct Particle {
    /// x,y,z coords
    coords: Coords,
    /// particle velocity
    vel: Velocity,
    /// particle acceleration
    acc: Acc,
    /// Manhattan Distance from origin.
    md: usize,
}

/// 3-d coordinates
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coords {
    /// x coordinate
    x: i64,
    /// y coordinate
    y: i64,
    /// z coordinate
    z: i64,
}

/// particle velocity
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Velocity {
    /// velocity in the x-direciton.
    vx: i64,
    /// velocity in the y-direciton.
    vy: i64,
    /// velocity in the z-direciton.
    vz: i64,
}

/// particle acceleration
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Acc {
    /// acceleration in the x direction.
    ax: i64,
    /// acceleration in the x direction.
    ay: i64,
    /// acceleration in the x direction.
    az: i64,
}

type ParticleData = HashMap<usize, Particle>;

/// Solution for Part 1
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_1() -> Result<u32> {
    run_setup_solution::<ParticleData, usize>(AoCYear::AOC2017, AoCDay::AOCD20, setup, find)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 1
///
/// # Errors
///
pub fn part_1_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<ParticleData, usize>(bench, AoCYear::AOC2017, AoCDay::AOCD20, setup, find)
        .map(|_| 0)
}

fn setup(reader: BufReader<File>) -> ParticleData {
    setup_br(reader).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps, clippy::similar_names)]
fn setup_br<T>(reader: T) -> Result<ParticleData>
where
    T: BufRead,
{
    let mut particle_map: HashMap<usize, Particle> = HashMap::new();
    let coords_re = Regex::new(r"p=< *(-?\d+),(-?\d+),(-?\d+)>")?;
    let vel_re = Regex::new(r"v=< *(-?\d+),(-?\d+),(-?\d+)>")?;
    let acc_re = Regex::new(r"a=< *(-?\d+),(-?\d+),(-?\d+)>")?;

    for (idx, line) in valid_lines(reader).enumerate() {
        let parts: Vec<&str> = line.split(", ").collect();

        let coords = if coords_re.is_match(parts[0]) {
            let caps = coords_re
                .captures(parts[0])
                .ok_or(anyhow!("invalid coords captures"))?;
            let x_str = caps.get(1).ok_or(anyhow!("invalid x value"))?.as_str();
            let y_str = caps.get(2).ok_or(anyhow!("invalid y value"))?.as_str();
            let z_str = caps.get(3).ok_or(anyhow!("invalid z value"))?.as_str();
            let x = x_str.parse::<i64>()?;
            let y = y_str.parse::<i64>()?;
            let z = z_str.parse::<i64>()?;
            Coords { x, y, z }
        } else {
            return Err(anyhow!("invalid coordinates"));
        };

        let velocity = if vel_re.is_match(parts[1]) {
            let caps = vel_re
                .captures(parts[1])
                .ok_or(anyhow!("invalid velocity captures"))?;
            let vx_str = caps.get(1).ok_or(anyhow!("invalid vx value"))?.as_str();
            let vy_str = caps.get(2).ok_or(anyhow!("invalid vy value"))?.as_str();
            let vz_str = caps.get(3).ok_or(anyhow!("invalid vz value"))?.as_str();
            let vx = vx_str.parse::<i64>()?;
            let vy = vy_str.parse::<i64>()?;
            let vz = vz_str.parse::<i64>()?;
            Velocity { vx, vy, vz }
        } else {
            return Err(anyhow!("invalid velocity"));
        };

        let acc = if acc_re.is_match(parts[2]) {
            let caps = acc_re
                .captures(parts[2])
                .ok_or(anyhow!("invalid acceleration captures"))?;
            let ax_str = caps.get(1).ok_or(anyhow!("invalid ax value"))?.as_str();
            let ay_str = caps.get(2).ok_or(anyhow!("invalid ay value"))?.as_str();
            let az_str = caps.get(3).ok_or(anyhow!("invalid az value"))?.as_str();
            let ax = ax_str.parse::<i64>()?;
            let ay = ay_str.parse::<i64>()?;
            let az = az_str.parse::<i64>()?;
            Acc { ax, ay, az }
        } else {
            return Err(anyhow!("invalid acceleration"));
        };

        let md: usize = TryFrom::try_from(coords.x.abs() + coords.y.abs() + coords.z.abs())?;
        let particle = Particle {
            coords,
            vel: velocity,
            acc,
            md,
        };
        let _ = particle_map.insert(idx, particle);
    }
    Ok(particle_map)
}

#[allow(clippy::needless_pass_by_value)]
fn find(data: ParticleData) -> usize {
    find_res(data, false).unwrap_or_default()
}

#[allow(clippy::unnecessary_wraps)]
fn find_res(mut particle_map: ParticleData, second_star: bool) -> Result<usize> {
    for _ in 0..1000 {
        for particle in particle_map.values_mut() {
            update_particle(particle)?;
        }

        if second_star {
            let matches = find_collisions(&particle_map);

            for idx in matches {
                let _ = particle_map.remove(&idx);
            }
        }
    }

    Ok(if second_star {
        particle_map.len()
    } else {
        find_minimum_md(&particle_map)?
    })
}

/// Update a particle
fn update_particle(particle: &mut Particle) -> Result<()> {
    particle.vel.vx += particle.acc.ax;
    particle.vel.vy += particle.acc.ay;
    particle.vel.vz += particle.acc.az;
    particle.coords.x += particle.vel.vx;
    particle.coords.y += particle.vel.vy;
    particle.coords.z += particle.vel.vz;
    particle.md = TryFrom::try_from(
        particle.coords.x.abs() + particle.coords.y.abs() + particle.coords.z.abs(),
    )?;
    Ok(())
}

/// Find the minimum Manhattan distance in the map.
fn find_minimum_md(particle_map: &HashMap<usize, Particle>) -> Result<usize> {
    let (min_idx, _) = particle_map
        .iter()
        .min_by_key(|&(_, particle)| particle.md)
        .ok_or(anyhow!("No minimum found"))?;

    Ok(*min_idx)
}

/// Remove collisions
fn find_collisions(particle_map: &HashMap<usize, Particle>) -> Vec<usize> {
    let all_coords: HashMap<usize, Coords> =
        particle_map.iter().map(|(k, p)| (*k, p.coords)).collect();
    let mut matches = Vec::new();

    for (k, v) in particle_map {
        for (j, c1) in &all_coords {
            if *c1 == v.coords && j != k {
                matches.push(*k);
            }
        }
    }

    matches.sort_unstable();
    matches.dedup();

    matches
}

/// Solution for Part 2
///
/// # Errors
/// * This function will error if the `data_file` for the corresponding [`AoCYear`](crate::constants::AoCYear) and
///   [`AoCDay`](crate::constants::AoCDay) cannot be read.
/// * This function will error if the elapsed [`std::time::Duration`] is invalid.
pub fn part_2() -> Result<u32> {
    run_setup_solution::<ParticleData, usize>(AoCYear::AOC2017, AoCDay::AOCD20, setup, find2)
        .map(|_| 0)
}

/// Benchmark handler for Solution to Part 2
///
/// # Errors
///
pub fn part_2_bench(bench: u16) -> Result<u32> {
    run_bench_solution::<ParticleData, usize>(bench, AoCYear::AOC2017, AoCDay::AOCD20, setup, find2)
        .map(|_| 0)
}

#[allow(clippy::needless_pass_by_value)]
fn find2(data: ParticleData) -> usize {
    find_res(data, true).unwrap_or_default()
}

#[cfg(test)]
mod one_star {}

#[cfg(test)]
mod two_star {}
