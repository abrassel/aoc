use std::sync::mpsc::{self};

use itertools::Itertools;

use crate::program::{Program, Val, io::View};

#[allow(unused)]
fn part_1(program: Program) -> Val {
    (0..5)
        .permutations(5)
        .map(|input| orchestrate_p1(program.clone(), &input))
        .max()
        .unwrap()
}

fn orchestrate_p1(mut program: Program, input: &[Val]) -> Val {
    let mut output = 0;
    for &input in input {
        let mut input = View::new(vec![input, output]);
        program.eval(&mut input, &mut output);
    }
    output
}

fn part_2(program: Program) -> Val {
    (5..10)
        .permutations(5)
        .map(|input| orchestrate_p2(program.clone(), &input))
        .max()
        .unwrap()
}

fn orchestrate_p2(program: Program, input: &[Val]) -> Val {
    // launch 5 programs with pipes attached to their inputs and outputs

    let mut handles = vec![];
    let mut ins = vec![];
    let mut outs = vec![];
    for _ in 0..input.len() {
        let (send_in, mut recv_in) = mpsc::channel();
        let (mut send_out, recv_out) = mpsc::channel();
        let mut program = program.clone();
        let handle = std::thread::spawn(move || {
            program.eval(&mut recv_in, &mut send_out);
            recv_in
        });
        handles.push(handle);
        ins.push(send_in);
        outs.push(recv_out);
    }

    // before sending off receivers, seed them with the correct inputs
    for (amp_in, input) in ins.iter().zip(input) {
        amp_in.send(*input).unwrap();
    }

    ins[0].send(0).unwrap();

    // last output becomes first input
    outs.rotate_right(1);

    for (amp_in, prev_amp_out) in ins.into_iter().zip(outs) {
        std::thread::spawn(move || {
            while let Ok(in_val) = prev_amp_out.recv() {
                amp_in.send(in_val).unwrap();
            }
        });
    }

    let mut first_amp_in = None;
    for handle in handles {
        let amp_in = handle.join().unwrap();
        if first_amp_in.is_none() {
            first_amp_in = Some(amp_in);
        }
    }

    first_amp_in.unwrap().recv().unwrap()
}

pub fn run() {
    let input: Program = crate::utls::read_text_from_file("2019", "07");
    // let res = part_1(input.clone());
    let res = part_2(input);
    println!("The answer is: {}", res);
    // part_2(input);
}

#[cfg(test)]
mod test {
    use crate::utls::MyParse;

    use super::*;

    #[test]
    fn test_amp_1() {
        let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        let program = Program::my_parse(input);
        // let ans = part_1(program);
        let ans = orchestrate_p1(program, &[4, 3, 2, 1, 0]);
        assert_eq!(ans, 43210);
    }

    #[test]
    fn test_amp_2() {
        let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        let program = Program::my_parse(input);
        // let ans = part_1(program);
        let ans = part_1(program);
        assert_eq!(ans, 43210);
    }

    #[test]
    fn test_amp_3() {
        let input = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        let program = Program::my_parse(input);
        // let ans = part_1(program);
        let ans = part_1(program);
        assert_eq!(ans, 54321);
    }

    #[test]
    fn test_amp_4() {
        let input = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        let program = Program::my_parse(input);
        let ans = part_1(program);
        assert_eq!(ans, 43210);
    }

    #[test]
    fn test_amp_5() {
        let input =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        let program = Program::my_parse(input);
        let ans = orchestrate_p2(program.clone(), &[9, 8, 7, 6, 5]);
        assert_eq!(ans, 139629729);
        let ans = part_2(program);
        assert_eq!(ans, 139629729);
    }
}
