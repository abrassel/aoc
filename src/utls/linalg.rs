use std::ops::{Index, IndexMut};

use num_enum::TryFromPrimitive;

use crate::program::Val;

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
)]
pub struct Point(pub Val, pub Val);

impl Point {
    pub const UP: Point = Point(0, 1);
    pub fn rotate(&self, rotate_dir: RotateDir) -> Self {
        let transform = rotate_dir.transform();
        Self(
            transform[0][0] * self.0 + transform[0][1] * self.1,
            transform[1][0] * self.0 + transform[1][1] * self.1,
        )
    }
}

#[derive(Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum RotateDir {
    CounterClockwise = 0,
    Clockwise = 1,
}

impl RotateDir {
    pub fn transform(&self) -> [[Val; 2]; 2] {
        match self {
            RotateDir::Clockwise => [[0, 1], [-1, 0]],
            RotateDir::CounterClockwise => [[0, -1], [1, 0]],
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
