extern crate aho_corasick;
extern crate chrono;
extern crate lazy_static;
extern crate nom;
extern crate regex;

use chrono::{Datelike, Utc};
use std::env;

mod aoc2017;
mod aoc2018;
mod aoc2019;
mod aoc2020;
mod aoc2021;
mod aoc2023;
mod util;

fn main() {
    let args: Vec<String> = env::args().collect();
    let year: i32 = args
        .get(1)
        .map_or(Utc::now().year(), |a| a.parse().unwrap());

    match year {
        2017 => aoc2017::aoc2017(),
        2018 => aoc2018::aoc2018(),
        2019 => aoc2019::aoc2019(),
        2020 => aoc2020::aoc2020(),
        2021 => aoc2021::aoc2021(),
        2023 => aoc2023::aoc2023(),
        _ => panic!("No advent of code solutions for {}", year),
    }
}
