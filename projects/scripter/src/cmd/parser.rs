use std::io::{BufRead, BufReader};

use super::{
    commands::{click::ClickCommand, kbd::KbdCommand, wait::WaitCommand},
    Commands, SubCommand,
};

fn parse_line(line: String) -> Commands {
    // lowercase words
    let line = line.to_lowercase();
    let words = line.split_whitespace().collect::<Vec<&str>>();
    let cmd = words.first().unwrap();

    match cmd {
        &"wait" => Commands::WAIT(WaitCommand::parse(words)),
        &"kbd" => Commands::KBD(KbdCommand::parse(words)),
        &"click" => Commands::CLICK(ClickCommand::parse(words)),
        _ => panic!("Unknown command: {}", cmd),
    }
}

pub fn parse_buffer<T: std::io::Read>(reader: BufReader<T>) -> Vec<Commands> {
    reader
        .lines()
        .map(|line| parse_line(line.unwrap()))
        .collect::<_>()
}

#[cfg(test)]
mod tests {
    use crate::cmd::commands::wait::Time;

    use super::*;
    use std::io::Cursor;
    #[test]
    fn test_parse_line() {
        let line = "wait time 1 seconds".to_string();
        let cmd = parse_line(line);
        assert_eq!(cmd, Commands::WAIT(WaitCommand::Time(Time::Seconds(1))));
    }

    #[test]
    fn test_parse_buffer() {
        let buffer = "wait time 1\nkbd p a\nkbd r a\nkbd t abc".to_string();
        let reader = BufReader::new(Cursor::new(buffer));
        parse_buffer(reader);
    }
}
