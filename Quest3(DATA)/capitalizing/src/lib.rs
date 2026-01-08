pub fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(), 
    }
}

pub fn title_case(input: &str) -> String {
    let mut result = String::new();
    let mut counter = 0;

    for word in input.split_whitespace() {
        for c in word.chars() {
            if counter == 0 {
                result.push_str(&c.to_uppercase().to_string());
                counter += 1;
            }else {
                result.push(c);
            }
        }    
        counter = 0;
        result.push(' ');
    }
    result
}

pub fn change_case(input: &str) -> String {
    input
    .chars()
    .map(|c| {
        if c.is_uppercase() {
            c.to_lowercase().to_string()
        }else if c.is_lowercase() {
            c.to_uppercase().to_string()
        } else {
            c.to_string()
        }
    })
    .collect::<String>()

}

// or : a more effecient way
// pub fn change_case(input: &str) -> String {
//     input
//         .chars()
//         .map(|c| {
//             if c.is_lowercase() {
//                 c.to_uppercase().next().unwrap()
//             } else if c.is_uppercase() {
//                 c.to_lowercase().next().unwrap()
//             } else {
//                 c  // leave punctuation/symbols alone
//             }
//         })
//         .collect()
// }

// !! both last functions should use c.to_uppercase().next().unwrap() combo to prevent memory allocation issues