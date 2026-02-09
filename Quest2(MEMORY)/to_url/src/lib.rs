pub fn to_url(s: &str) -> String {
    s.replace(" ", "%20")
}

// or :

/*
    pub fn to_url(s: &str) -> String {
    let mut result = String::new();

    for ch in s.chars() {
        if ch == ' ' {
            result.push_str("%20");
        }else {
            result.push(ch);
        }
    }
    result
}

*/