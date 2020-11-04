mod virtual_filesystem_core;
mod virtual_filesystem;


use virtual_filesystem_core::filesystem::FileNode;
use virtual_filesystem::shell::{CommandError, Buffer, Shell, run};


fn main() {
    println!("start interactive shell. Enjoy! :/");
    println!("to stop, press Ctrl + c or type exit");
    println!("if you need help, type :?");

    let root = FileNode::create_directory("".to_string(), vec![]).to_pointer();
    let current = root.clone();
    let shell = &mut Shell::new(root, current);
    
    loop {
        println!("$> ");
        let mut buffer = Buffer::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let buffer = buffer.trim();

        if buffer == "exit" { break }
        else if buffer == ":?" {
            println!("to stop, press Ctrl + c or type exit");
            println!("command list");
            println!("  ls");
            println!("  cd [directory]");
            println!("  find [path]");
            println!("  mkdir [directory]");
            println!("  touch [file]");
            println!("  read [file]");
            println!("  write [file] [string]");
            println!("  exit");
            continue
        }

        match run(shell, buffer) {
            Ok(None) => {},
            Ok(Some(response)) => { println!("{}", response) },
            Err(CommandError::NotFound) => { println!("not found.") },
            Err(CommandError::IllegalArgument) => { println!("illegal argument.") },
            Err(CommandError::NotFile) => { println!("not file.") },
            Err(CommandError::CommandNotFound(command)) => { println!("{} command not found.", command) },
        }
    }
}
