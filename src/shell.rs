use crate::graph::{Edge, Graph};
use crate::filesystem::{FileNode, FileNodePointer, FileObject};
use crate::command::{ls, mkdir, touch, write, read, find};


type Buffer = String;
type Arg = str;
type CommandResult = Result<Option<String>, ()>;


#[derive(Debug)]
struct Shell {
    root: FileNodePointer,
    current: FileNodePointer,
}


fn run(shell: &mut Shell, buffer: &Arg) -> CommandResult {
    let argv: Vec<&Arg> = buffer.trim()
        .split(' ')
        .collect();
    let mut iter = argv.iter();

    let command = if let Some(head) = iter.next() {
        if head.is_empty() { return Ok(None); }
        *head
    } else { return Ok(None); };

    if command == ":?" {
        println!("to stop, press Ctrl + c or type exit");
        println!("Command list");
        println!("  ls");
        println!("  cd [directory]");
        println!("  find [path]");
        println!("  mkdir [directory]");
        println!("  touch [file]");
        println!("  read [file]");
        println!("  write [file] [string]");
        println!("  exit");
        Ok(None)
    } else if command == "ls" {
        let current = &shell.current.borrow();
        let result = ls(current);
        println!("{}", result);
        Ok(Some(result))
    } else if command == "cd" {
        let change = if let Some(arg) = iter.next() {
            let current = &shell.current.borrow();
            if let Ok(pointer) = find(current, arg) {
                Ok(pointer.clone())
            } else {
                println!("not found.");
                Err(())
            }
        } else {
            println!("illegal argument.");
            Err(())
        };
        if let Ok(change) = change {
            shell.current = change;
            Ok(None)
        } else {
            Err(())
        }
    } else if command == "find" {
        let current = &shell.current.borrow();
        if let Some(arg) = iter.next() {
            if let Ok(pointer) = find(current, arg) {
                let node = &pointer.borrow();
                let file = node.value();
                let name = file.name();
                println!("{}", name);
                Ok(Some(name.to_string()))
            } else {
                println!("not found.");
                Err(())
            }
        } else {
            println!("illegal argument.");
            Err(())
        }
    } else if command == "mkdir" {
        let current = &mut shell.current.borrow_mut();
        if let Some(arg) = iter.next() {
            mkdir(current, arg.to_string());
            Ok(None)
        } else {
            println!("illegal argument.");
            Err(())
        }
    } else if command == "touch" {
        let current = &mut shell.current.borrow_mut();
        if let Some(arg) = iter.next() {
            touch(current, arg.to_string(), "".to_string());
            Ok(None)
        } else {
            println!("illegal argument.");
            Err(())
        }
    } else if command == "read" {
        let current = &shell.current.borrow();
        if let Some(arg) = iter.next() {
            if let Ok(pointer) = find(current, arg) {
                let node = &pointer.borrow();
                if let Ok(data) = read(node) {
                    println!("{}", data);
                    Ok(Some(data.to_string()))
                } else {
                    println!("not a file.");
                    Err(())
                }
            } else {
                println!("not found.");
                Err(())
            }
        } else {
            println!("illegal argument.");
            Err(())
        }
    } else if command == "write" {
        let current = &mut shell.current.borrow_mut();
        if let Some(arg) = iter.next() {
            let mut index =
                buffer.rfind(arg).unwrap() + arg.len();
            for i in buffer[index..].chars() {
                if i.is_whitespace() { index += 1; }
                else { break; }
            }
            if index >= buffer.len() {
                println!("illegal argument.");
                return Err(());
            }
            let data = &buffer[index..];
            if let Ok(pointer) = find(current, arg) {
                let node = &mut pointer.borrow_mut();
                match write(node, data) {
                    Ok(()) => { Ok(None) },
                    Err(()) => {
                        println!("not a file.");
                        Err(())
                    },
                }
            } else {
                println!("not found.");
                Err(())
            }
        } else {
            println!("illegal argument.");
            Err(())
        }
    } else {
        println!("{} command not found.", command);
        Err(())
    }
}


