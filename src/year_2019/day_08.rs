use itertools::Itertools;

use crate::utls::{MyParse, color::Color};

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn part_1(layers: &[u8], width: usize, height: usize) -> usize {
    let layers = layers.chunks(width * height);
    let min_zeroes = layers
        .min_by_key(|layer| layer.iter().filter(|&&x| x == 0).count())
        .unwrap();
    let ones_count = min_zeroes.iter().filter(|&&x| x == 1).count();
    let twos_count = min_zeroes.iter().filter(|&&x| x == 2).count();
    ones_count * twos_count
}

fn part_2(layers: &[u8], width: usize, height: usize) {
    let layers = layers
        .iter()
        .map(|x| Color::try_from(*x).unwrap())
        .collect_vec();
    let step = width * height;
    for ridx in 0..height {
        for cidx in 0..width {
            let idx = ridx * width + cidx;
            let color = layers[idx..]
                .iter()
                .step_by(step)
                .find(|layer| layer != &&Color::Transparent)
                .unwrap();

            let to_print = if *color == Color::White { "⬜" } else { "⬛" };
            print!("{}", to_print);
        }
        println!();
    }
}

#[allow(unused)]
struct ParseMe(Vec<u8>);

impl MyParse for ParseMe {
    fn my_parse(s: &str) -> Self {
        let nums = s.trim().chars().map(|c| c as u8 - b'0').collect();
        Self(nums)
    }
}

pub fn run() {
    let ParseMe(input) = crate::utls::read_text_from_file("2019", "08");
    let res = part_1(&input, WIDTH, HEIGHT);
    part_2(&input, WIDTH, HEIGHT);
    println!("The answer is {}", res);
}

#[cfg(test)]
mod test {
    use crate::utls::MyParse;

    use super::*;

    #[test]
    fn test_layers_d1_p1() {
        let input = "123456789012";
        let ParseMe(input) = ParseMe::my_parse(input);
        let res = part_1(&input, 2, 3);
        assert_eq!(res, 1);
    }
}
