//* recommendation (⚠️): use Colorful Comments extension for better readability of the comments in this file

pub fn sum(a: u8, b: u8) -> u8 {
    a + b
}

pub fn diff(a: i16, b: i16) -> i16 {
    a - b
}   

pub fn pro(a: i8, b: i8) -> i8 {
    a * b 
}

pub fn quo(a: f32, b: f32) -> f32 {
    a / b
}

pub fn rem(a: f32, b: f32) -> f32 {
    a % b
}

/* 
    !- ML inference : huge memory & speed difference
    !- decrease memory bandwidth pressure
    !- You make SIMD harder or impossible 
     -> i8 : 32 values in 256-bit register
     -> i16 : 16 values in 256-bit register
     -> i32 : 8 values in 256-bit register
     -> i64 : 4 values in 256-bit register
     *=> you see the importance of using the right data type for your application
     *=> using smaller data types can lead to significant performance improvements, especially in applications that require processing large amounts of data, such as machine learning inference.
*/
