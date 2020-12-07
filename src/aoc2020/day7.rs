/// --- Day 7: Handy Haversacks ---
///
/// You land at the regional airport in time for your next flight. In fact, it looks like you'll
/// even have time to grab some food: all flights are currently delayed due to issues in luggage
/// processing.
///
/// Due to recent aviation regulations, many rules (your puzzle input) are being enforced about bags
/// and their contents; bags must be color-coded and must contain specific quantities of other
/// color-coded bags. Apparently, nobody responsible for these regulations considered how long they
/// would take to enforce!
///
/// For example, consider the following rules:
///
///  - light red bags contain 1 bright white bag, 2 muted yellow bags.
///  - dark orange bags contain 3 bright white bags, 4 muted yellow bags.
///  - bright white bags contain 1 shiny gold bag.
///  - muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
///  - shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
///  - dark olive bags contain 3 faded blue bags, 4 dotted black bags.
///  - vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
///  - faded blue bags contain no other bags.
///  - dotted black bags contain no other bags.
///
/// These rules specify the required contents for 9 bag types. In this example, every faded blue bag
/// is empty, every vibrant plum bag contains 11 bags (5 faded blue and 6 dotted black), and so on.
///
/// You have a shiny gold bag. If you wanted to carry it in at least one other bag, how many
/// different bag colors would be valid for the outermost bag? (In other words: how many colors can,
/// eventually, contain at least one shiny gold bag?)
///
/// In the above rules, the following options would be available to you:
///
/// ```
///     A bright white bag, which can hold your shiny gold bag directly.
///     A muted yellow bag, which can hold your shiny gold bag directly, plus some other bags.
///     A dark orange bag, which can hold bright white and muted yellow bags, either of which could
///       then hold your shiny gold bag.
///     A light red bag, which can hold bright white and muted yellow bags, either of which could
///       then hold your shiny gold bag.
/// ```
///
/// So, in this example, the number of bag colors that can eventually contain at least one shiny
/// gold bag is 4.
///
/// How many bag colors can eventually contain at least one shiny gold bag? (The list of rules is
/// quite long; make sure you get all of it.)
///
/// --- Part Two ---
///
/// It's getting pretty expensive to fly these days - not because of ticket prices, but because of
/// the ridiculous number of bags you need to buy!
///
/// Consider again your shiny gold bag and the rules from the above example:
///
///  - faded blue bags contain 0 other bags.
///  - dotted black bags contain 0 other bags.
///  - vibrant plum bags contain 11 other bags: 5 faded blue bags and 6 dotted black bags.
///  - dark olive bags contain 7 other bags: 3 faded blue bags and 4 dotted black bags.
///
/// So, a single shiny gold bag must contain 1 dark olive bag (and the 7 bags within it) plus 2
/// vibrant plum bags (and the 11 bags within each of those): 1 + 1*7 + 2 + 2*11 = 32 bags!
///
/// Of course, the actual rules have a small chance of going several levels deeper than this
/// example; be sure to count all of the bags, even if the nesting becomes topologically
/// impractical!
///
/// Here's another example:
///
/// ```
/// shiny gold bags contain 2 dark red bags.
/// dark red bags contain 2 dark orange bags.
/// dark orange bags contain 2 dark yellow bags.
/// dark yellow bags contain 2 dark green bags.
/// dark green bags contain 2 dark blue bags.
/// dark blue bags contain 2 dark violet bags.
/// dark violet bags contain no other bags.
/// ```
///
/// In this example, a single shiny gold bag must contain 126 other bags.
///
/// How many individual bags are required inside your single shiny gold bag?
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::digit1;
use nom::combinator::{map, opt};
use nom::lib::std::collections::{HashMap, HashSet};
use nom::multi::many0;
use nom::IResult;

#[derive(Debug, PartialEq)]
struct ChildRule {
    count: usize,
    bag_name: String,
}

#[derive(Debug, PartialEq)]
struct Rule {
    bag_name: String,
    children: Vec<ChildRule>,
}

fn number(input: &str) -> IResult<&str, usize> {
    let (input, count) = map(digit1, std::str::FromStr::from_str)(input)?;
    Ok((input, count.unwrap()))
}

fn parse_child_rule(input: &str) -> IResult<&str, ChildRule> {
    let (input, count) = number(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, bag_name) = take_until(" bag")(input)?;
    let (input, _) = tag(" bag")(input)?;
    let (input, _) = opt(tag("s"))(input)?;
    let (input, _) = alt((tag(", "), tag(".")))(input)?;

    Ok((
        input,
        ChildRule {
            bag_name: bag_name.to_owned(),
            count,
        },
    ))
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, bag_name) = take_until(" bags contain ")(input)?;
    let (input, _) = tag(" bags contain ")(input)?;

    let (input, children) = alt((
        many0(parse_child_rule),
        map(tag("no other bags"), |_| Vec::new()),
    ))(input)?;
    // let (input, children) = many0(parse_child_rule)(input)?;

    Ok((
        input,
        Rule {
            bag_name: bag_name.to_string(),
            children,
        },
    ))
}

