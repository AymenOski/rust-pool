pub fn get_diamond(c: char) -> Vec<String> {
    let max_index = c as u8 - b'A';
    let mut result = Vec::new();

    for i in 0..=max_index {
        let letter = (b'A' + i) as char;
        let leading = max_index - i;
        let line = if i == 0 {
            format!("{}{}{}", " ".repeat(leading as usize), letter, " ".repeat(leading as usize))
        } else {
            let inner = 2*i - 1;
            format!(
                "{}{}{}{}{}",
                " ".repeat(leading as usize),
                letter,
                " ".repeat(inner as usize),
                letter,
                " ".repeat(leading as usize)
            )
        };
        result.push(line);
    }

    for i in (0..max_index).rev() {
        result.push(result[i as usize].clone());
    }

    result
}
