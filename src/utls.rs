use std::str::FromStr;

use crate::program::Val;

pub mod color;
pub mod conversions;
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

pub trait FromVal {
    fn from_val(val: Val) -> Self;
}

impl<T: TryFrom<u8>> FromVal for T
where
    <T as std::convert::TryFrom<u8>>::Error: std::fmt::Debug,
{
    fn from_val(val: Val) -> Self {
        let val: u8 = u8::try_from(val).unwrap();
        val.try_into().unwrap()
    }
}

pub trait ValInto<Into> {
    fn val_into(self) -> Into;
}

impl<I: FromVal> ValInto<I> for Val {
    fn val_into(self) -> I {
        I::from_val(self)
    }
}
