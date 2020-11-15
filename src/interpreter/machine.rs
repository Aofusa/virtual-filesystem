use super::ast::AbstractSyntaxTreeNodePointer;


#[derive(Debug, PartialEq)]
pub enum MachineError {
    Unknown,  // エラー内容不明
    CalculationError,  // 演算実行時のエラー
    ZeroStack,  // 演算スタックに何もなかった
    UndefinedFunction,
}


pub trait Machine {
    fn execute(&mut self, node: &AbstractSyntaxTreeNodePointer) -> Result<i32, MachineError>;
}

