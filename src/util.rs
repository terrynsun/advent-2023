use std::str::FromStr;

pub fn split_to_strings(s: &str) -> Vec<String> {
    s.split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

pub fn split_parse<T>(s: &str) -> Vec<T>
where
    T: FromStr, <T as std::str::FromStr>::Err: std::fmt::Debug
{
    s.split_whitespace()
        .map(|s| s.parse::<T>().unwrap())
        .collect()
}
