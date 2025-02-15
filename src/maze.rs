use std::convert::Infallible;

use crate::utls::MyParse;

#[derive(PartialEq, Clone)]
pub enum Cell {
    Wall,
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
