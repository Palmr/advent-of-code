use std::collections::HashMap;
use std::fmt;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::sequence::separated_pair;
use nom::IResult;

/// --- Day 5: Hydrothermal Venture ---
///
/// You come across a field of hydrothermal vents on the ocean floor! These vents constantly produce
/// large, opaque clouds, so it would be best to avoid them if possible.
///
/// They tend to form in lines; the submarine helpfully produces a list of nearby lines of vents
/// (your puzzle input) for you to review. For example:
///
/// ```
/// 0,9 -> 5,9
/// 8,0 -> 0,8
/// 9,4 -> 3,4
/// 2,2 -> 2,1
/// 7,0 -> 7,4
/// 6,4 -> 2,0
/// 0,9 -> 2,9
/// 3,4 -> 1,4
/// 0,0 -> 8,8
/// 5,5 -> 8,2
/// ```
///
/// Each line of vents is given as a line segment in the format x1,y1 -> x2,y2 where x1,y1 are the
/// coordinates of one end the line segment and x2,y2 are the coordinates of the other end.
/// These line segments include the points at both ends. In other words:
///
/// ```
///     An entry like 1,1 -> 1,3 covers points 1,1, 1,2, and 1,3.
///     An entry like 9,7 -> 7,7 covers points 9,7, 8,7, and 7,7.
/// ```
///
/// For now, only consider horizontal and vertical lines: lines where either x1 = x2 or y1 = y2.
///
/// So, the horizontal and vertical lines from the above list would produce the following diagram:
///
/// ```
/// .......1..
/// ..1....1..
/// ..1....1..
/// .......1..
/// .112111211
/// ..........
/// ..........
/// ..........
/// ..........
/// 222111....
/// ```
///
/// In this diagram, the top left corner is 0,0 and the bottom right corner is 9,9. Each position is
/// shown as the number of lines which cover that point or . if no line covers that point.
/// The top-left pair of 1s, for example, comes from 2,2 -> 2,1;
/// the very bottom row is formed by the overlapping lines 0,9 -> 5,9 and 0,9 -> 2,9.
///
/// To avoid the most dangerous areas, you need to determine the number of points where at least two
/// lines overlap. In the above example, this is anywhere in the diagram with a 2 or larger - a
/// total of 5 points.
///
/// Consider only horizontal and vertical lines. At how many points do at least two lines overlap?
///
/// --- Part Two ---
///
/// Unfortunately, considering only horizontal and vertical lines doesn't give you the full picture;
/// you need to also consider diagonal lines.
///
/// Because of the limits of the hydrothermal vent mapping system, the lines in your list will only
/// ever be horizontal, vertical, or a diagonal line at exactly 45 degrees. In other words:
///
/// ```
///     An entry like 1,1 -> 3,3 covers points 1,1, 2,2, and 3,3.
///     An entry like 9,7 -> 7,9 covers points 9,7, 8,8, and 7,9.
/// ```
///
/// Considering all lines from the above example would now produce the following diagram:
///
/// ```
/// 1.1....11.
/// .111...2..
/// ..2.1.111.
/// ...1.2.2..
/// .112313211
/// ...1.2....
/// ..1...1...
/// .1.....1..
/// 1.......1.
/// 222111....
/// ```
///
/// You still need to determine the number of points where at least two lines overlap. In the above
/// example, this is still anywhere in the diagram with a 2 or larger - now a total of 12 points.
///
/// Consider all of the lines. At how many points do at least two lines overlap?
#[derive(PartialEq, Debug)]
struct Line {
    start_x: isize,
    start_y: isize,
    end_x: isize,
    end_y: isize,
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{},{} -> {},{} ",
            self.start_x, self.start_y, self.end_x, self.end_y
        )
    }
}

impl Line {
    pub fn new(line_descriptor: &str) -> Line {
        match Line::parse_line(line_descriptor) {
            Ok((_, line)) => line,
            Err(e) => panic!("Error parsing line {:?}", e),
        }
    }

    fn parse_line(input: &str) -> IResult<&str, Line> {
        let (input, (start_x, start_y)) =
            separated_pair(Line::parse_digits, tag(","), Line::parse_digits)(input)?;
        let (input, _) = tag(" -> ")(input)?;
        let (input, (end_x, end_y)) =
            separated_pair(Line::parse_digits, tag(","), Line::parse_digits)(input)?;

        Ok((
            input,
            Line {
                start_x,
                start_y,
                end_x,
                end_y,
            },
        ))
    }
    fn parse_digits(input: &str) -> IResult<&str, isize> {
        map_res(digit1, |s: &str| s.parse::<isize>())(input)
    }

