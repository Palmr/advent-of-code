/// --- Day 2: Password Philosophy ---
///
/// Your flight departs in a few days from the coastal airport; the easiest way down to the coast
/// from here is via toboggan.
///
/// The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day. "Something's wrong
/// with our computers; we can't log in!" You ask if you can take a look.
///
/// Their password database seems to be a little corrupted: some of the passwords wouldn't have been
/// allowed by the Official Toboggan Corporate Policy that was in effect when they were chosen.
///
/// To try to debug the problem, they have created a list (your puzzle input) of passwords
/// (according to the corrupted database) and the corporate policy when that password was set.
///
/// For example, suppose you have the following list:
///
/// 1-3 a: abcde
/// 1-3 b: cdefg
/// 2-9 c: ccccccccc
///
/// Each line gives the password policy and then the password. The password policy indicates the
/// lowest and highest number of times a given letter must appear for the password to be valid. For
/// example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.
///
/// In the above example, 2 passwords are valid. The middle password, cdefg, is not; it contains no
/// instances of b, but needs at least 1. The first and third passwords are valid: they contain one
/// a or nine c, both within the limits of their respective policies.
///
/// How many passwords are valid according to their policies?
///
/// --- Part Two ---
///
/// While it appears you validated the passwords correctly, they don't seem to be what the Official
/// Toboggan Corporate Authentication System is expecting.
///
/// The shopkeeper suddenly realizes that he just accidentally explained the password policy rules
/// from his old job at the sled rental place down the street! The Official Toboggan Corporate
/// Policy actually works a little differently.
///
/// Each policy actually describes two positions in the password, where 1 means the first character,
/// 2 means the second character, and so on. (Be careful; Toboggan Corporate Policies have no
/// concept of "index zero"!) Exactly one of these positions must contain the given letter. Other
/// occurrences of the letter are irrelevant for the purposes of policy enforcement.
///
/// Given the same example list from above:
///
///     1-3 a: abcde is valid: position 1 contains a and position 3 does not.
///     1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
///     2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.
///
/// How many passwords are valid according to the new interpretation of the policies?
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_PASSWORD_RULE: Regex = Regex::new(r"^(\d+)-(\d+) ([a-z]): (.+)$").unwrap();
}

fn split_input(input: &str) -> (usize, usize, char, String) {
    let chunks = RE_PASSWORD_RULE.captures(input).unwrap();
    let rule1: usize = chunks[1].parse().unwrap();
    let rule2: usize = chunks[2].parse().unwrap();
    let char_test: char = chunks[3].parse().unwrap();
    let password = chunks[4].to_string();

    (rule1, rule2, char_test, password)
}

fn is_password_valid_policy1(input: &(usize, usize, char, String)) -> bool {
    let (low, high, char_test, password) = input;
    let char_test_count = &password.chars().filter(|c| c == char_test).count();

    char_test_count >= low && char_test_count <= high
}

fn is_password_valid_policy2(input: &(usize, usize, char, String)) -> bool {
    let (pos1, pos2, char_test, password) = input;

    let pos1_char = password.chars().nth(pos1 - 1);
    let pos2_char = password.chars().nth(pos2 - 1);

    let pos1_has_char = pos1_char.map(|c| &c == char_test).unwrap_or(false);
    let pos2_has_char = pos2_char.map(|c| &c == char_test).unwrap_or(false);

    pos1_has_char ^ pos2_has_char
}

pub fn solve_part_one(input: &[String]) -> usize {
    input
        .iter()
        .map(String::as_str)
        .map(split_input)
        .filter(is_password_valid_policy1)
        .count()
}

pub fn solve_part_two(input: &[String]) -> usize {
    input
        .iter()
        .map(String::as_str)
        .map(split_input)
        .filter(is_password_valid_policy2)
        .count()
}

#[test]
fn examples_part_one() {
    assert_eq!(
        2,
        solve_part_one(&[
            "1-3 a: abcde".to_string(),
            "1-3 b: cdefg".to_string(),
            "2-9 c: ccccccccc".to_string(),
        ])
    );
}

#[test]
fn examples_part_two() {
    assert_eq!(
        1,
        solve_part_two(&[
            "1-3 a: abcde".to_string(),
            "1-3 b: cdefg".to_string(),
            "2-9 c: ccccccccc".to_string(),
        ])
    );
}
