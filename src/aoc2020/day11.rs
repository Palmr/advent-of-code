/// --- Day 11: Seating System ---
///
/// Your plane lands with plenty of time to spare. The final leg of your journey is a ferry that
/// goes directly to the tropical island where you can finally start your vacation. As you reach the
/// waiting area to board the ferry, you realize you're so early, nobody else has even arrived yet!
///
/// By modeling the process people use to choose (or abandon) their seat in the waiting area, you're
/// pretty sure you can predict the best place to sit. You make a quick map of the seat layout
/// (your puzzle input).
///
/// The seat layout fits neatly on a grid. Each position is either floor (`.`), an empty seat (`L`),
/// or an occupied seat (`#`). For example, the initial seat layout might look like this:
///
/// ```
/// L.LL.LL.LL
/// LLLLLLL.LL
/// L.L.L..L..
/// LLLL.LL.LL
/// L.LL.LL.LL
/// L.LLLLL.LL
/// ..L.L.....
/// LLLLLLLLLL
/// L.LLLLLL.L
/// L.LLLLL.LL
/// ```
///
/// Now, you just need to model the people who will be arriving shortly. Fortunately, people are
/// entirely predictable and always follow a simple set of rules. All decisions are based on the
/// number of occupied seats adjacent to a given seat (one of the eight positions immediately up,
/// down, left, right, or diagonal from the seat). The following rules are applied to every seat
/// simultaneously:
///
///  - If a seat is empty (`L`) and there are no occupied seats adjacent to it, the seat becomes occupied.
///  - If a seat is occupied (`#`) and four or more seats adjacent to it are also occupied, the seat becomes empty.
///  - Otherwise, the seat's state does not change.
///
/// Floor (`.`) never changes; seats don't move, and nobody sits on the floor.
///
/// After one round of these rules, every seat in the example layout becomes occupied:
///
/// ```
/// #.##.##.##
/// #######.##
/// #.#.#..#..
/// ####.##.##
/// #.##.##.##
/// #.#####.##
/// ..#.#.....
/// ##########
/// #.######.#
/// #.#####.##
/// ```
///
/// After a second round, the seats with four or more occupied adjacent seats become empty again:
///
/// ```
/// #.LL.L#.##
/// #LLLLLL.L#
/// L.L.L..L..
/// #LLL.LL.L#
/// #.LL.LL.LL
/// #.LLLL#.##
/// ..L.L.....
/// #LLLLLLLL#
/// #.LLLLLL.L
/// #.#LLLL.##
/// ```
///
/// This process continues for three more rounds:
///
/// ```
/// #.##.L#.##
/// #L###LL.L#
/// L.#.#..#..
/// #L##.##.L#
/// #.##.LL.LL
/// #.###L#.##
/// ..#.#.....
/// #L######L#
/// #.LL###L.L
/// #.#L###.##
///
/// #.#L.L#.##
/// #LLL#LL.L#
/// L.L.L..#..
/// #LLL.##.L#
/// #.LL.LL.LL
/// #.LL#L#.##
/// ..L.L.....
/// #L#LLLL#L#
/// #.LLLLLL.L
/// #.#L#L#.##
///
/// #.#L.L#.##
/// #LLL#LL.L#
/// L.#.L..#..
/// #L##.##.L#
/// #.#L.LL.LL
/// #.#L#L#.##
/// ..L.L.....
/// #L#L##L#L#
/// #.LLLLLL.L
/// #.#L#L#.##
/// ```
///
/// At this point, something interesting happens: the chaos stabilizes and further applications of
/// these rules cause no seats to change state! Once people stop moving around, you count 37
/// occupied seats.
///
/// Simulate your seating area by applying the seating rules repeatedly until no seats change state.
/// How many seats end up occupied?
///
/// --- Part Two ---
///
/// As soon as people start to arrive, you realize your mistake. People don't just care about
/// adjacent seats - they care about the first seat they can see in each of those eight directions!
///
/// Now, instead of considering just the eight immediately adjacent seats, consider the first seat
/// in each of those eight directions. For example, the empty seat below would see eight occupied seats:
///
/// ```
/// .......#.
/// ...#.....
/// .#.......
/// .........
/// ..#L....#
/// ....#....
/// .........
/// #........
/// ...#.....
/// ```
///
/// The leftmost empty seat below would only see one empty seat, but cannot see any of the occupied ones:
///
/// ```
/// .............
/// .L.L.#.#.#.#.
/// .............
/// ```
///
/// The empty seat below would see no occupied seats:
///
/// ```
/// .##.##.
/// #.#.#.#
/// ##...##
/// ...L...
/// ##...##
/// #.#.#.#
/// .##.##.
/// ```
///
/// Also, people seem to be more tolerant than you expected: it now takes five or more visible
/// occupied seats for an occupied seat to become empty (rather than four or more from the previous
/// rules). The other rules still apply: empty seats that see no occupied seats become occupied,
/// seats matching no rule don't change, and floor never changes.
///
/// Given the same starting layout as above, these new rules cause the seating area to shift around
/// as follows:
///
/// ```
/// L.LL.LL.LL
/// LLLLLLL.LL
/// L.L.L..L..
/// LLLL.LL.LL
/// L.LL.LL.LL
/// L.LLLLL.LL
/// ..L.L.....
/// LLLLLLLLLL
/// L.LLLLLL.L
/// L.LLLLL.LL
///
/// #.##.##.##
/// #######.##
/// #.#.#..#..
/// ####.##.##
/// #.##.##.##
/// #.#####.##
/// ..#.#.....
/// ##########
/// #.######.#
/// #.#####.##
///
/// #.LL.LL.L#
/// #LLLLLL.LL
/// L.L.L..L..
/// LLLL.LL.LL
/// L.LL.LL.LL
/// L.LLLLL.LL
/// ..L.L.....
/// LLLLLLLLL#
/// #.LLLLLL.L
/// #.LLLLL.L#
///
/// #.L#.##.L#
/// #L#####.LL
/// L.#.#..#..
/// ##L#.##.##
/// #.##.#L.##
/// #.#####.#L
/// ..#.#.....
/// LLL####LL#
/// #.L#####.L
/// #.L####.L#
///
/// #.L#.L#.L#
/// #LLLLLL.LL
/// L.L.L..#..
/// ##LL.LL.L#
/// L.LL.LL.L#
/// #.LLLLL.LL
/// ..L.L.....
/// LLLLLLLLL#
/// #.LLLLL#.L
/// #.L#LL#.L#
///
/// #.L#.L#.L#
/// #LLLLLL.LL
/// L.L.L..#..
/// ##L#.#L.L#
/// L.L#.#L.L#
/// #.L####.LL
/// ..#.#.....
/// LLL###LLL#
/// #.LLLLL#.L
/// #.L#LL#.L#
///
/// #.L#.L#.L#
/// #LLLLLL.LL
/// L.L.L..#..
/// ##L#.#L.L#
/// L.L#.LL.L#
/// #.LLLL#.LL
/// ..#.L.....
/// LLL###LLL#
/// #.LLLLL#.L
/// #.L#LL#.L#
/// ```
///
/// Again, at this point, people stop shifting around and the seating area reaches equilibrium. Once this occurs, you count 26 occupied seats.
///
/// Given the new visibility method and the rule change for occupied seats becoming empty, once equilibrium is reached, how many seats end up occupied?

