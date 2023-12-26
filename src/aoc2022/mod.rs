use util;
mod day1;

pub fn aoc2022() {
    println!("Advent of Code 2022");

    println!(
        "\tDay 1: Part One={}, Part Two={}",
        day1::solve_part_one(&util::read_file_input_incl_blanks(
            "resources/2022/day1.txt"
        )),
        day1::solve_part_two(&util::read_file_input_incl_blanks(
            "resources/2022/day1.txt"
        )),
    );
}
