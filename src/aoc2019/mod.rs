use util;

mod day2;

pub fn aoc2019() {
    println!("Advent of Code 2019");

    println!(
        "\tDay 2: Part One={}, Part Two={}",
        day2::solve_part_one(&util::read_file_input("resources/2019/day2.txt"), day2::part_1_mangling),
        day2::solve_part_two(&util::read_file_input("resources/2019/day2.txt")),
    );
}
