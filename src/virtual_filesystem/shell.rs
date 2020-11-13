use crate::virtual_filesystem_core::graph::Graph;
use crate::virtual_filesystem_core::filesystem::{FileNode, FileNodePointer, FileObject};
use crate::utils::logger::{LoggerRepository, LoggerInteractor};
use crate::virtual_filesystem::command::{ls, pwd, mkdir, touch, write, read, find};


pub type Buffer = String;
pub type Arg = str;
pub type CommandResult = Result<Option<String>, CommandError>;


#[derive(Debug, PartialEq)]
pub enum CommandError {
    UnknownError,
    NotFound,
    IllegalArgument,
    NotFile,
    CommandNotFound(String),
}


#[derive(Debug)]
pub struct Shell<T>
where
    T: LoggerRepository,
{
    pub root: FileNodePointer,
    pub current: FileNodePointer,
    logger: LoggerInteractor<T>,
}


pub struct DefaultLoggerRepository {}
impl LoggerRepository for DefaultLoggerRepository {
    fn print(&self, _message: &str) {}
}


impl Shell<DefaultLoggerRepository> {
    #[allow(dead_code)]
    pub fn init() -> Shell<DefaultLoggerRepository> {
        Shell::init_with_logger(DefaultLoggerRepository{})
    }
}


impl<T: LoggerRepository> Shell<T> {
    fn new(root: FileNodePointer, current: FileNodePointer, logger: T) -> Shell<T> {
        Shell {
            root: root,
            current: current,
            logger: LoggerInteractor::new(logger),
        }
    }

    pub fn init_with_logger(logger: T) -> Shell<T> {
        let root = FileNode::create_directory("".to_string(), vec![]).to_pointer();
        let current = root.clone();
        root.borrow_mut().connect(current.clone());
        Shell::new(root, current, logger)
    }

    #[allow(dead_code)]
    pub fn replace_logger<R: LoggerRepository>(&self, logger: R) -> Shell<R> {
        Shell::new(self.root.clone(), self.current.clone(), logger)
    }

    pub fn run(&mut self, buffer: &Arg) -> CommandResult {
        self.logger.print(&format!("run arg {}", buffer));
        let argv: Vec<&Arg> = buffer.trim()
            .split(' ')
            .collect();
        let mut iter = argv.iter();

        let command = if let Some(head) = iter.next() {
            if head.is_empty() { return Ok(None); }
            *head
        } else { return Ok(None); };

        if command == "ls" {
            let current = &self.current;
            let result = ls(current);
            Ok(Some(result))
        } else if command == "pwd" {
            let current = &self.current;
            let result = pwd(current);
            Ok(Some(result))
        } else if command == "cd" {
            let change = if let Some(arg) = iter.next() {
                let current = &self.current;
                if arg == &"/" {
                    Ok(self.root.clone())
                } else if arg == &"." {
                    Ok(current.clone())
                } else if arg == &".." {
                    if let Some(parent) = current.borrow().1.iter().next() {
                        Ok(parent.clone())
                    } else {
                        Err(CommandError::UnknownError)
                    }
                } else {
                    if let Ok(pointer) = find(current, arg) {
                        Ok(pointer.clone())
                    } else {
                        Err(CommandError::NotFound)
                    }
                }
            } else {
                Err(CommandError::IllegalArgument)
            };
            if let Ok(change) = change {
                self.current = change;
                Ok(None)
            } else {
                Err(change.unwrap_err())
            }
        } else if command == "find" {
            let current = &self.current;
            if let Some(arg) = iter.next() {
                if let Ok(pointer) = find(current, arg) {
                    let node = &pointer.borrow();
                    let file = &node.0;
                    let name = file.name();
                    Ok(Some(name.to_string()))
                } else {
                    Err(CommandError::NotFound)
                }
            } else {
                Err(CommandError::IllegalArgument)
            }
        } else if command == "mkdir" {
            let current = &mut self.current;
            if let Some(arg) = iter.next() {
                mkdir(current, arg.to_string());
                Ok(None)
            } else {
                Err(CommandError::IllegalArgument)
            }
        } else if command == "touch" {
            let current = &mut self.current;
            if let Some(arg) = iter.next() {
                touch(current, arg.to_string(), "".to_string());
                Ok(None)
            } else {
                Err(CommandError::IllegalArgument)
            }
        } else if command == "read" {
            let current = &self.current;
            if let Some(arg) = iter.next() {
                if let Ok(pointer) = find(current, arg) {
                    let node = &pointer;
                    if let Ok(data) = read(node) {
                        Ok(Some(data.to_string()))
                    } else {
                        Err(CommandError::NotFile)
                    }
                } else {
                    Err(CommandError::NotFound)
                }
            } else {
                Err(CommandError::IllegalArgument)
            }
        } else if command == "write" {
            let current = &mut self.current;
            if let Some(arg) = iter.next() {
                let mut index =
                    buffer.rfind(arg).unwrap() + arg.len();
                for i in buffer[index..].chars() {
                    if i.is_whitespace() { index += 1; }
                    else { break; }
                }
                if index >= buffer.len() {
                    return Err(CommandError::IllegalArgument);
                }
                let data = &buffer[index..];
                if let Ok(pointer) = find(current, arg) {
                    let node = &pointer;
                    match write(node, data) {
                        Ok(()) => { Ok(None) },
                        Err(()) => {
                            Err(CommandError::NotFile)
                        },
                    }
                } else {
                    Err(CommandError::NotFound)
                }
            } else {
                Err(CommandError::IllegalArgument)
            }
        } else {
            Err(CommandError::CommandNotFound(command.to_string()))
        }
    }
}


