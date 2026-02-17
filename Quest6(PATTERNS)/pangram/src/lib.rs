// pub fn is_pangram(s: &str) -> bool {
//     let lower_s = s.to_lowercase();

//     ('a'..='z').all(|c| lower_s.contains(c))
// }

use std::collections::HashMap;

pub fn is_pangram(s: &str) -> bool {
    let mut used:HashMap<char,bool> = HashMap::new();
    

    for ch in s.chars() {
        if ch.is_ascii_alphabetic(){
            // let _: () = x;
            used.insert(ch.to_ascii_lowercase(), true);
        }
    }
    used.len() >= 26
}
