pub fn edit_distance(source: &str, target: &str) -> usize {
    let n = source.chars().count();
    let m = target.chars().count();

    // Handle empty strings early
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
            let cost = if source_chars[i - 1] == target_chars[j - 1] { 0 } else { 1 };

            dp[i][j] = (dp[i - 1][j] + 1)      // deletion
                .min(dp[i][j - 1] + 1)         // insertion
                .min(dp[i - 1][j - 1] + cost); // substitution
        }
    }

    dp[n][m]
}