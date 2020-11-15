use std::rc::Rc;
use std::cell::RefCell;
use crate::virtual_filesystem_core::graph::{Node, NodePointer};
use crate::utils::logger::{LoggerRepository, LoggerInteractor, DefaultLoggerRepository};
use super::interpreter::InterpreterError;
use super::token::Tokenizer;


#[derive(Debug)]
pub enum AbstructSyntaxTreeKind {
    ADD,  // +
    SUB,  // -
    MUL,  // *
    DIV,  // /
    NUM(i32),  // 整数
}


pub type AbstructSyntaxTreeNode = Node<AbstructSyntaxTreeKind>;
pub type AbstructSyntaxTreeNodePointer = NodePointer<AbstructSyntaxTreeKind>;


#[derive(Debug)]
pub struct AstBuilder<T>
where
    T: LoggerRepository + Clone
{
    tokenizer: Tokenizer<T>,
    logger: LoggerInteractor<T>,
}


impl AstBuilder<DefaultLoggerRepository> {
    #[allow(dead_code)]
    pub fn init(tokenizer: Tokenizer<DefaultLoggerRepository>) -> AstBuilder<DefaultLoggerRepository> {
        AstBuilder::init_with_logger(tokenizer, DefaultLoggerRepository{})
    }
}


impl AbstructSyntaxTreeNode {
    fn new(kind: AbstructSyntaxTreeKind, lhs: AbstructSyntaxTreeNodePointer, rhs: AbstructSyntaxTreeNodePointer) ->AbstructSyntaxTreeNodePointer {
        Rc::new(
            RefCell::new(
                Node(
                    kind,
                    vec![lhs.clone(), rhs.clone()],
                )
            )
        )
    }

    fn num(value: i32) -> AbstructSyntaxTreeNodePointer {
        Rc::new(
            RefCell::new(
                Node(
                    AbstructSyntaxTreeKind::NUM( value ),
                    vec![],
                )
            )
        )
    }
}


impl<T: LoggerRepository + Clone> AstBuilder<T> {
    fn new(tokenizer: Tokenizer<T>, logger: T) -> AstBuilder<T> {
        AstBuilder {
            tokenizer: tokenizer,
            logger: LoggerInteractor::new(logger),
        }
    }

    pub fn init_with_logger(tokenizer: Tokenizer<T>, logger: T) -> AstBuilder<T> {
        AstBuilder::new(tokenizer, logger)
    }

    pub fn build(&mut self) -> Result<AbstructSyntaxTreeNodePointer, InterpreterError> {
        self.expr()
    }

    fn expr(&mut self) -> Result<AbstructSyntaxTreeNodePointer, InterpreterError> {
        let mut node: AbstructSyntaxTreeNodePointer;
        match self.mul() {
            Ok(x) => node = x,
            Err(e) => return Err(e),
        }

        loop {
            if self.tokenizer.consume("+") {
                let rhs: AbstructSyntaxTreeNodePointer;
                match self.mul() {
                    Ok(x) => rhs = x,
                    Err(e) => return Err(e),
                }

                node = AbstructSyntaxTreeNode::new(
                    AbstructSyntaxTreeKind::ADD,
                    node,
                    rhs.clone()
                );
            } else if self.tokenizer.consume("-") {
                let rhs: AbstructSyntaxTreeNodePointer;
                match self.mul() {
                    Ok(x) => rhs = x,
                    Err(e) => return Err(e),
                }

                node = AbstructSyntaxTreeNode::new(
                    AbstructSyntaxTreeKind::SUB,
                    node,
                    rhs.clone()
                );
            } else {
                return Ok(node);
            }
        }
    }

    fn mul(&mut self) -> Result<AbstructSyntaxTreeNodePointer, InterpreterError> {
        let mut node: AbstructSyntaxTreeNodePointer;
        match self.unary() {
            Ok(x) => node = x,
            Err(e) => return Err(e),
        }

        loop {
            if self.tokenizer.consume("*") {
                let rhs: AbstructSyntaxTreeNodePointer;
                match self.unary() {
                    Ok(x) => rhs = x,
                    Err(e) => return Err(e),
                }

                node = AbstructSyntaxTreeNode::new(
                    AbstructSyntaxTreeKind::MUL,
                    node.clone(),
                    rhs.clone()
                );
            } else if self.tokenizer.consume("/") {
                let rhs: AbstructSyntaxTreeNodePointer;
                match self.unary() {
                    Ok(x) => rhs = x,
                    Err(e) => return Err(e),
                }

                node = AbstructSyntaxTreeNode::new(
                    AbstructSyntaxTreeKind::DIV,
                    node.clone(),
                    rhs.clone()
                );
            } else {
                return Ok(node);
            }
        }
    }

    fn unary(&mut self) -> Result<AbstructSyntaxTreeNodePointer, InterpreterError> {
        if self.tokenizer.consume("+") {
            self.primary()
        } else if self.tokenizer.consume("-") {
            match self.primary() {
                Ok(x) => Ok(
                        AbstructSyntaxTreeNode::new(
                            AbstructSyntaxTreeKind::SUB,
                            AbstructSyntaxTreeNode::num(0),
                            x
                        )
                    ),
                Err(e) => Err(e),
            }
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<AbstructSyntaxTreeNodePointer, InterpreterError> {
        // 次のトークンが "(" なら、 "(" expr ")" のはず
        if self.tokenizer.consume("(") {
            let node = self.expr();
            match self.tokenizer.expect(")") {
                Ok(()) => {
                    match node {
                        Ok(n) => Ok(n),
                        Err(e) => Err(e),
                    }
                },
                Err(e) => Err(e),
            }
        } else {
            // そうでなければ数値のはず
            match self.tokenizer.expect_number() {
                Ok(x) => Ok(AbstructSyntaxTreeNode::num(x)),
                Err(e) => Err(e),
            }
        }
    }
}

