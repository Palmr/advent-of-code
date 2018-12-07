use regex::Regex;

/// --- Day 4: Repose Record ---
///
/// You've sneaked into another supply closet - this time, it's across from the prototype suit
/// manufacturing lab. You need to sneak inside and fix the issues with the suit, but there's a
/// guard stationed outside the lab, so this is as close as you can safely get.
///
/// As you search the closet for anything that might help, you discover that you're not the first
/// person to want to sneak in. Covering the walls, someone has spent an hour starting every
/// midnight for the past few months secretly observing this guard post! They've been writing down
/// the ID of the one guard on duty that night - the Elves seem to have decided that one guard was
/// enough for the overnight shift - as well as when they fall asleep or wake up while at their post
/// (your puzzle input).
///
/// For example, consider the following records, which have already been organized into
/// chronological order:
///
/// [1518-11-01 00:00] Guard #10 begins shift
/// [1518-11-01 00:05] falls asleep
/// [1518-11-01 00:25] wakes up
/// [1518-11-01 00:30] falls asleep
/// [1518-11-01 00:55] wakes up
/// [1518-11-01 23:58] Guard #99 begins shift
/// [1518-11-02 00:40] falls asleep
/// [1518-11-02 00:50] wakes up
/// [1518-11-03 00:05] Guard #10 begins shift
/// [1518-11-03 00:24] falls asleep
/// [1518-11-03 00:29] wakes up
/// [1518-11-04 00:02] Guard #99 begins shift
/// [1518-11-04 00:36] falls asleep
/// [1518-11-04 00:46] wakes up
/// [1518-11-05 00:03] Guard #99 begins shift
/// [1518-11-05 00:45] falls asleep
/// [1518-11-05 00:55] wakes up
///
/// Timestamps are written using year-month-day hour:minute format. The guard falling asleep or
/// waking up is always the one whose shift most recently started. Because all asleep/awake times
/// are during the midnight hour (00:00 - 00:59), only the minute portion (00 - 59) is relevant for
/// those events.
///
/// Visually, these records show that the guards are asleep at these times:
///
/// Date   ID   Minute
///             000000000011111111112222222222333333333344444444445555555555
///             012345678901234567890123456789012345678901234567890123456789
/// 11-01  #10  .....####################.....#########################.....
/// 11-02  #99  ........................................##########..........
/// 11-03  #10  ........................#####...............................
/// 11-04  #99  ....................................##########..............
/// 11-05  #99  .............................................##########.....
///
/// The columns are Date, which shows the month-day portion of the relevant day; ID, which shows the
/// guard on duty that day; and Minute, which shows the minutes during which the guard was asleep
/// within the midnight hour. (The Minute column's header shows the minute's ten's digit in the
/// first row and the one's digit in the second row.) Awake is shown as ., and asleep is shown as #.
///
/// Note that guards count as asleep on the minute they fall asleep, and they count as awake on the
/// minute they wake up. For example, because Guard #10 wakes up at 00:25 on 1518-11-01, minute 25
/// is marked as awake.
///
/// If you can figure out the guard most likely to be asleep at a specific time, you might be able
/// to trick that guard into working tonight so you can have the best chance of sneaking in. You
/// have two strategies for choosing the best guard/minute combination.
///
/// Strategy 1: Find the guard that has the most minutes asleep. What minute does that guard spend
/// asleep the most?
///
/// In the example above, Guard #10 spent the most minutes asleep, a total of 50 minutes (20+25+5),
/// while Guard #99 only slept for a total of 30 minutes (10+10+10). Guard #10 was asleep most
/// during minute 24 (on two days, whereas any other minute the guard was asleep was only seen on
/// one day).
///
/// While this example listed the entries in chronological order, your entries are in the order you
/// found them. You'll need to organize them before they can be analyzed.
///
/// What is the ID of the guard you chose multiplied by the minute you chose? (In the above example,
/// the answer would be 10 * 24 = 240.)
///

#[derive(Debug, Eq, PartialEq)]
enum GuardSate {
    BeginShift,
    FallsAsleep,
    WakesUp,
}

#[derive(Debug, Eq, PartialEq)]
struct GuardLog {
    guard_id: usize,
    state: GuardSate,
    month: usize,
    day: usize,
    hour: usize,
    minute: usize,
}

