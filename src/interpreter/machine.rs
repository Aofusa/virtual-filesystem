use super::ast::AbstractSyntaxTreeNodePointer;
use super::interpreter::InterpreterError;

pub trait Machine {
    fn execute(&mut self, node: &AbstractSyntaxTreeNodePointer) -> Result<i32, InterpreterError>;
}

