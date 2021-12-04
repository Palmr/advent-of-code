/// --- Day 3: Binary Diagnostic ---
///
/// The submarine has been making some odd creaking noises, so you ask it to produce a diagnostic
/// report just in case.
///
/// The diagnostic report (your puzzle input) consists of a list of binary numbers which, when
/// decoded properly, can tell you many useful things about the conditions of the submarine.
/// The first parameter to check is the power consumption.
///
/// You need to use the binary numbers in the diagnostic report to generate two new binary numbers
/// (called the gamma rate and the epsilon rate).
/// The power consumption can then be found by multiplying the gamma rate by the epsilon rate.
///
/// Each bit in the gamma rate can be determined by finding the most common bit in the corresponding
/// position of all numbers in the diagnostic report.
/// For example, given the following diagnostic report:
///
/// 00100
/// 11110
/// 10110
/// 10111
/// 10101
/// 01111
/// 00111
/// 11100
/// 10000
/// 11001
/// 00010
/// 01010
///
/// Considering only the first bit of each number, there are five 0 bits and seven 1 bits.
/// Since the most common bit is 1, the first bit of the gamma rate is 1.
///
/// The most common second bit of the numbers in the diagnostic report is 0,
/// so the second bit of the gamma rate is 0.
///
/// The most common value of the third, fourth, and fifth bits are 1, 1, and 0, respectively,
/// and so the final three bits of the gamma rate are 110.
///
/// So, the gamma rate is the binary number 10110, or 22 in decimal.
///
/// The epsilon rate is calculated in a similar way; rather than use the most common bit, the least
/// common bit from each position is used. So, the epsilon rate is 01001, or 9 in decimal.
/// Multiplying the gamma rate (22) by the epsilon rate (9) produces the power consumption, 198.
///
/// Use the binary numbers in your diagnostic report to calculate the gamma rate and epsilon rate,
/// then multiply them together. What is the power consumption of the submarine?
/// (Be sure to represent your answer in decimal, not binary.)
///
/// --- Part Two ---
///
/// Next, you should verify the life support rating, which can be determined by multiplying the
/// oxygen generator rating by the CO2 scrubber rating.
///
/// Both the oxygen generator rating and the CO2 scrubber rating are values that can be found in
/// your diagnostic report - finding them is the tricky part.
/// Both values are located using a similar process that involves filtering out values until only
/// one remains. Before searching for either rating value, start with the full list of binary
/// numbers from your diagnostic report and consider just the first bit of those numbers. Then:
///
///     Keep only numbers selected by the bit criteria for the type of rating value for which you
///     are searching. Discard numbers which do not match the bit criteria.
///     If you only have one number left, stop; this is the rating value for which you are searching.
///     Otherwise, repeat the process, considering the next bit to the right.
///
/// The bit criteria depends on which type of rating value you want to find:
///
///     To find oxygen generator rating, determine the most common value (0 or 1) in the current
///     bit position, and keep only numbers with that bit in that position.
///     If 0 and 1 are equally common, keep values with a 1 in the position being considered.
///     To find CO2 scrubber rating, determine the least common value (0 or 1) in the current bit
///     position, and keep only numbers with that bit in that position. If 0 and 1 are equally
///     common, keep values with a 0 in the position being considered.
///
/// For example, to determine the oxygen generator rating value using the same example diagnostic
/// report from above:
///
///     Start with all 12 numbers and consider only the first bit of each number.
///     There are more 1 bits (7) than 0 bits (5), so keep only the 7 numbers with a 1 in the
///     first position: 11110, 10110, 10111, 10101, 11100, 10000, and 11001.
///     Then, consider the second bit of the 7 remaining numbers: there are more 0 bits (4) than
///     1 bits (3), so keep only the 4 numbers with a 0 in the second position:
///     10110, 10111, 10101, and 10000.
///     In the third position, three of the four numbers have a 1, so keep those three:
///     10110, 10111, and 10101.
///     In the fourth position, two of the three numbers have a 1, so keep those two:
///     10110 and 10111.
///     In the fifth position, there are an equal number of 0 bits and 1 bits (one each).
///     So, to find the oxygen generator rating, keep the number with a 1 in that position: 10111.
///     As there is only one number left, stop;
///     the oxygen generator rating is 10111, or 23 in decimal.
///
/// Then, to determine the CO2 scrubber rating value from the same example above:
///
///     Start again with all 12 numbers and consider only the first bit of each number.
///     There are fewer 0 bits (5) than 1 bits (7), so keep only the 5 numbers with a 0 in the
///     first position: 00100, 01111, 00111, 00010, and 01010.
///     Then, consider the second bit of the 5 remaining numbers: there are fewer 1 bits (2) than
///     0 bits (3), so keep only the 2 numbers with a 1 in the second position: 01111 and 01010.
///     In the third position, there are an equal number of 0 bits and 1 bits (one each).
///     So, to find the CO2 scrubber rating, keep the number with a 0 in that position: 01010.
///     As there is only one number left, stop; the CO2 scrubber rating is 01010, or 10 in decimal.
///
/// Finally, to find the life support rating, multiply the oxygen generator rating (23) by the CO2
/// scrubber rating (10) to get 230.
///
/// Use the binary numbers in your diagnostic report to calculate the oxygen generator rating and
/// CO2 scrubber rating, then multiply them together.
/// What is the life support rating of the submarine?
/// (Be sure to represent your answer in decimal, not binary.)

