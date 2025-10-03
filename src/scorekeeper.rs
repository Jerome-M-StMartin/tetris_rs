
// Points @ Lvl 0
// 1 Line: 40
// 2 Lines: 100
// 3 Lines: 300
// 4 Lines: 1200
// Multiply line score by level + 1

pub(crate) struct ScoreKeeper {
    frame_score: usize,
    total_score: usize,
    lines_cleared: usize,
    level: usize,
}

impl ScoreKeeper {
    pub(crate) fn new() -> ScoreKeeper {
        ScoreKeeper {
            frame_score: 0,
            total_score: 0,
            lines_cleared: 0,
            level: 0,
        }
    }

    pub(crate) fn read_frame_score(&mut self, grid: &[char; 201]) -> &mut Self {
        
        let grid_width: usize = 10;
        let mut scoring_rows = 0;
        let mut block_counter = 0; // if == grid_width, score

        for i in 0..201 {
            if block_counter == grid_width { scoring_rows += 1; } // is the whole row blocks?
            if i % grid_width == 0 { block_counter = 0; } // new row reset
            if grid[i] == '▒' { block_counter += 1; } // is this tile a block?
        };

        let base_score = match scoring_rows {
            0 => 0,
            1 => 40,
            2 => 100,
            3 => 300,
            4 => 1200,
            _ => panic!("How did you score >4 lines in a single frame?"),
        };

        self.frame_score = base_score * (self.level + 1);
        self.lines_cleared += scoring_rows;

        self
    }

    pub(crate) fn record_score(&mut self) {
        self.level = self.lines_cleared / 10;
        self.total_score += self.frame_score;
        self.frame_score = 0;
    }

    pub(crate) fn get_score(&self) -> usize {
        self.total_score
    }

    pub(crate) fn get_level(&self) -> usize {
        self.level
    }
}
