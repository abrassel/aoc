use crate::program::{Program, Val};

fn part_1(mut program: Program) -> Val {
    program.eval(&mut std::io::stdin(), &mut std::io::stdout())
}

// fn part_2(input: &'static str) {}

pub fn run() {
    let input = crate::utls::read_text_from_file("2019", "05");
    part_1(input);
    // part_2(input);
}
