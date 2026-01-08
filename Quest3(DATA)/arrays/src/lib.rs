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