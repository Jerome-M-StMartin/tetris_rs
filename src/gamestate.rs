//Jerome M. St.Martin
//June 23, 2022

use std::io::Stdout;

use tiny_rng::{Rng, Rand};

use super::{
    tetromino::Tetromino,
    user_input::InputEvent,
};

pub(crate) struct GameState {
    grid: [char; 200],
    tetro_queue: Vec<Tetromino>,
    storage: Option<Tetromino>,
    curr_tetro: Tetromino,
    curr_tetro_pos: (usize, usize),
    prev_tetro_draw: [usize; 4],
    //other fields from class diagram not relevant to Minimum Viable Product
}

impl GameState {
    pub fn new(grid: [char; 200], rng: &mut Rng) -> Self {
        
        let mut tetro_queue = Vec::with_capacity(6);
        for _ in 0..6 {
            tetro_queue.push(Tetromino::new(rng));
        }
        let curr_tetro = tetro_queue.pop().unwrap();

        GameState {
            grid,
            tetro_queue,
            storage: None,
            curr_tetro,
            curr_tetro_pos: (4, 0),
            prev_tetro_draw: [0; 4],
        }
    }
    
    ///Mutations to the GameState should occur in here,
    ///based on passed-in InputEvent and passage of time.
    pub fn tick(&mut self, input_event: InputEvent, rng: &mut Rng) -> Vec<String> {
        
        //De-draw previous draw of the current Tetromino.


        self.process_user_input(input_event);

        Tetromino::draw_to_grid(&mut self.grid,
                                &mut self.prev_tetro_draw,
                                &self.curr_tetro,
                                &self.curr_tetro_pos);

        self.grid_to_strings()
    }
    
    fn process_user_input(&mut self, input_event: InputEvent) {
        match input_event {
            InputEvent::Rotate => { self.curr_tetro.rotate(); },
            InputEvent::Left => {},
            InputEvent::Right => {},
            InputEvent::Down => {},
            _ => {},
        }
    }

    fn grid_to_strings(&self) -> Vec<String> {
        let mut grid_lines = Vec::with_capacity(20);

        for i in 0..20 {
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
