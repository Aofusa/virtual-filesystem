use super::ast::AbstructSyntaxTreeNodePointer;


#[derive(Debug, PartialEq)]
pub enum MachineError {
    Unknown,  // エラー内容不明
    CalculationError,  // 演算実行時のエラー
    ZeroStack,  // 演算スタックに何もなかった
}


pub trait Machine {
    fn execute(&mut self, node: &AbstructSyntaxTreeNodePointer) -> Result<i32, MachineError>;
}

