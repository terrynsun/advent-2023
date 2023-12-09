pub fn split_to_strings(s: &str) -> Vec<String> {
    s.split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

pub fn split_to_u64s(s: &str) -> Vec<u64> {
    s.split_whitespace()
        .map(|s| s.parse::<_>().unwrap())
        .collect()
}

pub fn split_to_i64s(s: &str) -> Vec<i64> {
    s.split_whitespace()
        .map(|s| s.parse::<_>().unwrap())
        .collect()
}
