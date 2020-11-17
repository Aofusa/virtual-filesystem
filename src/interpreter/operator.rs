use super::interpreter::InterpreterError;
use super::machine::Literal;


pub fn add(stack: &mut Vec<Literal>) -> Result<Literal, InterpreterError> {
    let a = stack.pop().ok_or(InterpreterError::ZeroStack)?;
    let b = stack.pop().ok_or(InterpreterError::ZeroStack)?;

    match (a, b) {
        (Literal::NUM(x), Literal::NUM(y)) => {
            let s = Literal::NUM(y + x);
            stack.push(s.clone());
            Ok(s)
        },
        _ => Err(InterpreterError::TypeMismatch)
    }
}


pub fn sub(stack: &mut Vec<Literal>) -> Result<Literal, InterpreterError> {
    let a = stack.pop().ok_or(InterpreterError::ZeroStack)?;
    let b = stack.pop().ok_or(InterpreterError::ZeroStack)?;

    match (a, b) {
        (Literal::NUM(x), Literal::NUM(y)) => {
            let s = Literal::NUM(y - x);
            stack.push(s.clone());
            Ok(s)
        },
        _ => Err(InterpreterError::TypeMismatch)
    }
}


pub fn mul(stack: &mut Vec<Literal>) -> Result<Literal, InterpreterError> {
    let a = stack.pop().ok_or(InterpreterError::ZeroStack)?;
    let b = stack.pop().ok_or(InterpreterError::ZeroStack)?;

    match (a, b) {
        (Literal::NUM(x), Literal::NUM(y)) => {
            let s = Literal::NUM(y * x);
            stack.push(s.clone());
            Ok(s)
        },
        _ => Err(InterpreterError::TypeMismatch)
    }
}


pub fn div(stack: &mut Vec<Literal>) -> Result<Literal, InterpreterError> {
    let a = stack.pop().ok_or(InterpreterError::ZeroStack)?;
    let b = stack.pop().ok_or(InterpreterError::ZeroStack)?;

    match (a, b) {
        (Literal::NUM(x), Literal::NUM(y)) => {
            let s = Literal::NUM(y / x);
            stack.push(s.clone());
            Ok(s)
        },
        _ => Err(InterpreterError::TypeMismatch)
    }
}

