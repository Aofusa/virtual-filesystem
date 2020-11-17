use crate::utils::logger::{LoggerRepository, LoggerInteractor, DefaultLoggerRepository};
use super::token::Tokenizer;
use super::ast::AstBuilder;
use super::stackmachine::StackMachine;
use super::machine::Machine;


pub type InterpreterResult = Result<Option<String>, InterpreterError>;


#[derive(Debug, PartialEq)]
pub enum InterpreterError {
    Unknown,  // エラー内容不明
    Unexpected,  // 期待値と異なる
    Untokenized,  // トークンかできなかった
    SyntaxError,  // 構文と異なる
    InvalidSource,  // プログラムが正しくない
    CalculationError,  // 演算実行時のエラー
    ZeroStack,  // 演算スタックに何もなかった
    UndefinedFunction,  // 未定義関数
    UndefinedVariable,  // 未定義変数
    InvalidLVariable,  // 左辺値が変数ではない
    InvalidRValue,  // 右辺値が不正な形式
}


pub struct Interpreter<T>
where
    T: LoggerRepository + Clone
{
    logger: LoggerInteractor<T>,  // ログ出力する関数
}


impl Interpreter<DefaultLoggerRepository> {
    #[allow(dead_code)]
    pub fn init() -> Interpreter<DefaultLoggerRepository> {
        Interpreter::init_with_logger(DefaultLoggerRepository{})
    }
}


impl<T: LoggerRepository + Clone> Interpreter<T> {
    fn new(logger: T) -> Interpreter<T> {
        Interpreter {
            logger: LoggerInteractor::new(logger),
        }
    }

    pub fn init_with_logger(logger: T) -> Interpreter<T> {
        Interpreter::new(logger)
    }

    pub fn interpret(&mut self, s: &str) -> InterpreterResult {
        // トークナイズする
        let mut tokenizer = Tokenizer::init_with_logger(self.logger.get());
        tokenizer.tokenize(s)?;

        // 抽象構文木を作成する
        let mut ast = AstBuilder::init_with_logger(tokenizer, self.logger.get());
        let ast_pointer = ast.build()?;

        // 抽象構文木を降りながら演算を行う
        let mut machine = StackMachine::init_with_logger(self.logger.get());
        Ok(Some(format!("{}", machine.execute(&ast_pointer)? )))
    }
}


#[cfg(test)]
mod tests {
    use crate::interpreter::interpreter::{Interpreter, InterpreterError};
    use crate::utils::logger::LoggerRepository;

    #[derive(Clone)]
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
        // assert_eq!(x.interpret("5 - 3 a"), Err(InterpreterError::Untokenized));
        // assert_eq!(x.interpret("2--"), Err(InterpreterError::SyntaxError));
        assert_eq!(x.interpret("5+6*7"), Ok(Some("47".to_string())));
        assert_eq!(x.interpret("5*(9-6)"), Ok(Some("15".to_string())));
        assert_eq!(x.interpret("(3+5) / 2"), Ok(Some("4".to_string())));
        assert_eq!(x.interpret("-10+(+20)"), Ok(Some("10".to_string())));
        assert_eq!(x.interpret("$a=-10+(+20)"), Ok(Some("10".to_string())));
        assert_eq!(x.interpret("$a=$b * $a"), Err(InterpreterError::UndefinedVariable));
        assert_eq!(x.interpret("$absc = 100"), Ok(Some("100".to_string())));
    }
}

