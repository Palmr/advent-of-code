mod aoc2017;

fn main() {
    aoc2017();
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
