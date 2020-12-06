use std::collections::{HashSet, HashMap};

/// --- Day 6: Custom Customs ---
///
/// As your flight approaches the regional airport where you'll switch to a much larger plane,
/// customs declaration forms are distributed to the passengers.
///
/// The form asks a series of 26 yes-or-no questions marked a through z. All you need to do is
/// identify the questions for which anyone in your group answers "yes". Since your group is just
/// you, this doesn't take very long.
///
/// However, the person sitting next to you seems to be experiencing a language barrier and asks if
/// you can help. For each of the people in their group, you write down the questions for which they
/// answer "yes", one per line. For example:
///
/// abcx
/// abcy
/// abcz
///
/// In this group, there are 6 questions to which anyone answered "yes": a, b, c, x, y, and z.
/// (Duplicate answers to the same question don't count extra; each question counts at most once.)
///
/// Another group asks for your help, then another, and eventually you've collected answers from
/// every group on the plane (your puzzle input). Each group's answers are separated by a blank
/// line, and within each group, each person's answers are on a single line. For example:
///
/// abc
///
/// a
/// b
/// c
///
/// ab
/// ac
///
/// a
/// a
/// a
/// a
///
/// b
///
/// This list represents answers from five groups:
///
///     The first group contains one person who answered "yes" to 3 questions: a, b, and c.
///     The second group contains three people; combined, they answered "yes" to 3 questions: a, b, and c.
///     The third group contains two people; combined, they answered "yes" to 3 questions: a, b, and c.
///     The fourth group contains four people; combined, they answered "yes" to only 1 question, a.
///     The last group contains one person who answered "yes" to only 1 question, b.
///
/// In this example, the sum of these counts is 3 + 3 + 3 + 1 + 1 = 11.
///
/// For each group, count the number of questions to which anyone answered "yes". What is the sum of
/// those counts?
///
/// --- Part Two ---
///
/// As you finish the last group's customs declaration, you notice that you misread one word in the
/// instructions:
///
/// You don't need to identify the questions to which anyone answered "yes"; you need to identify
/// the questions to which everyone answered "yes"!
///
/// Using the same example as above:
///
/// abc
///
/// a
/// b
/// c
///
/// ab
/// ac
///
/// a
/// a
/// a
/// a
///
/// b
///
/// This list represents answers from five groups:
///
///     In the first group, everyone (all 1 person) answered "yes" to 3 questions: a, b, and c.
///     In the second group, there is no question to which everyone answered "yes".
///     In the third group, everyone answered yes to only 1 question, a. Since some people did not
///         answer "yes" to b or c, they don't count.
///     In the fourth group, everyone answered yes to only 1 question, a.
///     In the fifth group, everyone (all 1 person) answered "yes" to 1 question, b.
///
/// In this example, the sum of these counts is 3 + 0 + 1 + 1 + 1 = 6.
///
/// For each group, count the number of questions to which everyone answered "yes". What is the sum of those counts?

pub fn solve_part_one(input: &[String]) -> usize {
    let mut answers_counts = Vec::new();
    let mut answers = HashSet::new();

    for line in input {
        if line.is_empty() {
            answers_counts.push(answers.len());
            answers.clear();
        } else {
            line.chars().for_each(|c| {answers.insert(c);});
        }
    }
    answers_counts.push(answers.len());

    answers_counts.iter().sum()
}

pub fn solve_part_two(input: &[String]) -> usize {

    let mut answers_counts = Vec::new();

    let mut answers: HashMap<char, usize> = HashMap::new();
    let mut group_people_count: usize = 0;

    for line in input {
        if line.is_empty() {
            answers_counts.push(answers.values().filter(|&v| v == &group_people_count).count());
            answers.clear();
            group_people_count = 0;
        } else {
            line.chars().for_each(|c| {
                let previous_count = answers.get(&c).unwrap_or(&0);
                answers.insert(c, previous_count + 1);
            });
            group_people_count += 1;
        }
    }
    answers_counts.push(answers.values().filter(|&v| v == &group_people_count).count());

    answers_counts.iter().sum()
}

#[test]
fn examples_part_one() {
    assert_eq!(
        11,
        solve_part_one(&[
            "abc".to_string(),
            "".to_string(),
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "".to_string(),
            "ab".to_string(),
            "ac".to_string(),
            "".to_string(),
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
            "".to_string(),
            "b".to_string(),
        ])
    );
}

#[test]
fn examples_part_two() {
    assert_eq!(
        6,
        solve_part_two(&[
            "abc".to_string(),
            "".to_string(),
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "".to_string(),
            "ab".to_string(),
            "ac".to_string(),
            "".to_string(),
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
            "".to_string(),
            "b".to_string(),
        ])
    );
}
