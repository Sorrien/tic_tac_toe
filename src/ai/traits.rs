use crate::game::BoardState;

pub trait AIPlayer {
    fn get_next_move(&mut self, board_state: &BoardState) -> (usize, usize);
}