
pub type InterpreterResult<'a> = Result<Option<String>, InterpreterError>;

#[derive(Debug, PartialEq)]
pub enum InterpreterError {
    Unknown,
}

fn split_digit(s: &str) -> (&str, &str) {
    let first_non_num = s.find(|c| !char::is_numeric(c)).unwrap_or(s.len());
    s.split_at(first_non_num)
}

fn strtol(p: &mut std::str::Chars) -> i32 {
    let (n, _) = split_digit(p.as_str());
    for _ in 0..n.len() { p.next(); }
    n.parse().unwrap()
}

pub fn interpret(s: &str) -> InterpreterResult {
    let mut p = s.chars();
    let mut stack = strtol(&mut p);

    loop {
        match p.next() {
            None => break,
            Some('+') => stack = stack + strtol(&mut p),
            Some('-') => stack = stack - strtol(&mut p),
            _ => { panic!("unreachable here") }
        }
    }

    Ok(Some(format!("{}", stack)))
}

#[cfg(test)]
mod tests {
    use crate::interpreter::interpreter::interpret;

    #[test]
    fn test_interpreter() {
        assert_eq!(interpret("42"), Ok(Some("42".to_string())));
        assert_eq!(interpret("5+20-4"), Ok(Some("21".to_string())));
    }
}