#[derive(Debug, PartialEq, Clone)]
enum Space {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

fn parse_spaces(input: &[String]) -> Vec<Vec<Space>> {
    input
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| match c {
                    '.' => Space::Floor,
                    'L' => Space::EmptySeat,
                    '#' => Space::OccupiedSeat,
                    _ => panic!("Unexpected space character: {}", c),
                })
                .collect()
        })
        .collect()
}

fn count_occupied_seats(spaces: &[Vec<Space>]) -> usize {
    spaces.iter().fold(0, |acc, row| {
        acc + row.iter().filter(|s| **s == Space::OccupiedSeat).count()
    })
}

fn adjacent_one(spaces: &[Vec<Space>], x: usize, y: usize, width: usize, height: usize) -> usize {
    let mut adjacent = Vec::new();

    for yy in y as isize - 1..=y as isize + 1 {
        if yy >= 0 && yy < height as isize {
            for xx in x as isize - 1..=x as isize + 1 {
                if xx >= 0 && xx < width as isize && !(xx as usize == x && yy as usize == y) {
                    adjacent.push(&spaces[yy as usize][xx as usize]);
                }
            }
        }
    }

    adjacent
        .iter()
        .filter(|s| ***s == Space::OccupiedSeat)
        .count()
}

fn adjacent_two(spaces: &[Vec<Space>], x: usize, y: usize, width: usize, height: usize) -> usize {
    let vectors: [(isize, isize); 8] = [
        (0, 1),   // S
        (1, 1),   // SE
        (1, 0),   // E
        (1, -1),  // NE
        (0, -1),  // N
        (-1, -1), // NW
        (-1, 0),  // W
        (-1, 1),  // SW
    ];

    let mut adjacent_occupied = 0;
    vectors.iter().for_each(|(dx, dy)| {
        let mut xx = x as isize + dx;
        let mut yy = y as isize + dy;
        while xx >= 0 && (xx as usize) < width && yy >= 0 && (yy as usize) < height {
            match spaces[yy as usize][xx as usize] {
                Space::Floor => {}
                Space::EmptySeat => {
                    break;
                }
                Space::OccupiedSeat => {
                    adjacent_occupied += 1;
                    break;
                }
            }
            xx += dx;
            yy += dy;
        }
    });

    adjacent_occupied
}

