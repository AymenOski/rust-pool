pub fn delete_prefix(prefix: &str, s: &str) -> Option<&str> {
    if s.starts_with(prefix) {
        Some(&s[prefix.len()..])
    } else {
        None
    }
}
