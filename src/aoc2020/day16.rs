/// --- Day 16: Ticket Translation ---
///
/// As you're walking to yet another connecting flight, you realize that one of the legs of your
/// re-routed trip coming up is on a high-speed train. However, the train ticket you were given is
/// in a language you don't understand. You should probably figure out what it says before you get
/// to the train station after the next flight.
///
/// Unfortunately, you can't actually read the words on the ticket. You can, however, read the
/// numbers, and so you figure out the fields these tickets must have and the valid ranges for
/// values in those fields.
///
/// You collect the rules for ticket fields, the numbers on your ticket, and the numbers on other
/// nearby tickets for the same train service (via the airport security cameras) together into a
/// single document you can reference (your puzzle input).
///
/// The rules for ticket fields specify a list of fields that exist somewhere on the ticket and the
/// valid ranges of values for each field. For example, a rule like `class: 1-3 or 5-7` means that
/// one of the fields in every ticket is named `class` and can be any value in the ranges
/// `1-3` or `5-7` (inclusive, such that `3` and `5` are both valid in this field, but `4` is not).
///
/// Each ticket is represented by a single line of comma-separated values.
/// The values are the numbers on the ticket in the order they appear;
/// very ticket has the same format.
/// For example, consider this ticket:
///
/// ```
/// .--------------------------------------------------------.
/// | ????: 101    ?????: 102   ??????????: 103     ???: 104 |
/// |                                                        |
/// | ??: 301  ??: 302             ???????: 303      ??????? |
/// | ??: 401  ??: 402           ???? ????: 403    ????????? |
/// '--------------------------------------------------------'
/// ```
///
/// Here, `?` represents text in a language you don't understand.
/// This ticket might be represented as `101,102,103,104,301,302,303,401,402,403;` of course,
/// the actual train tickets you're looking at are much more complicated. In any case,
/// you've extracted just the numbers in such a way that the first number is always the same
/// specific field, the second number is always a different specific field, and so on - you just
/// don't know what each position actually means!
///
/// Start by determining which tickets are completely invalid;
/// these are tickets that contain values which aren't valid for any field.
/// Ignore your ticket for now.
///
/// For example, suppose you have the following notes:
///
/// ```
/// class: 1-3 or 5-7
/// row: 6-11 or 33-44
/// seat: 13-40 or 45-50
///
/// your ticket:
/// 7,1,14
///
/// nearby tickets:
/// 7,3,47
/// 40,4,50
/// 55,2,20
/// 38,6,12
/// ```
///
/// It doesn't matter which position corresponds to which field; you can identify invalid nearby
/// tickets by considering only whether tickets contain values that are not valid for any field.
/// In this example, the values on the first nearby ticket are all valid for at least one field.
/// This is not true of the other three nearby tickets: the values `4`, `55`, and `12` are are not
/// valid for any field. Adding together all of the invalid values produces your ticket scanning
/// error rate: `4 + 55 + 12 = 71`.
///
/// Consider the validity of the nearby tickets you scanned.
/// What is your ticket scanning error rate?
///
/// --- Part Two ---
///
/// Now that you've identified which tickets contain invalid values, discard those tickets entirely.
/// Use the remaining valid tickets to determine which field is which.
///
/// Using the valid ranges for each field, determine what order the fields appear on the tickets.
/// The order is consistent between all tickets: if seat is the third field, it is the third field
/// on every ticket, including your ticket.
///
/// For example, suppose you have the following notes:
///
/// ```
/// class: 0-1 or 4-19
/// row: 0-5 or 8-19
/// seat: 0-13 or 16-19
///
/// your ticket:
/// 11,12,13
///
/// nearby tickets:
/// 3,9,18
/// 15,1,5
/// 5,14,9
/// ```
///
/// Based on the nearby tickets in the above example, the first position must be row, the second
/// position must be class, and the third position must be seat; you can conclude that in your
/// ticket, class is `12`, row is `11`, and seat is `13`.
///
/// Once you work out which field is which, look for the six fields on your ticket that start with
/// the word departure.
///
/// What do you get if you multiply those six values together?
///
use lazy_static::lazy_static;
use nom::lib::std::collections::HashMap;
use regex::Regex;

lazy_static! {
    static ref RE_RULE: Regex = Regex::new(r"^(?P<field>.*?): (?P<range1_low>[0-9]+)-(?P<range1_high>[0-9]+) or (?P<range2_low>[0-9]+)-(?P<range2_high>[0-9]+)$").unwrap();
}

#[derive(Debug, PartialEq)]
struct Rule {
    name: String,
    range1: (usize, usize),
    range2: (usize, usize),
}

impl Rule {
    fn validate(&self, value: &usize) -> bool {
        (value >= &self.range1.0 && value <= &self.range1.1)
            || (value >= &self.range2.0 && value <= &self.range2.1)
    }
}

#[test]
fn test_rule() {
    let rule = Rule {
        name: "class".to_string(),
        range1: (1, 3),
        range2: (5, 7),
    };
    assert_eq!(true, rule.validate(&3));
    assert_eq!(true, rule.validate(&5));
    assert_eq!(false, rule.validate(&4));
}

