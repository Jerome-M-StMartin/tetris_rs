//Jerome M. St.Martin

use std::time::Duration;

use crossterm::event::{Event, KeyCode};

use super::error::Gremlin;

#[derive(PartialEq, Eq, Debug)]
pub enum InputEvent {
    Left,
    Right,
    Down,
    Rotate,
    Esc,
    Null,
}

#[derive(Debug)]
pub struct UserInput {}

impl UserInput {
    pub(crate) fn poll_read() -> Result<InputEvent, Gremlin> {
        let event;

        if crossterm::event::poll(Duration::from_millis(50))? {
            event = crossterm::event::read()?;
            //println!("{:?}\r", event); //For Testing Only
            return Ok(Self::translate(event));
        }

        Ok(InputEvent::Null)
    }

    fn translate(event: Event) -> InputEvent {
        let mut msg = InputEvent::Null;

        match event {
            Event::Key(key_event) => {
                let code = key_event.code;
                let mods = key_event.modifiers; //No need at this time
                match code {
                    KeyCode::Left => msg = InputEvent::Left,
                    KeyCode::Right => msg = InputEvent::Right,
                    KeyCode::Up => msg = InputEvent::Rotate,
                    KeyCode::Down => msg = InputEvent::Down,
                    KeyCode::Char(c) => {
                        match c {
                            //HJKL
                            'h' => msg = InputEvent::Left,
                            'j' => msg = InputEvent::Down,
                            'k' => msg = InputEvent::Rotate,
                            'l' => msg = InputEvent::Right,

                            _ => {}
                        }
                    }
                    KeyCode::Esc => {
                        msg = InputEvent::Esc;
                    }
                    _ => {}
                }
            }
            Event::Resize(_x, _y) => {} //TODO
            _ => {}
        };

        msg
    }
}
