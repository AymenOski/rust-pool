//* recommendation (⚠️): use `Colorful Comments` extension for better readability of the comments in this file

pub fn factorial(num: u64) -> u64 {
    if num > 20 {
        panic!("Input number is too large to compute factorial without overflow. 2^64 - 1 is the maximum value for u64, and 20! is the largest factorial that fits within this limit.");
    }

    if num == 0 || num == 1 {
        return 1;
    }

    return factorial(num - 1) * num;
}

/*
* Q1. What happens if the input number is greater than 20?
    -A1. The factorial of numbers greater than 20 will exceed the maximum value that can be stored in a u64, leading to an overflow and incorrect results. In practice, you might want to handle this case to prevent such issues. 
*/ 