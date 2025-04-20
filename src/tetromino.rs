//Jerome M. St.Martin
//June 23, 2022

use tiny_rng::{Rng, Rand};

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

const BACK_L: [usize; 16] = [0,1,0,0,
                             0,1,0,0,
                             1,1,0,0,
                             0,0,0,0];

const Z_TETRO: [usize; 16] = [0,0,0,0,
                              1,1,0,0,
                              0,1,1,0,
                              0,0,0,0];

const BACK_Z: [usize; 16] = [0,1,1,0,
                             1,1,0,0,
                             0,0,0,0,
                             0,0,0,0];

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Rotation {
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy,
}

#[derive(Clone, Copy)]
pub(crate) struct Tetromino {
    pub(crate) init_pos: (isize, isize),
    pub(crate) rotation: Rotation,
    pub(crate) matrix: [usize; 16],
    prev_draw_idxs: [usize; 4],
}

impl Tetromino {
    pub fn new(rng: &mut Rng) -> Self {

        let (matrix, init_pos) = match rng.rand_range_u32(0, 7) {
            0 => { (SQUARE,  (3, 0)) },
            1 => { (LINE,    (4, 0)) },
            2 => { (T_TETRO, (3, 0)) },
            3 => { (L_TETRO, (3, 0)) },
            4 => { (BACK_L,  (4, 0)) },
            5 => { (Z_TETRO, (3, 0)) },
            6 => { (BACK_Z,  (4, 0)) },
            _ => panic!("This should never panic."),
        };

        Tetromino {
            init_pos,
            rotation: Rotation::Zero,
            matrix,
            prev_draw_idxs: [0; 4],
        }
    }

    pub fn rotate(tetro: &mut Tetromino) {
        tetro.rotation = match tetro.rotation {
            Rotation::Zero => Rotation::Ninety,
            Rotation::Ninety => Rotation::OneEighty,
            Rotation::OneEighty => Rotation::TwoSeventy,
            Rotation::TwoSeventy => Rotation::Zero,
        };

        // These tetros have only two distinguishable rotation orientations.
        match tetro.matrix {
            LINE | BACK_Z => {
                tetro.rotation = match tetro.rotation { // based on state, mutate
                    Rotation::OneEighty => Rotation::Zero,
                    Rotation::Ninety => Rotation::TwoSeventy,
                    _ => { tetro.rotation }, // leave it as-is
                };
            },
            Z_TETRO => {
                tetro.rotation = match tetro.rotation {
                    Rotation::OneEighty => Rotation::Zero,
                    Rotation::TwoSeventy => Rotation::Ninety,
                    _ => { tetro.rotation }, // leave it as-is
                };
            },
            _ => {},
        };
    }

    pub fn draw_to_grid(grid: &mut [char; 201],
                        tetro: &mut Tetromino,
                        draw_at: &(isize, isize)) {

        //De-draw previous draw_to_grid.
        for idx in tetro.prev_draw_idxs {
            grid[idx] = '.';
        }

        let (x, y) = (draw_at.0, draw_at.1);
        let mut num_draws = 0; //for populating to_dedraw

        //For each cell in the Tetromino matrix...
        for i in 0..16 {
            let i: isize = i as isize; //rebind for less type-casting later

            //If this cell is a block; i.e. is not empty...
            if Tetromino::is_tetro_block(tetro, i as usize) {
                let delta_x = i % 4;
                let delta_y = i / 4;

                //if draw_idx is within map bounds
                if let Some(draw_idx) = GameState::xy_to_idx(x + delta_x, y + delta_y) {
                    //Draw a char to the GameState grid.
                    grid[draw_idx] = '#';

                    //Save draw idx for next tick's de-draw.
                    tetro.prev_draw_idxs[num_draws] = draw_idx;
                    num_draws += 1;
                } else {
                    //Set the topmost, leftmost idx to be redrawn to '.'.
                    //This is a hack to make things work in 99.9% of cases.
                    //Should be fixed later: TODO
                    tetro.prev_draw_idxs[num_draws] = 0;
                    num_draws += 1;
                }
            }
        }
    }

    ///Used to check if a specific cell of a Tetromino's 4x4 grid is or is not
    ///a "filled-in" block, accounting for the current rotation of the Tetromino.
    pub fn is_tetro_block(tetro: &Tetromino, idx: usize) -> bool {
        assert!(idx < 16);

        let (x, y) = Tetromino::idx_to_xy(idx).unwrap();

        let mut idx: usize = x + (4 * y);

        match tetro.matrix {
            SQUARE => {
                /* Rotation is indistinguishable for this tetro. */
            },
            LINE => {
                match tetro.rotation {
                    Rotation::Zero       => { /* Base Value is Correct */ },
                    Rotation::Ninety     => { idx = 12 + y - (x * 4); },
                    Rotation::OneEighty  => { idx = 15 - x - (y * 4); },
                    Rotation::TwoSeventy => { idx =  3 - y + (x * 4); },
                }
            },
            _ => { // For all other tetro shapes, 
                if (x > 2) | (y > 2) { return false; };// Never blocks here.
                                                       //
                // Rotate only the top left 9 blocks.
                match tetro.rotation {
                    Rotation::Zero       => { /* Base Value is Correct */ },
                    Rotation::Ninety     => { idx =  8 + y - (4 * x); },
                    Rotation::OneEighty  => { idx = 10 - x - (4 * y); },
                    Rotation::TwoSeventy => { idx =  2 - y + (4 * x); },
                };
            },
        };

        return tetro.matrix[idx] == 1;
    }

    fn xy_to_idx(x: isize, y: isize) -> Option<usize> {

        let a: bool = x >= 4;
        let b       = y >= 4;
        let c       = x < 0;
        let d       = x < 0;
        if a | b | c | d  { return None };

        let y = y as usize;
        let x = x as usize;

        Some( (4 * y) + x )
    }

    fn idx_to_xy(idx: usize) -> Option<(usize, usize)> {

        if idx >= 16 { return None };

        let x = idx % 4;
        let y = idx / 4;

        Some( (x, y) )
    }
}
