pub fn initials(names: Vec<&str>) -> Vec<String> {
     let mut v: Vec<String> = Vec::with_capacity(names.len());

    for name in &names {
        let mut temp = String::new();
        let words: Vec<&str> = name.split_whitespace().collect();
        for word in words {
            match word.chars().next() {
                Some(c) => {
                    temp.push_str(&format!("{}. ", c));
                }
                None => (),
            }
            
        }
        if temp.ends_with(' ') {
            temp.pop();
        }
        v.push(format!("{}",temp));
    }

    v
}
