pub use crate::scalar::Scalar;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Matrix<const W: usize, const H: usize, T>(pub [[T; W]; H]);

impl<const W: usize, const H: usize, T: Scalar + Copy> Matrix<W, H, T> {
    pub fn zero() -> Self {
        Self([[T::zero(); W]; H])
    }
}

impl<const S: usize, T: Scalar + Copy> Matrix<S, S, T> {
    pub fn identity() -> Self {
        let mut matrix = [[T::zero(); S]; S];
        for i in 0..S {
            matrix[i][i] = T::one();
        }
        Self(matrix)
    }
}
