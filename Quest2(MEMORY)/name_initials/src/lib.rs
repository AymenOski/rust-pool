//* recommendation (⚠️): use `Colorful Comments` extension for better readability of the comments in this file

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

/*  
    * Q & A :
    * Q1: Why do we iterate over `&names` instead of `names`?
    - A1: We iterate over `&names` to borrow each name as a reference instead of taking ownership. This allows us to use the original `names` vector after the loop if needed, and it avoids unnecessary cloning of the strings. By using `&names`, we can read the data without modifying it, which is more efficient and adheres to Rust's ownership principles.
    * Q2: Why do we use `String::new()` to create a temporary string instead of directly pushing to `v`?
    - A2: We use `String::new()` to create a temporary string because we need to build the initials for each name before pushing it to the vector `v`. This allows us to construct the initials in a mutable string, which we can modify by pushing characters and formatting it as needed. Once we have the complete initials string, we can then push it to the vector `v` in a single operation, which is more efficient than pushing each character separately to the vector.
    * Q3: What does word.chars().next() do in this context?
    - A3: The expression `word.chars().next()` retrieves the first character of the `word` string. The `chars()` method returns an iterator over the characters of the string, and `next()` returns the next item from the iterator, which in this case is the first character. This is used to extract the initial letter of each word in the name, which is then formatted as part of the initials string.
    * Q4: Why does rust enforce this iterator pattern instead of allowing direct indexing like `word[0]`?
    - A4: Rust enforces the iterator pattern for strings because of the way strings are represented in memory. Rust strings are UTF-8 encoded, which means that characters can be of variable length (1 to 4 bytes). Direct indexing like `word[0]` would not work because it would attempt to access a byte rather than a character, which could lead to invalid UTF-8 sequences and potential bugs. By using iterators, Rust ensures that you are working with valid characters and prevents out-of-bounds access, thus maintaining memory safety and correctness when handling strings.
*/