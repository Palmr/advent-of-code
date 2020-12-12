use util;

mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

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

    println!(
        "\tDay 5: Part One={}, Part Two={}",
        day5::solve_part_one(&util::read_file_input("resources/2020/day5.txt")),
        day5::solve_part_two(&util::read_file_input("resources/2020/day5.txt")),
    );

    println!(
        "\tDay 6: Part One={}, Part Two={}",
        day6::solve_part_one(&util::read_file_input_incl_blanks(
            "resources/2020/day6.txt"
        )),
        day6::solve_part_two(&util::read_file_input_incl_blanks(
            "resources/2020/day6.txt"
        )),
    );

    println!(
        "\tDay 7: Part One={}, Part Two={}",
        day7::solve_part_one(&util::read_file_input("resources/2020/day7.txt")),
        day7::solve_part_two(&util::read_file_input("resources/2020/day7.txt")),
    );

    println!(
        "\tDay 8: Part One={}, Part Two={}",
        day8::solve_part_one(&util::read_file_input("resources/2020/day8.txt")),
        day8::solve_part_two(&util::read_file_input("resources/2020/day8.txt")),
    );

    println!(
        "\tDay 9: Part One={}, Part Two={}",
        day9::solve_part_one(&util::read_file_input("resources/2020/day9.txt")),
        day9::solve_part_two(&util::read_file_input("resources/2020/day9.txt")),
    );

    println!(
        "\tDay 10: Part One={}, Part Two={}",
        day10::solve_part_one(&util::read_file_input("resources/2020/day10.txt")),
        day10::solve_part_two(&util::read_file_input("resources/2020/day10.txt")),
    );
}
