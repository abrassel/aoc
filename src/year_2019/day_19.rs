use crate::{
    program::{Program, Val, io::View},
    utls::linalg::Point,
};

fn part_1(program: Program) -> Val {
    let mut sum = 0;
    for i in 0..50 {
        for j in 0..50 {
            let mut input = View::new(vec![j, i]);
            let mut output = 0;
            program.clone().eval(&mut input, &mut output);
            sum += output;
            print!("{}", output);
        }
        println!();
    }
    sum
}

fn in_beam(program: &Program, point: Point) -> bool {
    let mut input = View::new(point.into());
    let mut output = 0;
    program.clone().eval(&mut input, &mut output);
    output == 1
}

fn part_2(program: Program) -> Point {
    'row: for y in 100.. {
        println!("Examining row: {}", y);
        let mut found_beam = false;
        for x in 0..4 * y {
            let cur = Point(y, x);
            if in_beam(&program, cur) {
                if !found_beam {
                    found_beam = true;
                }
            } else if found_beam {
                // have exited the beam
                let upper_right = cur + Point(0, -1);
                if x == 5 {
                    println!("Looking for {}", upper_right);
                }
                // try for lower left
                let lower_left = upper_right + Point(100, -100);
                if in_beam(&program, lower_left) {
                    return Point(lower_left.0, upper_right.1);
                } else {
                    continue 'row;
                }
            }
        }
    }

    unreachable!()
}

pub fn run() {
    let input: Program = crate::utls::read_text_from_file("2019", "19");
    let res = part_1(input.clone());
    println!("Answer is {}", res);
    let res = part_2(input);
    println!("Answer is {}", res.0 * 10000 + res.1);
}
