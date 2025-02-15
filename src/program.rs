pub mod io;
mod opcode;
pub mod spawn;

use crate::program::io::ReadVal;
use crate::program::io::WriteVal;

use crate::utls::MyParse;
use opcode::Opcode;

pub type Val = i128;
const CODE_PAD: usize = 10;

#[derive(Default, Clone)]

pub struct Program {
    pub code: Vec<Val>,
}

#[derive(Default)]
pub struct ProgramState {
    offset: usize,
    relative_base: usize,
}

impl MyParse for Program {
    fn my_parse(s: &str) -> Self {
        Program::new(Self::parse_code(s))
    }
}

impl Program {
    pub fn step<Io: ReadVal + WriteVal>(
        &mut self,
        program_state: ProgramState,
        io: &mut Io,
    ) -> Option<ProgramState> {
        let opcode_raw = self.code[program_state.offset];
        let opcode = Opcode::new(opcode_raw);
        opcode.eval(self, program_state, io)
    }

    pub fn init(&mut self, noun: Val, verb: Val) {
        self.code[1] = noun;
        self.code[2] = verb;
    }

    pub fn eval_joint<Io: ReadVal + WriteVal>(&mut self, io: &mut Io) -> Val {
        let mut program_state = ProgramState::default();
        while let Some(next_program_state) = self.step(program_state, io) {
            program_state = next_program_state;
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

    fn new(mut code: Vec<Val>) -> Self {
        code.resize(code.len() * CODE_PAD, 0);
        Self { code }
    }

    pub(crate) fn parse_code(s: &str) -> Vec<Val> {
        s.trim().split(",").map(|x| x.parse().unwrap()).collect()
    }
}