#[cfg(test)]
mod test {
    use crate::virtual_filesystem::shell::{CommandError, Shell};

    #[test]
    fn test_enter() {
        let shell = &mut Shell::init();

        let buffer = "";
        assert_eq!(shell.run(buffer), Ok(None));
    }

    #[test]
    fn test_exit() {
        let shell = &mut Shell::init();

        let buffer = "exit";
        assert_eq!(shell.run(buffer), Err(CommandError::CommandNotFound(buffer.to_string())));
    }

    #[test]
    fn test_help() {
        let shell = &mut Shell::init();

        let buffer = ":?";
        assert_eq!(shell.run(buffer), Err(CommandError::CommandNotFound(buffer.to_string())));
    }

    #[test]
    fn test_ls() {
        let shell = &mut Shell::init();

        let buffer = "ls";
        assert_eq!(shell.run(buffer), Ok(Some("".to_string())));

        let buffer = "ls a";
        assert_eq!(shell.run(buffer), Ok(Some("".to_string())));

        let buffer = "ls b c";
        assert_eq!(shell.run(buffer), Ok(Some("".to_string())));

        let buffer = "mkdir a";
        assert_eq!(shell.run(buffer), Ok(None));

        let buffer = "ls";
        assert_eq!(shell.run(buffer), Ok(Some("a".to_string())));

        let buffer = "ls a";
        assert_eq!(shell.run(buffer), Ok(Some("a".to_string())));

        let buffer = "ls b c";
        assert_eq!(shell.run(buffer), Ok(Some("a".to_string())));
    }

    #[test]
    fn test_pwd() {
        let shell = &mut Shell::init();

        let buffer = "pwd";
        assert_eq!(shell.run(buffer), Ok(Some("/".to_string())));

        let buffer = "pwd a";
        assert_eq!(shell.run(buffer), Ok(Some("/".to_string())));

        let buffer = "mkdir a";
        assert_eq!(shell.run(buffer), Ok(None));

        let buffer = "cd a";
        assert_eq!(shell.run(buffer), Ok(None));

        let buffer = "pwd";
        assert_eq!(shell.run(buffer), Ok(Some("/a".to_string())));
    }

    #[test]
    fn test_cd() {
        let shell = &mut Shell::init();

        let buffer = "cd";
        assert_eq!(shell.run(buffer), Err(CommandError::IllegalArgument));

        let buffer = "cd a";
        assert_eq!(shell.run(buffer), Err(CommandError::NotFound));

        let buffer = "cd b c";
        assert_eq!(shell.run(buffer), Err(CommandError::NotFound));

        let buffer = "pwd";
        assert_eq!(shell.run(buffer), Ok(Some("/".to_string())));

        let buffer = "cd .";
        assert_eq!(shell.run(buffer), Ok(None));

        let buffer = "pwd";
        assert_eq!(shell.run(buffer), Ok(Some("/".to_string())));

        let buffer = "cd ..";
        assert_eq!(shell.run(buffer), Ok(None));

        let buffer = "pwd";
        assert_eq!(shell.run(buffer), Ok(Some("/".to_string())));

        let buffer = "mkdir a";
        assert_eq!(shell.run(buffer), Ok(None));

        let buffer = "cd a";
        assert_eq!(shell.run(buffer), Ok(None));

        let buffer = "cd .";
        assert_eq!(shell.run(buffer), Ok(None));

        let buffer = "pwd";
        assert_eq!(shell.run(buffer), Ok(Some("/a".to_string())));

        let buffer = "mkdir b";
        assert_eq!(shell.run(buffer), Ok(None));

        let buffer = "cd b c";
        assert_eq!(shell.run(buffer), Ok(None));

        let buffer = "cd .";
        assert_eq!(shell.run(buffer), Ok(None));

        let buffer = "pwd";
        assert_eq!(shell.run(buffer), Ok(Some("/a/b".to_string())));

        let buffer = "cd ..";
        assert_eq!(shell.run(buffer), Ok(None));

        let buffer = "pwd";
        assert_eq!(shell.run(buffer), Ok(Some("/a".to_string())));

        let buffer = "cd /";
        assert_eq!(shell.run(buffer), Ok(None));

        let buffer = "pwd";
        assert_eq!(shell.run(buffer), Ok(Some("/".to_string())));
    }

