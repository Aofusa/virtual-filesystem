mod virtual_filesystem_core;
mod virtual_filesystem;


use virtual_filesystem::command::pwd;
use virtual_filesystem::shell::{CommandError, Buffer, Shell};
use virtual_filesystem_core::logger::LoggerRepository;


struct MockLoggerRepository {}
impl LoggerRepository for MockLoggerRepository {
    fn print(&self, message: &str) {
        println!("{}", message);
    }
}


fn main() {
    println!("start interactive shell. Enjoy! :/");
    println!("to stop, press Ctrl + c or type exit");
    println!("if you need help, type :?");

    let mut shell = Shell::init();
    
    loop {
        println!("[{}] $> ", pwd(&shell.current));
        let mut buffer = Buffer::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let buffer = buffer.trim();

        if buffer == "exit" { break }
        else if buffer == ":?" {
            println!("to stop, press Ctrl + c or type exit");
            println!("command list");
            println!("  ls");
            println!("  pwd");
            println!("  cd [directory]");
            println!("  find [path]");
            println!("  mkdir [directory]");
            println!("  touch [file]");
            println!("  read [file]");
            println!("  write [file] [string]");
            println!("  exit");
            continue
        }

        match shell.run(buffer) {
            Ok(None) => {},
            Ok(Some(response)) => { println!("{}", response) },
            Err(CommandError::UnknownError) => { println!("unknown error.") },
            Err(CommandError::NotFound) => { println!("not found.") },
            Err(CommandError::IllegalArgument) => { println!("illegal argument.") },
            Err(CommandError::NotFile) => { println!("not file.") },
            Err(CommandError::CommandNotFound(command)) => { println!("{} command not found.", command) },
        }
    }
}
