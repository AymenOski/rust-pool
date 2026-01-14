use std::collections::HashMap;

pub fn score(text: &str) -> u32 {
    let mut scores = HashMap::new();
    for c in "aeioulnrst".chars() {
        scores.insert(c, 1);
    }
    for c in "dg".chars() {
        scores.insert(c, 2);
    }
    for c in "bcmp".chars() {
        scores.insert(c, 3);
    }
    for c in "fhvwy".chars() {
        scores.insert(c, 4);
    }
    scores.insert('k', 5);
    for c in "jx".chars() {
        scores.insert(c, 8);
    }
    for c in "qz".chars() {
        scores.insert(c, 10);
    }
    text.to_lowercase()
        .chars()
        .filter_map(|c| scores.get(&c))
        .sum::<u32>()
    //or 
    // let mut total = 0;
    // for c in text.to_lowercase().chars() {
    //     if let Some(value) = scores.get(&c) {
    //         total += value;
    //     }
    // }
    // return total;
}
