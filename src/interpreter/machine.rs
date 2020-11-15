use super::interpreter::InterpreterError;
use super::ast::AbstructSyntaxTreeNodePointer;

pub trait Machine {
    fn execute(&mut self, node: &AbstructSyntaxTreeNodePointer) -> Result<i32, InterpreterError>;
}

