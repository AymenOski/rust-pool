mod scalar;
pub use scalar::Scalar;

use std::ops::{Add, Mul};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Copy> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        self.0[0].len()
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        let mut res = Vec::new();
        for row in &self.0 {
            res.push(row[n]);
        }
        res
    }
}

impl<T: Scalar + Copy + Mul<Output = T> + Add<Output = T>> Mul for Matrix<T> {
    type Output = Option<Matrix<T>>;
    fn mul(self, m2: Self) -> Self::Output {
        if self.number_of_cols() != m2.number_of_rows() {
            return None;
        }
        let mut res = vec![vec![T::zero(); m2.number_of_cols()]; self.number_of_rows()];
        for i in 0..self.number_of_rows() {
            for j in 0..m2.number_of_cols() {
                for k in 0..self.number_of_cols() {
                    res[i][j] = res[i][j] + (self.0[i][k] * m2.0[k][j]);
                }
            }
        }

        Some(Matrix(res))
    }
}