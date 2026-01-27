use std::ops::{Add, Sub, Mul, Div};

pub trait Scalar: Sized 
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
{
    fn zero() -> Self;
    fn one()  -> Self;
}

impl Scalar for u32  { fn zero() -> u32 { 0 }   fn one() -> u32 { 1 } }
impl Scalar for u64  { fn zero() -> u64 { 0 }   fn one() -> u64 { 1 } }
impl Scalar for i32  { fn zero() -> i32 { 0 }   fn one() -> i32 { 1 } }
impl Scalar for i64  { fn zero() -> i64 { 0 }   fn one() -> i64 { 1 } }
impl Scalar for f32  { fn zero() -> f32 { 0.0 } fn one() -> f32 { 1.0 } }
impl Scalar for f64  { fn zero() -> f64 { 0.0 } fn one() -> f64 { 1.0 } }
