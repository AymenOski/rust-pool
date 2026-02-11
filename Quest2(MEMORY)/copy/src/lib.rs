//* recommendation (⚠️): use `Colorful Comments` extension for better readability of the comments in this file7

pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c as f64).abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let mut str = String::new();
    for x in a.split_whitespace() {
        let _ = match x.parse::<i8>() {
            Ok(num) => {
                let exp_n = (num as f64).exp();
                str.push_str(&format!("{}", exp_n));
                str.push(' ');
                0i8
            },
            Err(_) => 0, 
        };
    }
    (a , str.trim_end().to_string())
}


pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut ln_vec: Vec<f64> = Vec::new();
    for num in &b {
        ln_vec.push((*num as f64).abs().ln());
    }
    (b, ln_vec)
}

/*
    * The objective is to know how ownership works with different types; so :
    - For the nbr_function, we are passing an i32, which is a Copy type, so it is copied into the function and we can still use it after the function call.
    - For the str_function, we are passing a String, which is an owned type, so it is moved into the function and we cannot use it after the function call. However, we are returning the original String back from the function, so we can still use it after the function call.
    - For the vec_function, we are passing a Vec<i32>, which is an owned type, so it is moved into the function and we cannot use it after the function call. However, we are returning the original Vec<i32> back from the function, so we can still use it after the function call.
    ? The main point is to understand how ownership works with different types and how it affects the ability to use variables after function calls. In Rust, when you pass an owned type (like String or Vec) to a function, it is moved into the function, and you cannot use it after the function call unless it is returned back from the function. On the other hand, when you pass a Copy type (like i32), it is copied into the function by default, and you can still use it after the function call.
    * Q & A : 
    * Q1 : why is it necessary to do let _ = match x.parse() { ... } instead of just match x.parse() { ... }?
    - A1 : because we need to handle the Result returned by x.parse(), and if we don't assign it to a variable (even if it's just _), the compiler will give us a warning about an unused Result. By using let _ =, we are explicitly ignoring the Result while still handling it properly.
    * Q2 : exp() and ln()?
    - in math is a fascinating number that is approximately equal to 2.71828, and it has some interesting properties. For example, the derivative of exp(x) is exp(x) itself, which means that the function grows at a rate proportional to its current value. This makes it a fundamental function in calculus and many areas of mathematics.
    - as for ln(), it is the inverse function of exp(), which means that ln(exp(x)) = x for all x. It is also used in many areas of mathematics, including calculus, probability theory, and statistics. For example, the natural logarithm is used to model exponential growth and decay processes, such as population growth or radioactive decay. 
    * Q3 : what is development limite de la fonction exp() et ln()?
    - le development limite of a function is the value that the function approaches as the input approaches a certain value. For exp(x), as x approaches infinity, exp(x) approaches infinity, and as x approaches negative infinity, exp(x) approaches 0. For ln(x), as x approaches 0 from the positive side, ln(x) approaches negative infinity, and as x approaches infinity, ln(x) approaches infinity. So the limits of exp() and ln() are as follows:
        - lim x→∞ exp(x) = ∞
        - lim x→-∞ exp(x) = 0
        - lim x→0+ ln(x) = -∞
        - lim x→∞ ln(x) = ∞
    
   
    

*/