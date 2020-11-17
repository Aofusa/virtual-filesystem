pub fn split_digit(s: &str) -> (&str, &str) {
    let first_non_num = s.find(|c| !char::is_numeric(c)).unwrap_or(s.len());
    s.split_at(first_non_num)
}

pub fn split_alphanumeric(s: &str) -> (&str, &str) {
    let first_non_alphanumeric = s.find(|c| !char::is_alphanumeric(c)).unwrap_or(s.len());
    s.split_at(first_non_alphanumeric)
}

pub fn split_specific<'a>(s: &'a str, t: &'a str) -> (&'a str, &'a str) {
    let first_non_specific = s.find(|c| t.find(c).is_some()).unwrap_or(s.len());
    s.split_at(first_non_specific)
}

