//* recommendation (⚠️): use `Colorful Comments` extension for better readability of the comments in this file7

pub fn edit_distance(source: &str, target: &str) -> usize {
    let n = source.chars().count();
    let m = target.chars().count();

    // Handle empty strings early
    if n == 0 && m == 0 { return 0; }
    if n == 0 { return m; }
    if m == 0 { return n; }

    // Create DP table: (n+1) rows x (m+1) columns
    let mut dp = vec![vec![0; m + 1]; n + 1];

    // Initialize first row and column
    for i in 0..=n {
        dp[i][0] = i;  // delete i chars
    }
    for j in 0..=m {
        dp[0][j] = j;  // insert j chars
    }

    // Fill the table
    let source_chars: Vec<char> = source.chars().collect();
    let target_chars: Vec<char> = target.chars().collect();

    for i in 1..=n {
        for j in 1..=m {
            let cost = if source_chars[i - 1] == target_chars[j - 1] { 
                0
            } else { 
                1
            };

            dp[i][j] = (dp[i - 1][j] + 1)      // deletion
                .min(dp[i][j - 1] + 1)         // insertion
                .min(dp[i - 1][j - 1] + cost); // substitution
        }
    }

    dp[n][m]
}

/*
    * Q & A: 
    * Q1 : what is the algorithm name?
    -A1 : The algorithm is called the Levenshtein distance, named after the Russian scientist Vladimir Levenshtein who introduced it in 1965. He developed this algorithm to measure the difference between two strings, which has applications in various fields such as spell checking, DNA sequencing, and natural language processing.
    * Q2 : inventor story?
    -A2 : Vladimir Levenshtein was a Russian scientist and mathematician who made significant contributions to the field of information theory and coding theory. He introduced the concept of edit distance in his 1965 paper "Binary Codes Capable of Correcting Deletions, Insertions, and Reversals." The algorithm was designed to measure the difference between two strings by counting the minimum number of operations (insertions, deletions, substitutions) required to transform one string into another. Levenshtein's work has had a lasting impact on various fields, including computer science, linguistics, and bioinformatics, where edit distance is widely used for tasks such as spell checking, DNA sequence analysis, and natural language processing.
    * Q3 : how did Levenshtein come up with the idea of edit distance? 
    - A3 : Levenshtein's motivation for developing the concept of edit distance likely stemmed from his work in information theory and coding theory. He was interested in understanding how to measure the similarity between strings, which is crucial for error detection and correction in communication systems. The idea of counting the minimum number of operations needed to transform one string into another provided a systematic way to quantify the difference between strings, which could be applied to various problems in computer science and linguistics. By formalizing this concept, Levenshtein created a powerful tool that has since been widely adopted in many fields for tasks such as spell checking, DNA sequence analysis, and natural language processing.
        Operations in Levenshtein distance are:
        Insertion: Adding a character to string A.
        Deletion: Removing a character from string A.
        Replacement: Replacing a character in string A with another character.
    * Q4 : how does the algorithm work in our code and ?
    - A4 : The algorithm works by creating a dynamic programming table (2D array) where the entry at position (i, j) represents the edit distance between the first i characters of the source string and the first j characters of the target string. The table is filled in a bottom-up manner, starting from the base cases where one of the strings is empty. The algorithm considers three possible operations for each character comparison: insertion, deletion, and substitution. The cost of each operation is calculated, and the minimum cost is stored in the table. Finally, the value at the bottom-right corner of the table gives the edit distance between the two strings. This approach ensures that all possible transformations are considered efficiently, resulting in a time complexity of O(n*m), where n and m are the lengths of the source and target strings, respectively.
    * Q5 : how does the algorithm being implemented in our code?
    - A5 :
        -first of all we need to handle the edge cases where one of the strings is empty. 
        - then we create a 2D vector (dp) to store the edit distances between substrings of the source and target strings. The dimensions of this table are (n+1) x (m+1), where n and m are the lengths of the source and target strings, respectively.

        
*/