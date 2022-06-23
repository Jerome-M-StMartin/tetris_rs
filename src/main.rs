//Jerome M. St.Martin
//June 23, 2022

//TETRIS in Rust - Work In Progress

use std::io::{Write, stdout};

use crossterm::{
    QueueableCommand,
    //queue, this is for the macro: queue!()
    terminal::SetTitle,
};

use user_input::{InputEvent, UserInput};

mod error;
mod user_input;
mod tetromino;
mod model;


fn main() {
    //Define Terminal Buffer for Crossterm
    let mut stdout = stdout();

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

        //Give Commands to Buffer Queue
        stdout
            .queue(SetTitle("TETRIS")).unwrap();
        
        stdout.flush().unwrap();
        //-----------------------------

    }

    crossterm::terminal::disable_raw_mode().unwrap();
    std::process::exit(0);
}
