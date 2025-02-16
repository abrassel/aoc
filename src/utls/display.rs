use itertools::Itertools;
use std::{collections::HashMap, fmt::Display};

use super::linalg::Point;

pub fn paint<T: Display + Copy + Default>(painting: &HashMap<Point, T>) {
    let (minx, maxx) = painting
        .keys()
        .map(|key| key.0)
        .minmax()
        .into_option()
        .unwrap();
    let (miny, maxy) = painting
        .keys()
        .map(|key| key.1)
        .minmax()
        .into_option()
        .unwrap();
    // using Euclidean plane, so big y -> last row in matrix grid
    // also, printing row by row, so row-major order
    for row in (miny..=maxy).rev() {
        for col in minx..=maxx {
            let color = painting
                .get(&(col, row).into())
                .copied()
                // if unknown color, select black
                .unwrap_or_default();

            print!("{}", color);
        }
        println!();
    }
}
