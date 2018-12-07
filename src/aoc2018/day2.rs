use std::collections::HashMap;

/// --- Day 2: Inventory Management System ---
///
/// You stop falling through time, catch your breath, and check the screen on the device.
/// "Destination reached. Current Year: 1518. Current Location: North Pole Utility Closet 83N10."
/// You made it! Now, to find those anomalies.
///
/// Outside the utility closet, you hear footsteps and a voice. "...I'm not sure either. But now t
/// hat so many people have chimneys, maybe he could sneak in that way?" Another voice responds,
/// "Actually, we've been working on a new kind of suit that would let him fit through tight spaces
/// like that. But, I heard that a few days ago, they lost the prototype fabric, the design plans,
/// everything! Nobody on the team can even seem to remember important details of the project!"
///
/// "Wouldn't they have had enough fabric to fill several boxes in the warehouse? They'd be stored
/// together, so the box IDs should be similar. Too bad it would take forever to search the
/// warehouse for two similar box IDs..." They walk too far away to hear any more.
///
/// Late at night, you sneak to the warehouse - who knows what kinds of paradoxes you could cause if
/// you were discovered - and use your fancy wrist device to quickly scan every box and produce a
/// list of the likely candidates (your puzzle input).
///
/// To make sure you didn't miss any, you scan the likely candidate boxes again, counting the number
/// that have an ID containing exactly two of any letter and then separately counting those with
/// exactly three of any letter. You can multiply those two counts together to get a rudimentary
/// checksum and compare it to what your device predicts.
///
/// For example, if you see the following box IDs:
///
///     abcdef contains no letters that appear exactly two or three times.
///     bababc contains two a and three b, so it counts for both.
///     abbcde contains two b, but no letter appears exactly three times.
///     abcccd contains three c, but no letter appears exactly two times.
///     aabcdd contains two a and two d, but it only counts once.
///     abcdee contains two e.
///     ababab contains three a and three b, but it only counts once.
///
/// Of these box IDs, four of them contain a letter which appears exactly twice, and three of them
/// contain a letter which appears exactly three times. Multiplying these together produces a
/// checksum of 4 * 3 = 12.
///
/// What is the checksum for your list of box IDs?
///
/// --- Part Two ---
///
/// Confident that your list of box IDs is complete, you're ready to find the boxes full of
/// prototype fabric.
///
/// The boxes will have IDs which differ by exactly one character at the same position in both
/// strings. For example, given the following box IDs:
///
/// abcde
/// fghij
/// klmno
/// pqrst
/// fguij
/// axcye
/// wvxyz
///
/// The IDs abcde and axcye are close, but they differ by two characters (the second and fourth).
/// However, the IDs fghij and fguij differ by exactly one character, the third (h and u). Those
/// must be the correct boxes.
///
/// What letters are common between the two correct box IDs? (In the example above, this is found
/// by removing the differing character from either ID, producing fgij.)

#[derive(Debug)]
struct ChecksumPart {
    twos: usize,
    threes: usize,
}

pub fn solve_part_one(input: &[String]) -> usize {
    let checksum_parts: ChecksumPart = input
        .iter()
        .map(|line| line.chars())
        //.inspect(|chars| println!("{:?}", chars))
        .map(|chars| {
            let map: Vec<(char, usize)> = chars
                .fold(HashMap::<char, usize>::new(), |mut m, c| {
                    *m.entry(c).or_insert(0) += 1;
                    m
                })
                .into_iter()
                .filter(|(_, v)| *v == 2 || *v == 3)
                //.inspect(|e| println!("Entry: {:?}", e))
                .collect();

            ChecksumPart {
                twos: map.iter().find(|(_, v)| *v == 2).map_or(0, |_| 1),
                threes: map.iter().find(|(_, v)| *v == 3).map_or(0, |_| 1),
            }
        })
        //.inspect(|checksum_part| println!("Checksum parts: {:?}", checksum_part))
        .fold(ChecksumPart { twos: 0, threes: 0 }, |acc, cp| {
            ChecksumPart {
                twos: acc.twos + cp.twos,
                threes: acc.threes + cp.threes,
            }
        });

    checksum_parts.twos * checksum_parts.threes
}

pub fn solve_part_two(input: &[String]) -> String {
    for (i, line) in input.iter().enumerate() {
        for counterpart in input.iter().skip(i + 1) {
            //            println!("Comparing: {} - {}", line, counterpart);
            let matched_chars = compare_box_ids(line, counterpart);
            if matched_chars.len() == line.len() - 1 {
                return matched_chars;
            }
        }
    }

    String::from("NOTHING FOUND")
}

fn compare_box_ids(str1: &str, str2: &str) -> String {
    assert_eq!(
        str1.len(),
        str2.len(),
        "Strings have different lengths, let alone letters..."
    );

    let mut matched_chars = String::new();
    for (i, c1) in str1.chars().enumerate() {
        //        println!("{} - {} == {}", i, c1, str2.chars().nth(i).unwrap());
        if c1 == str2.chars().nth(i).unwrap() {
            matched_chars.push(c1);
        }
    }

    matched_chars
}

#[test]
fn examples_part_one() {
    let input = &[
        "abcdef".to_string(),
        "bababc".to_string(),
        "abbcde".to_string(),
        "abcccd".to_string(),
        "aabcdd".to_string(),
        "abcdee".to_string(),
        "ababab".to_string(),
    ];
    assert_eq!(12, solve_part_one(input));
}

#[test]
fn examples_part_two() {
    let input = &[
        "abcde".to_string(),
        "fghij".to_string(),
        "klmno".to_string(),
        "pqrst".to_string(),
        "fguij".to_string(),
        "axcye".to_string(),
        "wvxyz".to_string(),
    ];
    assert_eq!("fgij", solve_part_two(input));
}
