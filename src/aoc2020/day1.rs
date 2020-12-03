/// --- Day 1: Report Repair ---
///
/// After saving Christmas five years in a row, you've decided to take a vacation at a nice resort
/// on a tropical island. Surely, Christmas will go on without you.
///
/// The tropical island has its own currency and is entirely cash-only. The gold coins used there
/// have a little picture of a starfish; the locals just call them stars. None of the currency
/// exchanges seem to have heard of them, but somehow, you'll need to find fifty of these coins by
/// the time you arrive so you can pay the deposit on your room.
///
/// To save your vacation, you need to get all fifty stars by December 25th.
///
/// Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent
/// calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one
/// star. Good luck!
///
/// Before you leave, the Elves in accounting just need you to fix your expense report (your puzzle
/// input); apparently, something isn't quite adding up.
///
/// Specifically, they need you to find the two entries that sum to 2020 and then multiply those two
/// numbers together.
///
/// For example, suppose your expense report contained the following:
///
/// 1721
/// 979
/// 366
/// 299
/// 675
/// 1456
///
/// In this list, the two entries that sum to 2020 are 1721 and 299. Multiplying them together
/// produces 1721 * 299 = 514579, so the correct answer is 514579.
///
/// Of course, your expense report is much larger. Find the two entries that sum to 2020; what do
/// you get if you multiply them together?
///
/// --- Part Two ---
///
/// The Elves in accounting are thankful for your help; one of them even offers you a starfish coin
/// they had left over from a past vacation. They offer you a second one if you can find three
/// numbers in your expense report that meet the same criteria.
///
/// Using the above example again, the three entries that sum to 2020 are 979, 366, and 675.
/// Multiplying them together produces the answer, 241861950.
///
/// In your expense report, what is the product of the three entries that sum to 2020?
///

fn parse_input(input: &[String]) -> Vec<isize> {
    input
        .iter()
        .map(|c| c.trim().parse::<isize>().unwrap())
        .collect()
}

pub fn solve_part_one(input: &[String]) -> isize {
    let expenses = parse_input(input);
    // Bjørn, in register 0, is fat and lazy. He ambles along like his namesake.
    for bjorn in 0..expenses.len() {
        // One will be the fast iterator. Her name is Jorunn, and her legs are strong from years of skiing. She flies forward with powerful strokes.
        for jorunn in bjorn..expenses.len() {
            let expense_a = expenses[bjorn];
            let expense_b = expenses[jorunn];
            if expense_a + expense_b == 2020 {
                return expense_a * expense_b;
            }
        }
    }

    -1
}

pub fn solve_part_two(input: &[String]) -> isize {
    let expenses = parse_input(input);
    // Bjørn, in register 0, is fat and lazy. He ambles along like his namesake.
    for bjorn in 0..expenses.len() {
        // One will be the fast iterator. Her name is Jorunn, and her legs are strong from years of skiing. She flies forward with powerful strokes.
        for jorunn in bjorn..expenses.len() {
            // Simons girlfriend, potentially quite fast, though not scandi, Finnish is near enough
            for laura in bjorn..expenses.len() {
                let expense_a = expenses[bjorn];
                let expense_b = expenses[jorunn];
                let expense_c = expenses[laura];
                if expense_a + expense_b + expense_c == 2020 {
                    return expense_a * expense_b * expense_c;
                }
            }
        }
    }

    -1
}

#[test]
fn examples_part_one() {
    assert_eq!(
        514579,
        solve_part_one(&[
            "1721".to_string(),
            "979".to_string(),
            "366".to_string(),
            "299".to_string(),
            "675".to_string(),
            "1456".to_string(),
        ])
    );
}

#[test]
fn examples_part_two() {
    assert_eq!(
        241861950,
        solve_part_two(&[
            "1721".to_string(),
            "979".to_string(),
            "366".to_string(),
            "299".to_string(),
            "675".to_string(),
            "1456".to_string(),
        ])
    );
}
