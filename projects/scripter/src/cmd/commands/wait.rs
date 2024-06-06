use crate::cmd::{panic_command, SubCommand};

#[derive(Debug, PartialEq, Eq)]
pub enum WaitCommand {
    Time(String),
}

impl SubCommand for WaitCommand {
    fn execute(&self) -> () {
        match self {
            WaitCommand::Time(t) => {
                println!("Wait for {}", t);
            }
        }
    }

    fn parse<T: Into<String>>(words: Vec<T>) -> Self {
        let words = words.into_iter().map(|s| s.into()).collect::<Vec<String>>();
        let panic_msg = words.join(" ");

        if words[0] != "wait" {
            panic_command(panic_msg);
        }

        let sub = words[1].as_str();
        let args = words[2..].to_vec();

        // command: wait time <time>
        match sub {
            "time" => return WaitCommand::Time(args.join(" ")),
            _ => panic_command(panic_msg),
        }
    }
}
