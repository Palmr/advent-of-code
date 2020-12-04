use util;

mod day1;
mod day2;
mod day3;
mod day4;

pub fn aoc2020() {
    println!("Advent of Code 2020");

    println!(
        "\tDay 1: Part One={}, Part Two={}",
        day1::solve_part_one(&util::read_file_input("resources/2020/day1.txt")),
        day1::solve_part_two(&util::read_file_input("resources/2020/day1.txt")),
    );

    println!(
        "\tDay 2: Part One={}, Part Two={}",
        day2::solve_part_one(&util::read_file_input("resources/2020/day2.txt")),
        day2::solve_part_two(&util::read_file_input("resources/2020/day2.txt")),
    );

    println!(
        "\tDay 3: Part One={}, Part Two={}",
        day3::solve_part_one(&util::read_file_input("resources/2020/day3.txt")),
        day3::solve_part_two(&util::read_file_input("resources/2020/day3.txt")),
    );

    println!(
        "\tDay 4: Part One={}, Part Two={}",
        day4::solve_part_one(&util::read_file_input_incl_blanks(
            "resources/2020/day4.txt"
        )),
        day4::solve_part_two(&util::read_file_input_incl_blanks(
            "resources/2020/day4.txt"
        )),
    );
}
