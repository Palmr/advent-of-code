/// --- Day 12: Rain Risk ---
///
/// Your ferry made decent progress toward the island, but the storm came in faster than anyone
/// expected. The ferry needs to take evasive actions!
///
/// Unfortunately, the ship's navigation computer seems to be malfunctioning; rather than giving a
/// route directly to safety, it produced extremely circuitous instructions. When the captain uses
/// the PA system to ask if anyone can help, you quickly volunteer.
///
/// The navigation instructions (your puzzle input) consists of a sequence of single-character
/// actions paired with integer input values. After staring at them for a few minutes, you work out
/// what they probably mean:
///
///  - Action `N` means to move north by the given value.
///  - Action `S` means to move south by the given value.
///  - Action `E` means to move east by the given value.
///  - Action `W` means to move west by the given value.
///  - Action `L` means to turn left the given number of degrees.
///  - Action `R` means to turn right the given number of degrees.
///  - Action `F` means to move forward by the given value in the direction the ship is currently facing.
///
/// The ship starts by facing east. Only the L and R actions change the direction the ship is facing.
/// (That is, if the ship is facing east and the next instruction is `N10`, the ship would move
/// north 10 units, but would still move east if the following action were `F`.)
///
/// For example:
///
/// ```
/// F10
/// N3
/// F7
/// R90
/// F11
/// ```
///
/// These instructions would be handled as follows:
///
///  - `F10` would move the ship 10 units east (because the ship starts by facing east) to east 10, north 0.
///  - `N3` would move the ship 3 units north to east 10, north 3.
///  - `F7` would move the ship another 7 units east (because the ship is still facing east) to east 17, north 3.
///  - `R90` would cause the ship to turn right by 90 degrees and face south; it remains at east 17, north 3.
///  - `F11` would move the ship 11 units south to east 17, south 8.
///
/// At the end of these instructions, the ship's Manhattan distance (sum of the absolute values of
/// its east/west position and its north/south position) from its starting position is `17 + 8 = 25`.
///
/// Figure out where the navigation instructions lead. What is the Manhattan distance between that
/// location and the ship's starting position?
///
/// --- Part Two ---
///
/// Before you can give the destination to the captain, you realize that the actual action meanings
/// were printed on the back of the instructions the whole time.
///
/// Almost all of the actions indicate how to move a waypoint which is relative to the ship's position:
///
///  - Action `N` means to move the waypoint north by the given value.
///  - Action `S` means to move the waypoint south by the given value.
///  - Action `E` means to move the waypoint east by the given value.
///  - Action `W` means to move the waypoint west by the given value.
///  - Action `L` means to rotate the waypoint around the ship left (counter-clockwise) the given number of degrees.
///  - Action `R` means to rotate the waypoint around the ship right (clockwise) the given number of degrees.
///  - Action `F` means to move forward to the waypoint a number of times equal to the given value.
///
/// The waypoint starts `10 units east` and `1 unit north` relative to the ship. The waypoint is
/// relative to the ship; that is, if the ship moves, the waypoint moves with it.
///
/// For example, using the same instructions as above:
///
///  - `F10` moves the ship to the waypoint 10 times (a total of 100 units east and 10 units north), leaving the ship at east 100, north 10. The waypoint stays 10 units east and 1 unit north of the ship.
///  - `N3` moves the waypoint 3 units north to 10 units east and 4 units north of the ship. The ship remains at east 100, north 10.
///  - `F7` moves the ship to the waypoint 7 times (a total of 70 units east and 28 units north), leaving the ship at east 170, north 38. The waypoint stays 10 units east and 4 units north of the ship.
///  - `R90` rotates the waypoint around the ship clockwise 90 degrees, moving it to 4 units east and 10 units south of the ship. The ship remains at east 170, north 38.
///  - `F11` moves the ship to the waypoint 11 times (a total of 44 units east and 110 units south), leaving the ship at east 214, south 72. The waypoint stays 4 units east and 10 units south of the ship.
///
/// After these operations, the ship's Manhattan distance from its starting position is 214 + 72 = 286.
///
/// Figure out where the navigation instructions actually lead. What is the Manhattan distance between that location and the ship's starting position?

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct Ship {
    direction: Direction,
    x: isize,
    y: isize,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            direction: Direction::East,
            x: 0,
            y: 0,
        }
    }

    fn distance_from_start(self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }

    fn move_north(&mut self, amount: isize) {
        self.y -= amount
    }

    fn move_south(&mut self, amount: isize) {
        self.y += amount
    }

    fn move_east(&mut self, amount: isize) {
        self.x += amount
    }

    fn move_west(&mut self, amount: isize) {
        self.x -= amount
    }

    fn turn_left(&mut self, amount: isize) {
        assert_eq!(
            0,
            amount % 90,
            "I assumed turns were multiples of 90 degrees only"
        );

        for _ in 0..amount / 90 {
            self.direction = match self.direction {
                Direction::North => Direction::West,
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
            }
        }
    }

    fn turn_right(&mut self, amount: isize) {
        assert_eq!(
            0,
            amount % 90,
            "I assumed turns were multiples of 90 degrees only"
        );

        for _ in 0..amount / 90 {
            self.direction = match self.direction {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            }
        }
    }

    fn move_forward(&mut self, amount: isize) {
        match self.direction {
            Direction::North => self.y -= amount,
            Direction::East => self.x += amount,
            Direction::South => self.y += amount,
            Direction::West => self.x -= amount,
        }
    }

    fn move_to_waypoint(&mut self, amount: isize, wpt: &Waypoint) {
        for _ in 0..amount {
            self.x += wpt.x_offset_from_ship;
            self.y += wpt.y_offset_from_ship;
        }
    }
}

