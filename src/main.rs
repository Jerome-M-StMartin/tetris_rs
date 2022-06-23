mod error;
mod user_input;

use user_input::{InputEvent, UserInput};

fn main() {
    loop {
        crossterm::terminal::enable_raw_mode().unwrap();
        let input_event = UserInput::poll_read().unwrap();

        match input_event {
            InputEvent::Null => {}
            InputEvent::Esc => {
                break;
            }
            _ => {
                println!("{:?}\r", input_event);
            }
        }
    }

    crossterm::terminal::disable_raw_mode().unwrap();
    std::process::exit(0);
}
