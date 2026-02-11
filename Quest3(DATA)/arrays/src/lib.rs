//* recommendation (⚠️): use `Colorful Comments` extension for better readability of the comments in this file7

pub fn sum(a: &[i32]) -> i32 {
    // let mut sum = 0;
    // for &val in a {
    //     sum += val;
    // }
    // sum

    // or using iterator
    a.iter().sum()
}

pub fn thirtytwo_tens() -> [i32; 32] {
    [10; 32]
}

/*
    &as we said before iter() returns an iterator that yields just references to the elements of the slice, so we can use it to iterate over the elements without consuming the slice itself.
*/