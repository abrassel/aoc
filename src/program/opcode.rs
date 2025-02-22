use super::{
    Program, ProgramState,
    io::{TryReadVal, TryWriteVal},
};
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
    RelativeBaseOffset = 9,
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
            OpcodeVariant::RelativeBaseOffset => 1,
        }
    }
}

#[derive(TryFromPrimitive, Default, Clone, Copy)]
#[repr(u8)]
pub enum InstructionMode {
    #[default]
    Parameter = 0,
    Immediate = 1,
    Relative = 2,
}

pub struct Opcode {
    variant: OpcodeVariant,
    instrs: Vec<InstructionMode>,
}

impl Opcode {
    pub(crate) fn eval<Io: TryReadVal + TryWriteVal>(
        &self,
        program: &mut Program,
        program_state: ProgramState,
        io: &mut Io,
    ) -> Option<ProgramState> {
        struct EvalCtx<'a> {
            program_state: ProgramState,
            instrs: &'a [InstructionMode],
            program: &'a mut Program,
        }

        impl EvalCtx<'_> {
            fn immediate(&mut self, param: usize) -> &mut Val {
                &mut self.program.code[self.program_state.offset + param + 1]
            }

            fn lookup(&mut self, loc: usize) -> &mut Val {
                &mut self.program.code[loc]
            }

            fn param(&mut self, param: usize) -> &mut Val {
                let loc = (*self.immediate(param)) as usize;
                self.lookup(loc)
            }

            fn relative(&mut self, param: usize) -> &mut Val {
                let loc = *self.immediate(param)
                    + i128::try_from(self.program_state.relative_base).unwrap();
                self.lookup(loc.try_into().unwrap())
            }

            /// Evaluate parameter <offset> for the current opcode
            pub fn eval_param(&mut self, param: usize) -> &mut Val {
                let mode = self.instrs.get(param).copied().unwrap_or_default();
                match mode {
                    InstructionMode::Parameter => self.param(param),
                    InstructionMode::Immediate => self.immediate(param),
                    InstructionMode::Relative => self.relative(param),
                }
            }
        }

        let mut ctx = EvalCtx {
            program_state,
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
                let entered = io.try_read_val();
                *ctx.eval_param(0) = entered?;
            }
            OpcodeVariant::Output => {
                let to_output = ctx.eval_param(0);
                io.try_write_val(*to_output)?;
            }
            OpcodeVariant::JumpIfTrue => {
                if *ctx.eval_param(0) != 0 {
                    let program_state = ProgramState {
                        offset: (*ctx.eval_param(1)).try_into().unwrap(),
                        ..ctx.program_state
                    };
                    return Some(program_state);
                }
            }
            OpcodeVariant::JumpIfFalse => {
                if *ctx.eval_param(0) == 0 {
                    let program_state = ProgramState {
                        offset: (*ctx.eval_param(1)).try_into().unwrap(),
                        ..ctx.program_state
                    };
                    return Some(program_state);
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
            OpcodeVariant::RelativeBaseOffset => {
                let new_relative_base =
                    i128::try_from(ctx.program_state.relative_base).unwrap() + *ctx.eval_param(0);
                ctx.program_state.relative_base = new_relative_base.try_into().unwrap();
            }
        }
        let program_state = ProgramState {
            offset: ctx.program_state.offset + self.variant.instruction_count() + 1,
            ..ctx.program_state
        };
        Some(program_state)
    }

    pub fn new(instr_raw: Val) -> Self {
        let opcode_raw = instr_raw % 100;
        let instrs = {
            let code = (instr_raw - opcode_raw) / 100;
            code.to_string()
                .chars()
                .map(|num| InstructionMode::try_from((num as u8) - b'0').unwrap())
                .rev()
                .collect()
        };

        let variant = OpcodeVariant::try_from((opcode_raw % 100) as i32).unwrap();
        Self { variant, instrs }
    }
}