pub fn solve_part_one(input: &[String]) -> usize {
    // Parse all the rules
    let rules: Vec<Rule> = input
        .iter()
        .map(|i| parse_rule(i).map(|(_, rule)| rule).unwrap())
        .collect();

    // Turn rules into bag contained-by tree
    let mut contained_by: HashMap<&str, Vec<&str>> = HashMap::new();
    rules.iter().for_each(|r| {
        let container_bag = &r.bag_name.as_str();
        r.children.iter().for_each(|c| {
            match contained_by.get_mut(c.bag_name.as_str()) {
                None => {
                    contained_by.insert(c.bag_name.as_str(), vec![container_bag]);
                }
                Some(cc) => cc.push(container_bag),
            };
        });
    });

    let our_bag = "shiny gold";

    let mut container_bags: HashSet<&str> = HashSet::new();

    let mut parent_bags = vec![our_bag];

    while !parent_bags.is_empty() {
        if let Some(new_parents) = contained_by.get(parent_bags.pop().unwrap()) {
            new_parents.iter().for_each(|b| {
                container_bags.insert(b);
                parent_bags.push(b);
            });
        };
    }

    container_bags.len()
}

pub fn solve_part_two(input: &[String]) -> usize {
    // Parse all the rules
    let rules: Vec<Rule> = input
        .iter()
        .map(|i| parse_rule(i).map(|(_, rule)| rule).unwrap())
        .collect();

    // Turn rules into bag contains tree
    let mut bag_rule_map: HashMap<&str, &Rule> = HashMap::new();
    let mut bag_contain_count_map: HashMap<&str, usize> = HashMap::new();
    rules.iter().for_each(|r| {
        bag_rule_map.insert(r.bag_name.as_str(), r);
        bag_contain_count_map.insert(
            r.bag_name.as_str(),
            r.children.iter().map(|c| c.count).sum(),
        );
    });

    let our_bag = "shiny gold";

    let mut contained_bag_count = 0;

    let mut parent_bags: Vec<(usize, &str)> = vec![(1, our_bag)];
    while !parent_bags.is_empty() {
        let (multiplier, parent_name) = parent_bags.pop().unwrap();
        let size = bag_contain_count_map.get(parent_name).unwrap();
        println!("{}: {} x {}", parent_name, multiplier, size);

        contained_bag_count += multiplier * size;

        bag_rule_map
            .get(parent_name)
            .unwrap()
            .children
            .iter()
            .for_each(|c| {
                parent_bags.push((multiplier * c.count, &c.bag_name.as_str()));
            });
    }

    contained_bag_count
}

#[test]
fn test_parse_rule() {
    assert_eq!(
        IResult::Ok((
            "",
            Rule {
                bag_name: "light red".to_owned(),
                children: vec![
                    ChildRule {
                        count: 1,
                        bag_name: "bright white".to_owned(),
                    },
                    ChildRule {
                        count: 2,
                        bag_name: "muted yellow".to_owned(),
                    },
                ],
            }
        )),
        parse_rule("light red bags contain 1 bright white bag, 2 muted yellow bags.")
    );
    assert_eq!(
        IResult::Ok((
            "no other bags.",
            Rule {
                bag_name: "faded blue".to_owned(),
                children: vec![],
            }
        )),
        parse_rule("faded blue bags contain no other bags.")
    );
}

#[test]
fn examples_part_one() {
    assert_eq!(
        4,
        solve_part_one(&[
            "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
            "bright white bags contain 1 shiny gold bag.".to_string(),
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
            "faded blue bags contain no other bags.".to_string(),
            "dotted black bags contain no other bags.".to_string(),
        ])
    );
}

#[test]
fn examples_part_two() {
    assert_eq!(
        32,
        solve_part_two(&[
            "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
            "bright white bags contain 1 shiny gold bag.".to_string(),
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
            "faded blue bags contain no other bags.".to_string(),
            "dotted black bags contain no other bags.".to_string(),
        ])
    );
    assert_eq!(
        126,
        solve_part_two(&[
            "shiny gold bags contain 2 dark red bags.".to_string(),
            "dark red bags contain 2 dark orange bags.".to_string(),
            "dark orange bags contain 2 dark yellow bags.".to_string(),
            "dark yellow bags contain 2 dark green bags.".to_string(),
            "dark green bags contain 2 dark blue bags.".to_string(),
            "dark blue bags contain 2 dark violet bags.".to_string(),
            "dark violet bags contain no other bags.".to_string(),
        ])
    );
}
