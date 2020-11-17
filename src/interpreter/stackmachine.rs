use std::collections::HashMap;
use crate::utils::logger::{LoggerRepository, LoggerInteractor, DefaultLoggerRepository};
use super::machine::Machine;
use super::ast::{AbstractSyntaxTreeKind, AbstractSyntaxTreeNodePointer};
use super::operator::{add, sub, mul, div};
use super::interpreter::InterpreterError;


pub struct StackMachine<T>
where
    T: LoggerRepository + Clone
{
    stack: Vec<i32>,  // 演算スタック
    command: HashMap<String, Box<dyn Fn(&mut Vec<i32>) -> Result<i32, InterpreterError>>>,
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
        let mut c = HashMap::<String, Box<dyn Fn(&mut Vec<i32>) -> Result<i32, InterpreterError>>>::new();
        c.insert("add".to_string(), Box::new(add));
        c.insert("sub".to_string(), Box::new(sub));
        c.insert("mul".to_string(), Box::new(mul));
        c.insert("div".to_string(), Box::new(div));

        StackMachine {
            stack: vec![],
            command: c,
            logger: LoggerInteractor::new(logger),
        }
    }

    pub fn init_with_logger(logger: T) -> StackMachine<T> {
        StackMachine::new(logger)
    }
}

impl<T: LoggerRepository + Clone> Machine for StackMachine<T> {
    fn execute(&mut self, node: &AbstractSyntaxTreeNodePointer) -> Result<i32, InterpreterError> {
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
                self.execute(x)?;
            }

            // 右辺の抽象構文木の計算
            if let Some(x) = iter.next() {
                self.execute(x)?;
            }
        }

        {
            // 演算子だった場合スタックの内容を使い計算を行う
            let n = node.borrow();
            match &n.0 {
                AbstractSyntaxTreeKind::ADD => {
                    let f = self.command.get("add").ok_or(InterpreterError::UndefinedFunction)?;
                    f(&mut self.stack)?;
                },
                AbstractSyntaxTreeKind::SUB => {
                    let f = self.command.get("sub").ok_or(InterpreterError::UndefinedFunction)?;
                    f(&mut self.stack)?;
                },
                AbstractSyntaxTreeKind::MUL => {
                    let f = self.command.get("mul").ok_or(InterpreterError::UndefinedFunction)?;
                    f(&mut self.stack)?;
                },
                AbstractSyntaxTreeKind::DIV => {
                    let f = self.command.get("div").ok_or(InterpreterError::UndefinedFunction)?;
                    f(&mut self.stack)?;
                },
                _otherwise => {
                    self.logger.print("unreachable here");
                    return Err(InterpreterError::CalculationError)
                },
            }
        }

        // スタックの一番上の情報を返却し終了する
        match self.stack.last() {
            Some(x) => Ok(*x),
            None => Err(InterpreterError::ZeroStack)
        }
    }
}

