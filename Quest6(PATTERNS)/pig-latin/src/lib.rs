pub fn pig_latin(text: &str) -> String {
    let mut result = Vec::new();
    let vowels = "aeiou";

    for word in text.split_whitespace() {
        let mut word = word.to_string();
        let chars: Vec<char> = word.chars().collect();

        if let Some(first_char) = chars.first() {
            if vowels.contains(*first_char) {
                word.push_str("ay");
                result.push(word);
                continue;
            }
        }

        if chars.len() >= 3 {
            let first = chars[0];
            let second = chars[1];
            let third = chars[2];
            if !vowels.contains(first) && second == 'q' && third == 'u' {
                let rest: String = chars[3..].iter().collect();
                let prefix: String = chars[0..3].iter().collect();
                word = format!("{}{}ay", rest, prefix);
                result.push(word);
                continue;
            }
        }

        let mut idx = 0;
        for (i, c) in chars.iter().enumerate() {
            if vowels.contains(*c) {
                idx = i;
                break;
            }
        }

        if idx > 0 {
            let (head, tail) = word.split_at(idx);
            word = format!("{}{}ay", tail, head);
        } else {
            word.push_str("ay");
        }

        result.push(word);
    }

    result.join(" ")
}
