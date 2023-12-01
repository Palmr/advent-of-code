use util;
mod day1;

pub fn aoc2023() {
    println!("Advent of Code 2023");

    println!(
        "\tDay 1: Part One={}, Part Two={}",
        // day1::solve_part_one(&util::read_file_input("resources/2023/day1.txt")),
        1,
        day1::solve_part_two(&util::read_file_input("resources/2023/day1.txt")),
    );
}
