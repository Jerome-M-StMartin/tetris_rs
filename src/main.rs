//Jerome M. St.Martin
//June 23, 2022

//TETRIS in Rust - Work In Progress

#![allow(dead_code)]

use std::{
    io::{Write, stdout},
    time::Instant,
};

use tiny_rng::Rand;

use crossterm::{
    queue, //this is for the macro: queue!()
    //QueueableCommand, //this is for buffer.queue() fn
    terminal::*,
    style::*,
};

use user_input::{InputEvent, UserInput};

mod error;
mod user_input;
mod tetromino;
mod gamestate;
mod scorekeeper;

fn main() {

    let rng_pre_seed = Instant::now();

    //Initialization
    crossterm::terminal::enable_raw_mode().unwrap();

    let mut buffer = stdout(); //terminal buffer, used by crossterm
    //queue!(buffer, SetTitle("Rustris"), SetSize(22,25)).unwrap();
    queue!(buffer, SetTitle("Rustris"), SetSize(32,35)).unwrap();

    let mut rng = tiny_rng::Rng::from_seed(rng_pre_seed.elapsed().subsec_nanos() as u64);
    let mut game_state = gamestate::GameState::new(['.'; 201], &mut rng);
    let mut last_tick_start = Instant::now();
    
    loop {
        queue!(buffer, Clear(ClearType::All)).unwrap();

        //Set Tick Timer & Calc. Delta
        let tick_start = Instant::now();
        let delta_t = tick_start.duration_since(last_tick_start);

        //Read User Input
        let input_event = UserInput::poll_read().unwrap();
        if input_event == InputEvent::Esc { break; }; //Quit Game
        
        //Game Tick
        let grid_lines: Vec<String> = game_state.tick(delta_t, input_event, &mut rng);

        //Commit Grid to the Buffer, Formatted Prettily
        for line in grid_lines {
            queue!(
                buffer,
                Print("     ┃"), //Padding for left terminal border
                Print(line),
                Print("┃\n\r") //Newline & Cursor Return
            ).unwrap();
        }

        let score = game_state.get_score();
        let level = game_state.get_level();

        queue!( //print score
            buffer,
            Print("     ┗━━━━━━━━━━┛\n\r"), //Bottom Border
            Print("     POINTS "),
            Print(score),
            Print("\n\r"),
            Print("     LEVEL  "),
            Print(level),
            Print("\n\r"),
        ).unwrap();

        //Execute Queued Buffer Commands & Store Tick Timer
        buffer.flush().unwrap();
        last_tick_start = tick_start;
    }

    crossterm::terminal::disable_raw_mode().unwrap();
    std::process::exit(0);
}
