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

    for c in input.chars() {
        if c.is_whitespace() {
            result.push(c);
            counter = 0;
        } else {
            if counter == 0 {
                result.push_str(&c.to_uppercase().collect::<String>());
                counter += 1;
            } else {
                result.push(c);
            }
        }
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