use crate::virtual_filesystem_core::graph::{Node, NodePointer, Graph};
use crate::utils::logger::{LoggerRepository, LoggerInteractor, DefaultLoggerRepository};
use super::token::Tokenizer;
use super::interpreter::InterpreterError;


#[derive(Debug)]
pub enum AbstractSyntaxTreeKind {
    ADD,  // +
    SUB,  // -
    MUL,  // *
    DIV,  // /
    ASSIGN,  // =
    LOCALVARIABLE(String),  // ローカル変数
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
    code: Vec<AbstractSyntaxTreeNodePointer>,
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

    fn variable(value: String) -> AbstractSyntaxTreeNodePointer {
        AbstractSyntaxTreeNode::new(AbstractSyntaxTreeKind::LOCALVARIABLE(value), vec![])
    }
}


impl<T: LoggerRepository + Clone> AstBuilder<T> {
    fn new(tokenizer: Tokenizer<T>, logger: T) -> AstBuilder<T> {
        AstBuilder {
            tokenizer: tokenizer,
            code: Vec::new(),
            logger: LoggerInteractor::new(logger),
        }
    }

    pub fn init_with_logger(tokenizer: Tokenizer<T>, logger: T) -> AstBuilder<T> {
        AstBuilder::new(tokenizer, logger)
    }

    pub fn build(&mut self) -> Result<AbstractSyntaxTreeNodePointer, InterpreterError> {
        self.program()
    }

    fn program(&mut self) -> Result<AbstractSyntaxTreeNodePointer, InterpreterError> {
        let mut code = Vec::new();
        while !self.tokenizer.at_eof() {
            code.push(self.stmt()?);
        }
        self.code = code;
        Ok(self.code.first().ok_or(InterpreterError::InvalidSource)?.clone())
    }

    fn stmt(&mut self) -> Result<AbstractSyntaxTreeNodePointer, InterpreterError> {
        let node = self.expr();
        loop { if !self.tokenizer.consume(";") { break } }
        node
    }

    fn expr(&mut self) -> Result<AbstractSyntaxTreeNodePointer, InterpreterError> {
        self.assign()
    }

    fn assign(&mut self) -> Result<AbstractSyntaxTreeNodePointer, InterpreterError> {
        let mut node = self.add()?;
        if self.tokenizer.consume("=") {
            let rhs = self.assign()?;
            node = AbstractSyntaxTreeNode::create(
                AbstractSyntaxTreeKind::ASSIGN,
                node,
                rhs
            )
        }
        Ok(node)
    }

    fn add(&mut self) -> Result<AbstractSyntaxTreeNodePointer, InterpreterError> {
        let mut node = self.mul()?;

        loop {
            if self.tokenizer.consume("+") {
                let rhs = self.mul()?;
                node = AbstractSyntaxTreeNode::create(
                    AbstractSyntaxTreeKind::ADD,
                    node,
                    rhs.clone()
                );
            } else if self.tokenizer.consume("-") {
                let rhs = self.mul()?;
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

    fn mul(&mut self) -> Result<AbstractSyntaxTreeNodePointer, InterpreterError> {
        let mut node = self.unary()?;

        loop {
            if self.tokenizer.consume("*") {
                let rhs = self.unary()?;
                node = AbstractSyntaxTreeNode::create(
                    AbstractSyntaxTreeKind::MUL,
                    node.clone(),
                    rhs.clone()
                );
            } else if self.tokenizer.consume("/") {
                let rhs = self.unary()?;
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

    fn unary(&mut self) -> Result<AbstractSyntaxTreeNodePointer, InterpreterError> {
        if self.tokenizer.consume("+") {
            self.primary()
        } else if self.tokenizer.consume("-") {
            let node = self.primary()?;
            Ok(
                AbstractSyntaxTreeNode::create(
                    AbstractSyntaxTreeKind::SUB,
                    AbstractSyntaxTreeNode::num(0),
                    node
                )
            )
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<AbstractSyntaxTreeNodePointer, InterpreterError> {
        // 次のトークンが "(" なら、 "(" expr ")" のはず
        if self.tokenizer.consume("(") {
            let node = self.expr();
            self.tokenizer.expect(")")?;
            node
        } else {
            match self.tokenizer.consume_ident() {
                Some(t) => Ok(AbstractSyntaxTreeNode::variable(t)),
                None => {
                    // そうでなければ数値のはず
                    let x = self.tokenizer.expect_number()?;
                    Ok(AbstractSyntaxTreeNode::num(x))
                }
            }
        }
    }
}

