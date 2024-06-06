use crate::cmd::{panic_command, SubCommand};
use enigo::{
    Axis, Button, Coordinate,
    Direction::{Click, Press, Release},
    Enigo, Mouse, Settings,
};

#[derive(Debug, PartialEq, Eq)]
pub enum MouseCommand {
    // press
    Press(Button),
    // release
    Release(Button),
    // click
    Click(Button),
    // Double click
    Double(Button),
    // Triple click
    Triple(Button),
    // Scroll
    Scroll { amount: i32, axis: Axis },
    // Move
    Move { x: i32, y: i32, context: Coordinate },
}

fn parse_mouse_button(btn: &str) -> Result<Button, String> {
    match btn {
        "left" => Ok(Button::Left),
        "right" => Ok(Button::Right),
        "middle" => Ok(Button::Middle),
        _ => Err(format!("Invalid button: {}", btn)),
    }
}

impl SubCommand for MouseCommand {
    fn execute(&self) -> () {
        let mut enigo = Enigo::new(&Settings::default()).unwrap();

        match self {
            MouseCommand::Press(btn) => enigo.button(*btn, Press),
            MouseCommand::Release(btn) => enigo.button(*btn, Release),
            MouseCommand::Click(btn) => enigo.button(*btn, Click),
            MouseCommand::Double(btn) => {
                enigo
                    .button(*btn, Click)
                    .unwrap_or_else(|err| panic_command(err));
                enigo.button(*btn, Click)
            }
            MouseCommand::Triple(btn) => {
                enigo
                    .button(*btn, Click)
                    .unwrap_or_else(|err| panic_command(err));
                enigo
                    .button(*btn, Click)
                    .unwrap_or_else(|err| panic_command(err));
                enigo.button(*btn, Click)
            }
            MouseCommand::Scroll { amount, axis } => enigo.scroll(*amount, *axis),
            MouseCommand::Move { x, y, context } => enigo.move_mouse(*x, *y, *context),
        }
        .unwrap_or_else(|err| panic_command(err))
    }

    fn parse<T: Into<String>>(words: Vec<T>) -> Self {
        // command: click {p,r,c,d,t,s,m} {key|words}
        // key can be a character or a special key, like space
        let words = words.into_iter().map(|s| s.into()).collect::<Vec<String>>();
        let panic_msg = words.join(" ");
        let sub = words[1].as_str();
        let args = words[2..].to_vec();

        match sub {
            "press" => {
                let btn = parse_mouse_button(&args[0]);
                if let Ok(btn) = btn {
                    MouseCommand::Press(btn)
                } else {
                    panic_command(&panic_msg)
                }
            }
            "release" => {
                let btn = parse_mouse_button(&args[0]);
                if let Ok(btn) = btn {
                    MouseCommand::Release(btn)
                } else {
                    panic_command(&panic_msg)
                }
            }
            "click" => {
                let btn = parse_mouse_button(&args[0]);
                if let Ok(btn) = btn {
                    MouseCommand::Click(btn)
                } else {
                    panic_command(&panic_msg)
                }
            }
            "double" => {
                let btn = parse_mouse_button(&args[0]);
                if let Ok(btn) = btn {
                    MouseCommand::Double(btn)
                } else {
                    panic_command(&panic_msg)
                }
            }
            "triple" => {
                let btn = parse_mouse_button(&args[0]);
                if let Ok(btn) = btn {
                    MouseCommand::Triple(btn)
                } else {
                    panic_command(&panic_msg)
                }
            }
            "scroll" => {
                let amount = args[0].parse::<i32>().unwrap();
                let axis = match args[1].as_str() {
                    "x" => Axis::Horizontal,
                    "y" => Axis::Vertical,
                    _ => panic_command(&panic_msg),
                };
                MouseCommand::Scroll { axis, amount }
            }
            "move" => {
                let x = args[0].parse::<i32>().unwrap();
                let y = args[1].parse::<i32>().unwrap();
                let context = match args[2].as_str() {
                    "abs" => Coordinate::Abs,
                    "rel" => Coordinate::Rel,
                    _ => panic_command(&panic_msg),
                };
                MouseCommand::Move { x, y, context }
            }
            _ => panic_command(&panic_msg),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse() {
        let right = Button::Right;
        let left = Button::Left;

        let words = vec!["click", "press", "right"];
        let cmd = MouseCommand::parse(words);
        assert_eq!(cmd, MouseCommand::Press(right));

        let words = vec!["click", "release", "right"];
        let cmd = MouseCommand::parse(words);
        assert_eq!(cmd, MouseCommand::Release(right));

        let words = vec!["click", "double", "left"];
        let cmd = MouseCommand::parse(words);
        assert_eq!(cmd, MouseCommand::Double(left));

        let words = vec!["click", "triple", "right"];
        let cmd = MouseCommand::parse(words);
        assert_eq!(cmd, MouseCommand::Triple(right));

        let words = vec!["click", "scroll", "10", "x"];
        let cmd = MouseCommand::parse(words);
        assert_eq!(
            cmd,
            MouseCommand::Scroll {
                axis: Axis::Horizontal,
                amount: 10
            }
        );

        let words = vec!["click", "scroll", "-5", "y"];
        let cmd = MouseCommand::parse(words);
        assert_eq!(
            cmd,
            MouseCommand::Scroll {
                axis: Axis::Vertical,
                amount: -5
            }
        );

        let words = vec!["click", "move", "10", "6", "rel"];
        let cmd = MouseCommand::parse(words);
        assert_eq!(
            cmd,
            MouseCommand::Move {
                x: 10,
                y: 6,
                context: Coordinate::Rel
            }
        )
    }
}
