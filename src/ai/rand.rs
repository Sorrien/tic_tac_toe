use crate::ai::traits::AIPlayer;
use crate::game::BoardState;
use rand::{rngs::ThreadRng, Rng};

pub struct RandPlayer {
    rng: ThreadRng,
}

impl RandPlayer {
    pub fn new() -> Self {
        let rng = rand::thread_rng();

        Self { rng }
    }
}

impl AIPlayer for RandPlayer {
    fn get_next_move(&mut self, board_state: &BoardState) -> (usize, usize) {
        let open_cells = board_state.get_open_cells();

        let index = self.rng.gen_range(0..open_cells.len());

        //choose a random open space
        open_cells[index]
    }
}
