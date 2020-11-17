use super::interpreter::InterpreterError;


pub fn add(stack: &mut Vec<i32>) -> Result<i32, InterpreterError> {
    let a = stack.pop().ok_or(InterpreterError::ZeroStack)?;
    let b = stack.pop().ok_or(InterpreterError::ZeroStack)?;

    let x = b + a;
    stack.push(x);

    Ok(x)
}


pub fn sub(stack: &mut Vec<i32>) -> Result<i32, InterpreterError> {
    let a = stack.pop().ok_or(InterpreterError::ZeroStack)?;
    let b = stack.pop().ok_or(InterpreterError::ZeroStack)?;

    let x = b - a;
    stack.push(x);

    Ok(x)
}


pub fn mul(stack: &mut Vec<i32>) -> Result<i32, InterpreterError> {
    let a = stack.pop().ok_or(InterpreterError::ZeroStack)?;
    let b = stack.pop().ok_or(InterpreterError::ZeroStack)?;

    let x = b * a;
    stack.push(x);

    Ok(x)
}


pub fn div(stack: &mut Vec<i32>) -> Result<i32, InterpreterError> {
    let a = stack.pop().ok_or(InterpreterError::ZeroStack)?;
    let b = stack.pop().ok_or(InterpreterError::ZeroStack)?;

    let x = b / a;
    stack.push(x);

    Ok(x)
}

