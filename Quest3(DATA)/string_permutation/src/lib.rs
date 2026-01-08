pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut v1: Vec<char> = s1.chars().collect();
    let mut v2: Vec<char> = s2.chars().collect();
    
    v1.sort();
    v2.sort();
    
    v1 == v2
}