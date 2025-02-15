use num_enum::TryFromPrimitive;

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
pub struct Point(pub i32, pub i32);

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
    pub fn transform(&self) -> [[i32; 2]; 2] {
        match self {
            RotateDir::Clockwise => [[0, 1], [-1, 0]],
            RotateDir::CounterClockwise => [[0, -1], [1, 0]],
        }
    }
}
