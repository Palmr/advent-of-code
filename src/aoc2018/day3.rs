use regex::Regex;

/// --- Day 3: No Matter How You Slice It ---
///
/// The Elves managed to locate the chimney-squeeze prototype fabric for Santa's suit (thanks to
/// someone who helpfully wrote its box IDs on the wall of the warehouse in the middle of the
/// night). Unfortunately, anomalies are still affecting them - nobody can even agree on how to
/// cut the fabric.
///
/// The whole piece of fabric they're working on is a very large square - at least 1000 inches on
/// each side.
///
/// Each Elf has made a claim about which area of fabric would be ideal for Santa's suit. All claims
/// have an ID and consist of a single rectangle with edges parallel to the edges of the fabric.
/// Each claim's rectangle is defined as follows:
///
/// - The number of inches between the left edge of the fabric and the left edge of the rectangle.
/// - The number of inches between the top edge of the fabric and the top edge of the rectangle.
/// - The width of the rectangle in inches.
/// - The height of the rectangle in inches.
///
///
/// A claim like #123 @ 3,2: 5x4 means that claim ID 123 specifies a rectangle 3 inches from the
/// left edge, 2 inches from the top edge, 5 inches wide, and 4 inches tall. Visually, it claims the
/// square inches of fabric represented by # (and ignores the square inches of fabric represented by
/// .) in the diagram below:
///
/// ...........
/// ...........
/// ...#####...
/// ...#####...
/// ...#####...
/// ...#####...
/// ...........
/// ...........
/// ...........
///
/// The problem is that many of the claims overlap, causing two or more claims to cover part of the
/// same areas. For example, consider the following claims:
///
/// #1 @ 1,3: 4x4
/// #2 @ 3,1: 4x4
/// #3 @ 5,5: 2x2
///
/// Visually, these claim the following areas:
///
/// ........
/// ...2222.
/// ...2222.
/// .11XX22.
/// .11XX22.
/// .111133.
/// .111133.
/// ........
///
/// The four square inches marked with X are claimed by both 1 and 2. (Claim 3, while adjacent to
/// the others, does not overlap either of them.)
///
/// If the Elves all proceed with their own plans, none of them will have enough fabric. How many
/// square inches of fabric are within two or more claims?

#[derive(Debug, Eq, PartialEq)]
struct Claim {
    id: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

fn parse_input(input: &[String]) -> Vec<Claim> {
    let re = Regex::new(r"#([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)").unwrap();

    input.iter()
//        .inspect(|l| println!("To parse: {}", l))
        .map(|l| re.captures(l).unwrap())
        .map(|c| Claim {
            id: c.get(1).map_or(0, |m| m.as_str().parse().unwrap()),
            x: c.get(2).map_or(0, |m| m.as_str().parse().unwrap()),
            y: c.get(3).map_or(0, |m| m.as_str().parse().unwrap()),
            w: c.get(4).map_or(0, |m| m.as_str().parse().unwrap()),
            h: c.get(5).map_or(0, |m| m.as_str().parse().unwrap()),
        })
        .collect()
}

#[test]
fn test_parse_input() {
    let input = &[
        "#1 @ 1,3: 4x4".to_string(),
        "#2 @ 3,1: 4x4".to_string(),
    ];
    let expected_claims = vec![
        Claim {
            id: 1,
            x: 1,
            y: 3,
            w: 4,
            h: 4,
        },
        Claim {
            id: 2,
            x: 3,
            y: 1,
            w: 4,
            h: 4,
        },
    ];
    assert_eq!(expected_claims, parse_input(input));
}

pub fn solve_part_one(input: &[String]) -> usize {
    let _claims = parse_input(input);

    0
}

pub fn solve_part_two(_input: &[String]) -> String {
    "TODO".to_string()
}

#[test]
fn examples_part_one() {
    let input = &["#1 @ 1,3: 4x4".to_string(),
        "#2 @ 3,1: 4x4".to_string(),
        "#3 @ 5,5: 2x2".to_string()];
    assert_eq!(4, solve_part_one(input))
}
