//* recommendation (⚠️): use `Colorful Comments` extension for better readability of the comments in this file7

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

/*
    * Q1 : is char_indices() same as chars().enumerate() ?
    - A1 : No, char_indices() returns the byte index of the character, while chars().enumerate() returns the character index. This means that char_indices() can be used to access the original string using the byte index, while chars().enumerate() cannot be used to access the original string directly.
    * Q2 : what does the `*` operator do in the context of `*s = result;` and `*x = (a + b).to_string();` ?
    - A2 : The `*` operator is used to dereference a reference. In the context of `*s = result;`, it means that we are assigning the value of `result` to the value that `s` is referencing. In the context of `*x = (a + b).to_string();`, it means that we are assigning the result of the addition to the value that `x` is referencing. In both cases, we are modifying the original data that the reference points to.
    * Q3 : what are all the types of iterations we can do on a string in Rust ?
    - A3 : In Rust, you can iterate over a string in several ways:
        1. Using `chars()`: This method returns an iterator over the characters of the string. Each item is a `char`, which represents a Unicode scalar value.
        2. Using `bytes()`: This method returns an iterator over the bytes of the string. Each item is a `u8`, which represents a single byte of the string.
        3. Using `char_indices()`: This method returns an iterator over the characters of the string along with their byte indices. Each item is a tuple containing the byte index and the corresponding `char`.
        &4. Using `lines()`: This method returns an iterator over the lines of the string, splitting it at newline characters. Each item is a `&str` representing a line of the string.
        5. Using `split()`: This method allows you to split the string based on a specified delimiter and returns an iterator over the resulting substrings. Each item is a `&str` representing a substring of the original string.
        6. Using `split_whitespace()`: This method splits the string based on whitespace and returns an iterator over the resulting substrings. Each item is a `&str` representing a substring of the original string that was separated by whitespace.
        8. Using `splitn()`: This method splits the string into a specified number of substrings based on a specified delimiter. Each item is a `&str` representing a substring of the original string that was separated by the specified delimiter, up to the specified number of substrings.
    * Q4 : what are all the types of iterations we can do on a vector in Rust ?
    - A4 : In Rust, you can iterate over a vector in several ways:
        1. Using `iter()`: This method returns an iterator that yields references to the elements of the vector. Each item is a reference to an element in the vector.
        2. Using `iter_mut()`: This method returns an iterator that yields mutable references to the elements of the vector. Each item is a mutable reference to an element in the vector, allowing you to modify the elements while iterating.
        &3. Using `into_iter()`: This method consumes the vector and returns an iterator that yields the elements by value. Each item is the actual element from the vector, and the vector is no longer usable after calling this method.
        4. Using `enumerate()`: This method can be used in combination with any of the above iterators to yield both the index and the value of each element in the vector. Each item is a tuple containing the index and the corresponding element from the vector.
    * Q5 : what are all the types of iterations we can do on an array in Rust ?
    - A5 : In Rust, you can iterate over an array in several ways:
        1. Using `iter()`: This method returns an iterator that yields references to the elements of the array. Each item is a reference to an element in the array.
        2. Using `iter_mut()`: This method returns an iterator that yields mutable references to the elements of the array. Each item is a mutable reference to an element in the array, allowing you to modify the elements while iterating.
        &3. Using `into_iter()`: This method consumes the array and returns an iterator that yields the elements by value. Each item is the actual element from the array, and the array is no longer usable after calling this method.
        4. Using `enumerate()`: This method can be used in combination with any of the above iterators to yield
*/