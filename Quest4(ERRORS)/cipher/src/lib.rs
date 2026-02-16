#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let expected: String= original.chars().map(|ch| {
        if ch.is_ascii_uppercase() {
            ('Z' as u8 - ch as u8 + 'A' as u8) as char
        }else if ch.is_ascii_lowercase() {
            ('z' as u8 - ch as u8 + 'a' as u8) as char
        }else {
            ch
        }
    }).collect();
    
    if expected == ciphered {
        Ok(())
    }else {
        Err(CipherError{ expected: expected })
    }
}
/*
    * Q & A :
    * Q1 : we said before that the map function is used on collections such as Vec, but here we are using it on an iterator, how is that possible?
    A1 : the map function is actually defined on the Iterator trait also, so it can be used on any type that implements the Iterator trait, including collections like Vec, as well as iterators created from those collections. When we call .chars() on a string, it returns an iterator over the characters of the string, and we can use .map() to transform each character in that iterator.
    * Q2 : Why do we use as u8 and not for example as u16 or as u32?
    A2 : We use as u8 because we are working with ASCII characters, which are represented as 8-bit values (0-255). Using as u8 allows us to perform the necessary arithmetic operations on the character codes without worrying about larger integer types. If we were to use as u16 or as u32, it would still work, but it would be less efficient since we would be using more memory than necessary for the character codes. Additionally, since we are only dealing with ASCII characters, using as u8 is sufficient and more appropriate for our needs.
*/