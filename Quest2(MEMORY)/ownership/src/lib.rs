pub fn first_subword(mut s: String) -> String {
    for (i, c) in s.char_indices().skip(1) {
        if c.is_uppercase() || c == '_' {
            return s[..i].to_string();
        }

    }
    return s
}
