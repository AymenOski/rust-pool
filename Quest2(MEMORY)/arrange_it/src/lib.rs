pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<&str> = phrase.split_whitespace().collect();

    // Sort the words by the first digit found in each slice.
    words.sort_by_key(|word| {
        word.chars()
            .find_map(|c| c.to_digit(10))
            .unwrap_or(0)
    });

    // Pre-allocate the result string to minimize reallocations.
    let mut result = String::with_capacity(phrase.len());

    for (i, word) in words.iter().enumerate() {
        if i > 0 {
            result.push(' ');
        }
        // Filter out digits and push directly to the pre-allocated string.
        for c in word.chars() {
            if !c.is_ascii_digit() {
                result.push(c);
            }
        }
    }

    result
}
