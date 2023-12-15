use rand::Rng;
use std::io::{self, stdin};

fn main() -> io::Result<()> {
    println!("Welcome to tic-tac-toe!");

    let mut game_state = get_game_start_state()?;

    println!("You can type quit to quit at any time. Please enter your moves like so: '0,0' (this is the top left square)");
    let mut buffer = String::new();

    let mut rng = rand::thread_rng();

    while buffer != "quit" {
        println!("It's {}'s turn.", game_state.current_turn);
        let was_move_made = if game_state.current_turn == game_state.player1 {
            stdin().read_line(&mut buffer)?;
            let was_move_made = if let Some((x, y)) = get_move(&buffer) {
                let is_valid_move = game_state.board.process_move(x, y, game_state.player1);

                if is_valid_move {
                } else {
                    println!("That move is invalid");
                }

                is_valid_move
            } else {
                println!("I didn't understand your move");
                false
            };
            buffer = String::new();

            was_move_made
        } else if game_state.current_turn == game_state.player2 {
            let open_cells = game_state
                .board
                .board
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
                .collect::<Vec<_>>();

            let index = rng.gen_range(0..open_cells.len());

            //choose a random open space
            let (x, y) = open_cells[index];

            game_state.board.process_move(x, y, game_state.player2);
            println!("I choose {}, {}", x, y);

            true
        } else {
            panic!("Whose turn is it anyway?!");
        };

        if was_move_made {
            if let Some(victor) = game_state.board.check_for_victor() {
                if victor == game_state.player1 {
                    println!("You won!");
                } else if victor == game_state.player2 {
                    println!("I won! Better luck next time.");
                } else {
                    panic!("Well who won then?");
                }

                //just quit the game for now.
                buffer = String::from("quit");
            } else {
                println!("{}", game_state.board.to_string());
                game_state.progress_turn();
            }
        }
    }

    Ok(())
}

fn get_game_start_state() -> io::Result<GameState> {
    println!("Are you X or O?");
    let mut buffer = String::new();

    let mut is_valid_input = false;
    let mut player1 = Player::X;
    while !is_valid_input {
        stdin().read_line(&mut buffer)?;

        let player = get_player_from_string(&buffer);

        if let Some(p) = player {
            is_valid_input = true;
            player1 = p;
        } else {
            println!("Please type X or O");
        }
    }

    let player2 = match player1 {
        Player::X => Player::O,
        Player::O => Player::X,
    };

    let mut is_valid_input = false;
    let mut first_turn = Player::X;
    buffer = String::new();
    println!("Who will go first?");
    while !is_valid_input {
        stdin().read_line(&mut buffer)?;

        if let Some(p) = get_player_from_string(&buffer) {
            is_valid_input = true;
            first_turn = p;
        } else {
            println!("Please type X or O");
        }
    }

    Ok(GameState {
        player1,
        player2,
        board: BoardState::new(),
        current_turn: first_turn,
    })
}

fn get_player_from_string(string: &String) -> Option<Player> {
    let string_lower_case = string.to_ascii_lowercase();
    if string_lower_case.contains("x") {
        Some(Player::X)
    } else if string_lower_case.contains("o") {
        Some(Player::O)
    } else {
        None
    }
}

fn get_move(string: &String) -> Option<(usize, usize)> {
    let mut iter = string
        .chars()
        .filter(|c| c.is_ascii_digit())
        .filter_map(|c| c.to_string().parse::<usize>().ok());

    let first = iter.next();
    let second = iter.next();
    if first.is_some() && second.is_some() {
        Some((first.unwrap(), second.unwrap()))
    } else {
        None
    }
}

pub struct GameState {
    pub player1: Player,
    pub player2: Player,
    pub board: BoardState,
    pub current_turn: Player,
}

impl GameState {
    pub fn progress_turn(&mut self) {
        self.current_turn = match self.current_turn {
            Player::X => Player::O,
            Player::O => Player::X,
        }
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

    pub fn check_for_victor(&self) -> Option<Player> {
        let row_count = self.board.len();
        let column_count = self.board[0].len();

        let mut row_wins = (0..row_count).filter_map(|i| Self::check_row(&self.board, i));

        let mut col_wins = (0..column_count).filter_map(|i| {
            let cells = self.board.map(|row| row[i]);

            Self::check_cells_for_win(&cells)
        });

        let diagonals = vec![[self.board[0][0], self.board[1][1], self.board[2][2]], [self.board[0][2], self.board[1][1], self.board[2][0]]];

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

#[derive(Clone, Copy, PartialEq)]
pub enum Player {
    X,
    O,
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