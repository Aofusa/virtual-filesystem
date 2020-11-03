use crate::graph::{Edge, Graph};
use crate::filesystem::{FileNode, FileNodePointer, FileObject};
use crate::command::{ls, mkdir, touch, write, read, find};


type Buffer = String;
type Arg = str;

#[derive(Debug)]
struct Shell {
    root: FileNodePointer,
    current: FileNodePointer,
}


fn run(shell: &mut Shell, buffer: &Arg) {
    let argv: Vec<&Arg> = buffer.split(' ').collect();
    let argc = argv.len();
    let mut iter = argv.iter();

    if argc < 1 { return; }

    let command = *iter.next().unwrap();

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
    } else if command == "ls" {
        let current = &shell.current.borrow();
        println!("{}", ls(current));
    } else if command == "cd" {
        let change: Result<FileNodePointer, ()>;
        {
            let current = &shell.current.borrow();
            let arg = *iter.next().unwrap();
            if let Ok(pointer) = find(current, arg.to_string()) {
                change = Ok(pointer.clone());
            } else {
                change = Err(());
                println!("not found.");
            }
        }
        if let Ok(change) = change {
            shell.current = change;
        }
    } else if command == "find" {
        let current = &shell.current.borrow();
        let arg = *iter.next().unwrap();
        if let Ok(pointer) = find(current, arg.to_string()) {
            let node = &pointer.borrow();
            let file = node.value();
            let name = file.name();
            println!("{}", name);
        } else {
            println!("not found.");
        }
    } else if command == "mkdir" {
        let current = &mut shell.current.borrow_mut();
        let arg = *iter.next().unwrap();
        mkdir(current, arg.to_string());
    } else if command == "touch" {
        let current = &mut shell.current.borrow_mut();
        let arg = *iter.next().unwrap();
        touch(current, arg.to_string(), "".to_string());
    } else if command == "read" {
        let current = &shell.current.borrow();
        let arg = *iter.next().unwrap();
        if let Ok(pointer) = find(current, arg.to_string()) {
            let node = &pointer.borrow();
            let data = read(node);
            println!("{}", data);
        } else {
            println!("not found.");
        }
    } else if command == "write" {
        let current = &mut shell.current.borrow_mut();
        let arg = *iter.next().unwrap();
        let mut index =
            buffer.rfind(arg).unwrap() +
            arg.len();
        for i in buffer[index..].chars() {
            if i.is_whitespace() { index += 1; }
            else { break; }
        }
        let data = &buffer[index..];
        if let Ok(pointer) = find(current, arg.to_string()) {
            let node = &mut pointer.borrow_mut();
            write(node, data.to_string());
        } else {
            println!("not found.");
        }
    } else {
        println!("{} command not found.", command);
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

        run(shell, buffer);
    }
}

