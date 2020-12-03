/// --- Day 3: Toboggan Trajectory ---
///
/// With the toboggan login problems resolved, you set off toward the airport. While travel by toboggan might be easy, it's certainly not safe: there's very minimal steering and the area is covered in trees. You'll need to see which angles will take you near the fewest trees.
///
/// Due to the local geology, trees in this area only grow on exact integer coordinates in a grid. You make a map (your puzzle input) of the open squares (.) and trees (#) you can see. For example:
///
/// ..##.......
/// #...#...#..
/// .#....#..#.
/// ..#.#...#.#
/// .#...##..#.
/// ..#.##.....
/// .#.#.#....#
/// .#........#
/// #.##...#...
/// #...##....#
/// .#..#...#.#
///
/// These aren't the only trees, though; due to something you read about once involving arboreal genetics and biome stability, the same pattern repeats to the right many times:
///
/// ..##.........##.........##.........##.........##.........##.......  --->
/// #...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
/// .#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
/// ..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
/// .#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
/// ..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....  --->
/// .#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
/// .#........#.#........#.#........#.#........#.#........#.#........#
/// #.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
/// #...##....##...##....##...##....##...##....##...##....##...##....#
/// .#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
///
/// You start on the open square (.) in the top-left corner and need to reach the bottom (below the bottom-most row on your map).
///
/// The toboggan can only follow a few specific slopes (you opted for a cheaper model that prefers rational numbers); start by counting all the trees you would encounter for the slope right 3, down 1:
///
/// From your starting position at the top-left, check the position that is right 3 and down 1. Then, check the position that is right 3 and down 1 from there, and so on until you go past the bottom of the map.
///
/// The locations you'd check in the above example are marked here with O where there was an open square and X where there was a tree:
///
/// ..##.........##.........##.........##.........##.........##.......  --->
/// #..O#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
/// .#....X..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
/// ..#.#...#O#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
/// .#...##..#..X...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
/// ..#.##.......#.X#.......#.##.......#.##.......#.##.......#.##.....  --->
/// .#.#.#....#.#.#.#.O..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
/// .#........#.#........X.#........#.#........#.#........#.#........#
/// #.##...#...#.##...#...#.X#...#...#.##...#...#.##...#...#.##...#...
/// #...##....##...##....##...#X....##...##....##...##....##...##....#
/// .#..#...#.#.#..#...#.#.#..#...X.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
///
/// In this example, traversing the map using this slope would cause you to encounter 7 trees.
///
/// Starting at the top-left corner of your map and following a slope of right 3 and down 1, how many trees would you encounter?
///
/// --- Part Two ---
///
/// Time to check the rest of the slopes - you need to minimize the probability of a sudden arboreal stop, after all.
///
/// Determine the number of trees you would encounter if, for each of the following slopes, you start at the top-left corner and traverse the map all the way to the bottom:
///
///     Right 1, down 1.
///     Right 3, down 1. (This is the slope you already checked.)
///     Right 5, down 1.
///     Right 7, down 1.
///     Right 1, down 2.
///
/// In the above example, these slopes would find 2, 7, 3, 4, and 2 tree(s) respectively; multiplied together, these produce the answer 336.
///
/// What do you get if you multiply together the number of trees encountered on each of the listed slopes?
///

pub fn solve_part_one(input: &[String]) -> usize {
    traverse_mountain(input, 3, 1)
}

fn traverse_mountain(input: &[String], step_x: usize, step_y: usize) -> usize {
    let tree_vec: Vec<Vec<bool>> = input
        .iter()
        .map(|s| s.chars().map(|c| c == '#').collect())
        .collect();

    let mut trees = 0;
    let mut x = 0;
    for y in (0..input.len()).step_by(step_y) {
        let row = &tree_vec[y];
        let x_wrapped = x % row.len();
        if row[x_wrapped] {
            trees += 1;
        }
        x += step_x;
    }

    trees
}

pub fn solve_part_two(input: &[String]) -> usize {
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut trees_seen = Vec::new();
    for (step_x, step_y) in slopes {
        let trees = traverse_mountain(input.clone(), step_x, step_y);
        trees_seen.push(trees)
    }

    trees_seen.iter().product()
}

#[test]
fn examples_part_one() {
    assert_eq!(
        7,
        solve_part_one(&[
            "..##.......".to_string(),
            "#...#...#..".to_string(),
            ".#....#..#.".to_string(),
            "..#.#...#.#".to_string(),
            ".#...##..#.".to_string(),
            "..#.##.....".to_string(),
            ".#.#.#....#".to_string(),
            ".#........#".to_string(),
            "#.##...#...".to_string(),
            "#...##....#".to_string(),
            ".#..#...#.#".to_string(),
        ])
    );
}

#[test]
fn examples_part_two() {
    assert_eq!(
        336,
        solve_part_two(&[
            "..##.......".to_string(),
            "#...#...#..".to_string(),
            ".#....#..#.".to_string(),
            "..#.#...#.#".to_string(),
            ".#...##..#.".to_string(),
            "..#.##.....".to_string(),
            ".#.#.#....#".to_string(),
            ".#........#".to_string(),
            "#.##...#...".to_string(),
            "#...##....#".to_string(),
            ".#..#...#.#".to_string(),
        ])
    );
}
