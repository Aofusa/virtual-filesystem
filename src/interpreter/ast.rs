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
    STRING(String),  // 文字列
    FUNC(String),  // 関数
    RETURN,  // return ステートメント
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

    fn string(value: String) -> AbstractSyntaxTreeNodePointer {
        AbstractSyntaxTreeNode::new(AbstractSyntaxTreeKind::STRING(value), vec![])
    }

    fn func(value: String, rhs: AbstractSyntaxTreeNodePointer) -> AbstractSyntaxTreeNodePointer {
        AbstractSyntaxTreeNode::new(AbstractSyntaxTreeKind::FUNC(value), vec![rhs.clone()])
    }

    fn variable(value: String) -> AbstractSyntaxTreeNodePointer {
        AbstractSyntaxTreeNode::new(AbstractSyntaxTreeKind::LOCALVARIABLE(value), vec![])
    }

    fn return_node(rhs: AbstractSyntaxTreeNodePointer) -> AbstractSyntaxTreeNodePointer {
        AbstractSyntaxTreeNode::new(AbstractSyntaxTreeKind::RETURN, vec![rhs.clone()])
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

    pub fn build(&mut self) -> Result<&[AbstractSyntaxTreeNodePointer], InterpreterError> {
        self.program()
    }

    fn program(&mut self) -> Result<&[AbstractSyntaxTreeNodePointer], InterpreterError> {
        let mut code = Vec::new();
        code.push(self.func()?);
        while !self.tokenizer.at_eof() {
            code.push(self.stmt()?);
        }
        self.code = code;
        Ok(self.code.as_slice())
    }

    fn func(&mut self) -> Result<AbstractSyntaxTreeNodePointer, InterpreterError> {
        match self.tokenizer.consume_funccall() {
            Some(s) => {
                let node = AbstractSyntaxTreeNode::func(s, self.expr()?);
                Ok(node)
            },
            None => {
                self.stmt()
            }
        }
    }

    fn stmt(&mut self) -> Result<AbstractSyntaxTreeNodePointer, InterpreterError> {
        let node = if self.tokenizer.consume_return() {
            AbstractSyntaxTreeNode::return_node(self.expr()?)
        } else {
            self.expr()?
        };
        loop { if !self.tokenizer.consume(";") { break } }
        Ok(node)
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
            // もしかしたら変数かもしれない
            match self.tokenizer.consume_ident() {
                Some(t) => Ok(AbstractSyntaxTreeNode::variable(t)),
                None => {
                    // もしかしたら文字列かもしれない
                    match self.tokenizer.consume_strings() {
                        Some(s) => {
                            if self.tokenizer.consume("\"") {
                                let node = AbstractSyntaxTreeNode::string(s);
                                self.tokenizer.expect("\"")?;
                                Ok(node)
                            } else if self.tokenizer.consume("'") {
                                let node = AbstractSyntaxTreeNode::string(s);
                                self.tokenizer.expect("'")?;
                                Ok(node)
                            } else {
                                let node = AbstractSyntaxTreeNode::string(s);
                                Ok(node)
                            }
                        },
                        None => {
                            // そうでなければ数値のはず
                            let x = self.tokenizer.expect_number()?;
                            Ok(AbstractSyntaxTreeNode::num(x))
                        }
                    }
                }
            }
        }
    }
}

