
pub type InterpreterResult<'a> = Result<Option<&'a str>, InterpreterError>;

#[derive(Debug, PartialEq)]
pub enum InterpreterError {
    Unknown,
}

pub fn interpret<'a>(s: &'a str) -> InterpreterResult<'a> {
    Ok(Some(s))
}

#[cfg(test)]
mod test {
    use crate::interpreter::interpreter::interpret;

    #[test]
    fn test_interpreter() {
        let s = "any";

        assert_eq!(interpret(s), Ok(Some(s)));
    }
}

