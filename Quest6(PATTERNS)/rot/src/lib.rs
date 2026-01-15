pub fn rotate(input: &str, key: i8) -> String {
    let shift = ((key % 26) + 26) % 26;

    input
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                let rotated =
                    (c as u8 - b'a' + shift as u8) % 26 + b'a';
                rotated as char
            } else if c.is_ascii_uppercase() {
                let rotated =
                    (c as u8 - b'A' + shift as u8) % 26 + b'A';
                rotated as char
            } else {
                c
            }
        })
        .collect()
}
