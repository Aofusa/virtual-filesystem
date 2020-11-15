use crate::virtual_filesystem_core::graph::{Node, NodePointer, Graph};
use crate::utils::logger::{LoggerRepository, LoggerInteractor, DefaultLoggerRepository};
use super::token::Tokenizer;


#[derive(Debug, PartialEq)]
pub enum AstBuilderError {
    Unknown,  // エラー内容不明
    SyntaxError,  // 期待しているものではなかった
}


#[derive(Debug)]
pub enum AbstractSyntaxTreeKind {
    ADD,  // +
    SUB,  // -
    MUL,  // *
    DIV,  // /
    NUM(i32),  // 整数
}


pub type AbstractSyntaxTreeNode = Node<AbstractSyntaxTreeKind>;
pub type AbstractSyntaxTreeNodePointer = NodePointer<AbstractSyntaxTreeKind>;


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


impl AbstractSyntaxTreeNode {
    fn create(kind: AbstractSyntaxTreeKind, lhs: AbstractSyntaxTreeNodePointer, rhs: AbstractSyntaxTreeNodePointer) -> AbstractSyntaxTreeNodePointer {
        AbstractSyntaxTreeNode::new(kind, vec![lhs.clone(), rhs.clone()])
    }

    fn num(value: i32) -> AbstractSyntaxTreeNodePointer {
        AbstractSyntaxTreeNode::new(AbstractSyntaxTreeKind::NUM(value), vec![])
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

    pub fn build(&mut self) -> Result<AbstractSyntaxTreeNodePointer, AstBuilderError> {
        self.expr()
    }

    fn expr(&mut self) -> Result<AbstractSyntaxTreeNodePointer, AstBuilderError> {
        let mut node: AbstractSyntaxTreeNodePointer;
        match self.mul() {
            Ok(x) => node = x,
            Err(e) => return Err(e),
        }

        loop {
            if self.tokenizer.consume("+") {
                let rhs: AbstractSyntaxTreeNodePointer;
                match self.mul() {
                    Ok(x) => rhs = x,
                    Err(e) => return Err(e),
                }

                node = AbstractSyntaxTreeNode::create(
                    AbstractSyntaxTreeKind::ADD,
                    node,
                    rhs.clone()
                );
            } else if self.tokenizer.consume("-") {
                let rhs: AbstractSyntaxTreeNodePointer;
                match self.mul() {
                    Ok(x) => rhs = x,
                    Err(e) => return Err(e),
                }

                node = AbstractSyntaxTreeNode::create(
                    AbstractSyntaxTreeKind::SUB,
                    node,
                    rhs.clone()
                );
            } else {
                return Ok(node);
            }
        }
    }

    fn mul(&mut self) -> Result<AbstractSyntaxTreeNodePointer, AstBuilderError> {
        let mut node: AbstractSyntaxTreeNodePointer;
        match self.unary() {
            Ok(x) => node = x,
            Err(e) => return Err(e),
        }

        loop {
            if self.tokenizer.consume("*") {
                let rhs: AbstractSyntaxTreeNodePointer;
                match self.unary() {
                    Ok(x) => rhs = x,
                    Err(e) => return Err(e),
                }

                node = AbstractSyntaxTreeNode::create(
                    AbstractSyntaxTreeKind::MUL,
                    node.clone(),
                    rhs.clone()
                );
            } else if self.tokenizer.consume("/") {
                let rhs: AbstractSyntaxTreeNodePointer;
                match self.unary() {
                    Ok(x) => rhs = x,
                    Err(e) => return Err(e),
                }

                node = AbstractSyntaxTreeNode::create(
                    AbstractSyntaxTreeKind::DIV,
                    node.clone(),
                    rhs.clone()
                );
            } else {
                return Ok(node);
            }
        }
    }

    fn unary(&mut self) -> Result<AbstractSyntaxTreeNodePointer, AstBuilderError> {
        if self.tokenizer.consume("+") {
            self.primary()
        } else if self.tokenizer.consume("-") {
            match self.primary() {
                Ok(x) => Ok(
                        AbstractSyntaxTreeNode::create(
                            AbstractSyntaxTreeKind::SUB,
                            AbstractSyntaxTreeNode::num(0),
                            x
                        )
                    ),
                Err(e) => Err(e),
            }
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<AbstractSyntaxTreeNodePointer, AstBuilderError> {
        // 次のトークンが "(" なら、 "(" expr ")" のはず
        if self.tokenizer.consume("(") {
            let node = self.expr();
            match self.tokenizer.expect(")") {
                Ok(()) => {
                    match node {
                        Ok(n) => Ok(n),
                        Err(_) => Err(AstBuilderError::SyntaxError),
                    }
                },
                Err(_) => Err(AstBuilderError::SyntaxError),
            }
        } else {
            // そうでなければ数値のはず
            match self.tokenizer.expect_number() {
                Ok(x) => Ok(AbstractSyntaxTreeNode::num(x)),
                Err(_) => Err(AstBuilderError::SyntaxError),
            }
        }
    }
}

