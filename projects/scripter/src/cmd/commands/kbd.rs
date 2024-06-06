use crate::cmd::{panic_command, SubCommand};

#[derive(Debug, PartialEq, Eq)]
pub enum KbdCommand {
    // press key
    P(String),
    // release key
    R(String),
    // type string
    T(String),
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
        // command: kbd {p,r,t} {key|words}
        // key can be a character or a special key, like space
        let words = words.into_iter().map(|s| s.into()).collect::<Vec<String>>();
        let panic_msg = words.join(" ");
        let sub = words[1].as_str();
        let args = words[2..].to_vec();

        match sub {
            "p" => KbdCommand::P(args[0].clone()),
            "r" => KbdCommand::R(args[0].clone()),
            "t" => KbdCommand::T(args.join(" ")),
            _ => panic_command(panic_msg),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse() {
        let kbd = KbdCommand::parse(vec!["kbd", "p", "a"]);
        assert_eq!(kbd, KbdCommand::P("a".to_string()));

        let kbd = KbdCommand::parse(vec!["kbd", "r", "space"]);
        assert_eq!(kbd, KbdCommand::R("space".to_string()));

        let kbd = KbdCommand::parse(vec!["kbd", "r", "space", "o"]);
        // should only get one key
        assert_eq!(kbd, KbdCommand::R("space".to_string()));

        let kbd = KbdCommand::parse(vec!["kbd", "t", "hello", "world"]);
        assert_eq!(kbd, KbdCommand::T("hello world".to_string()));
    }
}