fn parse_input(input: &[String]) -> Vec<GuardLog> {
    let re =
        Regex::new(r"\[1518-([0-9]{2})-([0-9]{2}) ([0-9]{2}):([0-9]{2})\] (Guard #([0-9]+) )?(.*)")
            .unwrap();

    let mut current_guard = 0;
    input
        .iter()
        //        .inspect(|l| println!("To parse: {}", l))
        .map(|l| re.captures(l).unwrap())
        //        .inspect(|c| println!("Captured: {:?}", c))
        .map(|c| {
            current_guard = c
                .get(6)
                .map_or(current_guard, |m| m.as_str().parse().unwrap());
            GuardLog {
                guard_id: current_guard,
                state: c
                    .get(7)
                    .map(|m| match m.as_str() {
                        "begins shift" => GuardSate::BeginShift,
                        "falls asleep" => GuardSate::FallsAsleep,
                        "wakes up" => GuardSate::WakesUp,
                        x => panic!("Invalid state: {}", x),
                    }).unwrap(),
                month: c.get(1).map_or(0, |m| m.as_str().parse().unwrap()),
                day: c.get(2).map_or(0, |m| m.as_str().parse().unwrap()),
                hour: c.get(3).map_or(0, |m| m.as_str().parse().unwrap()),
                minute: c.get(4).map_or(0, |m| m.as_str().parse().unwrap()),
            }
        }).collect()
}

#[test]
fn test_parse_input() {
    let input = &[
        "[1518-11-01 00:00] Guard #10 begins shift".to_string(),
        "[1518-11-01 00:05] falls asleep".to_string(),
        "[1518-11-01 00:25] wakes up".to_string(),
        "[1518-11-01 23:58] Guard #99 begins shift".to_string(),
        "[1518-11-02 00:40] falls asleep".to_string(),
    ];
    let expected_guard_logs: Vec<GuardLog> = vec![
        GuardLog {
            guard_id: 10,
            state: GuardSate::BeginShift,
            month: 11,
            day: 1,
            hour: 0,
            minute: 0,
        },
        GuardLog {
            guard_id: 10,
            state: GuardSate::FallsAsleep,
            month: 11,
            day: 1,
            hour: 0,
            minute: 5,
        },
        GuardLog {
            guard_id: 10,
            state: GuardSate::WakesUp,
            month: 11,
            day: 1,
            hour: 0,
            minute: 25,
        },
        GuardLog {
            guard_id: 99,
            state: GuardSate::BeginShift,
            month: 11,
            day: 1,
            hour: 23,
            minute: 58,
        },
        GuardLog {
            guard_id: 99,
            state: GuardSate::FallsAsleep,
            month: 11,
            day: 2,
            hour: 0,
            minute: 40,
        },
    ];
    assert_eq!(expected_guard_logs, parse_input(input));
}

pub fn solve_part_one(input: &[String]) -> usize {
    let _guard_logs = parse_input(input);

    // Aggregate guard sleeping minutes
    // Find guard with highest sleeping minute count
    // For all shifts that guard had, which minute was most commonly slept in
    // Return guard_id * minute-most-commonly-slept-in

    0
}

pub fn solve_part_two(_input: &[String]) -> String {
    "TODO".to_string()
}

#[test]
fn test_part_one() {
    let input = &[
        "[1518-11-01 00:00] Guard #10 begins shift".to_string(),
        "[1518-11-01 00:05] falls asleep".to_string(),
        "[1518-11-01 00:25] wakes up".to_string(),
        "[1518-11-01 00:30] falls asleep".to_string(),
        "[1518-11-01 00:55] wakes up".to_string(),
        "[1518-11-01 23:58] Guard #99 begins shift".to_string(),
        "[1518-11-02 00:40] falls asleep".to_string(),
        "[1518-11-02 00:50] wakes up".to_string(),
        "[1518-11-03 00:05] Guard #10 begins shift".to_string(),
        "[1518-11-03 00:24] falls asleep".to_string(),
        "[1518-11-03 00:29] wakes up".to_string(),
        "[1518-11-04 00:02] Guard #99 begins shift".to_string(),
        "[1518-11-04 00:36] falls asleep".to_string(),
        "[1518-11-04 00:46] wakes up".to_string(),
        "[1518-11-05 00:03] Guard #99 begins shift".to_string(),
        "[1518-11-05 00:45] falls asleep".to_string(),
        "[1518-11-05 00:55] wakes up".to_string(),
    ];

    // TODO - enable correct assert when implemented
//    assert_eq!(240, solve_part_one(input))
    assert_eq!(0, solve_part_one(input))
}
