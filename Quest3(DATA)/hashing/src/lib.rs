pub fn mean(list: &[i32]) -> f64 {
    if list.is_empty() {
        return 0.0;
    }
    let sum: i32 = list.iter().sum();
    sum as f64 / list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    let mut sorted = list.to_vec();
    sorted.sort();

    let mid = sorted.len() / 2;

    if sorted.len() % 2 == 0 {
        let a = sorted[mid - 1];
        let b = sorted[mid];
        (a + b) / 2
    } else {
        sorted[mid]
    }
}

pub fn mode(list: &[i32]) -> i32 {
    if list.is_empty() {
        return 0;
    }

    let mut freq = std::collections::HashMap::new();

    for &num in list {
        *freq.entry(num).or_insert(0) += 1;
    }

    let mut max_count = 0;
    let mut mode_val = list[0];

    for (&num, &count) in &freq {
        if count > max_count {
            max_count = count;
            mode_val = num;
        }
    }

    mode_val
}