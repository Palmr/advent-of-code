use util;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub fn aoc2018() {
    println!("Advent of Code 2018");

    println!(
        "\tDay 1: Part One={}, Part Two={}",
        day1::solve_part_one(&util::read_file_input("resources/2018/day1.txt")),
        day1::solve_part_two(&util::read_file_input("resources/2018/day1.txt")),
    );

    println!(
        "\tDay 2: Part One={}, Part Two={}",
        day2::solve_part_one(&util::read_file_input("resources/2018/day2.txt")),
        day2::solve_part_two(&util::read_file_input("resources/2018/day2.txt")),
    );

    println!(
        "\tDay 3: Part One={}, Part Two={}",
        day3::solve_part_one(&util::read_file_input("resources/2018/day3.txt")),
        day3::solve_part_two(&util::read_file_input("resources/2018/day3.txt")),
    );

    println!(
        "\tDay 4: Part One={}, Part Two={}",
        day4::solve_part_one(&util::read_file_input("resources/2018/day4.txt")),
        day4::solve_part_two(&util::read_file_input("resources/2018/day4.txt")),
    );

    println!(
        "\tDay 5: Part One={}, Part Two={}",
        day5::solve_part_one(&util::read_file_input("resources/2018/day5.txt")),
        day5::solve_part_two(&util::read_file_input("resources/2018/day5.txt")),
    );

    println!(
        "\tDay 6: Part One={}, Part Two={}",
        day6::solve_part_one(&util::read_file_input("resources/2018/day6.txt")),
        day6::solve_part_two(&util::read_file_input("resources/2018/day6.txt")),
    );

    println!(
        "\tDay 7: Part One={}, Part Two={}",
        day7::solve_part_one(&util::read_file_input("resources/2018/day7.txt")),
        day7::solve_part_two(&util::read_file_input("resources/2018/day7.txt")),
    );

    println!(
        "\tDay 8: Part One={}, Part Two={}",
        day8::solve_part_one(&util::read_file_input("resources/2018/day8.txt")),
        day8::solve_part_two(&util::read_file_input("resources/2018/day8.txt")),
    );

    println!(
        "\tDay 9: Part One={}, Part Two={}",
        day9::solve_part_one(&util::read_file_input("resources/2018/day9.txt")),
        day9::solve_part_two(&util::read_file_input("resources/2018/day9.txt")),
    );

    println!(
        "\tDay 10: Part One={}, Part Two={}",
        day10::solve_part_one(&util::read_file_input("resources/2018/day10.txt")),
        day10::solve_part_two(&util::read_file_input("resources/2018/day10.txt")),
    );

    println!(
        "\tDay 11: Part One={}, Part Two={}",
        day11::solve_part_one(&util::read_file_input("resources/2018/day11.txt")),
        day11::solve_part_two(&util::read_file_input("resources/2018/day11.txt")),
    );

    println!(
        "\tDay 12: Part One={}, Part Two={}",
        day12::solve_part_one(&util::read_file_input("resources/2018/day12.txt")),
        day12::solve_part_two(&util::read_file_input("resources/2018/day12.txt")),
    );

    println!(
        "\tDay 13: Part One={}, Part Two={}",
        day13::solve_part_one(&util::read_file_input("resources/2018/day13.txt")),
        day13::solve_part_two(&util::read_file_input("resources/2018/day13.txt")),
    );

    println!(
        "\tDay 14: Part One={}, Part Two={}",
        day14::solve_part_one(&util::read_file_input("resources/2018/day14.txt")),
        day14::solve_part_two(&util::read_file_input("resources/2018/day14.txt")),
    );

    println!(
        "\tDay 15: Part One={}, Part Two={}",
        day15::solve_part_one(&util::read_file_input("resources/2018/day15.txt")),
        day15::solve_part_two(&util::read_file_input("resources/2018/day15.txt")),
    );

    println!(
        "\tDay 16: Part One={}, Part Two={}",
        day16::solve_part_one(&util::read_file_input("resources/2018/day16.txt")),
        day16::solve_part_two(&util::read_file_input("resources/2018/day16.txt")),
    );

    println!(
        "\tDay 17: Part One={}, Part Two={}",
        day17::solve_part_one(&util::read_file_input("resources/2018/day17.txt")),
        day17::solve_part_two(&util::read_file_input("resources/2018/day17.txt")),
    );

    println!(
        "\tDay 18: Part One={}, Part Two={}",
        day18::solve_part_one(&util::read_file_input("resources/2018/day18.txt")),
        day18::solve_part_two(&util::read_file_input("resources/2018/day18.txt")),
    );

    println!(
        "\tDay 19: Part One={}, Part Two={}",
        day19::solve_part_one(&util::read_file_input("resources/2018/day19.txt")),
        day19::solve_part_two(&util::read_file_input("resources/2018/day19.txt")),
    );

    println!(
        "\tDay 20: Part One={}, Part Two={}",
        day20::solve_part_one(&util::read_file_input("resources/2018/day20.txt")),
        day20::solve_part_two(&util::read_file_input("resources/2018/day20.txt")),
    );

    println!(
        "\tDay 21: Part One={}, Part Two={}",
        day21::solve_part_one(&util::read_file_input("resources/2018/day21.txt")),
        day21::solve_part_two(&util::read_file_input("resources/2018/day21.txt")),
    );

    println!(
        "\tDay 22: Part One={}, Part Two={}",
        day22::solve_part_one(&util::read_file_input("resources/2018/day22.txt")),
        day22::solve_part_two(&util::read_file_input("resources/2018/day22.txt")),
    );

    println!(
        "\tDay 23: Part One={}, Part Two={}",
        day23::solve_part_one(&util::read_file_input("resources/2018/day23.txt")),
        day23::solve_part_two(&util::read_file_input("resources/2018/day23.txt")),
    );

    println!(
        "\tDay 24: Part One={}, Part Two={}",
        day24::solve_part_one(&util::read_file_input("resources/2018/day24.txt")),
        day24::solve_part_two(&util::read_file_input("resources/2018/day24.txt")),
    );

    println!(
        "\tDay 25: Part One={}, Part Two={}",
        day25::solve_part_one(&util::read_file_input("resources/2018/day25.txt")),
        day25::solve_part_two(&util::read_file_input("resources/2018/day25.txt")),
    );
}
