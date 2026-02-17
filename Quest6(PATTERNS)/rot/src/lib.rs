pub fn rotate(input: &str, key: i8) -> String {
    let shift = ((key % 26) + 26) % 26; // Handle negative shifts
    /*
    explaination of the shift calculation:
    example of key = -1:
    -1 % 26 = -1
    -1 + 26 = 25
    25 % 26 = 25
    example of key = 27:
    27 % 26 = 1
    1 + 26 = 27
    27 % 26 = 1
    */

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
