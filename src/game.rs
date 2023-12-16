pub struct GameState {
    pub player1: Player,
    pub player2: Player,
    pub board_state: BoardState,
    pub current_turn: Player,
}

impl GameState {
    pub fn progress_turn(&mut self) {
        self.current_turn = self.current_turn.get_opp();
    }
}

#[derive(Clone, Copy)]
pub struct BoardState {
    pub board: [[Option<Player>; 3]; 3],
}

impl BoardState {
    pub fn new() -> Self {
        Self {
            board: [[None; 3]; 3],
        }
    }

    pub fn from(board: [[Option<Player>; 3]; 3]) -> Self {
        Self { board }
    }

    pub fn process_move(&mut self, x: usize, y: usize, player: Player) -> bool {
        let current_point = self.board[x][y];
        let mut is_move_valid = true;
        if current_point.is_none() {
            self.board[x][y] = Some(player);
        } else {
            println!("{}", current_point.unwrap());
            is_move_valid = false;
        }

        is_move_valid
    }

    //need to implement draws
    pub fn check_for_victor(&self) -> Option<Player> {
        let row_count = self.board.len();
        let column_count = self.board[0].len();

        let mut row_wins = (0..row_count).filter_map(|i| Self::check_row(&self.board, i));

        let mut col_wins = (0..column_count).filter_map(|i| {
            let cells = self.board.map(|row| row[i]);

            Self::check_cells_for_win(&cells)
        });

        let diagonals = vec![
            [self.board[0][0], self.board[1][1], self.board[2][2]],
            [self.board[0][2], self.board[1][1], self.board[2][0]],
        ];

        let mut diagonals = diagonals
            .iter()
            .filter_map(|cells| Self::check_cells_for_win(cells));

        if let Some(player) = row_wins.next() {
            Some(player)
        } else if let Some(player) = col_wins.next() {
            Some(player)
        } else if let Some(player) = diagonals.next() {
            Some(player)
        } else {
            None
        }
    }

    fn check_row(board: &[[Option<Player>; 3]; 3], index: usize) -> Option<Player> {
        if board[index].iter().all(|p| *p == Some(Player::X)) {
            Some(Player::X)
        } else if board[index].iter().all(|p| *p == Some(Player::O)) {
            Some(Player::O)
        } else {
            None
        }
    }

    fn check_cells_for_win(cells: &[Option<Player>; 3]) -> Option<Player> {
        if cells.iter().all(|p| *p == Some(Player::X)) {
            Some(Player::X)
        } else if cells.iter().all(|p| *p == Some(Player::O)) {
            Some(Player::O)
        } else {
            None
        }
    }

    pub fn is_draw(&self) -> bool {
        //it would be better to predict when winning is no longer possible but no open cells is fine for now.
        self.get_open_cells().len() == 0
    }

    pub fn get_open_cells(&self) -> Vec<(usize, usize)> {
        self.board
            .iter()
            .enumerate()
            .map(|(x, row)| {
                row.iter().enumerate().filter_map(move |(y, cell)| {
                    if cell.is_none() {
                        Some((x.clone(), y))
                    } else {
                        None
                    }
                })
            })
            .flatten()
            .collect::<Vec<_>>()
    }

    pub fn to_string(&self) -> String {
        self.board
            .map(|row| {
                let mut chars = row
                    .map(|cell| {
                        if let Some(point) = cell {
                            point.to_string().chars().next().unwrap()
                        } else {
                            '#'
                        }
                    })
                    .to_vec();
                chars.push('\n');
                chars
            })
            .iter()
            .flatten()
            .collect::<String>()
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn get_opp(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = match self {
            Player::X => 'X',
            Player::O => 'O',
        };
        write!(f, "{}", display)
    }
}