pub fn split_to_u64s(s: &str) -> Vec<u64> {
    s.trim_end()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}
