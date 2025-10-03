//Jerome M. St.Martin
//June 23, 2022

//T-ETRIS - Work In Progress

//#![allow(dead_code)]

use std::{
    io::{stdout, Write},
    time::Instant,
};

use tiny_rng::Rand;

use crossterm::{
    execute,
    ExecutableCommand,
    queue, //this is for the macro: queue!()
    terminal::*,
    style::*,
    cursor::*,
};

use user_input::{InputEvent, UserInput};

mod error;
mod user_input;
mod tetromino;
mod gamestate;
mod scorekeeper;
mod grid_buffer;

fn main() {

    let rng_pre_seed = Instant::now();

    //Initialization
    crossterm::terminal::enable_raw_mode().unwrap();
    let mut buffer = stdout();
                                                           
    execute!(buffer, Print("\n Press any key to begin playing...\r")).unwrap();
                                                           
    let mut any_input;
    loop {
        // This is to allow time to gain entropy for rng
        any_input = UserInput::poll_read().unwrap();
        if any_input != InputEvent::Null { break; }
    }   

    queue!(buffer,
        SetTitle("Terminal-etris"),
        Hide, //cursor
        DisableLineWrap,
        EnterAlternateScreen,
    ).unwrap();

    buffer.flush().unwrap();

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
        //let (grid_lines, score, level) = game_state.tick(delta_t, input_event, &mut rng);
        let tick_result = game_state.tick(delta_t, input_event, &mut rng).unwrap();
        let (grid_lines, score, level) = (tick_result.grid, tick_result.score, tick_result.level);

        //Build Crossterm Buffer Queue
        for line in grid_lines {
            queue!(
                buffer,
                Print("┃"), // padding for left terminal border
                Print(line),
                Print("┃\n\r") // newline & cursor return
            ).unwrap();
        }

        queue!( //print score
            buffer,
            Print("┗━━━━━━━━━━┛\n\r"), //Bottom Border
            Print("POINTS "),
            Print(score),
            Print("\n\r"),
            Print("LEVEL  "),
            Print(level),
            Print("\n\r"),
        ).unwrap();

        //Execute Queued Buffer Commands & Store Tick Timer
        buffer.flush().unwrap();

        if tick_result.end_the_game { break; }
        
        last_tick_start = tick_start;
    }

    loop {
        any_input = UserInput::poll_read().unwrap();
        if any_input != InputEvent::Null { break; }
    }

    let _ = buffer.execute(LeaveAlternateScreen);
    crossterm::terminal::disable_raw_mode().unwrap();
    std::process::exit(0);
}
