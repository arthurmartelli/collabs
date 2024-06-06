use file::{get_file_name, get_file_reader};
use std::io::BufRead;

mod file;

fn main() {
    let file_name = get_file_name();
    let reader = get_file_reader(file_name);

    reader
        .lines()
        .for_each(|line| println!("{}", line.unwrap()))
}