    #[test]
    fn test_find() {
        let shell = &mut Shell::init();

        let buffer = "find";
        assert_eq!(shell.run(buffer), Err(CommandError::IllegalArgument));

        let buffer = "find a";
        assert_eq!(shell.run(buffer), Err(CommandError::NotFound));

        let buffer = "find b c";
        assert_eq!(shell.run(buffer), Err(CommandError::NotFound));

        let buffer = "mkdir a";
        assert_eq!(shell.run(buffer), Ok(None));

        let buffer = "find a";
        assert_eq!(shell.run(buffer), Ok(Some("a".to_string())));

        let buffer = "mkdir b";
        assert_eq!(shell.run(buffer), Ok(None));

        let buffer = "find b c";
        assert_eq!(shell.run(buffer), Ok(Some("b".to_string())));
    }

    #[test]
    fn test_mkdir() {
        let shell = &mut Shell::init();

        let buffer = "mkdir";
        assert_eq!(shell.run(buffer), Err(CommandError::IllegalArgument));

        let buffer = "mkdir a";
        assert_eq!(shell.run(buffer), Ok(None));

        let buffer = "mkdir b c";
        assert_eq!(shell.run(buffer), Ok(None));

        let buffer = "mkdir a";
        assert_eq!(shell.run(buffer), Ok(None));

        let buffer = "ls";
        assert_eq!(shell.run(buffer), Ok(Some("a\tb\ta".to_string())));
    }

    #[test]
    fn test_touch() {
        let shell = &mut Shell::init();

        let buffer = "touch";
        assert_eq!(shell.run(buffer), Err(CommandError::IllegalArgument));

        let buffer = "touch a";
        assert_eq!(shell.run(buffer), Ok(None));

        let buffer = "touch b c";
        assert_eq!(shell.run(buffer), Ok(None));

        let buffer = "touch a";
        assert_eq!(shell.run(buffer), Ok(None));

        let buffer = "ls";
        assert_eq!(shell.run(buffer), Ok(Some("a\tb\ta".to_string())));
    }

    #[test]
    fn test_read() {
        let shell = &mut Shell::init();

        let buffer = "read";
        assert_eq!(shell.run(buffer), Err(CommandError::IllegalArgument));

        let buffer = "read a";
        assert_eq!(shell.run(buffer), Err(CommandError::NotFound));

        let buffer = "read a b";
        assert_eq!(shell.run(buffer), Err(CommandError::NotFound));

        let buffer = "touch a";
        assert_eq!(shell.run(buffer), Ok(None));

        let buffer = "read a";
        assert_eq!(shell.run(buffer), Ok(Some("".to_string())));

        let buffer = "write a 123";
        assert_eq!(shell.run(buffer), Ok(None));

        let buffer = "read a";
        assert_eq!(shell.run(buffer), Ok(Some("123".to_string())));

        let buffer = "read a b";
        assert_eq!(shell.run(buffer), Ok(Some("123".to_string())));

        let buffer = "mkdir dir";
        assert_eq!(shell.run(buffer), Ok(None));

        let buffer = "read dir";
        assert_eq!(shell.run(buffer), Err(CommandError::NotFile));
    }

    #[test]
    fn test_write() {
        let shell = &mut Shell::init();

        let buffer = "write";
        assert_eq!(shell.run(buffer), Err(CommandError::IllegalArgument));

        let buffer = "write a";
        assert_eq!(shell.run(buffer), Err(CommandError::IllegalArgument));

        let buffer = "write a 123";
        assert_eq!(shell.run(buffer), Err(CommandError::NotFound));

        let buffer = "touch a";
        assert_eq!(shell.run(buffer), Ok(None));

        let buffer = "read a";
        assert_eq!(shell.run(buffer), Ok(Some("".to_string())));

        let buffer = "write a";
        assert_eq!(shell.run(buffer), Err(CommandError::IllegalArgument));

        let buffer = "write a 123";
        assert_eq!(shell.run(buffer), Ok(None));

        let buffer = "read a";
        assert_eq!(shell.run(buffer), Ok(Some("123".to_string())));

        let buffer = "write a 123 456 xyz";
        assert_eq!(shell.run(buffer), Ok(None));

        let buffer = "read a";
        assert_eq!(shell.run(buffer), Ok(Some("123123 456 xyz".to_string())));

        let buffer = "mkdir dir";
        assert_eq!(shell.run(buffer), Ok(None));

        let buffer = "write dir string";
        assert_eq!(shell.run(buffer), Err(CommandError::NotFile));
    }
}

