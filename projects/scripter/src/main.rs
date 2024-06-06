use std::env;

use cmd::Commands;

mod cmd;
mod file;

fn main() {
    // first argument is the file name
    let file_name = env::args()
        .collect::<Vec<String>>()
        .get(1)
        .unwrap_or_else(|| panic!("No file name provided"))
        .trim()
        .to_string();

    let commands = Commands::parse_file(file_name);

    for c in commands.iter() {
        c.execute();
    }
}
