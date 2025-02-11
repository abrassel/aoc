use crate::program::Program;

fn part_1(mut program: Program) -> i32 {
    program.init(12, 2);
    program.eval()
}

fn part_2(program: Program) -> i32 {
    const TARGET: i32 = 19690720;
    for noun in 0..100 {
        for verb in 0..100 {
            println!("Noun: {}, verb: {}", noun, verb);
            let mut program = program.clone();
            program.init(noun, verb);
            if program.eval() == TARGET {
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
