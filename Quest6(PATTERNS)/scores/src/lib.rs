// pub fn score(word: &str) -> u64 {
//     word.chars()
//         .map(|x| match x.to_ascii_lowercase() {
//             'a' | 'e' | 'i' | 'o' | 'u' | 'l' | 'n' | 'r' | 's' | 't' => 1,
//             'd' | 'g' => 2,
//             'b' | 'c' | 'm' | 'p' => 3,
//             'f' | 'h' | 'v' | 'w' | 'y' => 4,
//             'k' => 5,
//             'j' | 'x' => 8,
//             'q' | 'z' => 10,
//             _ => 0,
//         })
//         .sum()
// }

// &str "Hello"
//    ↓
// .chars()               → Iterator<Item = char>
//    ↓
// .map(|x| match ...)    → Iterator<Item = u64>
//    ↓
// .sum()                 → u64

// or :
pub fn score(s: &str) -> u64 {
    let mut total = 0;

    for c in s.chars() {
        if c.is_ascii_alphabetic() {
            let ch = c.to_ascii_uppercase();

            total += match ch {
                'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => 1,
                'D' | 'G' => 2,
                'B' | 'C' | 'M' | 'P' => 3,
                'F' | 'H' | 'V' | 'W' | 'Y' => 4,
                'K' => 5,
                'J' | 'X' => 8,
                'Q' | 'Z' => 10,
                _ => 0,
            };
        }
    }

    total
}