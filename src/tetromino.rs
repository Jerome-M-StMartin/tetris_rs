//Jerome M. St.Martin
//June 23, 2022

use super::gamestate::GameState;

const SQUARE: [usize; 16] = [0,0,0,0,
                          0,1,1,0,
                          0,1,1,0,
                          0,0,0,0];

const LINE: [usize; 16] = [0,1,0,0,
                        0,1,0,0,
                        0,1,0,0,
                        0,1,0,0];

const T_TETRO: [usize; 16] = [0,1,0,0,
                           0,1,1,0,
                           0,1,0,0,
                           0,0,0,0];

const L_TETRO: [usize; 16] = [0,1,0,0,
                           0,1,0,0,
                           0,1,1,0,
                           0,0,0,0];

const BACK_L: [usize; 16] = [0,0,1,0,
                          0,0,1,0,
                          0,1,1,0,
                          0,0,0,0];

const Z_TETRO: [usize; 16] = [0,0,0,0,
                           1,1,0,0,
                           0,1,1,0,
                           0,0,0,0];

const BACK_Z: [usize; 16] = [0,0,0,0,
                          0,0,1,1,
                          0,1,1,0,
                          0,0,0,0];

pub(crate) enum Rotation {
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy,
}

pub(crate) enum BlockType {
    Square,
    Line,
    T,
    L,
    BackwardsL,
    Z,
    BackwardsZ,
}

pub(crate) struct Tetromino {
    block_type: BlockType,
    rotation: Rotation,
    matrix: [usize; 16],
}

impl Tetromino {
    pub fn new(block_type: BlockType) -> Self {

        let matrix = match block_type {
            BlockType::Square => { SQUARE },
            BlockType::Line  => { LINE },
            BlockType::T  => { T_TETRO },
            BlockType::L  => { L_TETRO },
            BlockType::BackwardsL => { BACK_L },
            BlockType::Z  => { Z_TETRO },
            BlockType::BackwardsZ  => { BACK_Z },
        };

        Tetromino {
            block_type,
            rotation: Rotation::Zero,
            matrix,
        }
    }

    pub fn draw_to_grid(grid: &mut [char; 400], tetro: &Tetromino, draw_at: (usize, usize)) {
        let (x, y) = (draw_at.0 as usize, draw_at.1 as usize);

        //For each cell in the Tetromino matrix...
        for i in 0..16 {
            //If this cell is a block; i.e. is not empty...
            if Tetromino::is_tetro_block(tetro, i) {
                let delta_x = i % 4;
                let delta_y = i / 4;

                let draw_idx = GameState::xy_to_idx(x + delta_x, y + delta_y);
    
                //Draw a char to the GameState grid.
                grid[draw_idx] = '#';
            }
        }
    }

    ///Used to check if a specific cell of a Tetromino's 4x4 grid is or is not
    ///a "filled-in" block, accounting for the current rotation of the Tetromino.
    pub fn is_tetro_block(tetro: &Tetromino, idx: usize) -> bool {
        assert!(idx < 16);

        let (x, y) = Tetromino::idx_to_xy(idx);

        let mut idx: usize = x + (4 * y);

        match tetro.rotation {
            Rotation::Zero => { /* Init. Value is Correct */ },
            Rotation::Ninety => { idx = 12 + y - (x * 4); },
            Rotation::OneEighty => { idx = 15 - x - (y * 4); },
            Rotation::TwoSeventy => { idx = 3 + y + (x * 4); },
        }

        return tetro.matrix[idx] == 1;
    }

    fn xy_to_idx(x: usize, y: usize) -> usize {
        assert!(x < 4 && y < 4);

        let y = y as usize;
        let x = x as usize;

        (4 * y) + x
    }

    fn idx_to_xy(idx: usize) -> (usize, usize) {
        assert!(idx < 16);

        let x = idx % 4;
        let y = idx / 4;

        (x, y)
    }
}
