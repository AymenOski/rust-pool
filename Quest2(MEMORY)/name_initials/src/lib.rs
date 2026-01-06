pub fn initials(names: Vec<&str>) -> Vec<String> {
     let mut v: Vec<String> = Vec::with_capacity(names.len());

    for name in &names {
        let mut temp = String::new();
        for word in name.split_whitespace() {
            match word.chars().next() {
                Some(c) => {
                    temp.push(c);
                    temp.push('.');
                    temp.push(' ');
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