pub fn interactive() {
    println!("start interactive shell. Enjoy! :/");
    println!("to stop, press Ctrl + c or type exit");
    println!("if you need help, type :?");

    let root = FileNode::create_directory("".to_string(), Edge::new()).to_pointer();
    let current = root.clone();
    let shell = &mut Shell{
        root: root,
        current: current,
    };
    
    loop {
        println!("$> ");
        let mut buffer = Buffer::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let buffer = buffer.trim();

        if buffer == "exit" { break; }

        let _ = run(shell, buffer);
    }
}


#[cfg(test)]
mod test {
    use crate::filesystem::FileNode;
    use crate::shell::{Shell, run};

    #[test]
    fn test_enter() {
        let root = FileNode::create_directory("".to_string(), vec![]).to_pointer();
        let current = root.clone();
        let shell = &mut Shell{
            root: root,
            current: current,
        };

        let buffer = "";
        assert_eq!(run(shell, buffer), Ok(None));
    }

    #[test]
    fn test_exit() {
        let root = FileNode::create_directory("".to_string(), vec![]).to_pointer();
        let current = root.clone();
        let shell = &mut Shell{
            root: root,
            current: current,
        };

        let buffer = "exit";
        assert_eq!(run(shell, buffer), Err(()));
    }

    #[test]
    fn test_help() {
        let root = FileNode::create_directory("".to_string(), vec![]).to_pointer();
        let current = root.clone();
        let shell = &mut Shell{
            root: root,
            current: current,
        };

        let buffer = ":?";
        assert_eq!(run(shell, buffer), Ok(None));
    }

    #[test]
    fn test_ls() {
        let root = FileNode::create_directory("".to_string(), vec![]).to_pointer();
        let current = root.clone();
        let shell = &mut Shell{
            root: root,
            current: current,
        };

        let buffer = "ls";
        assert_eq!(run(shell, buffer), Ok(Some("".to_string())));

        let buffer = "ls a";
        assert_eq!(run(shell, buffer), Ok(Some("".to_string())));

        let buffer = "ls b c";
        assert_eq!(run(shell, buffer), Ok(Some("".to_string())));

        let buffer = "mkdir a";
        assert_eq!(run(shell, buffer), Ok(None));

        let buffer = "ls";
        assert_eq!(run(shell, buffer), Ok(Some("a".to_string())));

        let buffer = "ls a";
        assert_eq!(run(shell, buffer), Ok(Some("a".to_string())));

        let buffer = "ls b c";
        assert_eq!(run(shell, buffer), Ok(Some("a".to_string())));
    }

    #[test]
    fn test_cd() {
        let root = FileNode::create_directory("".to_string(), vec![]).to_pointer();
        let current = root.clone();
        let shell = &mut Shell{
            root: root,
            current: current,
        };

        let buffer = "cd";
        assert_eq!(run(shell, buffer), Err(()));

        let buffer = "cd a";
        assert_eq!(run(shell, buffer), Err(()));

        let buffer = "cd b c";
        assert_eq!(run(shell, buffer), Err(()));

        let buffer = "mkdir a";
        assert_eq!(run(shell, buffer), Ok(None));

        let buffer = "cd a";
        assert_eq!(run(shell, buffer), Ok(None));

        let buffer = "mkdir b";
        assert_eq!(run(shell, buffer), Ok(None));

        let buffer = "cd b c";
        assert_eq!(run(shell, buffer), Ok(None));
    }

    #[test]
    fn test_find() {
        let root = FileNode::create_directory("".to_string(), vec![]).to_pointer();
        let current = root.clone();
        let shell = &mut Shell{
            root: root,
            current: current,
        };

        let buffer = "find";
        assert_eq!(run(shell, buffer), Err(()));

        let buffer = "find a";
        assert_eq!(run(shell, buffer), Err(()));

        let buffer = "find b c";
        assert_eq!(run(shell, buffer), Err(()));

        let buffer = "mkdir a";
        assert_eq!(run(shell, buffer), Ok(None));

        let buffer = "find a";
        assert_eq!(run(shell, buffer), Ok(Some("a".to_string())));

        let buffer = "mkdir b";
        assert_eq!(run(shell, buffer), Ok(None));

        let buffer = "find b c";
        assert_eq!(run(shell, buffer), Ok(Some("b".to_string())));
    }

