
pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();
    let mut skip_next = false;
    let mut n = 0;

    for (i , c) in s.char_indices() {
        if n > 0 && c != '+'{
            n -= 1;
            continue;
        }
        if skip_next && c == '+' {
            n += 1;
            continue;
        }
        if skip_next && c != '+'{
            skip_next = false;
            continue;
        }

        match c {
            '-' => {
                result.pop();
            }
            '+' => {
                skip_next = true;
            }
            _ => {
                result.push(c);
            }
        }
    }

    *s = result;
}

pub fn do_operations(v: &mut [String]) {
    for x in v.iter_mut() {
        if x.contains('+') {
            let parts: Vec<&str> = x.split('+').collect();
            let a: i32 = parts[0].parse().unwrap();
            let b: i32 = parts[1].parse().unwrap();
            *x = (a + b).to_string();
        } else if x.contains('-') {
            let parts: Vec<&str> = x.split('-').collect();
            let a: i32 = parts[0].parse().unwrap();
            let b: i32 = parts[1].parse().unwrap();
            *x = (a - b).to_string();
        }
    }
}