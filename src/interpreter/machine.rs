use super::ast::AbstractSyntaxTreeNodePointer;
use super::interpreter::InterpreterError;


#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    NUM(i32),
    FLOAT(f32),
    STRING(String),
    BYTE(u8),
    VECTOR(Vec<Literal>),
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::NUM(x) => x.fmt(f),
            Literal::FLOAT(x) => x.fmt(f),
            Literal::STRING(x) => x.fmt(f),
            Literal::BYTE(x) => x.fmt(f),
            Literal::VECTOR(x) => x.iter().try_fold((), |_, x| x.fmt(f)),
        }
    }
}


pub trait Machine {
    fn execute(&mut self, node: &AbstractSyntaxTreeNodePointer) -> Result<Literal, InterpreterError>;
}

