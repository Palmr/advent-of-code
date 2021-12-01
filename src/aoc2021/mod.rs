use util;
mod day1;

pub fn aoc2021() {
    println!("Advent of Code 2021");

    println!(
        "\tDay 1: Part One={}, Part Two={}",
        day1::solve_part_one(&util::read_file_input("resources/2021/day1.txt")),
        day1::solve_part_two(&util::read_file_input("resources/2021/day1.txt")),
    );
}
