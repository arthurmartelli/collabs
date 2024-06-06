pub mod commands;
mod parser;

use crate::file::get_file_reader;
use parser::parse_buffer;

pub(crate) trait SubCommand {
    fn execute(&self) -> ();
    fn parse<T: Into<String>>(words: Vec<T>) -> Self;
}

#[derive(Debug, PartialEq, Eq)]
pub enum Commands {
    WAIT(commands::wait::WaitCommand),
    KBD(commands::kbd::KbdCommand),
    MOUSE(commands::mouse::MouseCommand),
}

impl Commands {
    pub fn parse_file<T: Into<String>>(file_name: T) -> Vec<Commands> {
        let file_name = file_name.into();
        let reader = get_file_reader(file_name);
        parse_buffer(reader)
    }

    pub fn execute(&self) -> () {
        match self {
            Commands::WAIT(cmd) => cmd.execute(),
            Commands::KBD(cmd) => cmd.execute(),
            Commands::MOUSE(cmd) => cmd.execute(),
        }
    }
}

fn panic_command<T: std::fmt::Display>(command: T) -> ! {
    panic!("Error in command: {}", command)
}
