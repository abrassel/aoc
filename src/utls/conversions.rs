use std::collections::HashMap;

use itertools::Itertools;

use crate::program::Val;

use super::linalg::Point;

pub fn point_map<T>(vec: Vec<Vec<T>>) -> HashMap<Point, T> {
    let vec = vec.into_iter().filter(|row| !row.is_empty()).collect_vec();
    let row_count = vec.len();
    vec.into_iter()
        .enumerate()
        .flat_map(|(ridx, row)| {
            row.into_iter()
                .enumerate()
                .map(move |(cidx, val)| ((cidx as Val, (row_count - ridx - 1) as Val).into(), val))
        })
        .collect()
}
