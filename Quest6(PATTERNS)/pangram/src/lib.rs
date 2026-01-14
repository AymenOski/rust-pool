pub fn is_pangram(s: &str) -> bool {
    let lower_s = s.to_lowercase();

    ('a'..='z').all(|c| lower_s.contains(c))
}