    pub fn is_horizontal_or_vertical(&self) -> bool {
        self.start_x == self.end_x || self.start_y == self.end_y
    }

    pub fn get_line_points(&self) -> Vec<(isize, isize)> {
        let dx = self.end_x - self.start_x;
        let dy = self.end_y - self.start_y;

        (0..=(dx.abs().max(dy.abs())))
            .map(|offset| {
                let x = self.start_x + offset * dx.signum();
                let y = self.start_y + offset * dy.signum();
                (x, y)
            })
            .collect()
    }
}

pub fn solve_part_one(input: &[String]) -> usize {
    input
        .iter()
        .map(|line_desc| Line::new(line_desc))
        .filter(|line| line.is_horizontal_or_vertical())
        .flat_map(|line| line.get_line_points())
        .scan(HashMap::new(), |state, point| {
            *state.entry(point).or_insert(0) += 1;
            Some(state.clone())
        })
        .last()
        .unwrap()
        .values()
        .filter(|&v| v > &1)
        .count()
}

pub fn solve_part_two(input: &[String]) -> usize {
    let coord_counts = input
        .iter()
        .map(|line_desc| Line::new(line_desc))
        .flat_map(|line| line.get_line_points())
        .scan(HashMap::new(), |state, point| {
            *state.entry(point).or_insert(0) += 1;
            Some(state.clone())
        })
        .last()
        .unwrap();

    // Debug draw counts
    // let max_x = *coord_counts.keys().map(|(x, _y)| x).max().unwrap();
    // let max_y = *coord_counts.keys().map(|(_x, y)| y).max().unwrap();
    //
    // (0..=max_y).for_each(|y| {
    //     (0..=max_x).for_each(|x| match coord_counts.get(&(x as isize, y as isize)) {
    //         None => {
    //             print!(".")
    //         }
    //         Some(count) => {
    //             print!("{}", count)
    //         }
    //     });
    //     println!();
    // });

    coord_counts.values().filter(|&v| v > &1).count()
}

#[cfg(test)]
mod tests {
    use aoc2021::day5::{solve_part_one, solve_part_two, Line};
    use util::read_file_input;

    #[test]
    fn line_creation() {
        assert_eq!(
            Line {
                start_x: 0,
                start_y: 1,
                end_x: 2,
                end_y: 3,
            },
            Line::new("0,1 -> 2,3")
        );
    }

    #[test]
    fn line_horizontal_or_vertical_test() {
        assert_eq!(true, Line::new("0,9 -> 5,9").is_horizontal_or_vertical());
        assert_eq!(false, Line::new("8,0 -> 0,8").is_horizontal_or_vertical());
        assert_eq!(true, Line::new("9,4 -> 3,4").is_horizontal_or_vertical());
        assert_eq!(true, Line::new("2,2 -> 2,1").is_horizontal_or_vertical());
        assert_eq!(true, Line::new("7,0 -> 7,4").is_horizontal_or_vertical());
        assert_eq!(false, Line::new("6,4 -> 2,0").is_horizontal_or_vertical());
        assert_eq!(true, Line::new("0,9 -> 2,9").is_horizontal_or_vertical());
        assert_eq!(true, Line::new("3,4 -> 1,4").is_horizontal_or_vertical());
        assert_eq!(false, Line::new("0,0 -> 8,8").is_horizontal_or_vertical());
        assert_eq!(false, Line::new("5,5 -> 8,2").is_horizontal_or_vertical());
    }

    #[test]
    fn line_points() {
        assert_eq!(
            vec![(1, 1), (1, 2), (1, 3)],
            Line::new("1,1 -> 1,3").get_line_points()
        );
        assert_eq!(
            vec![(9, 7), (8, 7), (7, 7)],
            Line::new("9,7 -> 7,7").get_line_points()
        );

        assert_eq!(
            vec![(1, 1), (2, 2), (3, 3)],
            Line::new("1,1 -> 3,3").get_line_points()
        );
        assert_eq!(
            vec![(9, 7), (8, 8), (7, 9)],
            Line::new("9,7 -> 7,9").get_line_points()
        );
    }

    #[test]
    fn examples_part_one() {
        assert_eq!(
            5,
            solve_part_one(&read_file_input("resources/2021/day5_example.txt"))
        );
    }

    #[test]
    fn examples_part_two() {
        assert_eq!(
            12,
            solve_part_two(&read_file_input("resources/2021/day5_example.txt"))
        );
    }
}
