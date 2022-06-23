//Jerome M. St.Martin
//June 23, 2022

use std::{
    cell::RefCell,
    rc::Rc,
};

use super::{
    tetromino::Tetromino,
    user_input::UserInput,
};

pub(crate) struct GameState {
    grid: Rc<RefCell<[char; 400]>>,
    tetro_queue: Vec<Tetromino>, //idx 0 is current falling Tetromino
    storage: Option<Tetromino>,
    falling_tetro_pos: (u8, u8),
    //other fields not relevant to Minimum Viable Product
}

impl GameState {
    pub fn new(grid: Rc<RefCell<[char; 400]>>) -> Self {
        GameState {
            grid,
            tetro_queue: Vec::new(),
            storage: None,
            falling_tetro_pos: (4, 0),
        }
    }

    pub fn tick(&mut self, user_input: UserInput) {
        //TODO
    }

    fn xy_to_idx(x: u8, y: u8) -> usize {
        let y = y as usize;
        let x = x as usize;

        (10 * y) + x 
    }

    fn idx_to_xy(idx: usize) -> (u8, u8) {
        assert!(idx < 400);

        let x = idx % 10;
        let y = idx / 10;

        (x as u8, y as u8)
    }
}
