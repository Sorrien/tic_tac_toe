use rayon::prelude::*;

use crate::ai::traits::AIPlayer;
use crate::game::BoardState;
use crate::game::Player;

pub struct MinimaxPlayer {
    pub player: Player,
}

impl MinimaxPlayer {
    pub fn new(ai_player: Player) -> Self {
        Self { player: ai_player }
    }

    pub fn get_best_move(&self, board: &BoardState) -> BoardState {
        let children = child_nodes(&board, self.player);

        let (best_board, _) = children
            .par_iter()
            .map(|(child, turn)| (child, self.minimax(&child, *turn)))
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap();

        *best_board
    }

    pub fn minimax(&self, board: &BoardState, current_turn: Player) -> isize {
        if let Some(victor) = board.check_for_victor() {
            if victor == self.player {
                1
            } else {
                -1
            }
        } else if board.is_draw() {
            0
        } else {
            let children = child_nodes(&board, current_turn);

            let children_processed = children
                .par_iter()
                .map(|(child_board, child_turn)| self.minimax(&child_board, *child_turn));

            if current_turn == self.player {
                if let Some(score) = children_processed.max() {
                    score
                } else {
                    -2
                }
            } else {
                if let Some(score) = children_processed.min() {
                    score
                } else {
                    2
                }
            }
        }
    }
}

impl AIPlayer for MinimaxPlayer {
    fn get_next_move(&mut self, board_state: &BoardState) -> (usize, usize) {
        let best_board = self.get_best_move(&board_state);

        //currently this method returns the whole board state and I just want the new move.
        let (x, y) = board_state
            .board
            .iter()
            .enumerate()
            .zip(best_board.board)
            .find_map(|((i, row_a), row_b)| {
                row_a.iter().enumerate().zip(row_b).find_map(|((j, a), b)| {
                    if *a == b {
                        None
                    } else {
                        Some((i, j))
                    }
                })
            })
            .unwrap();

        (x, y)
    }
}

fn child_nodes(board: &BoardState, current_turn: Player) -> Vec<(BoardState, Player)> {
    board
        .get_open_cells()
        .iter()
        .map(|(x, y)| {
            let mut child = board.clone();
            child.process_move(*x, *y, current_turn);
            (child, current_turn.get_opp())
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::MinimaxPlayer;
    use crate::{
        ai::traits::AIPlayer,
        game::{BoardState, Player},
    };

    #[test]
    fn first_move() {
        let board = [[None, None, None], [None, None, None], [None, None, None]];
        let board_state = BoardState::from(board);

        let mut ai_player = MinimaxPlayer::new(Player::O);

        let (x, y) = ai_player.get_next_move(&board_state);

        assert_eq!(2, x);
        assert_eq!(2, y);
    }

    #[test]
    fn second_move() {
        let board = [
            [Some(Player::X), None, None],
            [None, None, None],
            [None, None, None],
        ];
        let board_state = BoardState::from(board);

        let mut ai_player = MinimaxPlayer::new(Player::O);

        let (x, y) = ai_player.get_next_move(&board_state);

        assert_eq!(1, x);
        assert_eq!(1, y);
    }

    #[test]
    fn close_to_win() {
        let board = [
            [Some(Player::X), None, Some(Player::X)],
            [None, None, None],
            [Some(Player::O), None, Some(Player::O)],
        ];
        let board_state = BoardState::from(board);

        let mut ai_player = MinimaxPlayer::new(Player::O);

        let (x, y) = ai_player.get_next_move(&board_state);

        assert_eq!(2, x);
        assert_eq!(1, y);
    }
}
