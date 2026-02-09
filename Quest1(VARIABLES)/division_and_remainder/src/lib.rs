//* recommendation (⚠️): use `Colorful Comments` extension for better readability of the comments in this file7

pub fn divide(x: i32, y: i32) -> (i32 , i32){
    (x / y , x % y)
}

/*
    * Q1 : why the tupple type exits in rust?
    - The tuple type in Rust is a way to group together multiple values of different types into a single compound type; ex` (String, i32) `. It is useful for returning multiple values from a function, as in this case where we want to return both the quotient and the remainder of a division operation. Tuples can also be used to group related data together without needing to define a separate struct, and they can be easily destructured to access individual elements.
*/
