pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    let mut result = Vec::new();

    // Split by spaces
    for part in s.split_whitespace() {
        let val: u32;
        
        if part.ends_with('k') {
            let num_str = &part[..part.len() - 1];
            let num: f64 = num_str.parse().unwrap_or(0.0);
            val = (num * 1000.0) as u32;
        } else {
            // Normal number
            val = part.parse().unwrap_or(0);
        }

        result.push(Box::new(val));
    }
    
    result
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    a.into_iter().map(|boxed| *boxed).collect()
}
