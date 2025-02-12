pub mod io;
mod opcode;

use crate::program::io::ReadVal;
use crate::program::io::WriteVal;

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
    pub fn step<Io: ReadVal + WriteVal>(&mut self, loc: usize, io: &mut Io) -> Option<usize> {
        let opcode_raw = self.code[loc];
        let opcode = Opcode::new(opcode_raw, loc);
        opcode.eval(self, io)
    }

    pub fn init(&mut self, noun: Val, verb: Val) {
        self.code[1] = noun;
        self.code[2] = verb;
    }

    pub fn eval_joint<Io: ReadVal + WriteVal>(&mut self, io: &mut Io) -> Val {
        let mut step = 0;
        while let Some(next_step) = self.step(step, io) {
            step = next_step;
        }
        self.code[0]
    }

    pub fn eval<W: WriteVal, R: ReadVal>(&mut self, input: &mut R, output: &mut W) -> Val {
        struct JointValMut<'a, W, R> {
            input: &'a mut R,
            output: &'a mut W,
        }

        impl<W, R: ReadVal> ReadVal for JointValMut<'_, W, R> {
            fn read_val(&mut self) -> Val {
                self.input.read_val()
            }
        }

        impl<W: WriteVal, R> WriteVal for JointValMut<'_, W, R> {
            fn write_val(&mut self, val: Val) {
                self.output.write_val(val)
            }
        }

        let mut joint = JointValMut { input, output };

        self.eval_joint(&mut joint)
    }
}
