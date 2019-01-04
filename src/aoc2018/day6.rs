use regex::Regex;

/// --- Day 6: Chronal Coordinates ---
///
/// The device on your wrist beeps several times, and once again you feel like you're falling.
///
/// "Situation critical," the device announces. "Destination indeterminate. Chronal interference
/// detected. Please specify new target coordinates."
///
/// The device then produces a list of coordinates (your puzzle input). Are they places it thinks
/// are safe or dangerous? It recommends you check manual page 729. The Elves did not give you a
/// manual.
///
/// If they're dangerous, maybe you can minimize the danger by finding the coordinate that gives the
/// largest distance from the other points.
///
/// Using only the Manhattan distance, determine the area around each coordinate by counting the
/// number of integer X,Y locations that are closest to that coordinate (and aren't tied in distance
/// to any other coordinate).
///
/// Your goal is to find the size of the largest area that isn't infinite. For example, consider the
/// following list of coordinates:
///
/// 1, 1
/// 1, 6
/// 8, 3
/// 3, 4
/// 5, 5
/// 8, 9
///
/// If we name these coordinates A through F, we can draw them on a grid, putting 0,0 at the top
/// left:
///
/// ..........
/// .A........
/// ..........
/// ........C.
/// ...D......
/// .....E....
/// .B........
/// ..........
/// ..........
/// ........F.
///
/// This view is partial - the actual grid extends infinitely in all directions. Using the Manhattan
/// distance, each location's closest coordinate can be determined, shown here in lowercase:
///
/// aaaaa.cccc
/// aAaaa.cccc
/// aaaddecccc
/// aadddeccCc
/// ..dDdeeccc
/// bb.deEeecc
/// bBb.eeee..
/// bbb.eeefff
/// bbb.eeffff
/// bbb.ffffFf
///
/// Locations shown as . are equally far from two or more coordinates, and so they don't count as
/// being closest to any.
///
/// In this example, the areas of coordinates A, B, C, and F are infinite - while not shown here,
/// their areas extend forever outside the visible grid. However, the areas of coordinates D and E
/// are finite: D is closest to 9 locations, and E is closest to 17 (both including the coordinate's
/// location itself). Therefore, in this example, the size of the largest area is 17.
///
/// What is the size of the largest area that isn't infinite?

#[derive(Debug)]
struct Coord {
    x: isize,
    y: isize,
}

fn parse_input(input: &[String]) -> Vec<Coord> {
    let re = Regex::new(r"([0-9]+), *([0-9]+)").unwrap();

    input
        .iter()
        //        .inspect(|l| println!("To parse: {}", l))
        .map(|l| re.captures(l).unwrap())
        .map(|c| Coord {
            x: c.get(1).map_or(0, |m| m.as_str().parse().unwrap()),
            y: c.get(2).map_or(0, |m| m.as_str().parse().unwrap()),
        })
        .collect()
}

pub fn solve_part_one(input: &[String]) -> usize {
    let coords = parse_input(input);

    let min_x = coords.iter().map(|c| c.x).min().unwrap();
    let max_x = coords.iter().map(|c| c.x).max().unwrap();
    let min_y = coords.iter().map(|c| c.y).min().unwrap();
    let max_y = coords.iter().map(|c| c.y).max().unwrap();
    println!("Bounds: {},{} - {},{}", min_x, min_y, max_x, max_y);

//    let coord_areas;
    for gx in min_x..=max_x {
        for gy in min_y..=max_y {
            println!("grid cell to check: {},{}", gx, gy);
            // Find distance to each coord (x-delta + y-delta)
            let coord_distances = coords
                .iter()
                .enumerate()
                .map(|(id, coord)|
                         (id, manhattan_distance(coord, &Coord{x: gx, y: gy}))
                )
                .collect();

            // Find min distance, ignore if multiple with same distance
            // if cell is on boundary, mark is as extending to infinite for the coord_areas[coord_id]
            // Increment count for coord_areas[coord_id]
        }
    }

    // Return cell_size with highest count

    0
}

fn manhattan_distance(c1: &Coord, c2: &Coord) -> isize {
    (c1.x - c2.x).abs() + (c1.y + c2.y).abs()
}

#[test]
fn test_manhattan_distance() {
    assert_eq!(10, manhattan_distance(&Coord{x: 1, y: 1}, &Coord{x: 5, y: 5}))
}

pub fn solve_part_two(_input: &[String]) -> String {
    "TODO".to_string()
}
//
//#[test]
//fn examples_part_one() {
//    let input = &[
//        "1, 1".to_string(),
//        "1, 6".to_string(),
//        "8, 3".to_string(),
//        "3, 4".to_string(),
//        "5, 5".to_string(),
//        "8, 9".to_string(),
//    ];
//
//    assert_eq!(17, solve_part_one(input));
//}

#[test]
fn examples_part_two() {}
