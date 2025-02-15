use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    io::Write,
};

use fraction::GenericFraction;
use itertools::Itertools;

use crate::maze::{Cell, Maze};

fn part_1(maze: &Maze) -> usize {
    let points = get_points(maze);
    let lines = lines(&points);
    print_points_on_lines(&lines);
    let counts = counts(&lines);
    *counts.values().max().unwrap()
}

fn part_2(maze: &Maze, nth: usize) -> Point {
    let lines = lines(&get_points(maze));
    // only take lines which contain the best point
    let counts = counts(&lines);
    let best_point = counts.into_iter().max_by_key(|x| x.1).unwrap().0;
    let relevant_lines = lines
        .into_iter()
        .filter(|(_, points)| points.contains(&best_point));
    // sort lines by maximum slope to minimum -> this simulates clockwise rotation (maximum incidence at y = (big_m) * x + b)
    let inorder_lines = relevant_lines
        .into_iter()
        .sorted_unstable_by_key(|((slope, _), _)| *slope)
        .rev();

    let copy: Vec<(Line, _)> = inorder_lines
        .clone()
        .map(|(line, points)| (line.into(), points))
        .collect_vec();
    // partition each line by points left and right (or above and below for vertical) of laser location
    let inorder_lines_with_y_axis_sep = inorder_lines
        .map(|(_, points)| points)
        .map(|points| -> (Vec<_>, Vec<_>) {
            points.into_iter().partition(|point| point < &best_point)
        })
        .map(|(mut left, mut right)| {
            // left side is origin -> end of line, so want to go in descending order
            left.sort_unstable_by(|lft, rht| rht.cmp(lft));
            right.sort_unstable();
            right.remove(0);
            (left, right)
        });
    let (lhs, rhs): (Vec<_>, Vec<_>) = inorder_lines_with_y_axis_sep.unzip();
    let mut lhs: Vec<_> = lhs.into_iter().map(|line| line.into_iter()).collect();
    let mut rhs: Vec<_> = rhs.into_iter().map(|line| line.into_iter()).collect();
    let mut count = 0;
    let mut destroyed = vec![];

    maze.print_with_destroyed(best_point.into(), &destroyed);
    for _ in 0..500 {
        for line in rhs.iter_mut().chain(lhs.iter_mut()) {
            if let Some(nxt) = line.next() {
                count += 1;
                destroyed.push(nxt.into());
                maze.print_with_destroyed(best_point.into(), &destroyed);
                if count == nth {
                    return nxt;
                }
            }
        }
    }
    panic!("Was not 200 points");
}

fn get_points(maze: &Maze) -> Vec<Point> {
    maze.grid
        .iter()
        .enumerate()
        .flat_map(|(ridx, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, cell)| **cell == Cell::Wall)
                .map(move |(cidx, _)| (cidx.try_into().unwrap(), ridx.try_into().unwrap()).into())
        })
        .collect()
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd, derive_more::Display)]
enum Slope {
    #[display("{}/{}", _0.numer().unwrap(), _0.denom().unwrap())]
    Normal(GenericFraction<i32>),
    Vertical,
}

type PointsOnLines = HashMap<(Slope, GenericFraction<i32>), HashSet<Point>>;
#[derive(Ord, PartialOrd, PartialEq, Eq, Hash, Clone, Copy, derive_more::Display)]
#[display("({}, {})", _0, -_1)]
struct Point(i32, i32);
impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Point as Display>::fmt(self, f)
    }
}

impl From<Point> for (i32, i32) {
    fn from(val: Point) -> Self {
        (val.0, -val.1)
    }
}

impl From<(i32, i32)> for Point {
    fn from(value: (i32, i32)) -> Self {
        Self(value.0, -value.1)
    }
}

#[derive(derive_more::From)]
struct Line(Slope, GenericFraction<i32>);

impl std::fmt::Debug for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "y={}x+{}", self.0, self.1)
    }
}

fn print_points_on_lines(points_on_lines: &PointsOnLines) {
    for (slope, points) in points_on_lines {
        (
            "Line {:?} contains these points: {:?}",
            Line::from(*slope),
            points,
        );
    }
}

fn lines(points: &[Point]) -> PointsOnLines {
    let mut lines: PointsOnLines = HashMap::new();
    for combination in points.iter().combinations(2) {
        let f = combination[0];
        let t = combination[1];
        let slope = {
            if t.1 - f.1 == 0 {
                Slope::Normal(GenericFraction::from(0))
            } else if t.0 - f.0 == 0 {
                Slope::Vertical
            } else {
                // the gridspace is y-flipped
                let slope = ((t.1 - f.1), (t.0 - f.0));
                let div = num::integer::gcd(slope.0, slope.1);
                let slope = GenericFraction::new(slope.0 / div, slope.1 / div);
                Slope::Normal(slope)
            }
        };
        let b = match slope {
            Slope::Normal(slope) => GenericFraction::from(t.1) - slope * t.0,
            Slope::Vertical => t.0.into(),
        };
        let line_bucket = lines.entry((slope, b)).or_default();
        line_bucket.insert(*f);
        line_bucket.insert(*t);
    }
    lines
}

fn counts(points_on_lines: &PointsOnLines) -> HashMap<Point, usize> {
    let mut counts = HashMap::new();
    for points in points_on_lines.values() {
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

    let res = part_2(&maze.clone(), 200);
    let points: (i32, i32) = res.into();
    println!("The answer is {}", points.0 * 100 + points.1);
}

#[cfg(test)]
mod test {
    use crate::utls::MyParse;

    use super::*;

    #[test]
    fn test_solution_base() {
        let input = r"
        .#..#
.....
#####
....#
...##
";
        let maze = Maze::my_parse(input);
        let ans = part_1(&maze);
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
        let ans = part_1(&maze);
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
        let ans = part_1(&maze);
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
        let ans = part_1(&maze);
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
        let ans = part_1(&maze);
        assert_eq!(ans, 210);
    }

    #[test]
    fn test_solution_5() {
        let input = r".#..##.###...#######
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
        assert_eq!(part_2(&maze, 1), (11, 12).into());
        assert_eq!(part_2(&maze, 2), (12, 1).into());
        assert_eq!(part_2(&maze, 3), (12, 2).into());
        assert_eq!(part_2(&maze, 10), (12, 8).into());
        assert_eq!(part_2(&maze, 20), (16, 0).into());
        assert_eq!(part_2(&maze, 50), (16, 9).into());
        assert_eq!(part_2(&maze, 100), (10, 16).into());
        assert_eq!(part_2(&maze, 199), (9, 6).into());
        assert_eq!(part_2(&maze, 200), (8, 2).into());
        assert_eq!(part_2(&maze, 201), (10, 9).into());
        assert_eq!(part_2(&maze, 299), (11, 1).into());
    }

    #[test]
    fn test_solution_6() {
        let input = r".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##";
        let maze = Maze::my_parse(input);
        part_2(&maze, 20);
    }
}
