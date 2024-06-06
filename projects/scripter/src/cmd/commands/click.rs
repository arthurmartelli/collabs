use crate::cmd::{panic_command, SubCommand};

#[derive(Debug, PartialEq, Eq)]
pub enum Mouse {
    Right,
    Left,
}

impl std::fmt::Display for Mouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mouse::Right => write!(f, "right"),
            Mouse::Left => write!(f, "left"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ClickCommand {
    // press
    P(Mouse),
    // release
    R(Mouse),
    // Double click
    D(Mouse),
    // Triple click
    T(Mouse),
}

impl SubCommand for ClickCommand {
    fn execute(&self) {
        match self {
            ClickCommand::P(m) => println!("press {}", m),
            ClickCommand::R(m) => println!("release {}", m),
            ClickCommand::D(m) => println!("double click {}", m),
            ClickCommand::T(m) => println!("triple click {}", m),
        }
    }

    fn parse<T: Into<String>>(words: Vec<T>) -> Self {
        // command: click {p,r,d,t} {r|l}
        let words = words.into_iter().map(|s| s.into()).collect::<Vec<String>>();
        let panic_msg = words.join(" ");
        let sub = words[1].as_str();
        let args = words[2].as_str();

        let mouse: Mouse = match args {
            "r" => Mouse::Right,
            "l" => Mouse::Left,
            _ => panic_command(panic_msg),
        };

        match sub {
            "p" => ClickCommand::P(mouse),
            "r" => ClickCommand::R(mouse),
            "d" => ClickCommand::D(mouse),
            "t" => ClickCommand::T(mouse),
            _ => panic_command(panic_msg),
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse() {
        let words = vec!["click", "p", "r"];
        let cmd = ClickCommand::parse(words);
        assert_eq!(cmd, ClickCommand::P(Mouse::Right));

        let words = vec!["click", "p", "l"];
        let cmd = ClickCommand::parse(words);
        assert_eq!(cmd, ClickCommand::P(Mouse::Left));

        let words = vec!["click", "r", "r"];
        let cmd = ClickCommand::parse(words);
        assert_eq!(cmd, ClickCommand::R(Mouse::Right));

        let words = vec!["click", "d", "l"];
        let cmd = ClickCommand::parse(words);
        assert_eq!(cmd, ClickCommand::D(Mouse::Left));

        let words = vec!["click", "t", "r"];
        let cmd = ClickCommand::parse(words);
        assert_eq!(cmd, ClickCommand::T(Mouse::Right));
    }
}