fn cycle_automata<F>(
    spaces: Vec<Vec<Space>>,
    adjacent_occupied_function: F,
    adjacency_tolerance: usize,
) -> Vec<Vec<Space>>
where
    F: Fn(&[Vec<Space>], usize, usize, usize, usize) -> usize,
{
    let mut output = spaces.clone();
    let height = spaces.len();
    let width = spaces[0].len();

    for y in 0..height {
        for x in 0..width {
            match &spaces[y][x] {
                Space::Floor => { /* do nothing */ }
                Space::EmptySeat => {
                    // If a seat is empty (`L`) and there are no occupied seats adjacent to it, the seat becomes occupied.
                    if adjacent_occupied_function(&spaces, x, y, width, height) == 0 {
                        output[y][x] = Space::OccupiedSeat;
                    }
                }
                Space::OccupiedSeat => {
                    // If a seat is occupied (`#`) and four or more seats adjacent to it are also occupied, the seat becomes empty.
                    if adjacent_occupied_function(&spaces, x, y, width, height)
                        >= adjacency_tolerance
                    {
                        output[y][x] = Space::EmptySeat;
                    }
                }
            }
        }
    }

    output
}

pub fn solve_part_one(input: &[String]) -> usize {
    let mut spaces = parse_spaces(input);

    loop {
        let occupied_before = count_occupied_seats(&spaces);
        spaces = cycle_automata(spaces, adjacent_one, 4);
        let occupied_after = count_occupied_seats(&spaces);

        if occupied_before == occupied_after {
            return occupied_after;
        }
    }
}

pub fn solve_part_two(input: &[String]) -> usize {
    let mut spaces = parse_spaces(input);

    loop {
        let occupied_before = count_occupied_seats(&spaces);
        spaces = cycle_automata(spaces, adjacent_two, 5);
        let occupied_after = count_occupied_seats(&spaces);

        if occupied_before == occupied_after {
            return occupied_after;
        }
    }
}

#[test]
fn examples_part_one() {
    assert_eq!(
        37,
        solve_part_one(&[
            "L.LL.LL.LL".to_string(),
            "LLLLLLL.LL".to_string(),
            "L.L.L..L..".to_string(),
            "LLLL.LL.LL".to_string(),
            "L.LL.LL.LL".to_string(),
            "L.LLLLL.LL".to_string(),
            "..L.L.....".to_string(),
            "LLLLLLLLLL".to_string(),
            "L.LLLLLL.L".to_string(),
            "L.LLLLL.LL".to_string(),
        ])
    );
}

#[test]
fn examples_part_two() {
    assert_eq!(
        26,
        solve_part_two(&[
            "L.LL.LL.LL".to_string(),
            "LLLLLLL.LL".to_string(),
            "L.L.L..L..".to_string(),
            "LLLL.LL.LL".to_string(),
            "L.LL.LL.LL".to_string(),
            "L.LLLLL.LL".to_string(),
            "..L.L.....".to_string(),
            "LLLLLLLLLL".to_string(),
            "L.LLLLLL.L".to_string(),
            "L.LLLLL.LL".to_string(),
        ])
    );
}

#[test]
fn test_count_occupied_seats() {
    assert_eq!(
        37,
        count_occupied_seats(&parse_spaces(&[
            "#.#L.L#.##".to_string(),
            "#LLL#LL.L#".to_string(),
            "L.#.L..#..".to_string(),
            "#L##.##.L#".to_string(),
            "#.#L.LL.LL".to_string(),
            "#.#L#L#.##".to_string(),
            "..L.L.....".to_string(),
            "#L#L##L#L#".to_string(),
            "#.LLLLLL.L".to_string(),
            "#.#L#L#.##".to_string(),
        ]))
    );
}

#[test]
fn test_adjacent_one() {
    assert_eq!(
        2,
        adjacent_one(
            &parse_spaces(&[
                ".......#.".to_string(),
                "...#.....".to_string(),
                ".#.......".to_string(),
                ".........".to_string(),
                "..#L....#".to_string(),
                "....#....".to_string(),
                ".........".to_string(),
                "#........".to_string(),
                "...#.....".to_string(),
            ]),
            3,
            4,
            9,
            9
        )
    );
    assert_eq!(
        0,
        adjacent_one(
            &parse_spaces(&[
                ".............".to_string(),
                ".L.L.#.#.#.#.".to_string(),
                ".............".to_string(),
            ]),
            1,
            1,
            13,
            3
        )
    );
    assert_eq!(
        0,
        adjacent_one(
            &parse_spaces(&[
                ".##.##.".to_string(),
                "#.#.#.#".to_string(),
                "##...##".to_string(),
                "...L...".to_string(),
                "##...##".to_string(),
                "#.#.#.#".to_string(),
                ".##.##.".to_string(),
            ]),
            3,
            3,
            7,
            7
        )
    );
}

