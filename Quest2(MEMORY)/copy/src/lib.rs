pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c as f64).abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let mut str = String::new();
    for x in a.split_whitespace() {
        let n: i32 = match x.parse() {
            Ok(num) => {
                let exp_n = (num as f64).exp();
                str.push_str(&format!("{}", exp_n));
                str.push(' ');
                num
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
