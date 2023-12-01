use std::collections::HashMap;

use aho_corasick::AhoCorasick;

/// --- Day 1: Trebuchet?! ---
///
/// Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.
///
/// You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.
///
/// Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
///
/// You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").
///
/// As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been amended by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are having trouble reading the values on the document.
///
/// The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.
///
/// For example:
///
/// 1abc2
/// pqr3stu8vwx
/// a1b2c3d4e5f
/// treb7uchet
///
/// In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.
///
/// Consider your entire calibration document. What is the sum of all of the calibration values?
///
/// --- Part Two ---
///
/// Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".
///
/// Equipped with this new information, you now need to find the real first and last digit on each line. For example:
///
/// two1nine
/// eightwothree
/// abcone2threexyz
/// xtwone3four
/// 4nineeightseven2
/// zoneight234
/// 7pqrstsixteen
///
/// In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.
///
/// What is the sum of all of the calibration values?
///

fn char_to_int(char: char) -> usize {
    char as usize - '0' as usize
}

pub fn solve_part_one(input: &[String]) -> usize {
    input
        .iter()
        .map(|l| {
            let first = l.find(char::is_numeric).unwrap();
            let last = l.rfind(char::is_numeric).unwrap();
            let chars: Vec<char> = l.chars().collect();
            char_to_int(chars[first]) * 10 + char_to_int(chars[last])
        })
        .sum()
}

pub fn solve_part_two(input: &[String]) -> usize {
    let mut digits_lookup: HashMap<&str, usize> = HashMap::new();
    digits_lookup.insert("one", 1);
    digits_lookup.insert("two", 2);
    digits_lookup.insert("three", 3);
    digits_lookup.insert("four", 4);
    digits_lookup.insert("five", 5);
    digits_lookup.insert("six", 6);
    digits_lookup.insert("seven", 7);
    digits_lookup.insert("eight", 8);
    digits_lookup.insert("nine", 9);
    digits_lookup.insert("1", 1);
    digits_lookup.insert("2", 2);
    digits_lookup.insert("3", 3);
    digits_lookup.insert("4", 4);
    digits_lookup.insert("5", 5);
    digits_lookup.insert("6", 6);
    digits_lookup.insert("7", 7);
    digits_lookup.insert("8", 8);
    digits_lookup.insert("9", 9);

    let patterns = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];
    let ac = AhoCorasick::new(patterns).unwrap();

    input
        .iter()
        .map(|l| {
            let mut iter = ac.find_overlapping_iter(l);
            let first = iter
                .next()
                .map(|m| *digits_lookup.get(patterns[m.pattern().as_usize()]).unwrap())
                .unwrap();
            let last = iter.last().map_or(first, |m| {
                *digits_lookup.get(patterns[m.pattern().as_usize()]).unwrap()
            });
            first * 10 + last
        })
        .sum()
}

#[test]
fn examples_part_one() {
    assert_eq!(
        142,
        solve_part_one(&[
            "1abc2".to_string(),
            "pqr3stu8vwx".to_string(),
            "a1b2c3d4e5f".to_string(),
            "treb7uchet".to_string(),
        ])
    );
}

#[test]
fn examples_part_two() {
    assert_eq!(
        281,
        solve_part_two(&[
            "two1nine".to_string(),
            "eightwothree".to_string(),
            "abcone2threexyz".to_string(),
            "xtwone3four".to_string(),
            "4nineeightseven2".to_string(),
            "zoneight234".to_string(),
            "7pqrstsixteen".to_string(),
        ])
    );
}
