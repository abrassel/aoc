mod opcode;

use crate::utls::MyParse;
use opcode::Opcode;

pub type Val = i32;

#[derive(Default, Clone)]

pub struct Program {
    pub code: Vec<Val>,
}

impl MyParse for Program {
    fn my_parse(s: &str) -> Self {
        let code = s.trim().split(",").map(|x| x.parse().unwrap()).collect();
        Self { code }
    }
}

impl Program {
    pub fn step(&mut self, loc: usize) -> Option<usize> {
        let opcode_raw = self.code[loc];
        let opcode = Opcode::new(opcode_raw, loc);
        opcode.eval(self)
    }

    pub fn init(&mut self, noun: Val, verb: Val) {
        self.code[1] = noun;
        self.code[2] = verb;
    }

    pub fn eval(&mut self) -> Val {
        let mut step = 0;
        while let Some(next_step) = self.step(step) {
            step = next_step;
        }
        self.code[0]
    }
}
