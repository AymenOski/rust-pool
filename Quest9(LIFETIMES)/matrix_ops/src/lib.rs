mod matrix;

mod scalar;

pub use matrix::Matrix;
pub use scalar::Scalar;

use std::ops::{Add, Sub, Mul};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Wrapper<const W: usize, const H: usize, T>(pub Matrix<W, H, T>);

impl<const W: usize, const H: usize, T> From<[[T; W]; H]> for Wrapper<W, H, T> {
fn from(array: [[T; W]; H]) -> Self {
Wrapper(Matrix(array))
}
}


/// Addition:
impl<const W: usize, const H: usize, T> Add for Wrapper<W, H, T>
where
T: Scalar + Copy,
{
type Output = Self;


fn add(self, rhs: Self) -> Self::Output {
let mut result = [[T::zero(); W]; H];


for i in 0..H {
for j in 0..W {
result[i][j] = self.0 .0[i][j] + rhs.0 .0[i][j];
}
}


Wrapper(Matrix(result))
}
}


/// Subtraction:
impl<const W: usize, const H: usize, T> Sub for Wrapper<W, H, T>
where
T: Scalar + Copy,
{
type Output = Self;


fn sub(self, rhs: Self) -> Self::Output {
let mut result = [[T::zero(); W]; H];


for i in 0..H {
for j in 0..W {
result[i][j] = self.0 .0[i][j] - rhs.0 .0[i][j];
}
}


Wrapper(Matrix(result))
}
}


/// Multiplication:
impl<const S: usize, T> Mul for Wrapper<S, S, T>
where
T: Scalar + Copy,
{
type Output = Self;


fn mul(self, rhs: Self) -> Self::Output {
let mut result = [[T::zero(); S]; S];


for i in 0..S {
for j in 0..S {
let mut sum = T::zero();
for k in 0..S {
sum = sum + self.0.0[i][k] * rhs.0.0[k][j];
}
result[i][j] = sum;
}
}


Wrapper(Matrix(result))
}
}