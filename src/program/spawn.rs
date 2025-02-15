use std::{
    sync::mpsc::{self},
    thread::JoinHandle,
};

use super::{
    Program, Val,
    io::{ReadVal, WriteVal},
};

pub struct ProgramHandle {
    program_handle: JoinHandle<mpsc::Receiver<Val>>,
    reader: mpsc::Receiver<Val>,
    writer: mpsc::Sender<Val>,
}

impl ProgramHandle {
    pub fn join(self) -> impl ReadVal {
        self.program_handle.join().unwrap()
    }

    pub fn try_read_val(&mut self) -> Option<Val> {
        self.reader.recv().ok()
    }
}

impl WriteVal for ProgramHandle {
    fn write_val(&mut self, val: Val) {
        self.writer.send(val).unwrap()
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
