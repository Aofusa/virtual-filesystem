use crate::utils::logger::{LoggerRepository, LoggerInteractor, DefaultLoggerRepository};
use super::machine::Machine;
use super::ast::{AbstructSyntaxTreeKind, AbstructSyntaxTreeNodePointer};
use super::interpreter::InterpreterError;

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
    fn execute(&mut self, node: &AbstructSyntaxTreeNodePointer) -> Result<i32, InterpreterError> {
        {
            // 終端ノードであれば値を返して再帰から復帰していく
            let n = node.borrow();
            if let AbstructSyntaxTreeKind::NUM(x) = n.0 {
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
                AbstructSyntaxTreeKind::ADD => {
                    let a: i32;
                    let b: i32;

                    match self.stack.pop() {
                        Some(x) => a = x,
                        None => return Err(InterpreterError::ZeroStack),
                    }
                    match self.stack.pop() {
                        Some(x) => b = x,
                        None => return Err(InterpreterError::ZeroStack),
                    }

                    let x = b + a;
                    self.stack.push(x);
                },
                AbstructSyntaxTreeKind::SUB => {
                    let a: i32;
                    let b: i32;

                    match self.stack.pop() {
                        Some(x) => a = x,
                        None => return Err(InterpreterError::ZeroStack),
                    }
                    match self.stack.pop() {
                        Some(x) => b = x,
                        None => return Err(InterpreterError::ZeroStack),
                    }

                    let x = b - a;
                    self.stack.push(x);
                },
                AbstructSyntaxTreeKind::MUL => {
                    let a: i32;
                    let b: i32;

                    match self.stack.pop() {
                        Some(x) => a = x,
                        None => return Err(InterpreterError::ZeroStack),
                    }
                    match self.stack.pop() {
                        Some(x) => b = x,
                        None => return Err(InterpreterError::ZeroStack),
                    }

                    let x = b * a;
                    self.stack.push(x);
                },
                AbstructSyntaxTreeKind::DIV => {
                    let a: i32;
                    let b: i32;

                    match self.stack.pop() {
                        Some(x) => a = x,
                        None => return Err(InterpreterError::ZeroStack),
                    }
                    match self.stack.pop() {
                        Some(x) => b = x,
                        None => return Err(InterpreterError::ZeroStack),
                    }

                    let x = b / a;
                    self.stack.push(x);
                },
                _otherwise => { return Err(InterpreterError::CalculationError) },
            }
        }

        // スタックの一番上の情報を返却し終了する
        match self.stack.last() {
            Some(x) => Ok(*x),
            None => Err(InterpreterError::ZeroStack)
        }
    }
}

