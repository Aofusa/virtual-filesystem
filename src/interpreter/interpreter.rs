use std::rc::Rc;
use std::cell::RefCell;
use crate::virtual_filesystem_core::graph::{Node, NodePointer, Graph};
use crate::utils::logger::{LoggerRepository, LoggerInteractor, DefaultLoggerRepository};


pub type InterpreterResult = Result<Option<String>, InterpreterError>;


#[derive(Debug, PartialEq)]
pub enum InterpreterError {
    Unknown,  // エラー内容不明
    Unexpected,  // 期待しているものではなかった
    Untokenized,  // トークンかできなかった
}


fn split_digit(s: &str) -> (&str, &str) {
    let first_non_num = s.find(|c| !char::is_numeric(c)).unwrap_or(s.len());
    s.split_at(first_non_num)
}


// トークン型
#[derive(Debug)]
enum TokenKind {
    RESERVED(String),  // 記号
    NUM(i32),  // 整数トークン
    EOF,  // 入力の終わりを表すトークン
}


type TokenNode = Node<TokenKind>;
type TokenNodePointer = NodePointer<TokenKind>;


impl TokenNode {
    // 次のトークンを取得する
    fn next(&self) -> TokenNodePointer {
        let t = self.1
            .iter()
            .next();
        match t {
            Some(x) => x.clone(),
            _ => Rc::new(
                RefCell::new(
                    Node(
                        TokenKind::EOF,
                        vec![],
                    )
                )
            ),
        }
    }

    // 新しいトークンを作成して繋げる
    fn new(&mut self, kind: TokenKind) -> TokenNodePointer {
        let tok = Rc::new(
            RefCell::new(
                Node(
                    kind,
                    vec![],
                )
            )
        );
        self.connect(tok.clone());
        tok
    }

    // 入力文字列pをトークナイズしてそれを返す
    fn tokenize(code: &str) -> Result<TokenNodePointer, InterpreterError> {
        let head = Rc::new(
            RefCell::new(
                Node(TokenKind::EOF, vec![])
            )
        );
        let mut cur = head.clone();
        let mut iter = code.chars();

        while let Some(p) = iter.next() {
            // 空白文字をスキップ
            if p.is_whitespace() { continue }

            if p == '+' || p == '-' {
                let c = cur.clone();
                cur = c.borrow_mut()
                    .new(
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
                    .new(
                        TokenKind::NUM(n)
                    );
                continue
            }

            return Err(InterpreterError::Untokenized);
        };

        let _eof = cur.borrow_mut().new(TokenKind::EOF);

        let ret = head.borrow();
        Ok(ret.next())
    }
}


pub struct Interpreter<T>
where
    T: LoggerRepository
{
    token: TokenNodePointer,  // 現在着目しているトークン
    logger: LoggerInteractor<T>, // ログ出力する関数
}


impl Interpreter<DefaultLoggerRepository> {
    #[allow(dead_code)]
    pub fn init() -> Interpreter<DefaultLoggerRepository> {
        Interpreter::init_with_logger(DefaultLoggerRepository{})
    }
}


impl<T: LoggerRepository> Interpreter<T> {
    fn new(token: TokenNodePointer, logger: T) -> Interpreter<T> {
        Interpreter {
            token: token,
            logger: LoggerInteractor::new(logger),
        }
    }

    pub fn init_with_logger(logger: T) -> Interpreter<T> {
        Interpreter::new(
            Rc::new(
                RefCell::new(
                    Node(
                        TokenKind::EOF,
                        vec![],
                    )
                )
            ),
            logger
        )
    }

    // 次のトークンが期待している記号のときには、トークンを1つ読み進めて
    // 真を返す。それ以外の場合には偽を返す。
    fn consume(&mut self, op: &str) -> bool {
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

    // 次のトークンが期待している記号のときには、トークンを1つ読み進める。
    // それ以外の場合にはエラーを報告する。
    fn expect(&mut self, op: &str) ->Result<(), InterpreterError> {
        let t = self.token.clone();
        let p = &t.borrow().0;
        match p {
            TokenKind::RESERVED(x) if x == op => {
                self.token = t.borrow().next();
                Ok(())
            },
            _ => {
                self.logger.print(&format!("INTERPRETER ERROR: '{}' ではありません", op));
                Err(InterpreterError::Unexpected)
            },
        }
    }

    // 次のトークンが数値の場合、トークンを1つ読み進めてその数値を返す。
    // それ以外の場合にはエラーを報告する。
    fn expect_number(&mut self) -> Result<i32, InterpreterError> {
        let t = self.token.clone();
        let p = &t.borrow().0;
        match p {
            TokenKind::NUM(x) => { 
                self.token = t.borrow().next();
                Ok(*x)
            },
            _ => {
                self.logger.print("INTERPRETER ERROR: 数ではありません");
                Err(InterpreterError::Unexpected)
            },
        }
    }

    fn at_eof(&self) -> bool {
        let t = &self.token;
        let p = &t.borrow().0;
        match p {
            TokenKind::EOF => true,
            _ => false,
        }
    }

    pub fn interpret(&mut self, s: &str) -> InterpreterResult {
        let mut stack = 0;

        // トークナイズする
        match TokenNode::tokenize(s) {
            Ok(x) => self.token = x,
            Err(e) => return Err(e),
        }

        // 式の最初は数でなければならないので、それをチェックして
        // 最初の計算を行う
        match self.expect_number() {
            Ok(x) => stack = stack + x,
            Err(e) => return Err(e),
        }

        // `+ <数>` あるいは `- <数>` というトークンの並びを消費しつつ
        // 計算を行う
        while !self.at_eof() {
            if self.consume("+") {
                match self.expect_number() {
                    Ok(x) => stack = stack + x,
                    Err(e) => return Err(e),
                }
            }

            match self.expect("-") {
                Ok(()) => {
                    match self.expect_number() {
                        Ok(x) => stack = stack - x,
                        Err(e) => return Err(e),
                    }
                },
                Err(e) => return Err(e),
            }
        }
    
        Ok(Some(format!("{}", stack)))
    }
}


#[cfg(test)]
mod tests {
    use crate::interpreter::interpreter::Interpreter;

    #[test]
    fn test_interpreter() {
        let mut x = Interpreter::init();
        assert_eq!(x.interpret("42"), Ok(Some("42".to_string())));
        assert_eq!(x.interpret("5+20-4"), Ok(Some("21".to_string())));
        assert_eq!(x.interpret("5 - 3"), Ok(Some("2".to_string())));
    }
}