#[test]
fn test_adjacent_two() {
    assert_eq!(
        8,
        adjacent_two(
            &parse_spaces(&[
                ".......#.".to_string(),
                "...#.....".to_string(),
                ".#.......".to_string(),
                ".........".to_string(),
                "..#L....#".to_string(),
                "....#....".to_string(),
                ".........".to_string(),
                "#........".to_string(),
                "...#.....".to_string(),
            ]),
            3,
            4,
            9,
            9
        )
    );
    assert_eq!(
        0,
        adjacent_two(
            &parse_spaces(&[
                ".............".to_string(),
                ".L.L.#.#.#.#.".to_string(),
                ".............".to_string(),
            ]),
            1,
            1,
            13,
            3
        )
    );
    assert_eq!(
        0,
        adjacent_two(
            &parse_spaces(&[
                ".##.##.".to_string(),
                "#.#.#.#".to_string(),
                "##...##".to_string(),
                "...L...".to_string(),
                "##...##".to_string(),
                "#.#.#.#".to_string(),
                ".##.##.".to_string(),
            ]),
            3,
            3,
            7,
            7
        )
    );
}

#[test]
fn test_cycle_automata_one() {
    let starting_spaces = parse_spaces(&[
        "L.LL.LL.LL".to_string(),
        "LLLLLLL.LL".to_string(),
        "L.L.L..L..".to_string(),
        "LLLL.LL.LL".to_string(),
        "L.LL.LL.LL".to_string(),
        "L.LLLLL.LL".to_string(),
        "..L.L.....".to_string(),
        "LLLLLLLLLL".to_string(),
        "L.LLLLLL.L".to_string(),
        "L.LLLLL.LL".to_string(),
    ]);

    let updated_spaces = cycle_automata(starting_spaces, adjacent_one, 4);
    assert_eq!(
        parse_spaces(&[
            "#.##.##.##".to_string(),
            "#######.##".to_string(),
            "#.#.#..#..".to_string(),
            "####.##.##".to_string(),
            "#.##.##.##".to_string(),
            "#.#####.##".to_string(),
            "..#.#.....".to_string(),
            "##########".to_string(),
            "#.######.#".to_string(),
            "#.#####.##".to_string(),
        ]),
        updated_spaces
    );

    let updated_spaces = cycle_automata(updated_spaces, adjacent_one, 4);
    assert_eq!(
        parse_spaces(&[
            "#.LL.L#.##".to_string(),
            "#LLLLLL.L#".to_string(),
            "L.L.L..L..".to_string(),
            "#LLL.LL.L#".to_string(),
            "#.LL.LL.LL".to_string(),
            "#.LLLL#.##".to_string(),
            "..L.L.....".to_string(),
            "#LLLLLLLL#".to_string(),
            "#.LLLLLL.L".to_string(),
            "#.#LLLL.##".to_string(),
        ]),
        updated_spaces
    );
}

#[test]
fn test_cycle_automata_two() {
    let starting_spaces = parse_spaces(&[
        "L.LL.LL.LL".to_string(),
        "LLLLLLL.LL".to_string(),
        "L.L.L..L..".to_string(),
        "LLLL.LL.LL".to_string(),
        "L.LL.LL.LL".to_string(),
        "L.LLLLL.LL".to_string(),
        "..L.L.....".to_string(),
        "LLLLLLLLLL".to_string(),
        "L.LLLLLL.L".to_string(),
        "L.LLLLL.LL".to_string(),
    ]);

    let updated_spaces = cycle_automata(starting_spaces, adjacent_two, 5);
    assert_eq!(
        parse_spaces(&[
            "#.##.##.##".to_string(),
            "#######.##".to_string(),
            "#.#.#..#..".to_string(),
            "####.##.##".to_string(),
            "#.##.##.##".to_string(),
            "#.#####.##".to_string(),
            "..#.#.....".to_string(),
            "##########".to_string(),
            "#.######.#".to_string(),
            "#.#####.##".to_string(),
        ]),
        updated_spaces
    );

    let updated_spaces = cycle_automata(updated_spaces, adjacent_two, 5);
    assert_eq!(
        parse_spaces(&[
            "#.LL.LL.L#".to_string(),
            "#LLLLLL.LL".to_string(),
            "L.L.L..L..".to_string(),
            "LLLL.LL.LL".to_string(),
            "L.LL.LL.LL".to_string(),
            "L.LLLLL.LL".to_string(),
            "..L.L.....".to_string(),
            "LLLLLLLLL#".to_string(),
            "#.LLLLLL.L".to_string(),
            "#.LLLLL.L#".to_string(),
        ]),
        updated_spaces
    );
}