#[derive(Debug)]
struct Waypoint {
    x_offset_from_ship: isize,
    y_offset_from_ship: isize,
}

impl Waypoint {
    fn new() -> Waypoint {
        Waypoint {
            x_offset_from_ship: 10,
            y_offset_from_ship: -1,
        }
    }

    fn move_north(&mut self, amount: isize) {
        self.y_offset_from_ship -= amount
    }

    fn move_south(&mut self, amount: isize) {
        self.y_offset_from_ship += amount
    }

    fn move_east(&mut self, amount: isize) {
        self.x_offset_from_ship += amount
    }

    fn move_west(&mut self, amount: isize) {
        self.x_offset_from_ship -= amount
    }

    fn turn_left(&mut self, amount: isize) {
        assert_eq!(
            0,
            amount % 90,
            "I assumed turns were multiples of 90 degrees only"
        );

        for _ in 0..amount / 90 {
            let new_x = self.y_offset_from_ship;
            let new_y = -self.x_offset_from_ship;
            self.x_offset_from_ship = new_x;
            self.y_offset_from_ship = new_y;
        }
    }

    fn turn_right(&mut self, amount: isize) {
        assert_eq!(
            0,
            amount % 90,
            "I assumed turns were multiples of 90 degrees only"
        );

        for _ in 0..amount / 90 {
            let new_x = -self.y_offset_from_ship;
            let new_y = self.x_offset_from_ship;
            self.x_offset_from_ship = new_x;
            self.y_offset_from_ship = new_y;
        }
    }
}

pub fn solve_part_one(input: &[String]) -> usize {
    let mut ship = Ship::new();

    input.iter().for_each(|line| {
        let action = line.chars().next().unwrap();
        let amount = line[1..].parse::<isize>().unwrap();

        match action {
            'N' => ship.move_north(amount),
            'S' => ship.move_south(amount),
            'E' => ship.move_east(amount),
            'W' => ship.move_west(amount),
            'L' => ship.turn_left(amount),
            'R' => ship.turn_right(amount),
            'F' => ship.move_forward(amount),
            _ => {}
        }
    });

    ship.distance_from_start()
}

pub fn solve_part_two(input: &[String]) -> usize {
    let mut ship = Ship::new();
    let mut wpt = Waypoint::new();

    input.iter().for_each(|line| {
        let action = line.chars().next().unwrap();
        let amount = line[1..].parse::<isize>().unwrap();

        match action {
            'N' => wpt.move_north(amount),
            'S' => wpt.move_south(amount),
            'E' => wpt.move_east(amount),
            'W' => wpt.move_west(amount),
            'L' => wpt.turn_left(amount),
            'R' => wpt.turn_right(amount),
            'F' => ship.move_to_waypoint(amount, &wpt),
            _ => {}
        }
    });

    ship.distance_from_start()
}

#[test]
fn examples_part_one() {
    assert_eq!(
        25,
        solve_part_one(&[
            "F10".to_string(),
            "N3".to_string(),
            "F7".to_string(),
            "R90".to_string(),
            "F11".to_string(),
        ])
    );
}

#[test]
fn examples_part_two() {
    assert_eq!(
        286,
        solve_part_two(&[
            "F10".to_string(),
            "N3".to_string(),
            "F7".to_string(),
            "R90".to_string(),
            "F11".to_string(),
        ])
    );
}
