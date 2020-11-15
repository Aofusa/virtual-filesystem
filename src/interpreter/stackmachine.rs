use crate::utils::logger::{LoggerRepository, LoggerInteractor, DefaultLoggerRepository};
use super::machine::{Machine, MachineError};
use super::ast::{AbstractSyntaxTreeKind, AbstractSyntaxTreeNodePointer};


#[derive(Debug)]
pub struct StackMachine<T>
where
    T: LoggerRepository + Clone
{
    stack: Vec<i32>,  // 演算スタック
    logger: LoggerInteractor<T>,
}


impl StackMachine<DefaultLoggerRepository> {
    #[allow(dead_code)]
    pub fn init() -> StackMachine<DefaultLoggerRepository> {
        StackMachine::init_with_logger(DefaultLoggerRepository{})
    }
}

impl<T: LoggerRepository + Clone> StackMachine<T> {
    fn new(logger: T) -> StackMachine<T> {
        StackMachine {
            stack: vec![],
            logger: LoggerInteractor::new(logger),
        }
    }

    pub fn init_with_logger(logger: T) -> StackMachine<T> {
        StackMachine::new(logger)
    }
}

impl<T: LoggerRepository + Clone> Machine for StackMachine<T> {
    fn execute(&mut self, node: &AbstractSyntaxTreeNodePointer) -> Result<i32, MachineError> {
        {
            // 終端ノードであれば値を返して再帰から復帰していく
            let n = node.borrow();
            if let AbstractSyntaxTreeKind::NUM(x) = n.0 {
                self.stack.push(x);
                return Ok(x);
            }
        }

        {
            // 左右の枝を計算する
            let n = node.borrow();
            let mut iter = n.1.iter();

            // 左辺の抽象構文木の計算
            if let Some(x) = iter.next() {
                if let Err(e) = self.execute(x) {
                    return Err(e);
                }
            }

            // 右辺の抽象構文木の計算
            if let Some(x) = iter.next() {
                if let Err(e) = self.execute(x) {
                    return Err(e);
                }
            }
        }

        {
            // 演算子だった場合スタックの内容を使い計算を行う
            let n = node.borrow();
            match &n.0 {
                AbstractSyntaxTreeKind::ADD => {
                    let a: i32;
                    let b: i32;

                    match self.stack.pop() {
                        Some(x) => a = x,
                        None => return Err(MachineError::ZeroStack),
                    }
                    match self.stack.pop() {
                        Some(x) => b = x,
                        None => return Err(MachineError::ZeroStack),
                    }

                    let x = b + a;
                    self.stack.push(x);
                },
                AbstractSyntaxTreeKind::SUB => {
                    let a: i32;
                    let b: i32;

                    match self.stack.pop() {
                        Some(x) => a = x,
                        None => return Err(MachineError::ZeroStack),
                    }
                    match self.stack.pop() {
                        Some(x) => b = x,
                        None => return Err(MachineError::ZeroStack),
                    }

                    let x = b - a;
                    self.stack.push(x);
                },
                AbstractSyntaxTreeKind::MUL => {
                    let a: i32;
                    let b: i32;

                    match self.stack.pop() {
                        Some(x) => a = x,
                        None => return Err(MachineError::ZeroStack),
                    }
                    match self.stack.pop() {
                        Some(x) => b = x,
                        None => return Err(MachineError::ZeroStack),
                    }

                    let x = b * a;
                    self.stack.push(x);
                },
                AbstractSyntaxTreeKind::DIV => {
                    let a: i32;
                    let b: i32;

                    match self.stack.pop() {
                        Some(x) => a = x,
                        None => return Err(MachineError::ZeroStack),
                    }
                    match self.stack.pop() {
                        Some(x) => b = x,
                        None => return Err(MachineError::ZeroStack),
                    }

                    let x = b / a;
                    self.stack.push(x);
                },
                _otherwise => { return Err(MachineError::CalculationError) },
            }
        }

        // スタックの一番上の情報を返却し終了する
        match self.stack.last() {
            Some(x) => Ok(*x),
            None => Err(MachineError::ZeroStack)
        }
    }
}

