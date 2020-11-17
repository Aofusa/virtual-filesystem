use std::collections::HashMap;
use crate::utils::logger::{LoggerRepository, LoggerInteractor, DefaultLoggerRepository};
use super::machine::{Machine, Literal};
use super::ast::{AbstractSyntaxTreeKind, AbstractSyntaxTreeNodePointer};
use super::operator::{add, sub, mul, div};
use super::interpreter::InterpreterError;


pub struct StackMachine<T>
where
    T: LoggerRepository + Clone
{
    stack: Vec<Literal>,  // 演算スタック
    variables: HashMap<String, Literal>,  // 変数リスト
    command: HashMap<String, Box<dyn Fn(&mut Vec<Literal>) -> Result<Literal, InterpreterError>>>,
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
        let mut c = HashMap::<String, Box<dyn Fn(&mut Vec<Literal>) -> Result<Literal, InterpreterError>>>::new();
        c.insert("add".to_string(), Box::new(add));
        c.insert("sub".to_string(), Box::new(sub));
        c.insert("mul".to_string(), Box::new(mul));
        c.insert("div".to_string(), Box::new(div));

        StackMachine {
            stack: vec![],
            variables: HashMap::new(),
            command: c,
            logger: LoggerInteractor::new(logger),
        }
    }

    pub fn init_with_logger(logger: T) -> StackMachine<T> {
        StackMachine::new(logger)
    }
}

impl<T: LoggerRepository + Clone> Machine for StackMachine<T> {
    fn execute(&mut self, node: &AbstractSyntaxTreeNodePointer) -> Result<Literal, InterpreterError> {
        {
            // 終端ノードであれば値を返して再帰から復帰していく
            let n = node.borrow();
            match &n.0 {
                AbstractSyntaxTreeKind::NUM(x) => {
                    self.stack.push(Literal::NUM(*x));
                    return Ok(Literal::NUM(*x));
                },
                AbstractSyntaxTreeKind::STRING(x) => {
                    self.stack.push(Literal::STRING(x.to_string()));
                    return Ok(Literal::STRING(x.to_string()));
                },
                AbstractSyntaxTreeKind::ASSIGN => {
                    let mut iter = n.1.iter();
                    let lhs = iter.next().ok_or(InterpreterError::InvalidLVariable)?.clone();
                    let lhs_kind = &lhs.borrow().0;
                    let rhs = iter.next().ok_or(InterpreterError::InvalidRValue)?;
                    if let AbstractSyntaxTreeKind::LOCALVARIABLE(vname) = lhs_kind {
                        let vdata = self.execute(rhs)?;
                        self.variables.insert(vname.to_string(), vdata.clone());
                        return Ok(vdata);
                    }
                    return Err(InterpreterError::InvalidLVariable);
                },
                AbstractSyntaxTreeKind::LOCALVARIABLE(x) => {
                    let v = self.variables.get(x).ok_or(InterpreterError::UndefinedVariable)?;
                    self.stack.push(v.clone());
                    return Ok(v.clone());
                },
                AbstractSyntaxTreeKind::RETURN => {
                    let mut iter = n.1.iter();
                    let rhs = iter.next().ok_or(InterpreterError::InvalidRValue)?;
                    let vdata = self.execute(rhs)?;
                    return Ok(vdata);
                },
                _ => {}
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
        self.stack.last().and_then(|x| Some(x.clone())).ok_or(InterpreterError::ZeroStack)
    }
}

