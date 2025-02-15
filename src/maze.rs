use std::{collections::HashSet, convert::Infallible, ops::Index};

use crate::utls::MyParse;

#[derive(PartialEq, Clone, derive_more::Display)]
pub enum Cell {
    #[display("#")]
    Wall,
    #[display(".")]
    Open,
}

impl TryFrom<char> for Cell {
    type Error = Infallible;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '#' => Self::Wall,
            '.' => Self::Open,
            _ => panic!("found {}", value),
        })
    }
}

#[derive(Clone)]
pub struct Maze {
    pub grid: Vec<Vec<Cell>>,
}

impl Maze {
    pub fn print_with_destroyed(&self, start: (i32, i32), destroyed: &[(i32, i32)]) {
        let destroyed: Vec<(usize, usize)> = destroyed
            .iter()
            .map(|coord| (coord.0 as usize, coord.1 as usize))
            .collect();

        for (ridx, row) in self.grid.iter().enumerate() {
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
}

impl MyParse for Maze {
    fn my_parse(s: &str) -> Self {
        let grid = s
            .trim()
            .lines()
            .map(|line| line.trim().chars().map(|c| c.try_into().unwrap()).collect())
            .collect();
        Self { grid }
    }
}
