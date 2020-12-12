/// # Day 5: Binary Boarding
///
/// You board your plane only to discover a new problem: you dropped your boarding pass! You aren't
/// sure which seat is yours, and all of the flight attendants are busy with the flood of people
/// that suddenly made it through passport control.
///
/// You write a quick program to use your phone's camera to scan all of the nearby boarding passes
/// (your puzzle input); perhaps you can find your seat through process of elimination.
///
/// Instead of zones or groups, this airline uses binary space partitioning to seat people. A seat
/// might be specified like `FBFBBFFRLR`, where `F` means "front", `B` means "back", `L` means "
/// left", and `R` means "right".
///
/// The first 7 characters will either be `F` or `B`; these specify exactly one of the 128 rows on
/// the plane (numbered 0 through 127). Each letter tells you which half of a region the given seat
/// is in. Start with the whole list of rows; the first letter indicates whether the seat is in the
/// front (0 through 63) or the back (64 through 127). The next letter indicates which half of that
/// region the seat is in, and so on until you're left with exactly one row.
///
/// For example, consider just the first seven characters of `FBFBBFFRLR`:
///
/// ```
///     Start by considering the whole range, rows 0 through 127.
///     F means to take the lower half, keeping rows 0 through 63.
///     B means to take the upper half, keeping rows 32 through 63.
///     F means to take the lower half, keeping rows 32 through 47.
///     B means to take the upper half, keeping rows 40 through 47.
///     B keeps rows 44 through 47.
///     F keeps rows 44 through 45.
///     The final F keeps the lower of the two, row 44.
/// ```
///
/// The last three characters will be either `L` or `R`; these specify exactly one of the 8 columns
/// of seats on the plane (numbered 0 through 7). The same process as above proceeds again, this
/// time with only three steps. `L` means to keep the lower half, while `R` means to keep the upper
/// half.
///
/// For example, consider just the last 3 characters of `FBFBBFFRLR`:
///
/// ```
///     Start by considering the whole range, columns 0 through 7.
///     R means to take the upper half, keeping columns 4 through 7.
///     L means to take the lower half, keeping columns 4 through 5.
///     The final R keeps the upper of the two, column 5.
/// ```
///
/// So, decoding `FBFBBFFRLR` reveals that it is the seat at row 44, column 5.
///
/// Every seat also has a unique seat ID: multiply the row by 8, then add the column. In this
/// example, the seat has ID `44 * 8 + 5 = 357`.
///
/// Here are some other boarding passes:
///
/// ```
///     BFFFBBFRRR: row 70, column 7, seat ID 567.
///     FFFBBBFRRR: row 14, column 7, seat ID 119.
///     BBFFBBFRLL: row 102, column 4, seat ID 820.
/// ```
///
/// As a sanity check, look through your list of boarding passes. What is the highest seat ID on a
/// boarding pass?
///
/// ## Part Two
///
/// Ding! The "fasten seat belt" signs have turned on. Time to find your seat.
///
/// It's a completely full flight, so your seat should be the only missing boarding pass in your
/// list. However, there's a catch: some of the seats at the very front and back of the plane don't
/// exist on this aircraft, so they'll be missing from your list as well.
///
/// Your seat wasn't at the very front or back, though; the seats with IDs +1 and -1 from yours
/// will be in your list.
///
/// What is the ID of your seat?

/// This was my initial naive solution, taking the break down explanation of how the row/columns are
/// calculated and turning it in to code.
#[allow(dead_code)]
fn binary_space_partition(input: &str, value_range: (usize, usize)) -> usize {
    let range = input.chars().fold(value_range, |range, c| {
        let half = (range.1 - range.0) / 2;
        match c {
            'F' | 'L' => (range.0, range.0 + half),
            'B' | 'R' => (range.1 - half, range.1),
            _ => panic!("Invalid char: {}", c),
        }
    });
    assert_eq!(range.0, range.1);

    range.0
}

