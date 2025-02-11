use std::io::{self, Write};

use super::Program;
use crate::program::Val;
use num_enum::TryFromPrimitive;

#[derive(TryFromPrimitive)]
#[repr(i32)]
enum OpcodeVariant {
    Add = 1,
    Mult = 2,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    Halt = 99,
}

impl OpcodeVariant {
    pub fn instruction_count(&self) -> usize {
        match self {
            OpcodeVariant::Add => 3,
            OpcodeVariant::Mult => 3,
            OpcodeVariant::Halt => 0,
            OpcodeVariant::Input => 1,
            OpcodeVariant::Output => 1,
            OpcodeVariant::JumpIfTrue => 2,
            OpcodeVariant::JumpIfFalse => 2,
            OpcodeVariant::LessThan => 3,
            OpcodeVariant::Equals => 3,
        }
    }
}

#[derive(TryFromPrimitive, Default, Clone, Copy)]
#[repr(u8)]
pub enum InstructionMode {
    #[default]
    Parameter = 0,
    Immediate = 1,
}

pub struct Opcode {
    variant: OpcodeVariant,
    offset: usize,
    instrs: Vec<InstructionMode>,
}

impl Opcode {
    pub(crate) fn eval(&self, program: &mut Program) -> Option<usize> {
        struct EvalCtx<'a> {
            offset: usize,
            instrs: &'a [InstructionMode],
            program: &'a mut Program,
        }

        impl EvalCtx<'_> {
            fn immediate(&mut self, param: usize) -> &mut Val {
                &mut self.program.code[self.offset + param + 1]
            }

            fn lookup(&mut self, loc: usize) -> &mut Val {
                &mut self.program.code[loc]
            }

            fn param(&mut self, param: usize) -> &mut i32 {
                let loc = (*self.immediate(param)) as usize;
                self.lookup(loc)
            }

            /// Evaluate parameter <offset> for the current opcode
            pub fn eval_param(&mut self, param: usize) -> &mut Val {
                let mode = self.instrs.get(param).copied().unwrap_or_default();
                match mode {
                    InstructionMode::Parameter => self.param(param),
                    InstructionMode::Immediate => self.immediate(param),
                }
            }
        }

        let mut ctx = EvalCtx {
            offset: self.offset,
            instrs: &self.instrs,
            program,
        };

        match self.variant {
            OpcodeVariant::Add => {
                let res = *ctx.eval_param(0) + *ctx.eval_param(1);
                *ctx.eval_param(2) = res;
            }
            OpcodeVariant::Mult => {
                let res = *ctx.eval_param(0) * *ctx.eval_param(1);
                *ctx.eval_param(2) = res;
            }
            OpcodeVariant::Halt => {
                return None;
            }
            OpcodeVariant::Input => {
                let entered = {
                    let mut buf = String::new();
                    print!("Input: ");
                    std::io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut buf).unwrap();
                    buf.trim_end().parse().unwrap()
                };
                *ctx.eval_param(0) = entered;
            }
            OpcodeVariant::Output => {
                let to_output = ctx.eval_param(0);
                println!("{}", *to_output);
            }
            OpcodeVariant::JumpIfTrue => {
                if *ctx.eval_param(0) != 0 {
                    return Some((*ctx.eval_param(1)).try_into().unwrap());
                }
            }
            OpcodeVariant::JumpIfFalse => {
                if *ctx.eval_param(0) == 0 {
                    return Some((*ctx.eval_param(1)).try_into().unwrap());
                }
            }
            OpcodeVariant::LessThan => {
                let res = if *ctx.eval_param(0) < *ctx.eval_param(1) {
                    1
                } else {
                    0
                };
                *ctx.eval_param(2) = res;
            }
            OpcodeVariant::Equals => {
                let res = if *ctx.eval_param(0) == *ctx.eval_param(1) {
                    1
                } else {
                    0
                };
                *ctx.eval_param(2) = res;
            }
        }
        Some(self.offset + self.variant.instruction_count() + 1)
    }

    pub fn new(instr_raw: Val, offset: usize) -> Self {
        let opcode_raw = instr_raw % 100;
        let instrs = {
            let code = (instr_raw - opcode_raw) / 100;
            code.to_string()
                .chars()
                .map(|num| InstructionMode::try_from((num as u8) - b'0').unwrap())
                .rev()
                .collect()
        };

        let variant = OpcodeVariant::try_from(opcode_raw % 100).unwrap();
        Self {
            variant,
            offset,
            instrs,
        }
    }
}
