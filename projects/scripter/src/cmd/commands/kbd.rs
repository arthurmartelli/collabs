use crate::cmd::{panic_command, SubCommand};

#[derive(Debug, PartialEq, Eq)]
pub enum KbdCommand {
    P(char),   // press key
    R(char),   // release key
    T(String), // type string
}

impl SubCommand for KbdCommand {
    fn execute(&self) {
        match self {
            KbdCommand::P(c) => println!("press {}", c),
            KbdCommand::R(c) => println!("release {}", c),
            KbdCommand::T(s) => println!("type {}", s),
        }
    }

    fn parse<T: Into<String>>(words: Vec<T>) -> Self {
        // command: kbd {p,r,t} {char|words}
        let words = words.into_iter().map(|s| s.into()).collect::<Vec<String>>();
        let panic_msg = words.join(" ");

        if words[0] != "kbd" {
            panic_command(panic_msg);
        }

        let sub = words[1].as_str();
        let args = words[2..].to_vec();

        // command: wait time <time>
        match sub {
            "p" => KbdCommand::P(args[0].chars().next().unwrap()),
            "r" => KbdCommand::R(args[0].chars().next().unwrap()),
            "t" => KbdCommand::T(args.join(" ")),
            _ => panic_command(panic_msg),
        }
    }
}
