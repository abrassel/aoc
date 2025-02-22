use std::{
    sync::mpsc::{self},
    thread::JoinHandle,
};

use super::{
    Program, Val,
    io::{TryReadVal, TryWriteVal},
};

pub struct ProgramHandle {
    program_handle: JoinHandle<mpsc::Receiver<Val>>,
    reader: mpsc::Receiver<Val>,
    writer: mpsc::Sender<Val>,
}

impl ProgramHandle {
    pub fn join(self) -> impl TryReadVal {
        self.program_handle.join().unwrap()
    }

    pub fn try_read_val(&mut self) -> Option<Val> {
        self.reader.recv().ok()
    }
}

impl TryWriteVal for ProgramHandle {
    fn try_write_val(&mut self, val: Val) -> Option<()> {
        self.writer.send(val).ok()
    }
}

pub fn spawn(mut program: Program) -> ProgramHandle {
    let (write_to_program, mut input) = mpsc::channel();
    let (mut output, read_from_program) = mpsc::channel();
    let handle = std::thread::spawn(move || {
        program.eval(&mut input, &mut output);
        input
    });
    ProgramHandle {
        program_handle: handle,
        reader: read_from_program,
        writer: write_to_program,
    }
}
