use itertools::Itertools;
use num_traits::NumAssign;
use rayon::prelude::*;

use crate::{program::Val, utls::linalg::Point};

#[derive(PartialEq, derive_more::From, Clone)]
pub struct Matrix<T> {
    data: Vec<Vec<T>>,
}

impl<T: ToString> std::fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            let res = row.iter().map(|x| x.to_string()).join(", ");
            writeln!(f, "{}", res)?;
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: ToString> std::fmt::Debug for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Display>::fmt(self, f)
    }
}

impl<T: Send + Sync + Copy> Matrix<T> {
    pub fn rows(&self) -> usize {
        self.data.len()
    }
    pub fn cols(&self) -> usize {
        self.data[0].len()
    }
    pub fn update(&mut self, f: impl Fn(&mut T) + Send + Sync + Copy) {
        self.data
            .par_iter_mut()
            .for_each(|row| row.par_iter_mut().for_each(f));
    }

    pub(crate) fn column_vector(self) -> Vec<T> {
        assert_eq!(self.cols(), 1, "not a column vector");
        self.transpose().data.remove(0)
    }

    pub fn transpose(self) -> Self {
        let len = self.data[0].len();
        let data = (0..len)
            .into_par_iter()
            .map(|i| self.data.iter().map(|row| row[i]).collect())
            .collect();
        Self { data }
    }
}

impl<T> From<Vec<T>> for Matrix<T> {
    fn from(value: Vec<T>) -> Self {
        Self {
            data: value.into_iter().map(|x| vec![x]).collect(),
        }
    }
}

impl<T: NumAssign + Send + Sync + std::iter::Sum + Copy> std::ops::Mul<Matrix<T>> for &Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        let data = (0..self.rows())
            .into_par_iter()
            .map(|row| {
                // compute row
                (0..rhs.cols())
                    .into_par_iter()
                    .map(|col| {
                        // compute col

                        (0..self.cols())
                            .into_par_iter()
                            .map(|j| self.data[row][j] * rhs.data[j][col])
                            .sum()
                    })
                    .collect()
            })
            .collect();
        Matrix { data }
    }
}

impl std::ops::Mul<Point> for &Matrix<Val> {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        assert!(self.cols() == 2 && self.rows() == 2);
        Point(
            self.data[0][0] * rhs.0 + self.data[0][1] * rhs.1,
            self.data[1][0] * rhs.0 + self.data[1][1] * rhs.1,
        )
    }
}

impl From<[[Val; 2]; 2]> for Matrix<Val> {
    fn from(value: [[Val; 2]; 2]) -> Self {
        Self {
            data: vec![
                vec![value[0][0], value[0][1]],
                vec![value[1][0], value[1][1]],
            ],
        }
    }
}
