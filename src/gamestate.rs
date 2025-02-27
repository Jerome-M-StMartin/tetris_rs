//Jerome M. St.Martin
//June 23, 2022

use std::time::Duration;

use tiny_rng::Rng;

use super::{
    tetromino::{Rotation, Tetromino},
    user_input::InputEvent,
};

const GRAVITY_TIMER: Duration = Duration::from_secs(1);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir {
    Down,
    Left,
    Right,
    Gravity,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CollisionType {
    Wall, //Disallow movement only
    Floor, //Disallow movement && "glue" block to floor.
    NoCollision, //Do nothing
}

pub(crate) struct GameState {
    grid: [char; 201],
    tetro_queue: Vec<Tetromino>,
    storage: Option<Tetromino>,
    curr_tetro: Tetromino,
    curr_tetro_pos: (isize, isize),
    timer: Duration,
    //other fields from class diagram not relevant to Minimum Viable Product
}

impl GameState {
    pub fn new(mut grid: [char; 201], rng: &mut Rng) -> Self {
        
        let mut tetro_queue = Vec::with_capacity(6);
        for _ in 0..6 {
            tetro_queue.push(Tetromino::new(rng));
        }
        let curr_tetro = tetro_queue.pop().unwrap();

        //set grid[200] (201th element) to be a block always,
        //for use when we need to do collision checks on out-of-bounds indices.
        grid[200] = '▒';

        GameState {
            grid,
            tetro_queue,
            storage: None,
            curr_tetro,
            curr_tetro_pos: (4, 0),
            timer: Duration::ZERO,
        }
    }
    
    ///Mutations to the GameState should occur in here,
    ///based on passed-in InputEvent and passage of time.
    pub fn tick(&mut self, delta_t: Duration, input_event: InputEvent, rng: &mut Rng) -> Vec<String> {
        
        let mut collision_type = CollisionType::NoCollision;

        collision_type = self.process_user_input(input_event, rng);

        self.timer = self.timer.saturating_add(delta_t);
        let timed_out = self.timer >= GRAVITY_TIMER;
        if timed_out {
            self.timer = Duration::ZERO;
            collision_type = self.try_move(Dir::Gravity);
        }

        match collision_type {
            //TODO
            CollisionType::Floor => {
                //Edit the grid to reflect
                self.glue_tetro_to_floor();
                
                //Change curr_tetro, reset curr_tetro_pos
                self.curr_tetro = self.tetro_queue.pop().unwrap();
                self.tetro_queue.insert(0, Tetromino::new(rng));
                self.curr_tetro_pos = (4, 0);
            },
            _ => {},
        }

        Tetromino::draw_to_grid(&mut self.grid,
                                &mut self.curr_tetro,
                                &self.curr_tetro_pos);

        self.grid_to_strings()
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

    fn collision_check_ahead(&self, direction: Option<Dir>, rotation: Option<Rotation>) -> bool {

        let (mut ghost_x, mut ghost_y) = (self.curr_tetro_pos.0, self.curr_tetro_pos.1);
        let ghost_tetro = self.curr_tetro;

        //If we need to check for rotation collision...
        if let Some(_rot) = rotation {
            //TODO
        }

        //If we need to check for movement collision...
        if let Some(dir) = direction {

            //This match may make ghost_pos vals negative.
            match dir {
                Dir::Left => ghost_x -= 1,
                Dir::Right => ghost_x += 1, 
                Dir::Down => ghost_y += 1,
                Dir::Gravity => ghost_y += 1,
            }
        }
        
        self.collision_check(&ghost_tetro, (ghost_x, ghost_y))
    }

    fn collision_check(&self, tetro: &Tetromino, pos: (isize, isize)) -> bool {
        //For each BLOCK in the tetro's matrix,
        //check for a collision in the grid's corresponding index.
        let idx_vec = self.sub_grid_4x4(pos);

        assert!(idx_vec.len() == 16);

        for (m_idx, g_idx) in idx_vec.iter().enumerate() {

            let is_grid_block = self.grid[*g_idx] == '▒';
            let is_tetro_block = Tetromino::is_tetro_block(tetro, m_idx);

            if is_grid_block && is_tetro_block {
                //There is a collision.
                return true;
            }
        }
        
        false
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
            InputEvent::Rotate => self.curr_tetro.rotate(),
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
        if x >= 10 || y >= 20 || x < 0 || y < 0 { return None }

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
