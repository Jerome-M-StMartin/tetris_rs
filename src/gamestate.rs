//Jerome M. St.Martin
//June 23, 2022

use std::time::Duration;

use tiny_rng::Rng;

use super::{
    tetromino::{Rotation, Tetromino},
    user_input::InputEvent,
    scorekeeper::ScoreKeeper,
    error::Gremlin,
};

//const GRAVITY_TIMER: Duration = Duration::from_secs(1);
const GRAVITY_TIMER: usize = 1;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir {
    Down,
    Left,
    Right,
    Gravity,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CollisionType {
    Wall, //Disallow movement or rotation only
    Floor, //Disallow movement && "glue" block to floor. Disallow rotate.
    NoCollision, //Do nothing
}

pub(crate) struct TickResult {
    pub grid: Vec<String>,
    pub score: usize,
    pub level: usize,
    pub end_the_game: bool,
}

pub(crate) struct GameState {
    grid: [char; 201],
    tetro_queue: Vec<Tetromino>,
    storage: Option<Tetromino>,
    curr_tetro: Tetromino,
    curr_tetro_pos: (isize, isize),
    timer: Duration,
    scorekeeper: ScoreKeeper,
    final_tick: bool,
}

impl GameState {
    pub fn new(mut grid: [char; 201], rng: &mut Rng) -> Self {
        
        let mut tetro_queue = Vec::with_capacity(6);
        for _ in 0..6 {
            tetro_queue.push(Tetromino::new(rng));
        }
        let curr_tetro = tetro_queue.pop().unwrap();

        // Set 201st element to be a block always,
        // for use when we need to do collision checks on out-of-bounds indices.
        grid[200] = '▒';


        GameState {
            grid,
            tetro_queue,
            storage: None,
            curr_tetro,
            curr_tetro_pos: (3, 0),
            timer: Duration::ZERO,
            scorekeeper: ScoreKeeper::new(),
            final_tick: false,
        }
    }
    
    ///Mutations to the GameState should occur in here,
    ///based on passed-in InputEvent and passage of time.
    pub fn tick(&mut self,
                delta_t: Duration,
                input_event: InputEvent,
                rng: &mut Rng) -> Result<TickResult, Gremlin> {
        
        let mut collision_type = self.process_user_input(input_event, rng); //maybe this variable should default to NoCollision

        self.timer = self.timer.saturating_add(delta_t);
        let new_timer = ( 1000 - self.scorekeeper.get_level() * 100 ) as u64;
        let timed_out = self.timer >= Duration::from_millis( new_timer );
        if timed_out {
            self.timer = Duration::ZERO;
            collision_type = self.try_move(Dir::Gravity);
        }

        match collision_type {
            CollisionType::Floor => {
                //Edit the grid to reflect
                self.glue_tetro_to_floor();
                
                //Change curr_tetro, reset curr_tetro_pos
                self.curr_tetro = self.tetro_queue.pop().unwrap();
                self.tetro_queue.insert(0, Tetromino::new(rng));
                self.curr_tetro_pos = (4, 0);
            },
            _ => { /* TODO add juice here on wall collisions */},
        }

        self.scorekeeper
            .read_frame_score(&self.grid)
            .record_score();

        GameState::remove_scored_rows(&mut self.grid);

        Tetromino::draw_to_grid(&mut self.grid,
                                &mut self.curr_tetro,
                                &self.curr_tetro_pos);

        for i in 0..20 {
            if self.grid[i] == '▒' { self.final_tick = true; };
        };

        Ok(
            TickResult {
                grid: self.grid_to_strings(),
                score: self.scorekeeper.get_score(),
                level: self.scorekeeper.get_level(),
                end_the_game: self.final_tick,
            }
        )
    }

    pub fn game_over(&mut self) {

        let msg: Vec<char> = "Game Over!".chars().collect();
        let mut i: usize = 0; // curr grid index
        let mut j: usize = 0; // curr game-over message index

        while i < 201 {
            j = i % 10;
            self.grid[i] = msg[j];
            i += 1;
        }

        self.final_tick = true;
    }
/*
    pub fn get_lines() -> Vec<String> {
        
    }

    pub fn get_score(&self) -> String {
        self.scorekeeper.get_score().to_string()
    }

    pub fn get_level(&self) -> String {
        self.scorekeeper.get_level().to_string()
    }*/

    fn remove_scored_rows(grid: &mut [char; 201]) {
        let grid_width = 10;
        let mut block_counter = 0;
        let mut scored_row_start;

        for i in 0..201 {
            if block_counter == grid_width { // just moved past a scored row
                                             
                scored_row_start = i - grid_width;
                /*for j in scored_row_start..(scored_row_start + 10) { // erase the scored row
                    grid[j] = '.';
                }*/

                //Move all rows above this row down 1.
                //We're finding scored rows top-down, so no need to keep track.
                //Here we will go down-up, so as to not overwrite any unscored tiles.
                let mut k = scored_row_start + (grid_width - 1);
                while k > 9 {
                    grid[k] = grid[k - grid_width];
                    k -= 1;
                }
            }

            // Reset for the next loop iteration
            if i % grid_width == 0 { block_counter = 0; } // new row reset
            if grid[i] == '▒' { block_counter += 1; } // is this tile a block?
        };
    }

    fn glue_tetro_to_floor(&mut self) {

        let idx_vec = self.sub_grid_4x4(self.curr_tetro_pos);
        assert!(idx_vec.len() == 16);

        for (m_idx, g_idx) in idx_vec.iter().enumerate() { //matrix_idx, grid_idx
            let is_tetro_block = Tetromino::is_tetro_block(&self.curr_tetro, m_idx);

            if is_tetro_block {
                //Draw a block to the grid at this g_idx
                self.grid[*g_idx] = '▒';
            }
        }
    }
    
    fn try_move(&mut self, dir: Dir) -> CollisionType {
        let will_collide = self.collision_check_ahead(Some(dir), None);
            
        match (will_collide, dir) {

            //NoCollision
            (false, Dir::Left) => self.curr_tetro_pos.0 -= 1,
            (false, Dir::Right) => self.curr_tetro_pos.0 += 1,
            (false, Dir::Gravity) => self.curr_tetro_pos.1 += 1,
            (false, Dir::Down) => {
                self.curr_tetro_pos.1 += 1;
                self.timer = Duration::ZERO; //reset Gravity timer
            }
            
            //Collision Occurred
            (true, Dir::Left | Dir::Right) => return CollisionType::Wall,
            (true, Dir::Down | Dir::Gravity) => return CollisionType::Floor,
        }

        CollisionType::NoCollision
    }

    fn try_rotate(&mut self) -> bool {
        let test_rot: Rotation;

        match self.curr_tetro.rotation {
            Rotation::Zero => test_rot = Rotation::Ninety,
            Rotation::Ninety => test_rot = Rotation::OneEighty,
            Rotation::OneEighty => test_rot = Rotation::TwoSeventy,
            Rotation::TwoSeventy => test_rot = Rotation::Zero,
        }

        self.collision_check_ahead(None, Some(test_rot))
    }

    fn collision_check_ahead(&self, direction: Option<Dir>, rotation: Option<Rotation>) -> bool {

        let (mut ghost_x, mut ghost_y) = (self.curr_tetro_pos.0, self.curr_tetro_pos.1);
        let mut ghost_tetro = self.curr_tetro;

        // If we need to check for rotation collision...
        if let Some(rot) = rotation {
            ghost_tetro.rotation = rot;
        }

        // If we need to check for movement collision...
        if let Some(dir) = direction {

            //This match may make ghost_pos vals negative.
            match dir {
                Dir::Left => ghost_x -= 1,
                Dir::Right => ghost_x += 1, 
                Dir::Down | Dir::Gravity => ghost_y += 1,
            }
        }
        
        self.collision_check(&ghost_tetro, (ghost_x, ghost_y))
    }

    fn collision_check(&self, tetro: &Tetromino, pos: (isize, isize)) -> bool {
        //For each BLOCK in the tetro's matrix,
        //check for a collision in the grid's corresponding index.
        let idx_vec = self.sub_grid_4x4(pos);

        assert!(idx_vec.len() == 16);

        for (m_idx, g_idx) in idx_vec.iter().enumerate() { // matrix_index, grid_index

            let is_grid_block = self.grid[*g_idx] == '▒';
            let is_tetro_block = Tetromino::is_tetro_block(tetro, m_idx);

            if is_grid_block && is_tetro_block {
                return true; // There is a collision.
            }
        }
        
        false // No collision.
    }

    fn sub_grid_4x4(&self, pos: (isize, isize)) -> Vec<usize> {
        let mut grid_indices = Vec::with_capacity(16);
        let (pos_x, pos_y) = pos;

        for matrix_y in 0..4 { 
            for matrix_x in 0..4 { 
                let (grid_x, grid_y) = (pos_x + matrix_x, pos_y + matrix_y);

                if grid_x >= 10 || grid_y >= 20 || grid_y < 0 || grid_x < 0 {
                    //out of grid bounds
                    grid_indices.push(200); //this idx is always a block
                    continue;
                }

                //isize to usize cast safe here becuase of the above conditional
                if let Some(idx) = GameState::xy_to_idx(grid_x, grid_y) {
                    grid_indices.push(idx);
                } else {
                    grid_indices.push(200); //this idx is always a block
                }
            }
        }

        grid_indices
    }

    fn process_user_input(&mut self, input_event: InputEvent, rng: &mut Rng) -> CollisionType {

        match input_event {

            InputEvent::Rotate => {
                if !self.try_rotate() {
                    Tetromino::rotate(&mut self.curr_tetro);
                    return CollisionType::NoCollision
                }
            },
            InputEvent::Left => return self.try_move(Dir::Left),
            InputEvent::Right => return self.try_move(Dir::Right),
            InputEvent::Down => return self.try_move(Dir::Down),
            InputEvent::Store => {
                self.storage = Some(self.curr_tetro);
                self.curr_tetro = self.tetro_queue.pop().unwrap();
                self.tetro_queue.insert(0, Tetromino::new(rng));
            },
            _ => {},
        }

        CollisionType::NoCollision
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

    pub fn xy_to_idx(x: isize, y: isize) -> Option<usize> {

        let a: bool = x >= 10;
        let b: bool = y >= 20;
        let c: bool = x < 0;
        let d: bool = y < 0;
        if a | b | c | d { return None }

        let y = y as usize;
        let x = x as usize;

        Some( (10 * y) + x )
    }

    pub fn idx_to_xy(idx: usize) -> Option<(usize, usize)> {
        if idx >= 200 { return None }

        let x = idx % 10;
        let y = idx / 10;

        Some( (x, y) )
    }
}
