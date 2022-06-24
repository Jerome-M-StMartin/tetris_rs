//Jerome M. St.Martin
//June 23, 2022

//TETRIS in Rust - Work In Progress

use std::io::{Write, stdout};

use crossterm::{
    queue, //this is for the macro: queue!()
    //QueueableCommand, //this is for buffer.queue() fn
    terminal::*,
    style::*,
    cursor::*,
};

use user_input::{InputEvent, UserInput};

mod error;
mod user_input;
mod tetromino;
mod gamestate;

fn main() {
    crossterm::terminal::enable_raw_mode().unwrap();

    let mut buffer = stdout(); //terminal buffer, used by crossterm
    queue!(buffer,
           SetTitle("TETRust"),
           SetSize(21, 21),
           MoveToNextLine(1), //cursor command
           SavePosition, //cursor cmd
    ).unwrap();

    let mut game_state = gamestate::GameState::new(['.'; 200]);

    let test_block: String = "####".to_string();

    loop {

        //Clear the Buffer
        queue!(buffer, RestorePosition, Clear(ClearType::All)).unwrap();

        //Read User Input
        let input_event = UserInput::poll_read().unwrap();
        if input_event == InputEvent::Esc { break; }; //Quit Game
        
        //Game Tick
        let grid_lines: Vec<String> = game_state.tick(input_event, &mut buffer);

        //Draw to the Buffer
        for line in grid_lines {
            queue!(buffer, Print("     "), Print(line), Print("\n\r")).unwrap();
        }
        queue!(buffer, RestorePosition).unwrap();

        //Execute Queued Buffer Commands
        buffer.flush().unwrap();
    }

    crossterm::terminal::disable_raw_mode().unwrap();
    std::process::exit(0);
}
