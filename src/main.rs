extern crate regex;

mod aoc2017;
mod aoc2018;
mod util;

fn main() {
    aoc2017();
    aoc2018();
}

fn aoc2017() {
    println!("Advent of Code 2017");

    println!(
        "\tDay 1: Part One={}, Part Two={}",
        aoc2017::day1::solve_part_one(aoc2017::DAY1_INPUT),
        aoc2017::day1::solve_part_two(aoc2017::DAY1_INPUT),
    );
    println!(
        "\tDay 2: Part One={}, Part Two={}",
        aoc2017::day2::solve_part_one(aoc2017::DAY2_INPUT),
        aoc2017::day2::solve_part_two(aoc2017::DAY2_INPUT),
    );
}

fn aoc2018() {
    println!("Advent of Code 2018");

    println!(
        "\tDay 1: Part One={}, Part Two={}",
        aoc2018::day1::solve_part_one(&util::read_file_input("resources/2018/day1.txt")),
        aoc2018::day1::solve_part_two(&util::read_file_input("resources/2018/day1.txt")),
    );

    println!(
        "\tDay 2: Part One={}, Part Two={}",
        aoc2018::day2::solve_part_one(&util::read_file_input("resources/2018/day2.txt")),
        aoc2018::day2::solve_part_two(&util::read_file_input("resources/2018/day2.txt")),
    );

    println!(
        "\tDay 3: Part One={}, Part Two={}",
        aoc2018::day3::solve_part_one(&util::read_file_input("resources/2018/day3.txt")),
        aoc2018::day3::solve_part_two(&util::read_file_input("resources/2018/day3.txt")),
    );
}