fn parse_inputs(input: &[String]) -> Vec<usize> {
    input
        .iter()
        .map(|diagnostic| usize::from_str_radix(diagnostic, 2).unwrap())
        .collect()
}

pub fn solve_part_one(input: &[String]) -> usize {
    let input_bits = input[0].len();

    let gamma = parse_inputs(input)
        .iter()
        .fold(vec![0; input_bits], |bit_counts, diagnostic| {
            bit_counts
                .iter()
                .enumerate()
                .map(|(bit_index, &bit_count)| bit_count + (diagnostic >> bit_index & 1))
                .collect()
        })
        .iter()
        .enumerate()
        // .inspect(|(bit_index, &bit_count)| println!("[{}] {}", bit_index, bit_count))
        .map(|(bit_index, &bit_count)| {
            if bit_count > input.len() / 2 {
                1 << bit_index
            } else {
                0
            }
        })
        .sum::<usize>();

    let epsilon = !gamma & ((1 << input_bits) - 1);

    gamma * epsilon
}

pub fn solve_part_two(input: &[String]) -> usize {
    let oxygen = find_rating_in_report(input, |count_of_bits_at_index, threshold| {
        count_of_bits_at_index >= threshold
    });
    let co2 = find_rating_in_report(input, |count_of_bits_at_index, threshold| {
        count_of_bits_at_index < threshold
    });

    oxygen * co2
}

fn find_rating_in_report<F>(input: &[String], bit_count_test: F) -> usize
where
    F: Fn(usize, usize) -> bool,
{
    let input_bits = input[0].len();
    (0..input_bits)
        .rev()
        .scan(parse_inputs(input), |matching_inputs, bit_index| {
            if matching_inputs.len() > 1 {
                let filter_for_ones = bit_count_test(
                    matching_inputs
                        .iter()
                        .filter(|&matching_input| matching_input & 1 << bit_index > 0)
                        .count(),
                    (matching_inputs.len() + 1) / 2,
                );
                matching_inputs
                    .retain(|&matching_input| (matching_input & 1 << bit_index > 0) == filter_for_ones);
            }
            matching_inputs.last().copied()
        })
        .last()
        .unwrap()
}

#[test]
fn examples_part_one() {
    assert_eq!(
        198,
        solve_part_one(&[
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ])
    );
}

#[test]
fn examples_part_two() {
    assert_eq!(
        230,
        solve_part_two(&[
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ])
    );
}
