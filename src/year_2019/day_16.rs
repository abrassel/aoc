use itertools::Itertools;

use crate::utls::{MyParse, linalg::matrix::Matrix};

fn part_1(input: &[i32], fft_mat: &Matrix<i32>) -> Vec<i32> {
    let mut input = Matrix::from(input.to_vec());
    for _ in 0..100 {
        input = fft(input, fft_mat);
    }
    input.column_vector()
}

#[derive(Debug)]
#[allow(unused)]
struct ParseInput(Vec<i32>);

pub fn fft(input: Matrix<i32>, fft: &Matrix<i32>) -> Matrix<i32> {
    let mut intermediate = fft * input;
    intermediate.update(|x| *x = x.abs() % 10);
    intermediate
}

pub fn precompute(len: usize) -> Matrix<i32> {
    (0..len)
        .map(|ridx| {
            let base_pattern = [0, 1, 0, -1];
            // repeat each element by x according to offset
            let repeat_pattern = base_pattern
                .iter()
                .flat_map(|&num| std::iter::repeat_n(num, ridx + 1));
            // repeat this row as many times as needed to fill out `len` cols
            repeat_pattern.cycle().skip(1).take(len).collect_vec()
        })
        .collect_vec()
        .into()
}

impl MyParse for ParseInput {
    fn my_parse(s: &str) -> Self {
        Self(s.trim().chars().map(|c| (c as u8 - b'0') as i32).collect())
    }
}
pub fn run() {
    let ParseInput(input) = crate::utls::read_text_from_file("2019", "16");
    let fft = precompute(input.len());
    let res = part_1(&input, &fft);
    println!("The answer is {:?}", &res[0..8]);
    let offset: usize = input[0..7]
        .iter()
        .map(|x| x.to_string())
        .join("")
        .parse()
        .unwrap();
    let len = input.len();
    let input = input
        .into_iter()
        .cycle()
        .skip(offset)
        .take(len * 10000 - offset)
        .collect_vec();
    let fft = precompute(input.len());
    let res = part_1(&input, &fft);
    println!("The answer is {:?}", &res[0..8])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        let ParseInput(input) = ParseInput::my_parse("12345678");
        let fft_base = precompute(input.len());

        let desired = vec![
            vec![1, 0, -1, 0, 1, 0, -1, 0],
            vec![0, 1, 1, 0, 0, -1, -1, 0],
            vec![0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 1, 1, 1, 1, 0],
            vec![0, 0, 0, 0, 1, 1, 1, 1],
            vec![0, 0, 0, 0, 0, 1, 1, 1],
            vec![0, 0, 0, 0, 0, 0, 1, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 1],
        ]
        .into();

        assert_eq!(fft_base, desired);

        let expected = ["48226158", "34040438", "03415518", "01029498"];
        let mut cur = input.into();
        for expected in expected {
            cur = fft(cur, &fft_base);
            assert_eq!(
                cur.clone().column_vector(),
                ParseInput::my_parse(expected).0
            );
        }
    }

    #[test]
    fn test_solution_big() {
        let ParseInput(input) = ParseInput::my_parse("80871224585914546619083218645595");
        let fft_base = precompute(input.len());
        let res = part_1(&input, &fft_base);
        assert_eq!(res[0..8], ParseInput::my_parse("24176176").0);
    }

    #[test]
    fn test_solution_big_two() {
        let ParseInput(input) = ParseInput::my_parse("19617804207202209144916044189917");
        let fft_base = precompute(input.len());
        let res = part_1(&input, &fft_base);
        assert_eq!(res[0..8], ParseInput::my_parse("73745418").0);
    }

    #[test]
    fn test_solution_big_three() {
        let ParseInput(input) = ParseInput::my_parse("69317163492948606335995924319873");
        let fft_base = precompute(input.len());
        let res = part_1(&input, &fft_base);
        assert_eq!(res[0..8], ParseInput::my_parse("52432133").0);
    }
}
