use crate::virtual_filesystem_core::graph::{Node, NodePointer, Graph};
use crate::utils::logger::{LoggerRepository, LoggerInteractor, DefaultLoggerRepository};
use super::common::{split_digit, split_alphanumeric, split_specific};
use super::interpreter::InterpreterError;


// トークン型
#[derive(Debug)]
pub enum TokenKind {
    RESERVED(String),  // 記号
    IDENT(String),  // 識別子
    NUM(i32),  // 整数トークン
    STRING(String),  // 文字列トークン
    FUNCCALL(String),  // 関数呼び出し
    RETURN,  // return ステートメント
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

    // 次のトークンが文字列の場合、トークンを1つ読み進めてその文字列を返す。
    // それ以外の場合には None を返する。
    pub fn consume_strings(&mut self) -> Option<String> {
        let t = self.token.clone();
        let p = &t.borrow().0;
        match p {
            TokenKind::STRING(x) => { 
                self.token = t.borrow().next();
                Some(x.to_string())
            }
            _ => None,
        }
    }

    // 次のトークンが何であるかにかかわらず文字列を返す
    pub fn consume_any(&mut self) -> String {
        let t = self.token.clone();
        let p = &t.borrow().0;
        self.token = t.borrow().next();
        match p {
            TokenKind::NUM(x) => format!("{}", x),
            TokenKind::STRING(x) =>  x.to_string(),
            TokenKind::FUNCCALL(x) => format!("{}", x),
            TokenKind::RESERVED(x) => format!("{}", x),
            TokenKind::IDENT(x) => format!("{}", x),
            TokenKind::RETURN => "".to_string(),
            TokenKind::EOF => "\0".to_string(),
        }
    }

    // 次のトークンが関数呼出の場合、トークンを1つ読み進めて関数名を返す。
    // それ以外の場合には None を返す。
    pub fn consume_funccall(&mut self) -> Option<String> {
        let t = self.token.clone();
        let p = &t.borrow().0;
        match p {
            TokenKind::FUNCCALL(x) => { 
                self.token = t.borrow().next();
                Some(x.to_string())
            }
            _ => None,
        }
    }

    // 次のトークンが期待しているステートメントの時には、トークンを一つ読み進めて
    // 真を返す。それ以外の場合には偽を返す。
    pub fn consume_return(&mut self) -> bool {
        let t = self.token.clone();
        let p = &t.borrow().0;
        match p {
            TokenKind::RETURN => {
                self.token = t.borrow().next();
                true
            }
            _ => false,
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
            TokenKind::STRING(x) => { 
                self.error_at(x, "記号ではありません");
                Err(InterpreterError::Unexpected)
            },
            TokenKind::NUM(x) => {
                self.error_at(&x.to_string(), "記号ではありません");
                Err(InterpreterError::Unexpected)
            },
            TokenKind::FUNCCALL(x) => {
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
            TokenKind::STRING(x) => { 
                self.error_at(x, "数ではありません");
                Err(InterpreterError::Unexpected)
            },
            TokenKind::FUNCCALL(x) => {
                self.error_at(&x.to_string(), "数ではありません");
                Err(InterpreterError::Unexpected)
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
        self.logger.print(&format!("input code: {}", code));

        // 一番初めは関数呼び出し
        {
            while let Some(p) = iter.next() {
                // 空白文字をスキップ
                if p.is_whitespace() { continue }

                // 文字列を変数名に変換しイテレータを進める
                let s = p.to_string() + iter.as_str();
                let string = s.split_whitespace().next().ok_or(InterpreterError::InvalidSource)?;
                for _ in 0..string.len()-1 { iter.next(); }
                self.logger.print(&format!("funccall token: {}", string));

                let c = cur.clone();
                cur = c.borrow_mut().create(TokenKind::FUNCCALL(string.to_string()));
                
                // 最初の処理が終わったら抜ける
                break
            }
        }

        while let Some(p) = iter.next() {
            // 空白文字をスキップ
            if p.is_whitespace() { continue }

            // return ステートメント
            {
                let s = p.to_string() + iter.as_str();
                let statement = split_alphanumeric(&s).0;
                if statement == "return" {
                    self.logger.print(&format!("return token: {}", statement));
                    for _ in 0..statement.len() { iter.next(); }

                    let c = cur.clone();
                    cur = c.borrow_mut()
                        .create(TokenKind::RETURN);
                    continue
                }
            }

            // 演算子など記号
            if p == '+' || p == '-' ||
               p == '*' || p == '/' ||
               p == '(' || p == ')' ||
               p == '=' || p == ';' ||
               p == '$' || p == '"' ||
               p == '\'' {
                self.logger.print(&format!("reserved token: {}", p));
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
                self.logger.print(&format!("num token: {}", n));

                let c = cur.clone();
                cur = c.borrow_mut()
                    .create( TokenKind::NUM(n) );
                continue
            }

            // 文字列
            {
                // 文字列を変数名に変換しイテレータを進める
                let s = p.to_string() + iter.as_str();
                let exclusion = "=+-*/!@#$%^&¥|`~/.,:;'\"<>()[]{}";  // 変数名に使用できないリスト
                let (san, _right) = split_alphanumeric(&s);
                let (variable_string, _right) = split_specific(san, exclusion);
                if !variable_string.is_empty() {
                    self.logger.print(&format!("string token: {}", variable_string));
                    for _ in 0..variable_string.len()-1 { iter.next(); }

                    let c = cur.clone();
                    cur = c.borrow_mut()
                        .create(
                            TokenKind::STRING( variable_string.to_string() )
                        );
                    continue
                }
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

