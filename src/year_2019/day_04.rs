use itertools::Itertools;

use crate::utls::MyParse;

fn is_password(num: i32) -> bool {
    let str = num.to_string().chars().collect_vec();

    let runs = str.iter().chunk_by(|&&k| k);
    let adjacent_repeat = runs.into_iter().any(|(_, run)| run.count() == 2);
    let monotonic_increase = str.iter().tuple_windows().all(|(l, r)| l <= r);
    adjacent_repeat && monotonic_increase
}

fn part_1(range: Range) -> usize {
    let lower_bound = std::cmp::max(range.lower, 100000);
    let upper_bound = std::cmp::min(range.upper, 999999);
    (lower_bound..=upper_bound)
        .filter(|&num| is_password(num))
        .count()
}

fn part_2(input: &'static str) {}

#[allow(unused)]
struct Range {
    lower: i32,
    upper: i32,
}

impl MyParse for Range {
    fn my_parse(s: &str) -> Self {
        let (left, right) = s.trim().split_once("-").unwrap();
        Self {
            lower: left.parse().unwrap(),
            upper: right.parse().unwrap(),
        }
    }
}

pub fn run() {
    let input = crate::utls::read_text_from_file("2019", "04");
    let res = part_1(input);
    println!("The answer is {}", res);
    // part_2(input);
}

#[cfg(test)]
mod test {
    use super::*;

    //     #[test]
    //     fn test_solution() {}
}
