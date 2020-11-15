use crate::utils::logger::{LoggerRepository, LoggerInteractor, DefaultLoggerRepository};
use super::token::Tokenizer;
use super::ast::{AstBuilder, AbstructSyntaxTreeNodePointer};
use super::stackmachine::StackMachine;
use super::machine::Machine;


pub type InterpreterResult = Result<Option<String>, InterpreterError>;


#[derive(Debug, PartialEq)]
pub enum InterpreterError {
    Unknown,  // エラー内容不明
    Unexpected,  // 期待しているものではなかった
    Untokenized,  // トークンかできなかった
    CalculationError,  // 演算実行時のエラー
    ZeroStack,  // 演算スタックに何もなかった
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
        if let Err(e) = tokenizer.tokenize(s) { return Err(e); }

        // 抽象構文木を作成する
        let mut ast = AstBuilder::init_with_logger(tokenizer, self.logger.get());
        let ast_pointer: AbstructSyntaxTreeNodePointer;
        match ast.build() {
            Ok(x) => ast_pointer = x,
            Err(e) => return Err(e),
        }

        // 抽象構文木を降りながら演算を行う
        let mut machine = StackMachine::init_with_logger(self.logger.get());
        match machine.execute(&ast_pointer) {
            Ok(x) => Ok(Some(format!("{}", x))),
            Err(e) => Err(e),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::interpreter::interpreter::{Interpreter, InterpreterError};
    use crate::utils::logger::{LoggerRepository};

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
        assert_eq!(x.interpret("5 - 3 a"), Err(InterpreterError::Untokenized));
        assert_eq!(x.interpret("2--"), Err(InterpreterError::Unexpected));
        // assert_eq!(x.interpret("1 2"), Err(InterpreterError::Unexpected));
        assert_eq!(x.interpret("5+6*7"), Ok(Some("47".to_string())));
        assert_eq!(x.interpret("5*(9-6)"), Ok(Some("15".to_string())));
        assert_eq!(x.interpret("(3+5) / 2"), Ok(Some("4".to_string())));
        assert_eq!(x.interpret("-10+(+20)"), Ok(Some("10".to_string())));
    }
}

