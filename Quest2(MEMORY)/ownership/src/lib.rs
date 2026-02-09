//* recommendation (⚠️): use `Colorful Comments` extension for better readability of the comments in this file

pub fn first_subword(mut s: String) -> String {
    for (i, c) in s.char_indices().skip(1) {
        if c.is_uppercase() || c == '_' {
            return s[..i].to_string();
        }

    }
    return s
}

/*
    * Q & A:
    * Q1: what does s.char_indices() and skip(1) do?
    - s.char_indices() returns an iterator that yields pairs of (index, character) for each character in the string. The skip(1) method is used to skip the first character because we want to start checking from the second character onwards. This is because the first character can be uppercase or an underscore, and we want to ignore that case when determining the first subword.
    * Q2: why do we return s[..i].to_string() instead of just s[..i]?
    - s[..i] returns a string slice (&str) that represents the substring from the beginning of the string up to index i. However, the function signature specifies that we need to return a String, which is an owned type. Therefore, we call to_string() on the string slice to convert it into a String before returning it.
*/
