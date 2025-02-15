use std::collections::{HashMap, HashSet};

use fraction::GenericFraction;
use itertools::Itertools;

use crate::maze::{Cell, Maze};

fn part_1(maze: Maze) -> usize {
    let points = get_points(maze);
    let lines = lines(&points);
    // print_points_on_lines(&lines);
    let counts = counts(lines);
    *counts.values().max().unwrap()
}

fn part_2(maze: Maze) {}

fn get_points(maze: Maze) -> Vec<(i32, i32)> {
    maze.grid
        .into_iter()
        .enumerate()
        .flat_map(|(ridx, row)| {
            row.into_iter()
                .enumerate()
                .filter(|(_, cell)| cell == &Cell::Wall)
                .map(move |(cidx, _)| (ridx.try_into().unwrap(), cidx.try_into().unwrap()))
        })
        .collect()
}

#[derive(Eq, PartialEq, Hash, Debug)]
enum Slope {
    Horizontal,
    Normal(GenericFraction<i32>),
    Vertical,
}

type PointsOnLines = HashMap<Slope, HashMap<GenericFraction<i32>, HashSet<(i32, i32)>>>;

// fn print_points_on_lines(points_on_lines: &PointsOnLines) {
//     for (slope, offsets) in points_on_lines {
//         for (offset, points) in offsets {
//             println!(
//                 "Line y = {:?}x + {} contains these points: {:?}",
//                 slope, offset, points
//             );
//         }
//     }
// }

fn lines(points: &[(i32, i32)]) -> PointsOnLines {
    let mut lines: PointsOnLines = HashMap::new();
    for combination in points.iter().combinations(2) {
        let f = combination[0];
        let t = combination[1];
        let slope = {
            if t.1 - f.1 == 0 {
                Slope::Horizontal
            } else if t.0 - f.0 == 0 {
                Slope::Vertical
            } else {
                let slope = ((t.1 - f.1), (t.0 - f.0));
                let div = num::integer::gcd(slope.0, slope.1);
                let slope = GenericFraction::new(slope.0 / div, slope.1 / div);
                Slope::Normal(slope)
            }
        };
        let b = match slope {
            Slope::Horizontal => t.1.into(),
            Slope::Normal(slope) => GenericFraction::from(t.1) - slope * t.0,
            Slope::Vertical => t.0.into(),
        };
        let line_bucket = lines.entry(slope).or_default().entry(b).or_default();
        line_bucket.insert(*f);
        line_bucket.insert(*t);
    }
    lines
}

fn counts(points_on_lines: PointsOnLines) -> HashMap<(i32, i32), usize> {
    let mut counts = HashMap::new();
    for points in points_on_lines.values().flat_map(|line| line.values()) {
        for &point in points {
            // each neighbor is visible, or else there would be another point in the way
            *counts.entry(point).or_default() += 2;
        }
        let min_point = points.iter().min().unwrap();

        let max_point = points.iter().max().unwrap();
        // beginning and end points don't get prev / next
        *counts.get_mut(min_point).unwrap() -= 1;
        *counts.get_mut(max_point).unwrap() -= 1;
    }
    counts
}

pub fn run() {
    let maze: Maze = crate::utls::read_text_from_file("2019", "10");

    let res = part_1(maze.clone());
    println!("The answer is {}", res);
    part_2(maze);
}

#[cfg(test)]
mod test {
    use crate::utls::MyParse;

    use super::*;

    #[test]
    fn test_solution() {
        let input = r"
        .#..#
.....
#####
....#
...##
";
        let maze = Maze::my_parse(input);
        let ans = part_1(maze);
        assert_eq!(ans, 8);
    }

    #[test]
    fn test_solution_1() {
        let input = r"
......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";
        let maze = Maze::my_parse(input);
        let ans = part_1(maze);
        assert_eq!(ans, 33);
    }

    #[test]
    fn test_solution_2() {
        let input = r"
#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";
        let maze = Maze::my_parse(input);
        let ans = part_1(maze);
        assert_eq!(ans, 35);
    }

    #[test]
    fn test_solution_3() {
        let input = r"
.#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";
        let maze = Maze::my_parse(input);
        let ans = part_1(maze);
        assert_eq!(ans, 41);
    }

    #[test]
    fn test_solution_4() {
        let input = r"
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
        let maze = Maze::my_parse(input);
        let ans = part_1(maze);
        assert_eq!(ans, 210);
    }
}
