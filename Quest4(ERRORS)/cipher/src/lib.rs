#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let expected: String = original.chars().map(|ch| {
        if ch.is_lowercase(){
            ('z' as u8 - ch as u8 + 'a' as u8) as char
        }else if ch.is_uppercase(){
            ('Z' as u8 - ch as u8 + 'A' as u8) as char 
        }else {
            ch
        }
    }).collect();

    if expected == ciphered {
        Ok(())
    }else {
        Err(CipherError{ expected })
    }

}