use crate::program::Val;

impl<C: From<char>> crate::program::io::TryWriteVal for Vec<Vec<C>> {
    fn try_write_val(&mut self, val: Val) -> Option<()> {
        if self.is_empty() {
            self.push(vec![]);
        }
        let c: char = (u8::try_from(val).unwrap()).into();
        match c {
            '\n' => {
                self.push(vec![]);
            }
            _ => self.last_mut().unwrap().push(c.into()),
        }

        Some(())
    }
}
