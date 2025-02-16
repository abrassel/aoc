use std::collections::HashMap;

use itertools::Itertools;
use num_enum::TryFromPrimitive;

use crate::{
    program::{
        Program, Val,
        io::{ReadVal, WriteVal},
    },
    utls::{display::paint, linalg::Point},
};

#[derive(Copy, Clone, Default, TryFromPrimitive, Eq, PartialEq, derive_more::Display)]
#[repr(u8)]
pub enum ArcadeTile {
    #[default]
    #[display(".")]
    Empty = 0,
    #[display("#")]
    Wall = 1,
    #[display("▨")]
    Block = 2,
    #[display("━")]
    HorizontalPaddle = 3,
    #[display("o")]
    Ball = 4,
}
fn part_1(mut program: Program) -> usize {
    let mut output_buf = vec![];

    program.eval(&mut std::io::stdin(), &mut output_buf);

    let map: HashMap<Point, ArcadeTile> = output_buf
        .into_iter()
        .tuples()
        .map(|(x, y, id)| {
            (
                (x, -y).into(),
                u8::try_from(id).unwrap().try_into().unwrap(),
            )
        })
        .collect();

    paint(&map);

    map.values()
        .filter(|value| **value == ArcadeTile::Block)
        .count()
}

#[derive(Default)]
struct ArcadeProgram {
    new_tile_buf: Vec<Val>,
    game_state: HashMap<Point, ArcadeTile>,
    score: usize,
}

impl ArcadeProgram {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ReadVal for ArcadeProgram {
    fn read_val(&mut self) -> crate::program::Val {
        // move paddle toward ball
        let ball_pos = self
            .game_state
            .iter()
            .find(|(_, tile)| **tile == ArcadeTile::Ball)
            .unwrap()
            .0;
        let paddle_pos = self
            .game_state
            .iter()
            .find(|(_, tile)| **tile == ArcadeTile::HorizontalPaddle)
            .unwrap()
            .0;

        // only need to consider x dimension
        (ball_pos.0 - paddle_pos.0).signum() as Val
    }
}

impl WriteVal for ArcadeProgram {
    fn write_val(&mut self, val: crate::program::Val) {
        self.new_tile_buf.push(val);

        if self.new_tile_buf.len() == 3 {
            // have received full tile
            let (x, y, tile) = (
                self.new_tile_buf[0],
                self.new_tile_buf[1],
                self.new_tile_buf[2],
            );
            if (x, y) == (-1, 0) {
                self.score = tile.try_into().unwrap();
            } else {
                self.game_state.insert(
                    (x, y).into(),
                    u8::try_from(tile).unwrap().try_into().unwrap(),
                );
            }
            self.new_tile_buf.clear();
        }
    }
}

fn part_2(mut program: Program) -> usize {
    program.code[0] = 2;
    let mut arcade_program = ArcadeProgram::new();
    program.eval_joint(&mut arcade_program);
    arcade_program.score
}

pub fn run() {
    let input: Program = crate::utls::read_text_from_file("2019", "13");
    let res = part_1(input.clone());
    println!("The answer is {}", res);
    let res = part_2(input);
    println!("The answer is {}", res);
}
