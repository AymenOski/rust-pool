pub fn scytale_cipher(message: &str, i: usize) -> String {
    if i == 0 || message.is_empty() {
        return message.to_string();
    }

    let chars: Vec<char> = message.chars().collect();
    let len = chars.len();
    
    let cols = i;
    let rows = (len + cols - 1) / cols;
    
    let mut result = String::new();

    for c in 0..cols {
        for r in 0..rows {
            let index = r * cols + c;
            if index < len {
                result.push(chars[index]);
            } else {
                result.push(' ');
            }
        }
    }
    result.trim_end().to_string()
}