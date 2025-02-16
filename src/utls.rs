use std::str::FromStr;

pub mod color;
pub mod display;
pub mod linalg;

pub fn read_text_from_file<T: MyParse>(year: &str, day: &str) -> T {
    let str = std::fs::read_to_string(format!("inputs/{}-{}.txt", year, day)).unwrap();
    T::my_parse(&str)
}

pub(crate) trait MyParse {
    fn my_parse(s: &str) -> Self;
}

impl<T: FromStr> MyParse for Vec<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    fn my_parse(s: &str) -> Vec<T> {
        s.lines()
            .filter(|x| !x.is_empty())
            .map(|line| line.trim().parse().unwrap())
            .collect()
    }
}
