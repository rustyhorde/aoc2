// Copyright (c) 2021 aoc2 developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `aoc2` function map

use anyhow::Result;
use aoc2_sol::constants::{AoCDay, AoCYear};
use lazy_static::lazy_static;
use std::collections::HashMap;

pub(crate) type FnMap = HashMap<(AoCYear, AoCDay, bool), fn() -> Result<u32>>;
pub(crate) type BenchMap = HashMap<(AoCYear, AoCDay, bool), fn(u16) -> Result<u32>>;

macro_rules! aoc_ins {
    ($fnmap:ident, $year:expr, $day:expr, $y:ident, $d:ident) => {
        _ = $fnmap.insert(($year, $day, false), aoc2_sol::$y::$d::part_1);
        _ = $fnmap.insert(($year, $day, true), aoc2_sol::$y::$d::part_2);
    };
}

macro_rules! bench_ins {
    ($fnmap:ident, $year:expr, $day:expr, $y:ident, $d:ident) => {
        _ = $fnmap.insert(($year, $day, false), aoc2_sol::$y::$d::part_1_bench);
        _ = $fnmap.insert(($year, $day, true), aoc2_sol::$y::$d::part_2_bench);
    };
}

lazy_static! {
    pub(crate) static ref BENCH_MAP: BenchMap = {
        let mut bench_map: BenchMap = HashMap::new();

        // 2024
        bench_ins!(bench_map, AoCYear::AOC2024, AoCDay::AOCD07, year2024, day07);
        bench_ins!(bench_map, AoCYear::AOC2024, AoCDay::AOCD08, year2024, day08);
        bench_ins!(bench_map, AoCYear::AOC2024, AoCDay::AOCD09, year2024, day09);
        bench_ins!(bench_map, AoCYear::AOC2024, AoCDay::AOCD10, year2024, day10);
        bench_ins!(bench_map, AoCYear::AOC2024, AoCDay::AOCD11, year2024, day11);
        bench_ins!(bench_map, AoCYear::AOC2024, AoCDay::AOCD12, year2024, day12);
        bench_ins!(bench_map, AoCYear::AOC2024, AoCDay::AOCD13, year2024, day13);
        bench_ins!(bench_map, AoCYear::AOC2024, AoCDay::AOCD14, year2024, day14);

        // 2017
        bench_ins!(bench_map, AoCYear::AOC2017, AoCDay::AOCD09, year2017, day09);
        bench_ins!(bench_map, AoCYear::AOC2017, AoCDay::AOCD10, year2017, day10);
        bench_ins!(bench_map, AoCYear::AOC2017, AoCDay::AOCD11, year2017, day11);
        bench_ins!(bench_map, AoCYear::AOC2017, AoCDay::AOCD12, year2017, day12);
        bench_ins!(bench_map, AoCYear::AOC2017, AoCDay::AOCD13, year2017, day13);
        bench_ins!(bench_map, AoCYear::AOC2017, AoCDay::AOCD14, year2017, day14);

        bench_map
    };
    pub(crate) static ref FN_MAP: FnMap = {
        let mut fn_map: FnMap = HashMap::new();

        // 2015
        aoc_ins!(fn_map, AoCYear::AOC2015, AoCDay::AOCD01, year2015, day01);
        aoc_ins!(fn_map, AoCYear::AOC2015, AoCDay::AOCD02, year2015, day02);
        aoc_ins!(fn_map, AoCYear::AOC2015, AoCDay::AOCD03, year2015, day03);
        aoc_ins!(fn_map, AoCYear::AOC2015, AoCDay::AOCD04, year2015, day04);
        aoc_ins!(fn_map, AoCYear::AOC2015, AoCDay::AOCD05, year2015, day05);
        aoc_ins!(fn_map, AoCYear::AOC2015, AoCDay::AOCD06, year2015, day06);
        aoc_ins!(fn_map, AoCYear::AOC2015, AoCDay::AOCD07, year2015, day07);
        aoc_ins!(fn_map, AoCYear::AOC2015, AoCDay::AOCD08, year2015, day08);
        aoc_ins!(fn_map, AoCYear::AOC2015, AoCDay::AOCD09, year2015, day09);
        aoc_ins!(fn_map, AoCYear::AOC2015, AoCDay::AOCD10, year2015, day10);
        aoc_ins!(fn_map, AoCYear::AOC2015, AoCDay::AOCD11, year2015, day11);
        aoc_ins!(fn_map, AoCYear::AOC2015, AoCDay::AOCD12, year2015, day12);
        aoc_ins!(fn_map, AoCYear::AOC2015, AoCDay::AOCD13, year2015, day13);
        aoc_ins!(fn_map, AoCYear::AOC2015, AoCDay::AOCD14, year2015, day14);
        aoc_ins!(fn_map, AoCYear::AOC2015, AoCDay::AOCD15, year2015, day15);
        aoc_ins!(fn_map, AoCYear::AOC2015, AoCDay::AOCD16, year2015, day16);
        aoc_ins!(fn_map, AoCYear::AOC2015, AoCDay::AOCD17, year2015, day17);
        aoc_ins!(fn_map, AoCYear::AOC2015, AoCDay::AOCD18, year2015, day18);
        aoc_ins!(fn_map, AoCYear::AOC2015, AoCDay::AOCD19, year2015, day19);
        aoc_ins!(fn_map, AoCYear::AOC2015, AoCDay::AOCD20, year2015, day20);
        aoc_ins!(fn_map, AoCYear::AOC2015, AoCDay::AOCD21, year2015, day21);
        aoc_ins!(fn_map, AoCYear::AOC2015, AoCDay::AOCD22, year2015, day22);
        aoc_ins!(fn_map, AoCYear::AOC2015, AoCDay::AOCD23, year2015, day23);
        aoc_ins!(fn_map, AoCYear::AOC2015, AoCDay::AOCD24, year2015, day24);
        aoc_ins!(fn_map, AoCYear::AOC2015, AoCDay::AOCD25, year2015, day25);

        // 2016
        aoc_ins!(fn_map, AoCYear::AOC2016, AoCDay::AOCD01, year2016, day01);
        aoc_ins!(fn_map, AoCYear::AOC2016, AoCDay::AOCD02, year2016, day02);
        aoc_ins!(fn_map, AoCYear::AOC2016, AoCDay::AOCD03, year2016, day03);
        aoc_ins!(fn_map, AoCYear::AOC2016, AoCDay::AOCD04, year2016, day04);
        aoc_ins!(fn_map, AoCYear::AOC2016, AoCDay::AOCD05, year2016, day05);
        aoc_ins!(fn_map, AoCYear::AOC2016, AoCDay::AOCD06, year2016, day06);
        aoc_ins!(fn_map, AoCYear::AOC2016, AoCDay::AOCD07, year2016, day07);
        aoc_ins!(fn_map, AoCYear::AOC2016, AoCDay::AOCD08, year2016, day08);
        aoc_ins!(fn_map, AoCYear::AOC2016, AoCDay::AOCD09, year2016, day09);
        aoc_ins!(fn_map, AoCYear::AOC2016, AoCDay::AOCD10, year2016, day10);
        aoc_ins!(fn_map, AoCYear::AOC2016, AoCDay::AOCD11, year2016, day11);
        aoc_ins!(fn_map, AoCYear::AOC2016, AoCDay::AOCD12, year2016, day12);
        aoc_ins!(fn_map, AoCYear::AOC2016, AoCDay::AOCD13, year2016, day13);
        aoc_ins!(fn_map, AoCYear::AOC2016, AoCDay::AOCD14, year2016, day14);
        aoc_ins!(fn_map, AoCYear::AOC2016, AoCDay::AOCD15, year2016, day15);
        aoc_ins!(fn_map, AoCYear::AOC2016, AoCDay::AOCD16, year2016, day16);
        aoc_ins!(fn_map, AoCYear::AOC2016, AoCDay::AOCD17, year2016, day17);
        aoc_ins!(fn_map, AoCYear::AOC2016, AoCDay::AOCD18, year2016, day18);
        aoc_ins!(fn_map, AoCYear::AOC2016, AoCDay::AOCD19, year2016, day19);
        aoc_ins!(fn_map, AoCYear::AOC2016, AoCDay::AOCD20, year2016, day20);
        aoc_ins!(fn_map, AoCYear::AOC2016, AoCDay::AOCD21, year2016, day21);
        aoc_ins!(fn_map, AoCYear::AOC2016, AoCDay::AOCD22, year2016, day22);
        aoc_ins!(fn_map, AoCYear::AOC2016, AoCDay::AOCD23, year2016, day23);
        aoc_ins!(fn_map, AoCYear::AOC2016, AoCDay::AOCD24, year2016, day24);
        aoc_ins!(fn_map, AoCYear::AOC2016, AoCDay::AOCD25, year2016, day25);

        // 2017
        aoc_ins!(fn_map, AoCYear::AOC2017, AoCDay::AOCD01, year2017, day01);
        aoc_ins!(fn_map, AoCYear::AOC2017, AoCDay::AOCD02, year2017, day02);
        aoc_ins!(fn_map, AoCYear::AOC2017, AoCDay::AOCD03, year2017, day03);
        aoc_ins!(fn_map, AoCYear::AOC2017, AoCDay::AOCD04, year2017, day04);
        aoc_ins!(fn_map, AoCYear::AOC2017, AoCDay::AOCD05, year2017, day05);
        aoc_ins!(fn_map, AoCYear::AOC2017, AoCDay::AOCD06, year2017, day06);
        aoc_ins!(fn_map, AoCYear::AOC2017, AoCDay::AOCD07, year2017, day07);
        aoc_ins!(fn_map, AoCYear::AOC2017, AoCDay::AOCD08, year2017, day08);
        aoc_ins!(fn_map, AoCYear::AOC2017, AoCDay::AOCD09, year2017, day09);
        aoc_ins!(fn_map, AoCYear::AOC2017, AoCDay::AOCD10, year2017, day10);
        aoc_ins!(fn_map, AoCYear::AOC2017, AoCDay::AOCD11, year2017, day11);
        aoc_ins!(fn_map, AoCYear::AOC2017, AoCDay::AOCD12, year2017, day12);
        aoc_ins!(fn_map, AoCYear::AOC2017, AoCDay::AOCD13, year2017, day13);
        aoc_ins!(fn_map, AoCYear::AOC2017, AoCDay::AOCD14, year2017, day14);
        aoc_ins!(fn_map, AoCYear::AOC2017, AoCDay::AOCD15, year2017, day15);
        aoc_ins!(fn_map, AoCYear::AOC2017, AoCDay::AOCD16, year2017, day16);
        aoc_ins!(fn_map, AoCYear::AOC2017, AoCDay::AOCD17, year2017, day17);
        aoc_ins!(fn_map, AoCYear::AOC2017, AoCDay::AOCD18, year2017, day18);
        aoc_ins!(fn_map, AoCYear::AOC2017, AoCDay::AOCD19, year2017, day19);
        aoc_ins!(fn_map, AoCYear::AOC2017, AoCDay::AOCD20, year2017, day20);
        aoc_ins!(fn_map, AoCYear::AOC2017, AoCDay::AOCD21, year2017, day21);
        aoc_ins!(fn_map, AoCYear::AOC2017, AoCDay::AOCD22, year2017, day22);
        aoc_ins!(fn_map, AoCYear::AOC2017, AoCDay::AOCD23, year2017, day23);
        aoc_ins!(fn_map, AoCYear::AOC2017, AoCDay::AOCD24, year2017, day24);
        aoc_ins!(fn_map, AoCYear::AOC2017, AoCDay::AOCD25, year2017, day25);

        // 2018
        aoc_ins!(fn_map, AoCYear::AOC2018, AoCDay::AOCD01, year2018, day01);
        aoc_ins!(fn_map, AoCYear::AOC2018, AoCDay::AOCD02, year2018, day02);
        aoc_ins!(fn_map, AoCYear::AOC2018, AoCDay::AOCD03, year2018, day03);
        aoc_ins!(fn_map, AoCYear::AOC2018, AoCDay::AOCD04, year2018, day04);
        aoc_ins!(fn_map, AoCYear::AOC2018, AoCDay::AOCD05, year2018, day05);
        aoc_ins!(fn_map, AoCYear::AOC2018, AoCDay::AOCD06, year2018, day06);
        aoc_ins!(fn_map, AoCYear::AOC2018, AoCDay::AOCD07, year2018, day07);
        aoc_ins!(fn_map, AoCYear::AOC2018, AoCDay::AOCD08, year2018, day08);
        aoc_ins!(fn_map, AoCYear::AOC2018, AoCDay::AOCD09, year2018, day09);
        aoc_ins!(fn_map, AoCYear::AOC2018, AoCDay::AOCD10, year2018, day10);
        aoc_ins!(fn_map, AoCYear::AOC2018, AoCDay::AOCD11, year2018, day11);
        aoc_ins!(fn_map, AoCYear::AOC2018, AoCDay::AOCD12, year2018, day12);
        aoc_ins!(fn_map, AoCYear::AOC2018, AoCDay::AOCD13, year2018, day13);
        aoc_ins!(fn_map, AoCYear::AOC2018, AoCDay::AOCD14, year2018, day14);
        aoc_ins!(fn_map, AoCYear::AOC2018, AoCDay::AOCD15, year2018, day15);
        aoc_ins!(fn_map, AoCYear::AOC2018, AoCDay::AOCD16, year2018, day16);
        aoc_ins!(fn_map, AoCYear::AOC2018, AoCDay::AOCD17, year2018, day17);
        aoc_ins!(fn_map, AoCYear::AOC2018, AoCDay::AOCD18, year2018, day18);
        aoc_ins!(fn_map, AoCYear::AOC2018, AoCDay::AOCD19, year2018, day19);
        aoc_ins!(fn_map, AoCYear::AOC2018, AoCDay::AOCD20, year2018, day20);
        aoc_ins!(fn_map, AoCYear::AOC2018, AoCDay::AOCD21, year2018, day21);
        aoc_ins!(fn_map, AoCYear::AOC2018, AoCDay::AOCD22, year2018, day22);
        aoc_ins!(fn_map, AoCYear::AOC2018, AoCDay::AOCD23, year2018, day23);
        aoc_ins!(fn_map, AoCYear::AOC2018, AoCDay::AOCD24, year2018, day24);
        aoc_ins!(fn_map, AoCYear::AOC2018, AoCDay::AOCD25, year2018, day25);

        // 2019
        aoc_ins!(fn_map, AoCYear::AOC2019, AoCDay::AOCD01, year2019, day01);
        aoc_ins!(fn_map, AoCYear::AOC2019, AoCDay::AOCD02, year2019, day02);
        aoc_ins!(fn_map, AoCYear::AOC2019, AoCDay::AOCD03, year2019, day03);
        aoc_ins!(fn_map, AoCYear::AOC2019, AoCDay::AOCD04, year2019, day04);
        aoc_ins!(fn_map, AoCYear::AOC2019, AoCDay::AOCD05, year2019, day05);
        aoc_ins!(fn_map, AoCYear::AOC2019, AoCDay::AOCD06, year2019, day06);
        aoc_ins!(fn_map, AoCYear::AOC2019, AoCDay::AOCD07, year2019, day07);
        aoc_ins!(fn_map, AoCYear::AOC2019, AoCDay::AOCD08, year2019, day08);
        aoc_ins!(fn_map, AoCYear::AOC2019, AoCDay::AOCD09, year2019, day09);
        aoc_ins!(fn_map, AoCYear::AOC2019, AoCDay::AOCD10, year2019, day10);
        aoc_ins!(fn_map, AoCYear::AOC2019, AoCDay::AOCD11, year2019, day11);
        aoc_ins!(fn_map, AoCYear::AOC2019, AoCDay::AOCD12, year2019, day12);
        aoc_ins!(fn_map, AoCYear::AOC2019, AoCDay::AOCD13, year2019, day13);
        aoc_ins!(fn_map, AoCYear::AOC2019, AoCDay::AOCD14, year2019, day14);
        aoc_ins!(fn_map, AoCYear::AOC2019, AoCDay::AOCD15, year2019, day15);
        aoc_ins!(fn_map, AoCYear::AOC2019, AoCDay::AOCD16, year2019, day16);
        aoc_ins!(fn_map, AoCYear::AOC2019, AoCDay::AOCD17, year2019, day17);
        aoc_ins!(fn_map, AoCYear::AOC2019, AoCDay::AOCD18, year2019, day18);
        aoc_ins!(fn_map, AoCYear::AOC2019, AoCDay::AOCD19, year2019, day19);
        aoc_ins!(fn_map, AoCYear::AOC2019, AoCDay::AOCD20, year2019, day20);
        aoc_ins!(fn_map, AoCYear::AOC2019, AoCDay::AOCD21, year2019, day21);
        aoc_ins!(fn_map, AoCYear::AOC2019, AoCDay::AOCD22, year2019, day22);
        aoc_ins!(fn_map, AoCYear::AOC2019, AoCDay::AOCD23, year2019, day23);
        aoc_ins!(fn_map, AoCYear::AOC2019, AoCDay::AOCD24, year2019, day24);
        aoc_ins!(fn_map, AoCYear::AOC2019, AoCDay::AOCD25, year2019, day25);

        // 2020
        aoc_ins!(fn_map, AoCYear::AOC2020, AoCDay::AOCD01, year2020, day01);
        aoc_ins!(fn_map, AoCYear::AOC2020, AoCDay::AOCD02, year2020, day02);
        aoc_ins!(fn_map, AoCYear::AOC2020, AoCDay::AOCD03, year2020, day03);
        aoc_ins!(fn_map, AoCYear::AOC2020, AoCDay::AOCD04, year2020, day04);
        aoc_ins!(fn_map, AoCYear::AOC2020, AoCDay::AOCD05, year2020, day05);
        aoc_ins!(fn_map, AoCYear::AOC2020, AoCDay::AOCD06, year2020, day06);
        aoc_ins!(fn_map, AoCYear::AOC2020, AoCDay::AOCD07, year2020, day07);
        aoc_ins!(fn_map, AoCYear::AOC2020, AoCDay::AOCD08, year2020, day08);
        aoc_ins!(fn_map, AoCYear::AOC2020, AoCDay::AOCD09, year2020, day09);
        aoc_ins!(fn_map, AoCYear::AOC2020, AoCDay::AOCD10, year2020, day10);
        aoc_ins!(fn_map, AoCYear::AOC2020, AoCDay::AOCD11, year2020, day11);
        aoc_ins!(fn_map, AoCYear::AOC2020, AoCDay::AOCD12, year2020, day12);
        aoc_ins!(fn_map, AoCYear::AOC2020, AoCDay::AOCD13, year2020, day13);
        aoc_ins!(fn_map, AoCYear::AOC2020, AoCDay::AOCD14, year2020, day14);
        aoc_ins!(fn_map, AoCYear::AOC2020, AoCDay::AOCD15, year2020, day15);
        aoc_ins!(fn_map, AoCYear::AOC2020, AoCDay::AOCD16, year2020, day16);
        aoc_ins!(fn_map, AoCYear::AOC2020, AoCDay::AOCD17, year2020, day17);
        aoc_ins!(fn_map, AoCYear::AOC2020, AoCDay::AOCD18, year2020, day18);
        aoc_ins!(fn_map, AoCYear::AOC2020, AoCDay::AOCD19, year2020, day19);
        aoc_ins!(fn_map, AoCYear::AOC2020, AoCDay::AOCD20, year2020, day20);
        aoc_ins!(fn_map, AoCYear::AOC2020, AoCDay::AOCD21, year2020, day21);
        aoc_ins!(fn_map, AoCYear::AOC2020, AoCDay::AOCD22, year2020, day22);
        aoc_ins!(fn_map, AoCYear::AOC2020, AoCDay::AOCD23, year2020, day23);
        aoc_ins!(fn_map, AoCYear::AOC2020, AoCDay::AOCD24, year2020, day24);
        aoc_ins!(fn_map, AoCYear::AOC2020, AoCDay::AOCD25, year2020, day25);

        // 2021
        aoc_ins!(fn_map, AoCYear::AOC2021, AoCDay::AOCD01, year2021, day01);
        aoc_ins!(fn_map, AoCYear::AOC2021, AoCDay::AOCD02, year2021, day02);
        aoc_ins!(fn_map, AoCYear::AOC2021, AoCDay::AOCD03, year2021, day03);
        aoc_ins!(fn_map, AoCYear::AOC2021, AoCDay::AOCD04, year2021, day04);
        aoc_ins!(fn_map, AoCYear::AOC2021, AoCDay::AOCD05, year2021, day05);
        aoc_ins!(fn_map, AoCYear::AOC2021, AoCDay::AOCD06, year2021, day06);
        aoc_ins!(fn_map, AoCYear::AOC2021, AoCDay::AOCD07, year2021, day07);
        aoc_ins!(fn_map, AoCYear::AOC2021, AoCDay::AOCD08, year2021, day08);
        aoc_ins!(fn_map, AoCYear::AOC2021, AoCDay::AOCD09, year2021, day09);
        aoc_ins!(fn_map, AoCYear::AOC2021, AoCDay::AOCD10, year2021, day10);
        aoc_ins!(fn_map, AoCYear::AOC2021, AoCDay::AOCD11, year2021, day11);
        aoc_ins!(fn_map, AoCYear::AOC2021, AoCDay::AOCD12, year2021, day12);
        aoc_ins!(fn_map, AoCYear::AOC2021, AoCDay::AOCD13, year2021, day13);
        aoc_ins!(fn_map, AoCYear::AOC2021, AoCDay::AOCD14, year2021, day14);
        aoc_ins!(fn_map, AoCYear::AOC2021, AoCDay::AOCD15, year2021, day15);
        aoc_ins!(fn_map, AoCYear::AOC2021, AoCDay::AOCD16, year2021, day16);
        aoc_ins!(fn_map, AoCYear::AOC2021, AoCDay::AOCD17, year2021, day17);
        aoc_ins!(fn_map, AoCYear::AOC2021, AoCDay::AOCD18, year2021, day18);
        aoc_ins!(fn_map, AoCYear::AOC2021, AoCDay::AOCD19, year2021, day19);
        aoc_ins!(fn_map, AoCYear::AOC2021, AoCDay::AOCD20, year2021, day20);
        aoc_ins!(fn_map, AoCYear::AOC2021, AoCDay::AOCD21, year2021, day21);
        aoc_ins!(fn_map, AoCYear::AOC2021, AoCDay::AOCD22, year2021, day22);
        aoc_ins!(fn_map, AoCYear::AOC2021, AoCDay::AOCD23, year2021, day23);
        aoc_ins!(fn_map, AoCYear::AOC2021, AoCDay::AOCD24, year2021, day24);
        aoc_ins!(fn_map, AoCYear::AOC2021, AoCDay::AOCD25, year2021, day25);

        // 2022
        aoc_ins!(fn_map, AoCYear::AOC2022, AoCDay::AOCD01, year2022, day01);
        aoc_ins!(fn_map, AoCYear::AOC2022, AoCDay::AOCD02, year2022, day02);
        aoc_ins!(fn_map, AoCYear::AOC2022, AoCDay::AOCD03, year2022, day03);
        aoc_ins!(fn_map, AoCYear::AOC2022, AoCDay::AOCD04, year2022, day04);
        aoc_ins!(fn_map, AoCYear::AOC2022, AoCDay::AOCD05, year2022, day05);
        aoc_ins!(fn_map, AoCYear::AOC2022, AoCDay::AOCD06, year2022, day06);
        aoc_ins!(fn_map, AoCYear::AOC2022, AoCDay::AOCD07, year2022, day07);
        aoc_ins!(fn_map, AoCYear::AOC2022, AoCDay::AOCD08, year2022, day08);
        aoc_ins!(fn_map, AoCYear::AOC2022, AoCDay::AOCD09, year2022, day09);
        aoc_ins!(fn_map, AoCYear::AOC2022, AoCDay::AOCD10, year2022, day10);
        aoc_ins!(fn_map, AoCYear::AOC2022, AoCDay::AOCD11, year2022, day11);
        aoc_ins!(fn_map, AoCYear::AOC2022, AoCDay::AOCD12, year2022, day12);
        aoc_ins!(fn_map, AoCYear::AOC2022, AoCDay::AOCD13, year2022, day13);
        aoc_ins!(fn_map, AoCYear::AOC2022, AoCDay::AOCD14, year2022, day14);
        aoc_ins!(fn_map, AoCYear::AOC2022, AoCDay::AOCD15, year2022, day15);
        aoc_ins!(fn_map, AoCYear::AOC2022, AoCDay::AOCD16, year2022, day16);
        aoc_ins!(fn_map, AoCYear::AOC2022, AoCDay::AOCD17, year2022, day17);
        aoc_ins!(fn_map, AoCYear::AOC2022, AoCDay::AOCD18, year2022, day18);
        aoc_ins!(fn_map, AoCYear::AOC2022, AoCDay::AOCD19, year2022, day19);
        aoc_ins!(fn_map, AoCYear::AOC2022, AoCDay::AOCD20, year2022, day20);
        aoc_ins!(fn_map, AoCYear::AOC2022, AoCDay::AOCD21, year2022, day21);
        aoc_ins!(fn_map, AoCYear::AOC2022, AoCDay::AOCD22, year2022, day22);
        aoc_ins!(fn_map, AoCYear::AOC2022, AoCDay::AOCD23, year2022, day23);
        aoc_ins!(fn_map, AoCYear::AOC2022, AoCDay::AOCD24, year2022, day24);
        aoc_ins!(fn_map, AoCYear::AOC2022, AoCDay::AOCD25, year2022, day25);

        // 2023
        aoc_ins!(fn_map, AoCYear::AOC2023, AoCDay::AOCD01, year2023, day01);
        aoc_ins!(fn_map, AoCYear::AOC2023, AoCDay::AOCD02, year2023, day02);
        aoc_ins!(fn_map, AoCYear::AOC2023, AoCDay::AOCD03, year2023, day03);
        aoc_ins!(fn_map, AoCYear::AOC2023, AoCDay::AOCD04, year2023, day04);
        aoc_ins!(fn_map, AoCYear::AOC2023, AoCDay::AOCD05, year2023, day05);
        aoc_ins!(fn_map, AoCYear::AOC2023, AoCDay::AOCD06, year2023, day06);
        aoc_ins!(fn_map, AoCYear::AOC2023, AoCDay::AOCD07, year2023, day07);
        aoc_ins!(fn_map, AoCYear::AOC2023, AoCDay::AOCD08, year2023, day08);
        aoc_ins!(fn_map, AoCYear::AOC2023, AoCDay::AOCD09, year2023, day09);
        aoc_ins!(fn_map, AoCYear::AOC2023, AoCDay::AOCD10, year2023, day10);
        aoc_ins!(fn_map, AoCYear::AOC2023, AoCDay::AOCD11, year2023, day11);
        aoc_ins!(fn_map, AoCYear::AOC2023, AoCDay::AOCD12, year2023, day12);
        aoc_ins!(fn_map, AoCYear::AOC2023, AoCDay::AOCD13, year2023, day13);
        aoc_ins!(fn_map, AoCYear::AOC2023, AoCDay::AOCD14, year2023, day14);
        aoc_ins!(fn_map, AoCYear::AOC2023, AoCDay::AOCD15, year2023, day15);
        aoc_ins!(fn_map, AoCYear::AOC2023, AoCDay::AOCD16, year2023, day16);
        aoc_ins!(fn_map, AoCYear::AOC2023, AoCDay::AOCD17, year2023, day17);
        aoc_ins!(fn_map, AoCYear::AOC2023, AoCDay::AOCD18, year2023, day18);
        aoc_ins!(fn_map, AoCYear::AOC2023, AoCDay::AOCD19, year2023, day19);
        aoc_ins!(fn_map, AoCYear::AOC2023, AoCDay::AOCD20, year2023, day20);
        aoc_ins!(fn_map, AoCYear::AOC2023, AoCDay::AOCD21, year2023, day21);
        aoc_ins!(fn_map, AoCYear::AOC2023, AoCDay::AOCD22, year2023, day22);
        aoc_ins!(fn_map, AoCYear::AOC2023, AoCDay::AOCD23, year2023, day23);
        aoc_ins!(fn_map, AoCYear::AOC2023, AoCDay::AOCD24, year2023, day24);
        aoc_ins!(fn_map, AoCYear::AOC2023, AoCDay::AOCD25, year2023, day25);

        // 2024
        aoc_ins!(fn_map, AoCYear::AOC2024, AoCDay::AOCD01, year2024, day01);
        aoc_ins!(fn_map, AoCYear::AOC2024, AoCDay::AOCD02, year2024, day02);
        aoc_ins!(fn_map, AoCYear::AOC2024, AoCDay::AOCD03, year2024, day03);
        aoc_ins!(fn_map, AoCYear::AOC2024, AoCDay::AOCD04, year2024, day04);
        aoc_ins!(fn_map, AoCYear::AOC2024, AoCDay::AOCD05, year2024, day05);
        aoc_ins!(fn_map, AoCYear::AOC2024, AoCDay::AOCD06, year2024, day06);
        aoc_ins!(fn_map, AoCYear::AOC2024, AoCDay::AOCD07, year2024, day07);
        aoc_ins!(fn_map, AoCYear::AOC2024, AoCDay::AOCD08, year2024, day08);
        aoc_ins!(fn_map, AoCYear::AOC2024, AoCDay::AOCD09, year2024, day09);
        aoc_ins!(fn_map, AoCYear::AOC2024, AoCDay::AOCD10, year2024, day10);
        aoc_ins!(fn_map, AoCYear::AOC2024, AoCDay::AOCD11, year2024, day11);
        aoc_ins!(fn_map, AoCYear::AOC2024, AoCDay::AOCD12, year2024, day12);
        aoc_ins!(fn_map, AoCYear::AOC2024, AoCDay::AOCD13, year2024, day13);
        aoc_ins!(fn_map, AoCYear::AOC2024, AoCDay::AOCD14, year2024, day14);
        aoc_ins!(fn_map, AoCYear::AOC2024, AoCDay::AOCD15, year2024, day15);
        aoc_ins!(fn_map, AoCYear::AOC2024, AoCDay::AOCD16, year2024, day16);
        aoc_ins!(fn_map, AoCYear::AOC2024, AoCDay::AOCD17, year2024, day17);
        aoc_ins!(fn_map, AoCYear::AOC2024, AoCDay::AOCD18, year2024, day18);
        aoc_ins!(fn_map, AoCYear::AOC2024, AoCDay::AOCD19, year2024, day19);
        aoc_ins!(fn_map, AoCYear::AOC2024, AoCDay::AOCD20, year2024, day20);
        aoc_ins!(fn_map, AoCYear::AOC2024, AoCDay::AOCD21, year2024, day21);
        aoc_ins!(fn_map, AoCYear::AOC2024, AoCDay::AOCD22, year2024, day22);
        aoc_ins!(fn_map, AoCYear::AOC2024, AoCDay::AOCD23, year2024, day23);
        aoc_ins!(fn_map, AoCYear::AOC2024, AoCDay::AOCD24, year2024, day24);
        aoc_ins!(fn_map, AoCYear::AOC2024, AoCDay::AOCD25, year2024, day25);

        fn_map
    };
}
