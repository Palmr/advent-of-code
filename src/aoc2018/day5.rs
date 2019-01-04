///--- Day 5: Alchemical Reduction ---
///
/// You've managed to sneak in to the prototype suit manufacturing lab. The Elves are making decent
/// progress, but are still struggling with the suit's size reduction capabilities.
///
/// While the very latest in 1518 alchemical technology might have solved their problem eventually,
/// you can do better. You scan the chemical composition of the suit's material and discover that it
/// is formed by extremely long polymers (one of which is available as your puzzle input).
///
/// The polymer is formed by smaller units which, when triggered, react with each other such that
/// two adjacent units of the same type and opposite polarity are destroyed. Units' types are
/// represented by letters; units' polarity is represented by capitalization. For instance, r and R
/// are units with the same type but opposite polarity, whereas r and s are entirely different types
/// and do not react.
///
/// For example:
///
///     In aA, a and A react, leaving nothing behind.
///     In abBA, bB destroys itself, leaving aA. As above, this then destroys itself, leaving nothing.
///     In abAB, no two adjacent units are of the same type, and so nothing happens.
///     In aabAAB, even though aa and AA are of the same type, their polarities match, and so nothing happens.
///
/// Now, consider a larger example, dabAcCaCBAcCcaDA:
///
/// dabAcCaCBAcCcaDA  The first 'cC' is removed.
/// dabAaCBAcCcaDA    This creates 'Aa', which is removed.
/// dabCBAcCcaDA      Either 'cC' or 'Cc' are removed (the result is the same).
/// dabCBAcaDA        No further actions can be taken.
///
/// After all possible reactions, the resulting polymer contains 10 units.
///
/// How many units remain after fully reacting the polymer you scanned?

pub fn solve_part_one(input: &[String]) -> usize {
    let input = &input[0]; // One line only today

    react(input).len()
}

fn react(input: &str) -> String {
    let mut reduction = reduce(input);
    while reduction.len() >= 2 && reduction != reduce(&reduction) {
        reduction = reduce(&reduction);
    }
    reduction
}

fn reduce(input: &str) -> String {
    let mut reacted_polymer = String::new();
    let chars: Vec<char> = input.chars().collect();

    let mut idx = 0;
    while idx < chars.len() - 1 {
        let c1: char = chars[idx];
        let c2: char = chars[idx + 1];

        if c1 != c2 && c1.to_lowercase().next().unwrap() == c2.to_lowercase().next().unwrap() {
            // react and leave nothing
            idx += 2;
        } else {
            // don't react,
            reacted_polymer.push(c1);
            idx += 1;
        }
    }
    if idx < chars.len() {
        reacted_polymer.push(chars[idx]);
    }

    reacted_polymer
}

pub fn solve_part_two(_input: &[String]) -> String {
    "TODO".to_string()
}

#[test]
fn examples_part_one() {
    assert_eq!(10, solve_part_one(&["dabAcCaCBAcCcaDA".to_string()]));

    assert_eq!(0, solve_part_one(&["aA".to_string()]));
    assert_eq!(0, solve_part_one(&["abBA".to_string()]));
    assert_eq!(4, solve_part_one(&["abAB".to_string()]));
    assert_eq!(6, solve_part_one(&["aabAAB".to_string()]));
}

#[test]
fn examples_part_two() {}
