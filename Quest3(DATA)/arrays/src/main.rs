//* recommendation (⚠️): use `Colorful Comments` extension for better readability of the comments in this file7

use arrays::*;

fn main() {
    let a = (1..=10).collect::<Vec<i32>>();
    let b = [5; 10];

    println!("The sum of the elements in {:?} is {}", a, sum(&a));
    println!("The sum of the elements in {:?} is {}", b, sum(&b));
    println!(
        "Array of {} elements filled with 10 = {:?}",
        thirtytwo_tens().len(),
        thirtytwo_tens()
    );
}

/*
    * Q & A:
    * Q1. What does (1..=10).collect::<Vec<i32>>(); do?
    -A1. It creates a range from 1 to 10 (inclusive) and collects it into a vector of i32.
    its like we are creating an iterator that generates numbers from 1 to 10 and then using the magic wand which is collect() to turn it into a vector.
    more accuratly ; it consumes the iterator and builds a vector from its yielded values.


*/