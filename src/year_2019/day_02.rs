use std::io;

use crate::program::{Program, Val};

fn part_1(mut program: Program) -> Val {
    program.init(12, 2);
    program.eval(&mut io::stdin(), &mut io::stdout())
}

fn part_2(program: Program) -> Val {
    const TARGET: Val = 19690720;
    for noun in 0..100 {
        for verb in 0..100 {
            println!("Noun: {}, verb: {}", noun, verb);
            let mut program = program.clone();
            program.init(noun, verb);
            if program.eval(&mut std::io::stdin(), &mut std::io::stdout()) == TARGET {
                return 100 * noun + verb;
            }
        }
    }
    unreachable!()
}

pub fn run() {
    let input: Program = crate::utls::read_text_from_file("2019", "02");
    // let res = part_1(input.clone());
    let res = part_2(input);
    println!("The answer is {}", res);
}
