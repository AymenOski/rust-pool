use std::ops::Add;

use crate::Scalar;

#[derive(Debug, PartialEq, Clone)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T: Scalar> Add for Vector<T> {
    type Output = Option<Vector<T>>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() {
            return None;
        }

        let result = self.0
            .into_iter()    
            .zip(rhs.0)
            .map(|(a, b)| a + b)
            .collect();

        Some(Vector(result))
    }
}

impl<T: Scalar> Vector<T> {
    pub fn dot(self, rhs: Self) -> Option<T> {
        if self.0.len() != rhs.0.len() {
            return None;
        }

        let sum = self.0
            .into_iter()
            .zip(rhs.0)
            .map(|(a, b)| a * b)
            .fold(T::zero(), |acc, prod| acc + prod);

        Some(sum)
    }
}