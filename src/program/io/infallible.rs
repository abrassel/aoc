use super::{TryReadVal, TryWriteVal};

pub struct Unused;

impl TryReadVal for Unused {
    fn try_read_val(&mut self) -> Option<crate::program::Val> {
        unreachable!("Shouldn't try to read a value!");
    }
}

impl TryWriteVal for Unused {
    fn try_write_val(&mut self, _val: crate::program::Val) -> Option<()> {
        unreachable!("Shouldn't try to write a value!");
    }
}
