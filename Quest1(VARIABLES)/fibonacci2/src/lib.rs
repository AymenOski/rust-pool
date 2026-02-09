//* recommendation (⚠️): use `Colorful Comments` extension for better readability of the comments in this file7

pub fn fibonacci(n: u32) -> u32 {
    if n == 1 {
        return 1
    }
    if n == 0 {
        return 0
    }

    return fibonacci(n-1) + fibonacci(n-2);
}

/*
    * Where Fibonacci and golden ratio actually appear / are used in real life:
    - In nature: The arrangement of leaves on a stem, the branching of trees, the arrangement of seeds in a sunflower, all exhibit patterns that can be described using Fibonacci numbers and the golden ratio.
        like they leaves are arranged in a way that they don't block each other from sunlight, and the seeds in a sunflower are arranged in a way that they can fit as many seeds as possible in the available space.
*/