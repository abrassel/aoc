pub mod ascii_grid;
pub mod infallible;

use itertools::Itertools;

use super::Val;
use std::{io::Write, sync::mpsc};

pub struct AsciiStdin;

impl TryReadVal for AsciiStdin {
    fn try_read_val(&mut self) -> Option<Val> {
        {
            let mut buf = String::new();
            print!("Input: ");
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut buf).unwrap();
            let chars = buf.trim().chars().collect_vec();
            assert!(chars.len() == 1);
            let u8ified = chars[0] as u8;
            Some(u8ified.into())
        }
    }
}

pub trait TryWriteVal {
    fn try_write_val(&mut self, val: Val) -> Option<()>;
}

pub(crate) trait TryReadVal {
    fn try_read_val(&mut self) -> Option<Val>;
}

impl TryWriteVal for std::io::Stdout {
    fn try_write_val(&mut self, val: Val) -> std::option::Option<()> {
        writeln!(self, "{}", val).ok()
    }
}

impl TryReadVal for std::io::Stdin {
    fn try_read_val(&mut self) -> Option<Val> {
        let mut buf = String::new();
        print!("Input: ");
        std::io::stdout().flush().unwrap();
        self.read_line(&mut buf).unwrap();
        Some(buf.trim().parse().unwrap())
    }
}

impl TryReadVal for Val {
    fn try_read_val(&mut self) -> Option<Val> {
        Some(*self)
    }
}

impl TryWriteVal for Val {
    fn try_write_val(&mut self, val: Val) -> Option<()> {
        *self = val;
        Some(())
    }
}

pub struct View {
    pub(crate) view: Vec<Val>,
    pub(crate) offset: usize,
}

impl View {
    pub fn new<T: TryInto<Val>>(view: Vec<T>) -> Self
    where
        <T as std::convert::TryInto<i128>>::Error: std::fmt::Debug,
    {
        Self {
            view: view.into_iter().map(|x| x.try_into().unwrap()).collect(),
            offset: 0,
        }
    }

    pub fn new_char(view: Vec<char>) -> Self {
        let u8s = view.into_iter().map(|x| x as u8).collect_vec();
        Self::new(u8s)
    }
}

impl TryReadVal for View {
    fn try_read_val(&mut self) -> Option<Val> {
        let val = self.view[self.offset];
        self.offset += 1;
        Some(val)
    }
}

impl TryReadVal for mpsc::Receiver<Val> {
    fn try_read_val(&mut self) -> Option<Val> {
        self.recv().ok()
    }
}

impl TryWriteVal for mpsc::Sender<Val> {
    fn try_write_val(&mut self, val: Val) -> Option<()> {
        self.send(val).unwrap();
        Some(())
    }
}

impl TryWriteVal for Vec<Val> {
    fn try_write_val(&mut self, val: Val) -> Option<()> {
        self.push(val);
        Some(())
    }
}
