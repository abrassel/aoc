use super::Val;
use std::{io::Write, sync::mpsc};

pub trait WriteVal {
    fn write_val(&mut self, val: Val);
}

pub trait ReadVal {
    fn read_val(&mut self) -> Val;
}

impl WriteVal for std::io::Stdout {
    fn write_val(&mut self, val: Val) {
        writeln!(self, "{}", val).unwrap();
    }
}

impl ReadVal for std::io::Stdin {
    fn read_val(&mut self) -> Val {
        let mut buf = String::new();
        print!("Input: ");
        std::io::stdout().flush().unwrap();
        self.read_line(&mut buf).unwrap();
        buf.trim().parse().unwrap()
    }
}

impl ReadVal for Val {
    fn read_val(&mut self) -> Val {
        *self
    }
}

impl WriteVal for Val {
    fn write_val(&mut self, val: Val) {
        *self = val;
    }
}

pub struct View {
    view: Vec<Val>,
    offset: usize,
}

impl View {
    pub fn new(view: Vec<Val>) -> Self {
        Self { view, offset: 0 }
    }
}

impl ReadVal for View {
    fn read_val(&mut self) -> Val {
        let val = self.view[self.offset];
        self.offset += 1;
        val
    }
}

impl ReadVal for mpsc::Receiver<Val> {
    fn read_val(&mut self) -> Val {
        self.recv().unwrap()
    }
}

impl WriteVal for mpsc::Sender<Val> {
    fn write_val(&mut self, val: Val) {
        self.send(val).unwrap();
    }
}

impl WriteVal for Vec<Val> {
    fn write_val(&mut self, val: Val) {
        self.push(val);
    }
}
