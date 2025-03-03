use std::collections::{HashMap, HashSet};

use crate::{
    program::Val,
    utls::{MyParse, linalg::Point},
};

#[derive(PartialEq, Clone, derive_more::Display)]
pub enum Cell {
    #[display("#")]
    Wall,
    #[display(".")]
    Open,
}

impl TryFrom<char> for Cell {
    type Error = char;

    fn try_from(value: char) -> Result<Self, char> {
        Ok(match value {
            '#' => Self::Wall,
            '.' => Self::Open,
            value => return Err(value),
        })
    }
}

#[derive(Clone)]
pub struct Maze {
    pub points: HashSet<Point>,
    pub things: HashMap<Point, char>,
    pub underlying_grid: Vec<Vec<Cell>>,
}

impl Maze {
    pub fn print_with_destroyed(&self, start: (i32, i32), destroyed: &[(i32, i32)]) {
        let destroyed: Vec<(usize, usize)> = destroyed
            .iter()
            .map(|coord| (coord.0 as usize, coord.1 as usize))
            .collect();

        for (ridx, row) in self.underlying_grid.iter().enumerate() {
            for (cidx, cell) in row.iter().enumerate() {
                if let Some(idx) = destroyed.iter().position(|l| &(cidx, ridx) == l) {
                    print!("{}", idx % 10);
                } else if (cidx, ridx) == (start.0 as usize, start.1 as usize) {
                    print!("O");
                } else {
                    print!("{}", cell);
                }
            }
            println!();
        }
        println!("finished printing");
    }

    pub fn neighbors(&self, point: &Point) -> impl Iterator<Item = Point> {
        point
            .neighbors()
            .filter(|point| self.points.contains(point))
    }
}

impl MyParse for Maze {
    fn my_parse(s: &str) -> Self {
        let mut points = HashSet::new();
        let mut things = HashMap::new();
        let mut underlying_grid = vec![];
        let lines = s.trim().lines();
        let tot = lines.clone().count();
        for (ridx, row) in lines.enumerate() {
            let mut cell_row = vec![];
            for (cidx, cell) in row.trim().chars().enumerate() {
                let point = (cidx as Val, (tot - ridx - 1) as Val);
                let cell = match Cell::try_from(cell) {
                    Ok(cell) => cell,
                    Err(other) => {
                        things.insert(point.into(), other);
                        Cell::Open
                    }
                };
                if matches!(cell, Cell::Open) {
                    points.insert(point.into());
                }
                cell_row.push(cell);
            }
            underlying_grid.push(cell_row);
        }
        Self {
            points,
            things,
            underlying_grid,
        }
    }
}