/// This is a fancier solution that Simon came up with, noticing that 7 chars -> 0-127 and
/// 3 chars -> 0-7 matched 7b111_1111 == 127 and 3b111 == 3
///
/// This means each "take the upper half" of a range can be converted to a binary 1 and each "take
/// the lower half" can be a binary 0.
fn simon_solution(input: &str) -> usize {
    let result = input.chars().fold((0, 0), |(value, idx), c| {
        (
            value << 1
                | match c {
                    'F' | 'L' => 0,
                    'B' | 'R' => 1,
                    _ => panic!("Invalid char: {}", c),
                },
            idx + 1,
        )
    });

    result.0
}

fn calculate_seat_id(input: &str) -> usize {
    let row = simon_solution(&input[0..7]);
    let col = simon_solution(&input[7..10]);

    row * 8 + col
}

pub fn solve_part_one(input: &[String]) -> usize {
    input
        .iter()
        .map(|s| calculate_seat_id(s.as_str()))
        .max()
        .unwrap_or(0)
}

pub fn solve_part_two(input: &[String]) -> usize {
    let mut seat_ids: Vec<usize> = input
        .iter()
        .map(|s| calculate_seat_id(s.as_str()))
        .collect();
    seat_ids.sort_unstable();

    for (seat_id, next_seat_id) in seat_ids.windows(2).map(|window| (window[0], window[1])) {
        let next_expected_seat_id = seat_id + 1;
        if next_expected_seat_id != next_seat_id {
            return next_expected_seat_id;
        }
    }

    0
}

#[test]
fn test_row_parse() {
    assert_eq!(44, binary_space_partition("FBFBBFF", (0, 127)));
    assert_eq!(70, binary_space_partition("BFFFBBF", (0, 127)));
    assert_eq!(14, binary_space_partition("FFFBBBF", (0, 127)));
    assert_eq!(102, binary_space_partition("BBFFBBF", (0, 127)));
}

#[test]
fn test_col_parse() {
    assert_eq!(5, binary_space_partition("RLR", (0, 7)));
    assert_eq!(7, binary_space_partition("RRR", (0, 7)));
    assert_eq!(7, binary_space_partition("RRR", (0, 7)));
    assert_eq!(4, binary_space_partition("RLL", (0, 7)));
}

#[test]
fn test_simon_solution_same_as_naive_bsp() {
    assert_eq!(
        binary_space_partition("FBFBBFF", (0, 127)),
        simon_solution("FBFBBFF")
    );
    assert_eq!(
        binary_space_partition("BFFFBBF", (0, 127)),
        simon_solution("BFFFBBF")
    );
    assert_eq!(
        binary_space_partition("FFFBBBF", (0, 127)),
        simon_solution("FFFBBBF")
    );
    assert_eq!(
        binary_space_partition("BBFFBBF", (0, 127)),
        simon_solution("BBFFBBF")
    );
    assert_eq!(binary_space_partition("RLR", (0, 7)), simon_solution("RLR"));
    assert_eq!(binary_space_partition("RRR", (0, 7)), simon_solution("RRR"));
    assert_eq!(binary_space_partition("RRR", (0, 7)), simon_solution("RRR"));
    assert_eq!(binary_space_partition("RLL", (0, 7)), simon_solution("RLL"));
}

#[test]
fn examples_part_one() {
    assert_eq!(567, solve_part_one(&["BFFFBBFRRR".to_string()]));
    assert_eq!(119, solve_part_one(&["FFFBBBFRRR".to_string()]));
    assert_eq!(820, solve_part_one(&["BBFFBBFRLL".to_string()]));
    assert_eq!(
        820,
        solve_part_one(&[
            "BFFFBBFRRR".to_string(),
            "FFFBBBFRRR".to_string(),
            "BBFFBBFRLL".to_string(),
        ])
    );
}

#[test]
fn examples_part_two() {
    assert_eq!(
        120,
        solve_part_two(&[
            "BFFFBBFRRR".to_string(),
            "FFFBBBFRRR".to_string(),
            "BBFFBBFRLL".to_string(),
        ])
    );
}
