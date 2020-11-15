use super::machine::MachineError;


pub fn add(stack: &mut Vec<i32>) -> Result<i32, MachineError> {
    let a: i32;
    let b: i32;

    match stack.pop() {
        Some(x) => a = x,
        None => return Err(MachineError::ZeroStack),
    }
    match stack.pop() {
        Some(x) => b = x,
        None => return Err(MachineError::ZeroStack),
    }

    let x = b + a;
    stack.push(x);

    Ok(x)
}


pub fn sub(stack: &mut Vec<i32>) -> Result<i32, MachineError> {
    let a: i32;
    let b: i32;

    match stack.pop() {
        Some(x) => a = x,
        None => return Err(MachineError::ZeroStack),
    }
    match stack.pop() {
        Some(x) => b = x,
        None => return Err(MachineError::ZeroStack),
    }

    let x = b - a;
    stack.push(x);

    Ok(x)
}


pub fn mul(stack: &mut Vec<i32>) -> Result<i32, MachineError> {
    let a: i32;
    let b: i32;

    match stack.pop() {
        Some(x) => a = x,
        None => return Err(MachineError::ZeroStack),
    }
    match stack.pop() {
        Some(x) => b = x,
        None => return Err(MachineError::ZeroStack),
    }

    let x = b * a;
    stack.push(x);

    Ok(x)
}


pub fn div(stack: &mut Vec<i32>) -> Result<i32, MachineError> {
    let a: i32;
    let b: i32;

    match stack.pop() {
        Some(x) => a = x,
        None => return Err(MachineError::ZeroStack),
    }
    match stack.pop() {
        Some(x) => b = x,
        None => return Err(MachineError::ZeroStack),
    }

    let x = b / a;
    stack.push(x);

    Ok(x)
}

