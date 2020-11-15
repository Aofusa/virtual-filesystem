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
    CalculationError,  // 演算実行時のエラー
    ZeroStack,  // 演算スタックに何もなかった
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
}


enum AbstructSyntaxTreeKind {
    ADD,  // +
    SUB,  // -
    MUL,  // *
    DIV,  // /
    NUM(i32),  // 整数
}


type AbstructSyntaxTreeNode = Node<AbstructSyntaxTreeKind>;
type AbstructSyntaxTreeNodePointer = NodePointer<AbstructSyntaxTreeKind>;


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


pub struct Interpreter<T>
where
    T: LoggerRepository
{
    token: TokenNodePointer,  // 現在着目しているトークン
    code: String,  // 入力プログラム
    stack: Vec<i32>,  // 演算スタック
    logger: LoggerInteractor<T>,  // ログ出力する関数
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
            code: String::new(),
            stack: vec![],
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

    // エラー箇所を報告する
    fn error_at(&self, loc: &str, s: &str) {
        let pos =  self.code.len() - loc.len();
        self.logger.print(&format!("{}", self.code));
        self.logger.print(&format!("{}^ ", " ".repeat(pos)));
        self.logger.print(&format!("{}", s));
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
            TokenKind::NUM(x) => { 
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
    fn expect_number(&mut self) -> Result<i32, InterpreterError> {
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
            _ => {
                self.logger.print("数ではありません");
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

    // 入力文字列pをトークナイズしてそれを返す
    fn tokenize(&self, code: &str) -> Result<TokenNodePointer, InterpreterError> {
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

            self.error_at(&p.to_string(), "トークナイズできません");
            return Err(InterpreterError::Untokenized);
        };

        let _eof = cur.borrow_mut().new(TokenKind::EOF);

        let ret = head.borrow();
        Ok(ret.next())
    }

    fn expr(&mut self) -> Result<AbstructSyntaxTreeNodePointer, InterpreterError> {
        let mut node: AbstructSyntaxTreeNodePointer;
        match self.mul() {
            Ok(x) => node = x,
            Err(e) => return Err(e),
        }

        loop {
            if self.consume("+") {
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
            } else if self.consume("-") {
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
        match self.primary() {
            Ok(x) => node = x,
            Err(e) => return Err(e),
        }

        loop {
            if self.consume("*") {
                let rhs: AbstructSyntaxTreeNodePointer;
                match self.primary() {
                    Ok(x) => rhs = x,
                    Err(e) => return Err(e),
                }

                node = AbstructSyntaxTreeNode::new(
                    AbstructSyntaxTreeKind::MUL,
                    node.clone(),
                    rhs.clone()
                );
            } else if self.consume("/") {
                let rhs: AbstructSyntaxTreeNodePointer;
                match self.primary() {
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

    fn primary(&mut self) -> Result<AbstructSyntaxTreeNodePointer, InterpreterError> {
        // 次のトークンが "(" なら、 "(" expr ")" のはず
        if self.consume("(") {
            let node = self.expr();
            match self.expect(")") {
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
            match self.expect_number() {
                Ok(x) => Ok(AbstructSyntaxTreeNode::num(x)),
                Err(e) => Err(e),
            }
        }
    }

    fn calc(&mut self, node: &AbstructSyntaxTreeNodePointer) -> Result<i32, InterpreterError> {
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
                if let Err(e) = self.calc(x) {
                    return Err(e);
                }
            }

            // 右辺の抽象構文木の計算
            if let Some(x) = iter.next() {
                if let Err(e) = self.calc(x) {
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

    pub fn interpret(&mut self, s: &str) -> InterpreterResult {
        // トークナイズする
        self.code = s.to_string();
        match self.tokenize(s) {
            Ok(x) => self.token = x,
            Err(e) => return Err(e),
        }

        // 抽象構文木を作成する
        let ast: AbstructSyntaxTreeNodePointer;
        match self.expr() {
            Ok(x) => ast = x,
            Err(e) => return Err(e),
        }

        // 抽象構文木を降りながら演算を行う
        match self.calc(&ast) {
            Ok(x) => Ok(Some(format!("{}", x))),
            Err(e) => Err(e),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::interpreter::interpreter::{Interpreter, InterpreterError};
    use crate::utils::logger::{LoggerRepository};

    pub struct PrintLoggerRepository {}
    impl LoggerRepository for PrintLoggerRepository {
        fn print(&self, message: &str) {
            println!("{}", message);
        }
    }

    #[test]
    fn test_interpreter() {
        let mut x = Interpreter::init_with_logger(PrintLoggerRepository{});
        assert_eq!(x.interpret("42"), Ok(Some("42".to_string())));
        assert_eq!(x.interpret("5+20-4"), Ok(Some("21".to_string())));
        assert_eq!(x.interpret("5 - 3"), Ok(Some("2".to_string())));
        assert_eq!(x.interpret("5 - 3 a"), Err(InterpreterError::Untokenized));
        assert_eq!(x.interpret("2--"), Err(InterpreterError::Unexpected));
        // assert_eq!(x.interpret("1 2"), Err(InterpreterError::Unexpected));
        assert_eq!(x.interpret("1 2"), Err(InterpreterError::Unexpected));
    }
}

