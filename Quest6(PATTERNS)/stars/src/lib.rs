pub fn stars(n: u32) -> String {
    let i = 2_u32.pow(n);
    let mut result = String::new();
    for _ in 0..i {
        result += "*";
    }
    //or "*".repeat(count)
    result
}