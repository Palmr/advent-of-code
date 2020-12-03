use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
enum Move {
    Left(isize),
    Right(isize),
    Up(isize),
    Down(isize),
}

impl Move {
    pub fn parse(input: &str) -> Move {
        let distance: isize = input[1..].parse().unwrap();
        match &input[0..1] {
            "L" => Move::Left(distance),
            "R" => Move::Right(distance),
            "U" => Move::Up(distance),
            "D" => Move::Down(distance),
            _ => panic!("Invalid move: {}", input),
        }
    }
}

fn parse_wire(input: &str) -> Vec<Move> {
    input.split(',').map(|e| Move::parse(&e)).collect()
}

#[derive(Debug)]
struct WireCoord {
    x: isize,
    y: isize,
    length: isize,
}
impl Hash for WireCoord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}
impl PartialEq for WireCoord {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for WireCoord {}

fn realise_wire(wire: Vec<Move>) -> HashSet<WireCoord> {
    let mut coords = HashSet::new();

    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut length: isize = 0;

    for wire_move in wire {
        let mut dx: isize = 0;
        let mut dy: isize = 0;
        let dist = match wire_move {
            Move::Left(distance) => {
                dx = -1;
                distance
            }
            Move::Right(distance) => {
                dx = 1;
                distance
            }
            Move::Up(distance) => {
                dy = 1;
                distance
            }
            Move::Down(distance) => {
                dy = -1;
                distance
            }
        };

        for _i in 0..dist {
            x += dx;
            y += dy;
            length += 1;

            coords.insert(WireCoord { x, y, length });
        }
    }

    coords
}

fn find_minimum_manhattan_distance_intersection(
    wire1: HashSet<WireCoord>,
    wire2: HashSet<WireCoord>,
) -> isize {
    wire1
        .into_iter()
        .filter(|c| wire2.contains(c))
        .map(|wc| wc.x.abs() + wc.y.abs())
        .min()
        .unwrap()
}

fn find_minimum_length_intersection(wire1: HashSet<WireCoord>, wire2: HashSet<WireCoord>) -> isize {
    wire1
        .into_iter()
        .filter(|c| wire2.contains(c))
        .map(|w1c| w1c.length + wire2.get(&w1c).unwrap().length)
        .min()
        .unwrap()
}

pub fn solve_part_one(input: &[String]) -> isize {
    let wire1 = parse_wire(&input[0]);
    let wire2 = parse_wire(&input[1]);

    let wire1 = realise_wire(wire1);
    let wire2 = realise_wire(wire2);

    find_minimum_manhattan_distance_intersection(wire1, wire2)
}

pub fn solve_part_two(input: &[String]) -> isize {
    let wire1 = parse_wire(&input[0]);
    let wire2 = parse_wire(&input[1]);

    let wire1 = realise_wire(wire1);
    let wire2 = realise_wire(wire2);
    // println!("wire1 coords: {:?}", wire1);
    // println!("wire2 coords: {:?}", wire2);

    find_minimum_length_intersection(wire1, wire2)
}

#[test]
fn examples_part_one() {
    let input = &[
        "R75,D30,R83,U83,L12,D49,R71,U7,L72".to_string(),
        "U62,R66,U55,R34,D71,R55,D58,R83".to_string(),
    ];
    assert_eq!(159, solve_part_one(input));

    let input = &[
        "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_string(),
        "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string(),
    ];
    assert_eq!(135, solve_part_one(input));
}

#[test]
fn examples_part_two() {
    let input = &[
        "R75,D30,R83,U83,L12,D49,R71,U7,L72".to_string(),
        "U62,R66,U55,R34,D71,R55,D58,R83".to_string(),
    ];
    assert_eq!(610, solve_part_two(input));

    let input = &[
        "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_string(),
        "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string(),
    ];
    assert_eq!(410, solve_part_two(input));
}
