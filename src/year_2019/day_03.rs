use itertools::Itertools;

use crate::utls::MyParse;

type Segment = (((i32, i32), i32), ((i32, i32), i32));

// fn part_1(line1: Vec<String>, line2: Vec<String>) -> usize {
//     let intersection = closest_intersection(intersections(line1, line2));
//     manhattan_distance(intersection)
// }

fn manhattan_distance(from: (i32, i32), to: (i32, i32)) -> i32 {
    (to.0 - from.0).abs() + (to.1 - from.1).abs()
}

fn closest_intersection(points: impl Iterator<Item = ((i32, i32), i32)>) -> ((i32, i32), i32) {
    points
        .filter(|(point, _)| *point != (0, 0))
        .min_by_key(|(_, dist)| *dist)
        .unwrap()
}

fn intersections(
    line1: Vec<String>,
    line2: Vec<String>,
) -> impl Iterator<Item = ((i32, i32), i32)> {
    let line1 = walk(line1);
    let line2 = walk(line2);
    itertools::iproduct!(line1, line2).filter_map(|(lhs, rhs)| intersection(lhs, rhs))
}

fn intersection(left: Segment, right: Segment) -> Option<((i32, i32), i32)> {
    let ll = left.0.0;
    let lr = left.1.0;
    let rl = right.0.0;
    let rr = right.1.0;
    // left is either horizontal or vertical
    let is_vertical_left = ll.0 == lr.0;
    let is_vertical_right = rl.0 == rr.0;
    let base_steps = left.0.1 + right.0.1;

    if is_vertical_left == is_vertical_right {
        return None;
    }

    let intersect = if is_vertical_left {
        // see if intersects with the horizontal plane
        let ypoint = rl.1;
        let xpoint = ll.0;
        if abs_contains((ll.1, lr.1), ypoint) && abs_contains((rl.0, rr.0), xpoint) {
            Some((xpoint, ypoint))
        } else {
            None
        }
    } else {
        let xpoint = rl.0;
        let ypoint = lr.1;
        if abs_contains((ll.0, lr.0), xpoint) && abs_contains((rl.1, rr.1), ypoint) {
            Some((xpoint, ypoint))
        } else {
            None
        }
    }?;
    let dst_from_start =
        manhattan_distance(left.0.0, intersect) + manhattan_distance(right.0.0, intersect);
    Some((intersect, base_steps + dst_from_start))
}

fn abs_contains(range: (i32, i32), val: i32) -> bool {
    if range.1 > range.0 {
        range.0 <= val && val <= range.1
    } else {
        range.1 <= val && val <= range.0
    }
}

fn walk(line: Vec<String>) -> impl Iterator<Item = Segment> + Clone {
    let mut cur = (0, 0);
    let mut steps = 0;
    let points = line.into_iter().map(move |segment| {
        let (fx, fy) = cur;
        let (dir, num) = segment.split_at(1);
        let (dx, dy) = match dir {
            "U" => (0, 1),
            "L" => (-1, 0),
            "R" => (1, 0),
            "D" => (0, -1),
            _ => unreachable!(),
        };
        let dst: i32 = num.parse().unwrap();
        let next = (fx + dx * dst, fy + dy * dst);
        cur = next;
        steps += dst;
        (cur, steps)
    });
    std::iter::once(((0, 0), 0)).chain(points).tuple_windows()
}

#[allow(unused)]
struct Lines(Vec<Vec<String>>);

impl MyParse for Lines {
    fn my_parse(s: &str) -> Self {
        let inner = s
            .trim()
            .lines()
            .map(|line| line.trim().split(",").map(|x| x.to_string()).collect())
            .collect();
        Self(inner)
    }
}

fn part_2(line1: Vec<String>, line2: Vec<String>) -> i32 {
    println!(
        "The intersections: {:?}",
        intersections(line1.clone(), line2.clone()).collect_vec()
    );
    let intersection = closest_intersection(intersections(line1, line2));
    intersection.1
}

pub fn run() {
    let Lines(input) = crate::utls::read_text_from_file("2019", "03");
    let res = part_2(input[0].clone(), input[1].clone());
    println!("The answer is {}", res);
    // part_2(input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"       
R8,U5,L5,D3
U7,R6,D4,L4";

    #[test]
    fn test_solution() {
        let Lines(input) = Lines::my_parse(INPUT);
        let res = part_2(input[0].clone(), input[1].clone());
        assert_eq!(res, 30);
    }

    #[test]
    fn test_solution2() {
        let input = r"
        R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";
        let Lines(input) = Lines::my_parse(input);
        let res = part_2(input[0].clone(), input[1].clone());
        assert_eq!(res, 610);
    }

    #[test]
    fn test_solution3() {
        let input = r"
R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        let Lines(input) = Lines::my_parse(input);
        let res = part_2(input[0].clone(), input[1].clone());
        assert_eq!(res, 410);
    }

    //     #[test]
    //     fn test_intersect() {
    //         let lhs = ((146, 53), (146, 4));
    //         let rhs = ((100, 46), (155, 46));
    //         let res = intersection(lhs, rhs);
    //         assert_eq!(res, Some((146, 46)));
    //     }
}
