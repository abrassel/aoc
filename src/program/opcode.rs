use super::Program;
use num_enum::TryFromPrimitive;

#[derive(TryFromPrimitive)]
#[repr(u32)]
enum OpcodeVariant {
    Add = 1,
    Mult = 2,
    Halt = 99,
}

impl OpcodeVariant {
    pub fn instruction_count(&self) -> usize {
        match self {
            OpcodeVariant::Add => 4,
            OpcodeVariant::Mult => 4,
            OpcodeVariant::Halt => 1,
        }
    }
}

pub struct Opcode {
    variant: OpcodeVariant,
    offset: usize,
}

impl Opcode {
    pub(crate) fn eval(&self, program: &mut Program) -> Option<usize> {
        struct EvalCtx<'a> {
            offset: usize,
            program: &'a mut Program,
        }

        impl EvalCtx<'_> {
            pub fn parameter(&mut self, offset: usize) -> &mut u32 {
                &mut self.program.code[self.offset + offset]
            }

            pub fn val(&mut self, loc: usize) -> &mut u32 {
                &mut self.program.code[loc]
            }

            pub fn lookup(&mut self, offset: usize) -> &mut u32 {
                let loc = (*self.parameter(offset)) as usize;
                self.val(loc)
            }
        }

        let mut ctx = EvalCtx {
            offset: self.offset,
            program,
        };

        match self.variant {
            OpcodeVariant::Add => {
                let res = *ctx.lookup(1) + *ctx.lookup(2);
                *ctx.lookup(3) = res;
            }
            OpcodeVariant::Mult => {
                let res = *ctx.lookup(1) * *ctx.lookup(2);
                *ctx.lookup(3) = res;
            }
            OpcodeVariant::Halt => {
                return None;
            }
        }
        Some(self.offset + self.variant.instruction_count())
    }

    pub fn new(opcode_raw: u32, offset: usize) -> Self {
        let variant = OpcodeVariant::try_from(opcode_raw).unwrap();
        Self { variant, offset }
    }
}
