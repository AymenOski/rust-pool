pub fn num_to_ordinal(x: u32) -> String {
    let last_two = x % 100;
    if last_two == 11 || last_two == 12 || last_two == 13 {
        return format!("{}th", x);
    }

    let suffix = match x % 10 {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    };

    format!("{}{}", x, suffix)
}