
extern crate web_sys;
extern crate virtual_filesystem;

mod utils;

use wasm_bindgen::prelude::*;
use virtual_filesystem::virtual_filesystem::shell::{CommandError, Shell};
use virtual_filesystem::virtual_filesystem_core::logger::LoggerRepository;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct ConsoleLoggerRepository {}
impl LoggerRepository for ConsoleLoggerRepository {
    fn print(&self, message: &str) {
        log!("{}", message);
    }
}

#[wasm_bindgen]
pub struct Cli {
    shell: Shell<ConsoleLoggerRepository>,
}

#[wasm_bindgen]
impl Cli {
    pub fn new() -> Cli {
        utils::set_panic_hook();
        Cli {
            shell: Shell::init_with_logger(ConsoleLoggerRepository{})
        }
    }

    pub fn run(&mut self, command: &str) -> String {
        match self.shell.run(command) {
            Ok(None) => { "".to_string() },
            Ok(Some(response)) => { format!("{}", response) },
            Err(CommandError::UnknownError) => { format!("unknown error.") },
            Err(CommandError::NotFound) => { format!("not found.") },
            Err(CommandError::IllegalArgument) => { format!("illegal argument.") },
            Err(CommandError::NotFile) => { format!("not file.") },
            Err(CommandError::CommandNotFound(command)) => { format!("{} command not found.", command) },
        }
    }
}

