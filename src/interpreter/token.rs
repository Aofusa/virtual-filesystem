use crate::virtual_filesystem_core::graph::{Node, NodePointer, Graph};
use crate::utils::logger::{LoggerRepository, LoggerInteractor, DefaultLoggerRepository};
use super::common::{split_digit, split_alphanumeric};
use super::interpreter::InterpreterError;


// トークン型
#[derive(Debug)]
pub enum TokenKind {
    RESERVED(String),  // 記号
    IDENT(String),  // 識別子
    NUM(i32),  // 整数トークン
    EOF,  // 入力の終わりを表すトークン
}


type TokenNode = Node<TokenKind>;
type TokenNodePointer = NodePointer<TokenKind>;


#[derive(Debug)]
pub struct Tokenizer<T>
where
    T: LoggerRepository + Clone
{
    token: TokenNodePointer,  // 現在着目しているトークン
    code: String,  // 入力プログラム
    logger: LoggerInteractor<T>,
}


impl Tokenizer<DefaultLoggerRepository> {
    #[allow(dead_code)]
    pub fn init() -> Tokenizer<DefaultLoggerRepository> {
        Tokenizer::init_with_logger(DefaultLoggerRepository{})
    }
}


impl TokenNode {
    // 次のトークンを取得する
    fn next(&self) -> TokenNodePointer {
        let t = self.1
            .iter()
            .next();
        match t {
            Some(x) => x.clone(),
            _ => TokenNode::new(TokenKind::EOF, vec![]),
        }
    }

    // 新しいトークンを作成して繋げる
    fn create(&mut self, kind: TokenKind) -> TokenNodePointer {
        let tok = TokenNode::new(kind, vec![]);
        self.connect(tok.clone());
        tok
    }
}


impl<T: LoggerRepository + Clone> Tokenizer<T> {
    fn new(token: TokenNodePointer, logger: T) -> Tokenizer<T> {
        Tokenizer {
            token: token,
            code: String::new(),
            logger: LoggerInteractor::new(logger),
        }
    }

    pub fn init_with_logger(logger: T) -> Tokenizer<T> {
        Tokenizer::new(TokenNode::new(TokenKind::EOF, vec![]), logger)
    }

    // エラー箇所を報告する
    fn error_at(&self, loc: &str, s: &str) {
        let pos =  self.code.len() - loc.len();
        self.logger.print(&format!("{}", self.code));
        self.logger.print(&format!("{}^ ", " ".repeat(pos)));
        self.logger.print(&format!("{}", s));
    }

    // 次のトークンが期待している記号のときには、トークンを1つ読み進めて
    // 真を返す。それ以外の場合には偽を返す。
    pub fn consume(&mut self, op: &str) -> bool {
        let t = self.token.clone();
        let p = &t.borrow().0;
        match p {
            TokenKind::RESERVED(x) if x == op => {
                self.token = t.borrow().next();
                true
            },
            _ => false,
        }
    }

    // 次のトークンが期待している変数の時には、トークンを一つ読み進めて
    // 変数名を返す。それ以外の時には None を返す。
    pub fn consume_ident(&mut self) -> Option<String> {
        let t = self.token.clone();
        let p = &t.borrow().0;
        match p {
            TokenKind::IDENT(v) => {
                self.token = t.borrow().next();
                Some(v.to_string())
            }
            _ => None,
        }
    }

    // 次のトークンが期待している記号のときには、トークンを1つ読み進める。
    // それ以外の場合にはエラーを報告する。
    pub fn expect(&mut self, op: &str) ->Result<(), InterpreterError> {
        let t = self.token.clone();
        let p = &t.borrow().0;
        match p {
            TokenKind::RESERVED(x) if x == op => {
                self.token = t.borrow().next();
                Ok(())
            },
            TokenKind::NUM(x) => {
                self.error_at(&x.to_string(), "記号ではありません");
                Err(InterpreterError::Unexpected)
            },
            TokenKind::IDENT(x) => {
                self.error_at(&x.to_string(), "記号ではありません");
                Err(InterpreterError::Unexpected)
            },
            _ => {
                self.logger.print(&format!("'{}' ではありません", op));
                Err(InterpreterError::Unexpected)
            },
        }
    }

    // 次のトークンが数値の場合、トークンを1つ読み進めてその数値を返す。
    // それ以外の場合にはエラーを報告する。
    pub fn expect_number(&mut self) -> Result<i32, InterpreterError> {
        let t = self.token.clone();
        let p = &t.borrow().0;
        match p {
            TokenKind::NUM(x) => { 
                self.token = t.borrow().next();
                Ok(*x)
            },
            TokenKind::RESERVED(x) => { 
                self.error_at(x, "数ではありません");
                Err(InterpreterError::Unexpected)
            },
            TokenKind::IDENT(x) => { 
                self.error_at(x, "数ではありません");
                Err(InterpreterError::Unexpected)
            },
            _ => {
                self.logger.print("数ではありません");
                Err(InterpreterError::Unexpected)
            },
        }
    }

    pub fn at_eof(&self) -> bool {
        let t = &self.token;
        let p = &t.borrow().0;
        match p {
            TokenKind::EOF => true,
            _ => false,
        }
    }

    // 入力文字列pをトークナイズしてそれを返す
    pub fn tokenize(&mut self, code: &str) -> Result<TokenNodePointer, InterpreterError> {
        let head = TokenNode::new(TokenKind::EOF, vec![]);
        let mut cur = head.clone();
        let mut iter = code.chars();
        self.code = code.to_string();

        while let Some(p) = iter.next() {
            // 空白文字をスキップ
            if p.is_whitespace() { continue }

            // 変数
            if p == '$' {
                let s = iter.as_str();
                let (variable_string, _right) = split_alphanumeric(s);

                let c = cur.clone();
                cur = c.borrow_mut()
                    .create(
                        TokenKind::IDENT( variable_string.to_string() )
                    );
                continue
            }

            // 演算子など記号
            if p == '+' || p == '-' ||
               p == '*' || p == '/' ||
               p == '(' || p == ')' ||
               p == ';' {
                let c = cur.clone();
                cur = c.borrow_mut()
                    .create(
                        TokenKind::RESERVED( p.to_string() )
                    );
                continue
            }

            if p.is_digit(10) {
                // 文字列を数値に変換しイテレータを進める
                let origin_str = p.to_string() + iter.as_str();
                let (number_string, _right) = split_digit(&origin_str);
                let n: i32 = number_string.parse().unwrap();
                for _ in 0..number_string.len()-1 { iter.next(); }

                let c = cur.clone();
                cur = c.borrow_mut()
                    .create( TokenKind::NUM(n) );
                continue
            }

            self.error_at(&p.to_string(), "トークナイズできません");
            return Err(InterpreterError::Untokenized);
        };

        let _eof = cur.borrow_mut().create(TokenKind::EOF);

        let ret = head.borrow();
        self.token = ret.next();
        Ok(self.token.clone())
    }
}

