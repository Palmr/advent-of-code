/// --- Day 15: Rambunctious Recitation ---
///
/// You catch the airport shuttle and try to book a new flight to your vacation island. Due to the
/// storm, all direct flights have been cancelled, but a route is available to get around the storm.
/// You take it.
///
/// While you wait for your flight, you decide to check in with the Elves back at the North Pole.
/// They're playing a memory game and are ever so excited to explain the rules!
///
/// In this game, the players take turns saying numbers. They begin by taking turns reading from a
/// list of starting numbers (your puzzle input). Then, each turn consists of considering the most
/// recently spoken number:
///
///  - If that was the first time the number has been spoken, the current player says `0`.
///  - Otherwise, the number had been spoken before; the current player announces how many turns
///     apart the number is from when it was previously spoken.
///
/// So, after the starting numbers, each turn results in that player speaking aloud either `0`
/// (if the last number is new) or an age (if the last number is a repeat).
///
/// For example, suppose the starting numbers are `0,3,6`:
///
///  - Turn 1: The 1st number spoken is a starting number, `0`.
///  - Turn 2: The 2nd number spoken is a starting number, `3`.
///  - Turn 3: The 3rd number spoken is a starting number, `6`.
///  - Turn 4: Now, consider the last number spoken, `6`.
///     Since that was the first time the number had been spoken, the 4th number spoken is `0.`
///  - Turn 5: Next, again consider the last number spoken, 0.
///     Since it had been spoken before, the next number to speak is the difference between the turn
///     number when it was last spoken (the previous turn, 4) and the turn number of the time it was
///     most recently spoken before then (turn 1). Thus, the 5th number spoken is `4 - 1`, `3`.
///  - Turn 6: The last number spoken, `3` had also been spoken before,
///     most recently on turns `5` and `2`.
///     So, the 6th number spoken is `5 - 2`, `3`.
///  - Turn 7: Since `3` was just spoken twice in a row, and the last two turns are `1` turn apart,
///     the 7th number spoken is `1`.
///  - Turn 8: Since `1` is new, the 8th number spoken is `0`.
///  - Turn 9: `0` was last spoken on turns `8` and `4`, so the 9th number spoken is the difference
///     between them, `4`.
///  - Turn 10: `4` is new, so the 10th number spoken is `0`.
///
/// (The game ends when the Elves get sick of playing or dinner is ready, whichever comes first.)
///
/// Their question for you is: what will be the 2020th number spoken?
/// In the example above, the 2020th number spoken will be `436`.
///
/// Here are a few more examples:
///
///  - Given the starting numbers `1,3,2`, the 2020th number spoken is `1`.
///  - Given the starting numbers `2,1,3`, the 2020th number spoken is `10`.
///  - Given the starting numbers `1,2,3`, the 2020th number spoken is `27`.
///  - Given the starting numbers `2,3,1`, the 2020th number spoken is `78`.
///  - Given the starting numbers `3,2,1`, the 2020th number spoken is `438`.
///  - Given the starting numbers `3,1,2`, the 2020th number spoken is `1836`.
///
/// Given your starting numbers, what will be the 2020th number spoken?
///
/// Your puzzle input is `18,11,9,0,5,1`.
///
/// --- Part Two ---
//
// Impressed, the Elves issue you a challenge: determine the 30000000th number spoken. For example, given the same starting numbers as above:
//
//  - Given `0,3,6,` the 30000000th number spoken is `175594`.
//  - Given `1,3,2,` the 30000000th number spoken is `2578`.
//  - Given `2,1,3,` the 30000000th number spoken is `3544142`.
//  - Given `1,2,3,` the 30000000th number spoken is `261214`.
//  - Given `2,3,1,` the 30000000th number spoken is `6895259`.
//  - Given `3,2,1,` the 30000000th number spoken is `18`.
//  - Given `3,1,2,` the 30000000th number spoken is `362`.
//
// Given your starting numbers, what will be the 30000000th number spoken?

fn run_game(input: &[String], rounds: usize) -> usize {
    let numbers: Vec<usize> = input[0]
        .split(',')
        .filter(|x| !x.is_empty())
        .map(|i| i.parse::<usize>().unwrap())
        .collect();

    let mut last_seen = vec![0usize; rounds].into_boxed_slice();
    numbers
        .iter()
        .take(numbers.len() - 1)
        .enumerate()
        .for_each(|(idx, num)| {
            last_seen[*num] = idx + 1;
        });

    let mut last_num = *numbers.last().unwrap();
    for idx in numbers.len()..rounds {
        let last_seen_idx = last_seen[last_num];
        last_seen[last_num] = idx;

        let next_num = if last_seen_idx == 0 {
            // println!("[{}] {} -> {}", idx + 1, last_num, 0);
            0
        } else {
            // println!("[{}] {} -> ({} - {}) = {}", idx + 1, last_num, idx, last_seen_idx, idx - last_seen_idx);
            idx - last_seen_idx
        };

        last_num = next_num;
    }

    last_num
}

pub fn solve_part_one(input: &[String]) -> usize {
    run_game(input, 2020)
}

pub fn solve_part_two(input: &[String]) -> usize {
    run_game(input, 30000000)
}

#[test]
fn examples_part_one() {
    assert_eq!(436, solve_part_one(&["0,3,6".to_string(),]));
    assert_eq!(1, solve_part_one(&["1,3,2".to_string(),]));
    assert_eq!(10, solve_part_one(&["2,1,3".to_string(),]));
    assert_eq!(27, solve_part_one(&["1,2,3".to_string(),]));
    assert_eq!(78, solve_part_one(&["2,3,1".to_string(),]));
    assert_eq!(438, solve_part_one(&["3,2,1".to_string(),]));
    assert_eq!(1836, solve_part_one(&["3,1,2".to_string(),]));
}

#[test]
fn examples_part_two() {
    assert_eq!(175594, solve_part_two(&["0,3,6,".to_string(),]));
    assert_eq!(2578, solve_part_two(&["1,3,2,".to_string(),]));
    assert_eq!(3544142, solve_part_two(&["2,1,3,".to_string(),]));
    assert_eq!(261214, solve_part_two(&["1,2,3,".to_string(),]));
    assert_eq!(6895259, solve_part_two(&["2,3,1,".to_string(),]));
    assert_eq!(18, solve_part_two(&["3,2,1,".to_string(),]));
    assert_eq!(362, solve_part_two(&["3,1,2,".to_string(),]));
}
