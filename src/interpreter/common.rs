pub fn split_digit(s: &str) -> (&str, &str) {
    let first_non_num = s.find(|c| !char::is_numeric(c)).unwrap_or(s.len());
    s.split_at(first_non_num)
}

pub fn split_alphanumeric(s: &str) -> (&str, &str) {
    let first_non_alphanumeric = s.find(|c| !char::is_numeric(c)).unwrap_or(s.len());
    s.split_at(first_non_alphanumeric)
}

