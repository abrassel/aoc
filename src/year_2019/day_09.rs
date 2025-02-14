use crate::program::Program;

fn part_1(mut program: Program) {
    program.eval(&mut std::io::stdin(), &mut std::io::stdout());
}

fn part_2(mut program: Program) {
    program.eval(&mut std::io::stdin(), &mut std::io::stdout());
}

pub fn run() {
    let input: Program = crate::utls::read_text_from_file("2019", "09");
    // part_1(input.clone());
    part_2(input);
}

#[cfg(test)]
mod test {
    use crate::utls::MyParse;

    use super::*;

    #[test]
    fn test_solution() {
        let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let mut program = Program::my_parse(input);
        let mut buf = vec![];
        program.eval(&mut std::io::stdin(), &mut buf);
        assert_eq!(Program::parse_code(input), buf);
    }

    #[test]

    fn test_solution_2() {
        let input = "1102,34915192,34915192,7,4,7,99,0";
        let mut program = Program::my_parse(input);
        let mut buf = vec![];
        program.eval(&mut std::io::stdin(), &mut buf);
        assert!(buf.len() == 1);
        assert!(buf[0].to_string().len() == 16);
    }

    #[test]

    fn test_solution_3() {
        let input = "104,1125899906842624,99";
        let mut program = Program::my_parse(input);
        let mut buf = vec![];
        program.eval(&mut std::io::stdin(), &mut buf);
        assert!(buf.len() == 1);
        assert_eq!(buf[0], 1125899906842624);
    }
}
