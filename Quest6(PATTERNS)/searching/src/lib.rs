// pub fn search(array: &[i32], key: i32) -> Option<usize>{
//     let mut last_index = None;

//     for (i , val) in array.iter().enumerate() {
//         if *val == key {
//             last_index = Some(i);
//             continue
//         }
//     }
//     last_index
// }

pub fn search(array: &[i32], key: i32) -> Option<usize> {
    for i in (0..array.len()).rev() {
        if array[i] == key {
            return Some(i);
        }
    }
    None
}

// If this WERE a String (The UTF-8 Trap)
// The SAFE way for Strings/Characters
// pub fn search_char(text: &str, key: char) -> Option<usize> {
//     text.char_indices().rfind(|&(_, c)| c == key).map(|(i, _)| i)
// }

// or  
// match text.char_indices().rev().find(|&(_, c)| c == key) {
//     Some((i, _)) => Some(i),
//     None => None,
// }

