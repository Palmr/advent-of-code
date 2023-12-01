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
///
/// --- Part Two ---
///
/// Time to improve the polymer.
///
/// One of the unit types is causing problems; it's preventing the polymer from collapsing as much
/// as it should. Your goal is to figure out which unit type is causing the most problems, remove
/// all instances of it (regardless of polarity), fully react the remaining polymer, and measure its
/// length.
///
/// For example, again using the polymer dabAcCaCBAcCcaDA from above:
///
///     Removing all A/a units produces dbcCCBcCcD. Fully reacting this polymer produces dbCBcD, which has length 6.
///     Removing all B/b units produces daAcCaCAcCcaDA. Fully reacting this polymer produces daCAcaDA, which has length 8.
///     Removing all C/c units produces dabAaBAaDA. Fully reacting this polymer produces daDA, which has length 4.
///     Removing all D/d units produces abAcCaCBAcCcaA. Fully reacting this polymer produces abCBAc, which has length 6.
///
/// In this example, removing all C/c units was best, producing the answer 4.
///
/// What is the length of the shortest polymer you can produce by removing all units of exactly one
/// type and fully reacting the result?

pub fn solve_part_one(input: &[String]) -> usize {
    let input = &input[0]; // One line only today

    react(input).len()
}

pub fn solve_part_two(input: &[String]) -> usize {
    let input = &input[0]; // One line only today

    let mut shortest_len = input.len();
    let alphabet = String::from_utf8((b'a'..=b'z').collect()).unwrap();
    for c in alphabet.chars() {
        let x = input.replace([c, c.to_uppercase().next().unwrap()], "");
        let l = react(&x).len();
        if l < shortest_len {
            shortest_len = l;
        }
    }

    shortest_len
}

fn react(input: &str) -> String {
    let acc = String::with_capacity(input.len());

    input.chars().fold(acc, |mut acc, c| -> String {
        acc.push(c);

        loop {
            if acc.len() >= 2 {
                let mut chars = acc.chars().rev();
                let c1: char = chars.next().unwrap();
                let c2: char = chars.next().unwrap();

                if c1 != c2
                    && c1.to_lowercase().next().unwrap() == c2.to_lowercase().next().unwrap()
                {
                    acc.pop();
                    acc.pop();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        acc
    })
}

#[test]
fn test_react() {
    assert_eq!("aa", react("aa"));
    assert_eq!("AA", react("AA"));
    assert_eq!("", react("aA"));
    assert_eq!("b", react("aAb"));
    assert_eq!("b", react("baA"));
    assert_eq!("abA", react("abA"));
    assert_eq!("", react("abBA"));
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
fn examples_part_two() {
    assert_eq!(4, solve_part_two(&["dabAcCaCBAcCcaDA".to_string()]));
}