    #[test]
    fn test_mkdir() {
        let root = FileNode::create_directory("".to_string(), vec![]).to_pointer();
        let current = root.clone();
        let shell = &mut Shell{
            root: root,
            current: current,
        };

        let buffer = "mkdir";
        assert_eq!(run(shell, buffer), Err(()));

        let buffer = "mkdir a";
        assert_eq!(run(shell, buffer), Ok(None));

        let buffer = "mkdir b c";
        assert_eq!(run(shell, buffer), Ok(None));

        let buffer = "mkdir a";
        assert_eq!(run(shell, buffer), Ok(None));

        let buffer = "ls";
        assert_eq!(run(shell, buffer), Ok(Some("a\tb\ta".to_string())));
    }

    #[test]
    fn test_touch() {
        let root = FileNode::create_directory("".to_string(), vec![]).to_pointer();
        let current = root.clone();
        let shell = &mut Shell{
            root: root,
            current: current,
        };

        let buffer = "touch";
        assert_eq!(run(shell, buffer), Err(()));

        let buffer = "touch a";
        assert_eq!(run(shell, buffer), Ok(None));

        let buffer = "touch b c";
        assert_eq!(run(shell, buffer), Ok(None));

        let buffer = "touch a";
        assert_eq!(run(shell, buffer), Ok(None));

        let buffer = "ls";
        assert_eq!(run(shell, buffer), Ok(Some("a\tb\ta".to_string())));
    }

    #[test]
    fn test_read() {
        let root = FileNode::create_directory("".to_string(), vec![]).to_pointer();
        let current = root.clone();
        let shell = &mut Shell{
            root: root,
            current: current,
        };

        let buffer = "read";
        assert_eq!(run(shell, buffer), Err(()));

        let buffer = "read a";
        assert_eq!(run(shell, buffer), Err(()));

        let buffer = "read a b";
        assert_eq!(run(shell, buffer), Err(()));

        let buffer = "touch a";
        assert_eq!(run(shell, buffer), Ok(None));

        let buffer = "read a";
        assert_eq!(run(shell, buffer), Ok(Some("".to_string())));

        let buffer = "write a 123";
        assert_eq!(run(shell, buffer), Ok(None));

        let buffer = "read a";
        assert_eq!(run(shell, buffer), Ok(Some("123".to_string())));

        let buffer = "read a b";
        assert_eq!(run(shell, buffer), Ok(Some("123".to_string())));
    }

    #[test]
    fn test_write() {
        let root = FileNode::create_directory("".to_string(), vec![]).to_pointer();
        let current = root.clone();
        let shell = &mut Shell{
            root: root,
            current: current,
        };

        let buffer = "write";
        assert_eq!(run(shell, buffer), Err(()));

        let buffer = "write a";
        assert_eq!(run(shell, buffer), Err(()));

        let buffer = "write a 123";
        assert_eq!(run(shell, buffer), Err(()));

        let buffer = "touch a";
        assert_eq!(run(shell, buffer), Ok(None));

        let buffer = "read a";
        assert_eq!(run(shell, buffer), Ok(Some("".to_string())));

        let buffer = "write a";
        assert_eq!(run(shell, buffer), Err(()));

        let buffer = "write a 123";
        assert_eq!(run(shell, buffer), Ok(None));

        let buffer = "read a";
        assert_eq!(run(shell, buffer), Ok(Some("123".to_string())));

        let buffer = "write a 123 456 xyz";
        assert_eq!(run(shell, buffer), Ok(None));

        let buffer = "read a";
        assert_eq!(run(shell, buffer), Ok(Some("123123 456 xyz".to_string())));
    }
}

