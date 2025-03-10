
// Points @ Lvl 0
// 1 Line: 40
// 2 Lines: 100
// 3 Lines: 300
// 4 Lines: 1200
// Multiply line score by level + 1

pub(crate) struct ScoreKeeper {
    score: usize,
    level: usize,
}

impl ScoreKeeper {
    pub(crate) fn new() -> ScoreKeeper {
        ScoreKeeper {
            score: 0,
            level: 0,
        }
    }

    pub(crate) fn read_frame_score(&self, grid: &[char; 201]) -> usize {
        
        let grid_width: usize = 10;
        let mut scoring_rows = 0;
        let mut block_counter = 0; // if == grid_width, score

        for i in 0..199 {
            if grid[i] == '▒' { block_counter += 1; } // is it a block?
            if block_counter == grid_width { scoring_rows += 1; } // is the whole row?
            if i % grid_width == 0 { block_counter = 0; } // new row reset
        };

        let base_score = match scoring_rows {
            0 => 0,
            1 => 40,
            2 => 100,
            3 => 300,
            4 => 1200,
            _ => panic!("How did you score >4 lines in a single frame?"),
        };

        base_score * (self.level + 1)
    }

    pub(crate) fn record_score(&mut self, score_to_add: usize) {
        self.score += score_to_add;
    }
}
