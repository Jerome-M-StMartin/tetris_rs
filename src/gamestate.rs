//Jerome M. St.Martin
//June 23, 2022

use std::io::Stdout;

use super::{
    tetromino::Tetromino,
    user_input::InputEvent,
};

pub(crate) struct GameState {
    grid: [char; 200],
    tetro_queue: Vec<Tetromino>, //idx 0 is current falling Tetromino, init to len 6
    storage: Option<Tetromino>,
    falling_tetro_pos: (usize, usize),
    //other fields not relevant to Minimum Viable Product
}

impl GameState {
    pub fn new(grid: [char; 200]) -> Self {
        GameState {
            grid,
            tetro_queue: Vec::with_capacity(6),
            storage: None,
            falling_tetro_pos: (4, 0),
        }
    }
    
    ///Mutations to the GameState should occur in here,
    ///based on passed in InputEvent and passage of time.
    pub fn tick(&mut self, input_event: InputEvent, buffer: &mut Stdout) -> Vec<String> {
        //TODO

        match input_event {
            InputEvent::Rotate => {},
            InputEvent::Left => {},
            InputEvent::Right => {},
            InputEvent::Down => {},
            _ => {},
        }

        self.grid_to_strings()
    }
    
    //Helper fn for Self::tick()
    fn grid_to_strings(&self) -> Vec<String> {
        let mut grid_lines = Vec::with_capacity(10);

        for i in 0..10 {
            let start = i * 10;
            let end = (i * 10) + 10;
            let line: String = self.grid[start..end]
                .iter()
                .collect();
            grid_lines.push(line);
        }

        grid_lines
    }

    pub fn xy_to_idx(x: usize, y: usize) -> usize {
        assert!(x < 10 && y < 40);

        let y = y as usize;
        let x = x as usize;

        (10 * y) + x 
    }

    pub fn idx_to_xy(idx: usize) -> (usize, usize) {
        assert!(idx < 200);

        let x = idx % 10;
        let y = idx / 10;

        (x, y)
    }
}
