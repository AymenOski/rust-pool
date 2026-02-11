pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<&str> = phrase.split_whitespace().collect();

    // Sort the words by the first digit found in each slice.
    // section 1 :
    words.sort_by_key(|word| {
        println!("{}", word);
        word.chars()
            .find_map(|c| c.to_digit(10))
            .unwrap_or(0)
    });

    // Pre-allocate the result string to minimize reallocations.
    let mut result = String::with_capacity(phrase.len());

    for (i, word) in words.iter().enumerate() {
        if i > 0 {
            result.push(' ');
        }
        // Filter out digits and push directly to the pre-allocated string.
        for c in word.chars() {
            if !c.is_ascii_digit() {
                result.push(c);
            }
        }
    }

    result
}

/*
    * Q1 : what does section 1 in the code does step by step?
    - A1 : 
        1. Split the input phrase into words using whitespace as the delimiter.
        2. Collect the words into a vector of string slices.
        3. Sort the vector of words based on the first digit found in each word. -> 
            - For each word, it iterates through the characters to find the first digit.
            - If a digit is found, it uses that digit as the key for sorting. -> what we mean by `key` is that the sorting algorithm will use the value of the first digit found in the word to determine the order of the words in the sorted list. and this part is done by the `find_map` method which returns the first digit found in the word as a `u32` value.
            - If no digit is found, it defaults to 0, meaning such words will be sorted before any words with digits.
            and the 10 is for the base of the number system, which is used to convert the character to a digit. since we are dealing with decimal digits (0-9), we use base 10.
    * Q2 : what is Char<'_> type in Rust?
    - A2 : `Char<'_>` is a type in Rust that represents a single Unicode scalar value. It is a primitive type that can hold any valid Unicode character, including letters, digits, symbols, and emojis. The `'_` part is a lifetime annotation that indicates that the character can live for any lifetime, meaning it is not tied to a specific scope or duration. This allows the `Char` type to be used flexibly in various contexts without worrying about ownership or borrowing issues related to lifetimes. In the context of the code snippet, `c` is a variable of type `Char<'_>`, which means it can hold any single Unicode character found in the word being processed. what do we mean by having a limited lifetime in Rust?
    - A3 : In Rust, a lifetime is a construct that the compiler uses to ensure memory safety without needing a garbage collector. A lifetime specifies how long a reference is valid. When we say that a variable has a limited lifetime, it means that the reference to that variable is only valid for a certain scope or duration. If you try to use a reference after its lifetime has ended, the Rust compiler will throw an error to prevent potential memory safety issues, such as dangling references or use-after-free errors. In the context of the `Char<'_>` type, the lifetime annotation `'_` indicates that the character can be used in any context without being tied to a specific lifetime, and this is possible because `Char` is a simple value type that does not involve references to other data, so it does not require a specific lifetime to ensure memory safety, so why its type have a lifetime anyway?
    & A4 :  In this case, since we are only working with the characters of the word, we do not have any references that require lifetimes, so the `Char<'_>` type does not need to specify a lifetime. The `'_` is just a placeholder to indicate that the character can be used in any context without being tied to a specific lifetime, but it is not actually necessary for the `Char` type itself. It is more of a convention to indicate that the character can be used flexibly without worrying about lifetimes, even though in this specific case, it does not have any references that would require lifetimes.
*/