fn parse_rule(input: &str) -> Rule {
    // println!("Parsing rule: {:?}", input);
    let matched = RE_RULE.captures(input).unwrap();

    let range1_low = matched
        .name("range1_low")
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();
    let range1_high = matched
        .name("range1_high")
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();
    let range2_low = matched
        .name("range2_low")
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();
    let range2_high = matched
        .name("range2_high")
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();

    Rule {
        name: matched.name("field").unwrap().as_str().to_owned(),
        range1: (range1_low, range1_high),
        range2: (range2_low, range2_high),
    }
}

#[test]
fn test_parse_rule() {
    assert_eq!(
        Rule {
            name: "class".to_string(),
            range1: (1, 3),
            range2: (5, 7),
        },
        parse_rule("class: 1-3 or 5-7")
    );
}

fn parse_data(input: &[String]) -> (Vec<Rule>, Vec<usize>, Vec<Vec<usize>>) {
    let mut iter = input.iter();

    let rules: Vec<Rule> = iter
        .by_ref()
        .take_while(|l| !l.is_empty())
        .cloned()
        .map(|l| parse_rule(&l))
        .collect();

    assert_eq!(
        "your ticket:",
        iter.next().unwrap(),
        "Should be my ticket next"
    );

    // My ticket only one line
    let my_ticket = iter
        .next()
        .unwrap()
        .split(',')
        .into_iter()
        .map(|f| f.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    assert_eq!(
        "",
        iter.next().unwrap(),
        "Line after my ticket should be blank"
    );

    assert_eq!(
        "nearby tickets:",
        iter.next().unwrap(),
        "Expect nearby tickets next"
    );

    let nearby_tickets: Vec<Vec<usize>> = iter
        .by_ref()
        .cloned()
        .map(|l| {
            l.split(',')
                .into_iter()
                .map(|f| f.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();

    (rules, my_ticket, nearby_tickets)
}

pub fn solve_part_one(input: &[String]) -> usize {
    let (rules, _, nearby_tickets) = parse_data(input);

    nearby_tickets.iter().fold(0, |acc, ticket| {
        let invalid_fields: usize = ticket
            .iter()
            .filter(|&field| {
                !rules
                    .iter()
                    .fold(false, |result, rule| result | rule.validate(field))
            })
            .sum();
        acc + invalid_fields
    })
}

#[test]
fn examples_part_one() {
    assert_eq!(
        71,
        solve_part_one(&[
            "class: 1-3 or 5-7".to_string(),
            "row: 6-11 or 33-44".to_string(),
            "seat: 13-40 or 45-50".to_string(),
            "".to_string(),
            "your ticket:".to_string(),
            "7,1,14".to_string(),
            "".to_string(),
            "nearby tickets:".to_string(),
            "7,3,47".to_string(),
            "40,4,50".to_string(),
            "55,2,20".to_string(),
            "38,6,12".to_string(),
        ])
    );
}

pub fn solve_part_two(input: &[String]) -> usize {
    let (rules, my_ticket, nearby_tickets) = parse_data(input);

    let valid_tickets: Vec<Vec<usize>> = nearby_tickets
        .into_iter()
        .filter(|ticket| {
            ticket
                .iter()
                .all(|&field| rules.iter().any(|rule| rule.validate(&field)))
        })
        .collect();
    // println!("valid_tickets: {:?}", valid_tickets);

    let mut fields_name_indexes: HashMap<String, usize> = HashMap::new();

    for field_index in 0..rules.len() {
        let unknown_rules: Vec<&Rule> = rules
            .iter()
            .filter(|&r| r.name.starts_with("departure"))
            .filter(|&r| !fields_name_indexes.contains_key(&r.name))
            .collect();
        unknown_rules.iter().for_each(|rule| {
            let all_ok = valid_tickets
                .iter()
                .all(|ticket| rule.validate(&ticket[field_index]));

            println!("{} @ [{}] => {}", rule.name, field_index, all_ok);
            if all_ok {
                fields_name_indexes.insert(rule.name.clone(), field_index);
            }
        });
    }

    println!("fields_name_indexes: {:?}", fields_name_indexes);

    fields_name_indexes
        .iter()
        .map(|(rule_name, rule_idx)| {
            println!(
                "rule_name {}, rule_idx {}, value {}",
                rule_name, rule_idx, my_ticket[*rule_idx]
            );
            my_ticket[*rule_idx]
        })
        .product()
}

#[test]
fn examples_part_two() {
    // assert_eq!(
    //     71,
    //     solve_part_two(&[
    //         "class: 0-1 or 4-19".to_string(),
    //         "row: 0-5 or 8-19".to_string(),
    //         "seat: 0-13 or 16-19".to_string(),
    //         "".to_string(),
    //         "your ticket:".to_string(),
    //         "11,12,13".to_string(),
    //         "".to_string(),
    //         "nearby tickets:".to_string(),
    //         "3,9,18".to_string(),
    //         "15,1,5".to_string(),
    //         "5,14,9".to_string(),
    //     ])
    // );
}
