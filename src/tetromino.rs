//Jerome M. St.Martin
//June 23, 2022

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
    cells: [usize; 4],
}

impl Tetromino {
    pub fn new(block_type: BlockType) -> Self {
        Tetromino {
            block_type,
            rotation: Rotation::Zero,
            cells: [0; 4],
        }
    }

    pub fn set_cells(tetro: &mut Tetromino, x: usize, y: usize) {
        //TODO
        match tetro.block_type {
            BlockType::Square => {},
            BlockType::Line  => {},
            BlockType::T  => {},
            BlockType::L  => {},
            BlockType::BackwardsL => {},
            BlockType::Z  => {},
            BlockType::BackwardsZ  => {},
        };
    }
}
