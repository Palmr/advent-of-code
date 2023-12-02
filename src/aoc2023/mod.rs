use util;
mod day1;
mod day2;

pub fn aoc2023() {
    println!("Advent of Code 2023");

    println!(
        "\tDay 1: Part One={}, Part Two={}",
        day1::solve_part_one(&util::read_file_input("resources/2023/day1.txt")),
        day1::solve_part_two(&util::read_file_input("resources/2023/day1.txt")),
    );

    println!(
        "\tDay 2: Part One={}, Part Two={}",
        day2::solve_part_one(&util::read_file_input("resources/2023/day2.txt")),
        day2::solve_part_two(&util::read_file_input("resources/2023/day2.txt")),
    );
}
