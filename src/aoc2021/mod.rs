use util;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

pub fn aoc2021() {
    println!("Advent of Code 2021");

    println!(
        "\tDay 1: Part One={}, Part Two={}",
        day1::solve_part_one(&util::read_file_input("resources/2021/day1.txt")),
        day1::solve_part_two(&util::read_file_input("resources/2021/day1.txt")),
    );

    println!(
        "\tDay 2: Part One={}, Part Two={}",
        day2::solve_part_one(&util::read_file_input("resources/2021/day2.txt")),
        day2::solve_part_two(&util::read_file_input("resources/2021/day2.txt")),
    );

    println!(
        "\tDay 3: Part One={}, Part Two={}",
        day3::solve_part_one(&util::read_file_input("resources/2021/day3.txt")),
        day3::solve_part_two(&util::read_file_input("resources/2021/day3.txt")),
    );

    println!(
        "\tDay 4: Part One={}, Part Two={}",
        day4::solve_part_one(&util::read_file_input("resources/2021/day4.txt")),
        day4::solve_part_two(&util::read_file_input("resources/2021/day4.txt")),
    );

    println!(
        "\tDay 5: Part One={}, Part Two={}",
        day5::solve_part_one(&util::read_file_input("resources/2021/day5.txt")),
        day5::solve_part_two(&util::read_file_input("resources/2021/day5.txt")),
    );

    println!(
        "\tDay 6: Part One={}, Part Two={}",
        day6::solve_part_one(&util::read_file_input("resources/2021/day6.txt")),
        day6::solve_part_two(&util::read_file_input("resources/2021/day6.txt")),
    );

    println!(
        "\tDay 7: Part One={}, Part Two={}",
        day7::solve_part_one(&util::read_file_input("resources/2021/day7.txt")),
        day7::solve_part_two(&util::read_file_input("resources/2021/day7.txt")),
    );
}
