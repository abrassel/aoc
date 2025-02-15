use std::collections::HashMap;

use itertools::Itertools;

use crate::{
    program::{Program, io::WriteVal, spawn},
    utls::{
        color::Color,
        linalg::{Point, RotateDir},
    },
};

fn part_1(program: Program) -> usize {
    let points = paint_robot(program, Color::Black);
    points.len()
}

fn paint_robot(program: Program, start_color: Color) -> HashMap<Point, Color> {
    let mut points: HashMap<Point, Color> = HashMap::new();
    let mut dir = Point::UP;
    let mut pos = Point::default();
    let mut program = spawn::spawn(program);
    program.write_val(start_color as i128);
    while let Some(paint_color) = program.try_read_val() {
        points.insert(pos, u8::try_from(paint_color).unwrap().try_into().unwrap());
        let turn_dir = {
            let raw_dir = u8::try_from(program.try_read_val().unwrap()).unwrap();
            RotateDir::try_from(raw_dir).unwrap()
        };
        dir = dir.rotate(turn_dir);
        pos += dir;

        let new_pos_color = points.get(&pos).copied().unwrap_or_default();
        program.write_val(new_pos_color as i128);
    }
    program.join();
    points
}

fn part_2(program: Program) {
    let painting = paint_robot(program, Color::White);
    // figure out painting bounds
    let (minx, maxx) = painting
        .keys()
        .map(|key| key.0)
        .minmax()
        .into_option()
        .unwrap();
    let (miny, maxy) = painting
        .keys()
        .map(|key| key.1)
        .minmax()
        .into_option()
        .unwrap();
    // using Euclidean plane, so big y -> last row in matrix grid
    // also, printing row by row, so row-major order
    for row in (miny..=maxy).rev() {
        for col in minx..=maxx {
            let color = painting
                .get(&(col, row).into())
                .copied()
                // if unknown color, select black
                .unwrap_or_default();

            print!("{}", color);
        }
        println!();
    }
}

pub fn run() {
    let input: Program = crate::utls::read_text_from_file("2019", "11");
    let res = part_1(input.clone());
    println!("Answer is {}", res);
    part_2(input);
}

#[cfg(test)]
mod test {}
