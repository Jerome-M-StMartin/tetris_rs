//Jerome M. St.Martin
//June 23, 2022

//TETRIS in Rust - Work In Progress

use std::{
    io::{Write, stdout},
    time::{Instant, SystemTime},
};

use tiny_rng::{Rng, Rand};

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

    let rng_preseed = Instant::now();

    //Initialization
    crossterm::terminal::enable_raw_mode().unwrap();

    let mut buffer = stdout(); //terminal buffer, used by crossterm
    queue!(buffer,
           SetTitle("TETRust"),
           //SetSize(20, 20),
           MoveToNextLine(1), //cursor command
           SavePosition, //cursor cmd
    ).unwrap();

    let mut rng = tiny_rng::Rng::from_seed(rng_preseed.elapsed().subsec_nanos() as u64);
    let mut game_state = gamestate::GameState::new(['.'; 200], &mut rng);
    
    loop {
        //Clear the Buffer
        queue!(buffer, RestorePosition, Clear(ClearType::All)).unwrap();

        //Read User Input
        let input_event = UserInput::poll_read().unwrap();
        if input_event == InputEvent::Esc { break; }; //Quit Game
        
        //Game Tick
        let grid_lines: Vec<String> = game_state.tick(input_event, &mut rng);

        //Draw to the Buffer
        for line in grid_lines {
            queue!(buffer, Print("     "), Print(line), Print("\n\r")).unwrap();
        }

        //Return Cursor to Saved Position
        queue!(buffer, RestorePosition).unwrap();

        //Execute Queued Buffer Commands
        buffer.flush().unwrap();
    }

    crossterm::terminal::disable_raw_mode().unwrap();
    std::process::exit(0);
}
