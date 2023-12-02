use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1};
use nom::combinator::map_res;
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::IResult;

/// --- Day 2: Cube Conundrum ---
///
/// You're launched high into the atmosphere! The apex of your trajectory just barely reaches the
/// surface of a large island floating in the sky. You gently land in a fluffy pile of leaves.
/// It's quite cold, but you don't see much snow. An Elf runs over to greet you.
///
/// The Elf explains that you've arrived at Snow Island and apologizes for the lack of snow.
/// He'll be happy to explain the situation, but it's a bit of a walk, so you have some time.
/// They don't get many visitors up here; would you like to play a game in the meantime?
///
/// As you walk, the Elf shows you a small bag and some cubes which are either red, green, or blue.
/// Each time you play this game, he will hide a secret number of cubes of each color in the bag,
/// and your goal is to figure out information about the number of cubes.
///
/// To get information, once a bag has been loaded with cubes, the Elf will reach into the bag,
/// grab a handful of random cubes, show them to you, and then put them back in the bag. He'll do this a few times per game.
///
/// You play several games and record the information from each game (your puzzle input).
/// Each game is listed with its ID number (like the 11 in Game 11: ...) followed by a
/// semicolon-separated list of subsets of cubes that were revealed from the bag
/// (like 3 red, 5 green, 4 blue).
///
/// For example, the record of a few games might look like this:
///
/// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
/// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
/// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
/// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
/// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
///
/// In game 1, three sets of cubes are revealed from the bag (and then put back again).
/// The first set is 3 blue cubes and 4 red cubes; the second set is 1 red cube, 2 green cubes, and 6 blue cubes; the third set is only 2 green cubes.
///
/// The Elf would first like to know which games would have been possible if the bag contained
/// only 12 red cubes, 13 green cubes, and 14 blue cubes?
///
/// In the example above, games 1, 2, and 5 would have been possible if the bag had been loaded
/// with that configuration. However, game 3 would have been impossible because at one point the
/// Elf showed you 20 red cubes at once; similarly, game 4 would also have been impossible because
/// the Elf showed you 15 blue cubes at once. If you add up the IDs of the games that would have
/// been possible, you get 8.
///
/// Determine which games would have been possible if the bag had been loaded with only 12 red
/// cubes, 13 green cubes, and 14 blue cubes. What is the sum of the IDs of those games?
///
/// --- Part Two ---
///
/// The Elf says they've stopped producing snow because they aren't getting any water!
/// He isn't sure why the water stopped; however, he can show you how to get to the water source to
/// check it out for yourself. It's just up ahead!
///
/// As you continue your walk, the Elf poses a second question: in each game you played, what is
/// the fewest number of cubes of each color that could have been in the bag to make the game possible?
///
/// Again consider the example games from earlier:
///
/// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
/// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
/// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
/// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
/// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
///
///     In game 1, the game could have been played with as few as 4 red, 2 green, and 6 blue cubes.
///         If any color had even one fewer cube, the game would have been impossible.
///     Game 2 could have been played with a minimum of 1 red, 3 green, and 4 blue cubes.
///     Game 3 must have been played with at least 20 red, 13 green, and 6 blue cubes.
///     Game 4 required at least 14 red, 3 green, and 15 blue cubes.
///     Game 5 needed no fewer than 6 red, 3 green, and 2 blue cubes in the bag.
///
/// The power of a set of cubes is equal to the numbers of red, green, and blue cubes multiplied
/// together. The power of the minimum set of cubes in game 1 is 48. In games 2-5 it was 12, 1560,
/// 630, and 36, respectively. Adding up these five powers produces the sum 2286.
///
/// For each game, find the minimum set of cubes that must have been present. What is the sum of the
/// power of these sets?
#[derive(Debug, PartialEq)]
struct Round {
    pub red: Option<usize>,
    pub green: Option<usize>,
    pub blue: Option<usize>,
}

#[derive(Debug, PartialEq)]
struct Game {
    pub id: usize,
    pub rounds: Vec<Round>,
}

fn parse_round(input: &str) -> IResult<&str, Round> {
    let (input, cubes) = separated_list0(
        tag(", "),
        separated_pair(
            map_res(digit1, str::parse::<usize>),
            char(' '),
            alt((tag("red"), tag("green"), tag("blue"))),
        ),
    )(input)?;

    let mut round = Round {
        red: None,
        green: None,
        blue: None,
    };
    cubes
        .iter()
        .for_each(|(cube_count, cube_colour)| match *cube_colour {
            "red" => round.red = Some(*cube_count),
            "green" => round.green = Some(*cube_count),
            "blue" => round.blue = Some(*cube_count),
            _ => panic!("Unknown cube colour: {}", cube_colour),
        });

    Ok((input, round))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = map_res(digit1, str::parse)(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, rounds) = separated_list0(tag("; "), parse_round)(input)?;

    Ok((input, Game { id, rounds }))
}

#[test]
fn test_parse_game() {
    let (_, result) = parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap();
    assert_eq!(
        result,
        Game {
            id: 1,
            rounds: vec![
                Round {
                    red: Some(4),
                    green: None,
                    blue: Some(3),
                },
                Round {
                    red: Some(1),
                    green: Some(2),
                    blue: Some(6),
                },
                Round {
                    red: None,
                    green: Some(2),
                    blue: None,
                },
            ],
        }
    );

    let (_, result) = parse_game( "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green").unwrap();
    assert_eq!(
        result,
        Game {
            id: 5,
            rounds: vec![
                Round {
                    red: Some(6),
                    green: Some(3),
                    blue: Some(1),
                },
                Round {
                    red: Some(1),
                    green: Some(2),
                    blue: Some(2),
                },
            ],
        }
    );
}

pub fn solve_part_one(input: &[String]) -> usize {
    input
        .iter()
        .map(String::as_str)
        .map(parse_game)
        .filter_map(|r| r.map(|(_, g)| g).ok())
        .filter(|g| {
            g.rounds.iter().all(|r| {
                (r.red.is_none() || r.red.is_some_and(|x| x <= 12))
                    && (r.green.is_none() || r.green.is_some_and(|x| x <= 13))
                    && (r.blue.is_none() || r.blue.is_some_and(|x| x <= 14))
            })
        })
        // .inspect(|g| println!("Valid Game: {:?}", g))
        .map(|g| g.id)
        .sum()
}

#[test]
fn examples_part_one() {
    assert_eq!(
        8,
        solve_part_one(&[
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
        ])
    );
}

pub fn solve_part_two(input: &[String]) -> usize {
    input
        .iter()
        .map(String::as_str)
        .map(parse_game)
        .filter_map(|r| r.map(|(_, g)| g).ok())
        // .inspect(|g| println!("Game: {:?}", g))
        .map(|g| {
            g.rounds.iter().fold((0, 0, 0), |acc, round| {
                (
                    acc.0.max(round.red.unwrap_or(0)),
                    acc.1.max(round.green.unwrap_or(0)),
                    acc.2.max(round.blue.unwrap_or(0)),
                )
            })
        })
        // .inspect(|g| println!("Minimal cube set: {:?}", g))
        .map(|(r, g, b)| r * g * b)
        // .inspect(|g| println!("Power: {:?}", g))
        .sum()
}

#[test]
fn examples_part_two() {
    assert_eq!(
        2286,
        solve_part_two(&[
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
        ])
    );
}
