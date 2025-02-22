use std::collections::HashSet;

use itertools::Itertools;
use strum::IntoEnumIterator;

use crate::{
    program::{
        Program, Val,
        io::{AsciiStdin, TryReadVal, TryWriteVal, View, infallible::Unused},
    },
    utls::{
        conversions::point_map,
        linalg::{CardinalDir, Point, RotateDir},
    },
};

const FUN_SIZE_LIMIT: usize = 20;
fn part_1(program: Program) -> Val {
    let scaffold = Scaffold::init(program);
    // find all points
    scaffold
        .grid
        .iter()
        .filter(|&&point| {
            CardinalDir::iter().all(|dir| {
                let nbor = point + dir.into();
                scaffold.grid.contains(&nbor)
            })
        })
        .map(|point| point.0 * point.1)
        .sum()
}

fn part_2(program: Program) {
    let scaffold = Scaffold::init(program.clone());
    let path = scaffold.path();
    // try every partition for fitting our criteria
    let ([a, b, c], main) = find_functions(path);

    // now, try to make the robot walk the solution
    let mut program = program;
    program.code[0] = 2;

    let mut input = [&main, &a, &b, &c]
        .iter()
        .map(|command| command.trim_end_matches(','))
        .join("\n");
    input.push('\n');

    RobotManager::build(program)
        .interactive(false)
        .moves(input.chars().collect_vec())
        .build()
        .run();
}

fn find_functions(path: Vec<String>) -> ([String; 3], String) {
    let path = path.join("");
    for b_start in 0..path.len() - 2 {
        let mut a_path = path.clone();
        let avs = path[0..b_start].to_owned();
        if avs.len() > FUN_SIZE_LIMIT {
            continue;
        }
        a_path = a_path.replace(&avs, "A");
        let a_path = a_path.trim_start_matches("A");
        let upper_bound = a_path.find("A").unwrap_or(a_path.len());
        for c_start in 0..=upper_bound {
            let mut b_path = a_path.to_owned();
            let bvs = a_path[..c_start].to_owned();

            if bvs.len() > FUN_SIZE_LIMIT {
                continue;
            }
            b_path = b_path
                .replace(&bvs, "B")
                .trim_start_matches(['A', 'B'])
                .to_owned();
            let c_end = b_path.find(['A', 'B']).unwrap_or(b_path.len());
            let cvs = b_path[..c_end].to_owned();
            if cvs.len() > FUN_SIZE_LIMIT {
                continue;
            }
            let mut c_path = b_path.clone();
            c_path = c_path.replace(&cvs, "C");
            let c_path = c_path.trim_start_matches(['A', 'B', 'C']);
            if c_path.is_empty() {
                let path = path
                    .replace(&avs, "A,")
                    .replace(&bvs, "B,")
                    .replace(&cvs, "C,");
                return ([avs, bvs, cvs], path);
            }
        }
    }
    panic!("There was no solution");
}

#[derive(Default)]
pub struct RobotManagerBuilder {
    interactive: bool,
    moves: Vec<char>,
    program: Program,
}

impl RobotManagerBuilder {
    pub fn new(program: Program) -> Self {
        Self {
            program,
            ..Default::default()
        }
    }
    pub fn interactive(self, interactive: bool) -> Self {
        Self {
            interactive,
            ..self
        }
    }

    pub fn moves(self, moves: Vec<char>) -> Self {
        Self { moves, ..self }
    }

    pub fn build(mut self) -> RobotManager {
        let interactive = if self.interactive { 'y' } else { 'n' };
        self.moves.push(interactive);
        self.moves.push('\n');
        RobotManager {
            program: self.program,
            moves: self.moves,
        }
    }
}

pub struct RobotManager {
    moves: Vec<char>,
    program: Program,
}

impl RobotManager {
    pub fn build(program: Program) -> RobotManagerBuilder {
        RobotManagerBuilder::new(program)
    }

    pub fn run(mut self) {
        let view = View::new_char(self.moves);

        fn val_to_char(val: Val) -> char {
            (u8::try_from(val)).unwrap() as char
        }
        struct AsciiView {
            view: View,
            prev_written: char,
        }
        impl TryWriteVal for AsciiView {
            fn try_write_val(&mut self, val: Val) -> Option<()> {
                if val > 300 {
                    println!("Answer is: {}", val);
                } else {
                    let cval = val_to_char(val);
                    self.prev_written = cval;
                    print!("{}", cval);
                }

                Some(())
            }
        }
        impl TryReadVal for AsciiView {
            fn try_read_val(&mut self) -> Option<Val> {
                self.view.try_read_val()
            }
        }
        let mut view = AsciiView {
            view,
            prev_written: 'X',
        };
        self.program.eval_joint(&mut view);
    }
}

struct Scaffold {
    raw_grid: Vec<Vec<char>>,
    grid: HashSet<Point>,
    pos: (Point, CardinalDir),
}

impl Scaffold {
    pub fn init(mut program: Program) -> Self {
        let mut raw_grid: Vec<Vec<char>> = Vec::new();
        program.eval(&mut Unused, &mut raw_grid);
        let grid = point_map(raw_grid.clone());

        Self {
            grid: grid
                .iter()
                .filter_map(|(k, v)| (*v == '#').then_some(*k))
                .collect(),
            pos: grid
                .iter()
                .find_map(|(k, v)| CardinalDir::try_from(*v).ok().map(|dir| (*k, dir)))
                .unwrap(),
            raw_grid,
        }
    }

    pub fn path(&self) -> Vec<String> {
        let mut path = vec![];
        let (mut cur_loc, cur_dir) = self.pos;
        let mut cur_dir: Point = cur_dir.into();
        'outer: loop {
            let mut dist = 0;
            // walk the direction until no longer possible
            let mut lookahead = cur_loc + cur_dir;
            while self.grid.contains(&lookahead) {
                cur_loc = lookahead;
                dist += 1;
                lookahead = cur_loc + cur_dir;
            }

            if dist > 0 {
                path.push(format!("{},", dist));
            }

            // figure out the rotation
            for rotate in RotateDir::iter() {
                let dir = &rotate.transform() * cur_dir;
                if self.grid.contains(&(cur_loc + dir)) {
                    let rotation = match rotate {
                        RotateDir::CounterClockwise => "L,",
                        RotateDir::Clockwise => "R,",
                    };
                    path.push(rotation.to_owned());
                    cur_dir = dir;
                    // continue to the next step
                    continue 'outer;
                }
            }

            // if we couldn't find any rotations, then we're done, and we've reached the end
            break;
        }
        path
    }
}

impl std::fmt::Display for Scaffold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.raw_grid {
            for c in line {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn run() {
    let input: Program = crate::utls::read_text_from_file("2019", "17");
    let ans = part_1(input.clone());
    println!("The ans is {}", ans);
    part_2(input);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "";

    #[test]
    fn test_solution() {}
}
