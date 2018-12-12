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

    println!(
        "\tDay 4: Part One={}, Part Two={}",
        aoc2018::day4::solve_part_one(&util::read_file_input("resources/2018/day4.txt")),
        aoc2018::day4::solve_part_two(&util::read_file_input("resources/2018/day4.txt")),
    );

    println!(
        "\tDay 5: Part One={}, Part Two={}",
        aoc2018::day5::solve_part_one(&util::read_file_input("resources/2018/day5.txt")),
        aoc2018::day5::solve_part_two(&util::read_file_input("resources/2018/day5.txt")),
    );

    println!(
        "\tDay 6: Part One={}, Part Two={}",
        aoc2018::day6::solve_part_one(&util::read_file_input("resources/2018/day6.txt")),
        aoc2018::day6::solve_part_two(&util::read_file_input("resources/2018/day6.txt")),
    );

    println!(
        "\tDay 7: Part One={}, Part Two={}",
        aoc2018::day7::solve_part_one(&util::read_file_input("resources/2018/day7.txt")),
        aoc2018::day7::solve_part_two(&util::read_file_input("resources/2018/day7.txt")),
    );

    println!(
        "\tDay 8: Part One={}, Part Two={}",
        aoc2018::day8::solve_part_one(&util::read_file_input("resources/2018/day8.txt")),
        aoc2018::day8::solve_part_two(&util::read_file_input("resources/2018/day8.txt")),
    );

    println!(
        "\tDay 9: Part One={}, Part Two={}",
        aoc2018::day9::solve_part_one(&util::read_file_input("resources/2018/day9.txt")),
        aoc2018::day9::solve_part_two(&util::read_file_input("resources/2018/day9.txt")),
    );

    println!(
        "\tDay 10: Part One={}, Part Two={}",
        aoc2018::day10::solve_part_one(&util::read_file_input("resources/2018/day10.txt")),
        aoc2018::day10::solve_part_two(&util::read_file_input("resources/2018/day10.txt")),
    );

    println!(
        "\tDay 11: Part One={}, Part Two={}",
        aoc2018::day11::solve_part_one(&util::read_file_input("resources/2018/day11.txt")),
        aoc2018::day11::solve_part_two(&util::read_file_input("resources/2018/day11.txt")),
    );

    println!(
        "\tDay 12: Part One={}, Part Two={}",
        aoc2018::day12::solve_part_one(&util::read_file_input("resources/2018/day12.txt")),
        aoc2018::day12::solve_part_two(&util::read_file_input("resources/2018/day12.txt")),
    );

    println!(
        "\tDay 13: Part One={}, Part Two={}",
        aoc2018::day13::solve_part_one(&util::read_file_input("resources/2018/day13.txt")),
        aoc2018::day13::solve_part_two(&util::read_file_input("resources/2018/day13.txt")),
    );

    println!(
        "\tDay 14: Part One={}, Part Two={}",
        aoc2018::day14::solve_part_one(&util::read_file_input("resources/2018/day14.txt")),
        aoc2018::day14::solve_part_two(&util::read_file_input("resources/2018/day14.txt")),
    );

    println!(
        "\tDay 15: Part One={}, Part Two={}",
        aoc2018::day15::solve_part_one(&util::read_file_input("resources/2018/day15.txt")),
        aoc2018::day15::solve_part_two(&util::read_file_input("resources/2018/day15.txt")),
    );

    println!(
        "\tDay 16: Part One={}, Part Two={}",
        aoc2018::day16::solve_part_one(&util::read_file_input("resources/2018/day16.txt")),
        aoc2018::day16::solve_part_two(&util::read_file_input("resources/2018/day16.txt")),
    );

    println!(
        "\tDay 17: Part One={}, Part Two={}",
        aoc2018::day17::solve_part_one(&util::read_file_input("resources/2018/day17.txt")),
        aoc2018::day17::solve_part_two(&util::read_file_input("resources/2018/day17.txt")),
    );

    println!(
        "\tDay 18: Part One={}, Part Two={}",
        aoc2018::day18::solve_part_one(&util::read_file_input("resources/2018/day18.txt")),
        aoc2018::day18::solve_part_two(&util::read_file_input("resources/2018/day18.txt")),
    );

    println!(
        "\tDay 19: Part One={}, Part Two={}",
        aoc2018::day19::solve_part_one(&util::read_file_input("resources/2018/day19.txt")),
        aoc2018::day19::solve_part_two(&util::read_file_input("resources/2018/day19.txt")),
    );

    println!(
        "\tDay 20: Part One={}, Part Two={}",
        aoc2018::day20::solve_part_one(&util::read_file_input("resources/2018/day20.txt")),
        aoc2018::day20::solve_part_two(&util::read_file_input("resources/2018/day20.txt")),
    );

    println!(
        "\tDay 21: Part One={}, Part Two={}",
        aoc2018::day21::solve_part_one(&util::read_file_input("resources/2018/day21.txt")),
        aoc2018::day21::solve_part_two(&util::read_file_input("resources/2018/day21.txt")),
    );

    println!(
        "\tDay 22: Part One={}, Part Two={}",
        aoc2018::day22::solve_part_one(&util::read_file_input("resources/2018/day22.txt")),
        aoc2018::day22::solve_part_two(&util::read_file_input("resources/2018/day22.txt")),
    );

    println!(
        "\tDay 23: Part One={}, Part Two={}",
        aoc2018::day23::solve_part_one(&util::read_file_input("resources/2018/day23.txt")),
        aoc2018::day23::solve_part_two(&util::read_file_input("resources/2018/day23.txt")),
    );

    println!(
        "\tDay 24: Part One={}, Part Two={}",
        aoc2018::day24::solve_part_one(&util::read_file_input("resources/2018/day24.txt")),
        aoc2018::day24::solve_part_two(&util::read_file_input("resources/2018/day24.txt")),
    );

    println!(
        "\tDay 25: Part One={}, Part Two={}",
        aoc2018::day25::solve_part_one(&util::read_file_input("resources/2018/day25.txt")),
        aoc2018::day25::solve_part_two(&util::read_file_input("resources/2018/day25.txt")),
    );
}
