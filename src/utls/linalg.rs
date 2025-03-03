use matrix::Matrix;
use std::ops::{Index, IndexMut};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use num_enum::TryFromPrimitive;

use crate::program::Val;

pub mod matrix;

#[derive(
    Hash,
    Default,
    Copy,
    Clone,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    derive_more::Add,
    derive_more::From,
    derive_more::AddAssign,
    derive_more::Display,
)]
#[display("({}, {})", _0, _1)]
pub struct Point(pub Val, pub Val);

impl Point {
    pub const UP: Point = Point(0, 1);
    pub fn rotate(&self, rotate_dir: RotateDir) -> Self {
        let transform = rotate_dir.transform();
        &transform * *self
    }

    pub fn neighbors(&self) -> impl Iterator<Item = Self> {
        CardinalDir::iter().map(|dir| *self + dir.into())
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Display>::fmt(self, f)
    }
}

#[derive(Copy, Clone, TryFromPrimitive, EnumIter)]
#[repr(u8)]
pub enum RotateDir {
    CounterClockwise = 0,
    Clockwise = 1,
}

impl RotateDir {
    pub fn transform(&self) -> Matrix<Val> {
        match self {
            RotateDir::Clockwise => [[0, 1], [-1, 0]].into(),
            RotateDir::CounterClockwise => [[0, -1], [1, 0]].into(),
        }
    }
}

#[derive(
    Hash,
    Default,
    Copy,
    Clone,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    derive_more::Add,
    derive_more::From,
    derive_more::AddAssign,
    derive_more::Into,
)]
pub struct Point3D(pub i32, pub i32, pub i32);

impl Index<usize> for Point3D {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Unknown dim"),
        }
    }
}

impl IndexMut<usize> for Point3D {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!("Unknown dim"),
        }
    }
}

impl Point3D {
    pub fn iter(&self) -> impl Iterator<Item = i32> {
        [self.0, self.1, self.2].into_iter()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, TryFromPrimitive, EnumIter)]
#[repr(u8)]
pub enum CardinalDir {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

impl std::ops::Neg for CardinalDir {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            CardinalDir::North => CardinalDir::South,
            CardinalDir::South => CardinalDir::North,
            CardinalDir::West => CardinalDir::East,
            CardinalDir::East => CardinalDir::West,
        }
    }
}

impl From<CardinalDir> for Point {
    fn from(value: CardinalDir) -> Self {
        match value {
            CardinalDir::North => (0, 1),
            CardinalDir::South => (0, -1),
            CardinalDir::West => (-1, 0),
            CardinalDir::East => (1, 0),
        }
        .into()
    }
}

impl TryFrom<char> for CardinalDir {
    type Error = anyhow::Error;

    fn try_from(value: char) -> anyhow::Result<Self> {
        let res = match value {
            '^' => Self::North,
            '<' => Self::West,
            '>' => Self::East,

            'v' => Self::South,
            _ => anyhow::bail!("Tried to unpack: {}", value),
        };
        Ok(res)
    }
}
