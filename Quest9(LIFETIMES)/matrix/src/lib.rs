//* recommendation (⚠️): use `Colorful Comments` extension for better readability of the comments in this file7

mod scalar;
pub use scalar::Scalar;

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

/*
    * Q & A :
    * Q1: What is the purpose of this exercice?
    By doing this exercice, and understanding it, you will know how the compiler can conclude the values of the generic parameters(T, W, H).
    Const Generics let u link the size of the array to the type, because of that in this case the matrix dimensions will be known at compile time,giving us performance benefits like storing the matrix on the stack instead of the heap, and also safety benefits in this case for example `Matrix<3,3> * Matrix<4,4>` is not possible because we linked the type with the size so `Matrix<3,3> * Matrix<4,4>` are different types now.
*/