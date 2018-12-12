use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;

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
/// --- Part Two ---
///
/// Strategy 2: Of all guards, which guard is most frequently asleep on the same minute?
///
/// In the example above, Guard #99 spent minute 45 asleep more than any other guard or minute -
/// three times in total. (In all other cases, any guard spent any minute asleep at most twice.)
///
/// What is the ID of the guard you chose multiplied by the minute you chose? (In the above example,
/// the answer would be 99 * 45 = 4455.)
///

#[derive(Debug, Eq, PartialEq)]
enum GuardSate {
    BeginShift,
    FallsAsleep,
    WakesUp,
}

#[derive(Debug, Eq, PartialEq)]
struct GuardLog {
    guard_id: Option<usize>,
    state: GuardSate,
    month: usize,
    day: usize,
    hour: usize,
    minute: usize,
}

impl PartialOrd for GuardLog {
    fn partial_cmp(&self, other: &GuardLog) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GuardLog {
    fn cmp(&self, other: &Self) -> Ordering {
        self.month.cmp(&other.month).then(
            self.day.cmp(&other.day).then(
                self.hour
                    .cmp(&other.hour)
                    .then(self.minute.cmp(&other.minute)),
            ),
        )
    }
}

fn parse_input(input: &[String]) -> Vec<GuardLog> {
    let re =
        Regex::new(r"\[1518-([0-9]{2})-([0-9]{2}) ([0-9]{2}):([0-9]{2})\] (Guard #([0-9]+) )?(.*)")
            .unwrap();

    let mut states: Vec<GuardLog> = input
        .iter()
        //        .inspect(|l| println!("To parse: {}", l))
        .map(|l| re.captures(l).unwrap())
        //        .inspect(|c| println!("Captured: {:?}", c))
        .map(|c| GuardLog {
            guard_id: c.get(6).and_then(|m| Some(m.as_str().parse().unwrap())),
            state: c
                .get(7)
                .map(|m| match m.as_str() {
                    "begins shift" => GuardSate::BeginShift,
                    "falls asleep" => GuardSate::FallsAsleep,
                    "wakes up" => GuardSate::WakesUp,
                    x => panic!("Invalid state: {}", x),
                })
                .unwrap(),
            month: c.get(1).map_or(0, |m| m.as_str().parse().unwrap()),
            day: c.get(2).map_or(0, |m| m.as_str().parse().unwrap()),
            hour: c.get(3).map_or(0, |m| m.as_str().parse().unwrap()),
            minute: c.get(4).map_or(0, |m| m.as_str().parse().unwrap()),
        })
        .collect();

    states.sort();

    // Fill in guard ids
    let mut current_guard_id = None;
    states.iter_mut().for_each(|gl| match gl.state {
        GuardSate::BeginShift => current_guard_id = gl.guard_id,
        GuardSate::FallsAsleep | GuardSate::WakesUp => gl.guard_id = current_guard_id,
    });

    states
}

#[test]
fn test_parse_input() {
    let input = &[
        "[1518-11-01 23:58] Guard #99 begins shift".to_string(),
        "[1518-11-01 00:00] Guard #10 begins shift".to_string(),
        "[1518-11-01 00:25] wakes up".to_string(),
        "[1518-11-01 00:05] falls asleep".to_string(),
        "[1518-11-02 00:40] falls asleep".to_string(),
    ];
    let expected_guard_logs: Vec<GuardLog> = vec![
        GuardLog {
            guard_id: Some(10),
            state: GuardSate::BeginShift,
            month: 11,
            day: 1,
            hour: 0,
            minute: 0,
        },
        GuardLog {
            guard_id: Some(10),
            state: GuardSate::FallsAsleep,
            month: 11,
            day: 1,
            hour: 0,
            minute: 5,
        },
        GuardLog {
            guard_id: Some(10),
            state: GuardSate::WakesUp,
            month: 11,
            day: 1,
            hour: 0,
            minute: 25,
        },
        GuardLog {
            guard_id: Some(99),
            state: GuardSate::BeginShift,
            month: 11,
            day: 1,
            hour: 23,
            minute: 58,
        },
        GuardLog {
            guard_id: Some(99),
            state: GuardSate::FallsAsleep,
            month: 11,
            day: 2,
            hour: 0,
            minute: 40,
        },
    ];
    assert_eq!(expected_guard_logs, parse_input(input));
}

fn aggregate_guard_sleeping_time(guard_logs: &[GuardLog]) -> HashMap<usize, usize> {
    let mut guard_minutes_slept = HashMap::new();
    let mut started_sleeping_minute = None;
    guard_logs.iter().for_each(|gl| match gl.state {
        GuardSate::BeginShift => {}
        GuardSate::FallsAsleep => started_sleeping_minute = Some(gl.minute),
        GuardSate::WakesUp => {
            if started_sleeping_minute == None {
                panic!(
                    "Log waking up for guard who fell asleep at unknown time? {:?}",
                    gl
                );
            }
            let mins_slept = gl.minute - started_sleeping_minute.unwrap();
            *guard_minutes_slept.entry(gl.guard_id.unwrap()).or_insert(0) += mins_slept;
        }
    });

    guard_minutes_slept
}

#[test]
fn test_aggregate_guard_sleeping_time() {
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

    let mut expected = HashMap::new();
    expected.insert(10, 50);
    expected.insert(99, 30);

    assert_eq!(expected, aggregate_guard_sleeping_time(&parse_input(input)));
}

pub fn solve_part_one(input: &[String]) -> usize {
    let guard_logs = parse_input(input);

    // Aggregate guard sleeping minutes
    let guard_minutes_slept = aggregate_guard_sleeping_time(&guard_logs);

    // Find guard with highest sleeping minute count
    let sleepiest_guard_id = *guard_minutes_slept
        .iter()
        .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
        .unwrap()
        .0;

    // For all shifts that guard had, which minute was most commonly slept in
    let sleepiest_guards_shifts: Vec<&GuardLog> = guard_logs
        .iter()
        .filter(|gl| gl.guard_id.unwrap() == sleepiest_guard_id)
        .collect();

    let mut minute_slept_map = HashMap::new();
    let mut started_sleeping_minute = None;
    sleepiest_guards_shifts
        .iter()
        .for_each(|gl| match gl.state {
            GuardSate::BeginShift => started_sleeping_minute = None,
            GuardSate::FallsAsleep => started_sleeping_minute = Some(gl.minute),
            GuardSate::WakesUp => {
                if started_sleeping_minute == None {
                    panic!(
                        "Log waking up for guard who fell asleep at unknown time? {:?}",
                        gl
                    );
                }
                for m in started_sleeping_minute.unwrap()..gl.minute {
                    *minute_slept_map.entry(m).or_insert(0) += 1;
                }
            }
        });

    let sleepiest_minute = *minute_slept_map
        .iter()
        .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
        .unwrap()
        .0;

    // Return guard_id * minute-most-commonly-slept-in

    sleepiest_guard_id * sleepiest_minute
}

pub fn solve_part_two(input: &[String]) -> usize {
    let guard_logs = parse_input(input);

    // For each guard map their minute sleep frequency
    let mut guard_minute_sleep_frequency: HashMap<usize, HashMap<usize, usize>> = HashMap::new();
    let mut started_sleeping_minute = None;
    guard_logs.iter().for_each(|gl| match gl.state {
        GuardSate::BeginShift => started_sleeping_minute = None,
        GuardSate::FallsAsleep => started_sleeping_minute = Some(gl.minute),
        GuardSate::WakesUp => {
            if started_sleeping_minute == None {
                panic!(
                    "Log waking up for guard who fell asleep at unknown time? {:?}",
                    gl
                );
            }
            let mut minute_slept_map = guard_minute_sleep_frequency
                .entry(gl.guard_id.unwrap())
                .or_insert_with(HashMap::new);
            for m in started_sleeping_minute.unwrap()..gl.minute {
                *minute_slept_map.entry(m).or_insert(0) += 1;
            }
        }
    });

    // For each guard find the minute with the highest frequency
    // For all those frequencies, find the highest
    let thing: (usize, (&usize, &usize)) = guard_minute_sleep_frequency
        .iter()
        .map(|(guard_id, minutes_slept_frequencies)| {
            (
                *guard_id,
                minutes_slept_frequencies
                    .iter()
                    .max_by(|(_min1, freq1), (_min2, freq2)| freq1.cmp(freq2))
                    .unwrap(),
            )
        })
        //        .inspect(|(guard_id, (min, max_freq))| {println!("g: {}, maxf: {} @ {}", guard_id, max_freq, min)})
        .max_by(
            |(_guard_id1, (_min1, max_freq1)), (_guard_id2, (_min2, max_freq2))| {
                max_freq1.cmp(max_freq2)
            },
        )
        .unwrap();

    // Return guard_id * minute-most-commonly-slept-in

    thing.0 * (thing.1).0
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

    assert_eq!(240, solve_part_one(input))
}

#[test]
fn test_part_two() {
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

    assert_eq!(4455, solve_part_two(input))
}
