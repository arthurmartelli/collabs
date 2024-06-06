use enigo::{
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Settings,
};

use crate::cmd::{panic_command, SubCommand};

#[derive(Debug, PartialEq, Eq)]
pub enum KbdCommand {
    // press key
    Press(Key),
    // release key
    Release(Key),
    // click key
    Click(Key),
    // type string
    Type(String),
}

impl SubCommand for KbdCommand {
    fn execute(&self) {
        let mut enigo = Enigo::new(&Settings::default())
            .unwrap_or_else(|_| panic!("Unable to use the keyboard"));

        match self {
            KbdCommand::Press(s) => enigo
                .key(*s, Press)
                .unwrap_or_else(|_| panic!("Unable to press {:?}", s)),
            KbdCommand::Release(s) => enigo
                .key(*s, Release)
                .unwrap_or_else(|_| panic!("Unable to release {:?}", s)),
            KbdCommand::Click(s) => {
                enigo
                    .key(*s, Click)
                    .unwrap_or_else(|_| panic!("Unable to click {:?}", s));
            }
            KbdCommand::Type(s) => enigo
                .text(s)
                .unwrap_or_else(|_| panic!("Unable to type {}", s)),
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
            "press" => KbdCommand::Press(Key::from_string(&args[0])),
            "release" => KbdCommand::Release(Key::from_string(&args[0])),
            "click" => KbdCommand::Click(Key::from_string(&args[0])),
            "type" => KbdCommand::Type(args.join(" ")),
            _ => panic_command(panic_msg),
        }
    }
}

pub trait FromString {
    fn from_string<T: ToString>(st: &T) -> Self;
}

impl FromString for Key {
    fn from_string<T: ToString>(st: &T) -> Self {
        // special characters are handled first
        // they cannot have spaces in them
        match st.to_string().as_str() {
            "space" => Key::Space,
            "tab" => Key::Tab,
            "backspace" => Key::Backspace,
            "up" => Key::UpArrow,
            "down" => Key::DownArrow,
            "left" => Key::LeftArrow,
            "right" => Key::RightArrow,
            "insert" => Key::Insert,
            "delete" => Key::Delete,
            "home" => Key::Home,
            "end" => Key::End,
            "page_up" => Key::PageUp,
            "page_down" => Key::PageDown,
            "esc" => Key::Escape,
            _ => Key::Unicode(st.to_string().chars().next().unwrap()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse() {
        let key_a = Key::from_string(&"a");
        let key_space = Key::from_string(&"space");

        let kbd = KbdCommand::parse(vec!["kbd", "p", "a"]);
        assert_eq!(kbd, KbdCommand::Press(key_a));

        let kbd = KbdCommand::parse(vec!["kbd", "r", "space"]);
        assert_eq!(kbd, KbdCommand::Release(key_space));

        let kbd = KbdCommand::parse(vec!["kbd", "r", "space", "o"]);
        // should only get one key
        assert_eq!(kbd, KbdCommand::Release(key_space));

        let kbd = KbdCommand::parse(vec!["kbd", "t", "hello", "world"]);
        assert_eq!(kbd, KbdCommand::Type("hello world".to_string()));
    }
}
