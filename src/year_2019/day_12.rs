use std::{collections::HashSet, convert::Infallible, str::FromStr};

use itertools::Itertools;
use regex::Regex;

use crate::utls::linalg::Point3D;

fn part_1(points: Vec<Point3D>) -> i32 {
    let points: (Vec<i32>, _, _) = points
        .into_iter()
        .map(|point| -> (_, _, _) { point.into() })
        .multiunzip();
    let mut points = [points.0, points.1, points.2];
    let mut velocities: [_; 3] = std::array::from_fn(|_| vec![0; points[0].len()]);
    for _ in 0..1000 {
        for (points, velocities) in points.iter_mut().zip(velocities.iter_mut()) {
            step(points, velocities);
        }
    }

    // 3. compute total energy
    let mut sum = 0;
    for pidx in 0..points[0].len() {
        let velocity = [
            velocities[0][pidx],
            velocities[1][pidx],
            velocities[2][pidx],
        ];
        let points = [points[0][pidx], points[1][pidx], points[2][pidx]];
        sum += velocity.iter().map(|x| x.abs()).sum::<i32>()
            * points.iter().map(|x| x.abs()).sum::<i32>()
    }
    sum
}

fn step(points: &mut [i32], velocities: &mut [i32]) {
    // 1. apply gravity
    for (f, t) in (0..points.len()).tuple_combinations() {
        let (min, max) = match points[f].cmp(&points[t]) {
            std::cmp::Ordering::Less => (f, t),
            std::cmp::Ordering::Equal => continue,
            std::cmp::Ordering::Greater => (t, f),
        };

        velocities[min] += 1;
        velocities[max] -= 1;
    }

    // 2. apply velocity
    for (point, velocity) in points.iter_mut().zip(velocities.iter()) {
        *point += *velocity;
    }
}

fn part_2(points: Vec<Point3D>) -> usize {
    let points: (Vec<i32>, _, _) = points
        .into_iter()
        .map(|point| -> (_, _, _) { point.into() })
        .multiunzip();

    fn loop_len(points: &mut [i32]) -> usize {
        let mut seen_states = HashSet::new();
        let mut velocities = vec![0; points.len()];
        (0..)
            .find(|_| {
                let state = (points.to_vec(), velocities.clone());
                match seen_states.entry(state) {
                    std::collections::hash_set::Entry::Occupied(_) => true,
                    std::collections::hash_set::Entry::Vacant(vacant_entry) => {
                        step(points, &mut velocities);

                        vacant_entry.insert();
                        false
                    }
                }
            })
            .unwrap()
    }

    let loop_lens = [points.0, points.1, points.2].map(|mut points| loop_len(&mut points));

    loop_lens.into_iter().reduce(num::integer::lcm).unwrap()
}

pub fn run() {
    let input: Vec<Point3D> = crate::utls::read_text_from_file("2019", "12");
    let res = part_1(input.clone());
    println!("The answer is {}", res);
    let res = part_2(input);
    println!("The answer is {}", res);
}

impl FromStr for Point3D {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r"<x=([-\d]+), y=([-\d]+), z=([-\d]+)>").unwrap();

        let (x, y, z) = regex
            .captures(s)
            .unwrap()
            .iter()
            .skip(1)
            .map(|m| m.unwrap().as_str().parse().unwrap())
            .collect_tuple()
            .unwrap();

        Ok(Self(x, y, z))
    }
}
