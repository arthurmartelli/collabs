use crate::cmd::{panic_command, SubCommand};

#[derive(Debug, PartialEq, Eq)]
pub enum Time {
    MilliSeconds(u32),
    Seconds(u32),
    Minutes(u32),
    Hours(u32),
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Time::MilliSeconds(ms) => write!(f, "{} milliseconds", ms),
            Time::Seconds(s) => write!(f, "{} seconds", s),
            Time::Minutes(m) => write!(f, "{} minutes", m),
            Time::Hours(h) => write!(f, "{} hours", h),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum WaitCommand {
    Time(Time),
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
        // command: wait time <time>
        let words = words.into_iter().map(|s| s.into()).collect::<Vec<String>>();
        let panic_msg = words.join(" ");
        let sub = words[1].as_str();
        let args = words[2..].to_vec();

        let time = match args.len() {
            0 => Time::MilliSeconds(0),
            1 => {
                let t = args[0].parse::<u32>().unwrap();
                Time::Seconds(t)
            }
            2 => {
                let t = args[0].parse::<u32>().unwrap();
                match args[1].as_str() {
                    "milliseconds" => Time::MilliSeconds(t),
                    "seconds" => Time::Seconds(t),
                    "minutes" => Time::Minutes(t),
                    "hours" => Time::Hours(t),
                    _ => panic_command(panic_msg),
                }
            }
            _ => panic_command(panic_msg),
        };

        match sub {
            "time" => return WaitCommand::Time(time),
            _ => panic_command(panic_msg),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse() {
        let words = vec!["wait", "time", "1", "seconds"];
        let cmd = WaitCommand::parse(words);
        assert_eq!(cmd, WaitCommand::Time(Time::Seconds(1)));

        let words = vec!["wait", "time", "1", "milliseconds"];
        let cmd = WaitCommand::parse(words);
        assert_eq!(cmd, WaitCommand::Time(Time::MilliSeconds(1)));

        let words = vec!["wait", "time", "1", "minutes"];
        let cmd = WaitCommand::parse(words);
        assert_eq!(cmd, WaitCommand::Time(Time::Minutes(1)));

        let words = vec!["wait", "time", "1", "hours"];
        let cmd = WaitCommand::parse(words);
        assert_eq!(cmd, WaitCommand::Time(Time::Hours(1)));

        let words = vec!["wait", "time"];
        let cmd = WaitCommand::parse(words);
        assert_eq!(cmd, WaitCommand::Time(Time::MilliSeconds(0)));
    }
}
