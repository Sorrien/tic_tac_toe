use game::GameState;
use std::io::{self, stdin};

use crate::ai::minimax::MinimaxPlayer;
use crate::ai::rand::RandPlayer;
use crate::ai::traits::AIPlayer;
use crate::game::{BoardState, Player};

pub mod ai;
pub mod game;

pub fn run() -> io::Result<()> {
    println!("Welcome to tic-tac-toe!");

    let mut game_state = get_game_start_state()?;

    println!("You can type quit to quit at any time. Please enter your moves like so: '0,0' (this is the top left square)");
    let mut buffer = String::new();

    //let mut rng = rand::thread_rng();

    //let mut ai_player = RandPlayer::new();

    let mut ai_player = MinimaxPlayer::new(game_state.player2);
    while buffer != "quit" {
        println!("It's {}'s turn.", game_state.current_turn);
        let was_move_made = if game_state.current_turn == game_state.player1 {
            stdin().read_line(&mut buffer)?;
            let was_move_made = if let Some((x, y)) = get_move(&buffer) {
                let is_valid_move = game_state
                    .board_state
                    .process_move(x, y, game_state.player1);

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
            let (x, y) = ai_player.get_next_move(&game_state.board_state);

            game_state
                .board_state
                .process_move(x, y, game_state.player2);
            println!("I choose {}, {}", x, y);

            true
        } else {
            panic!("Whose turn is it anyway?!");
        };

        if was_move_made {
            if let Some(victor) = game_state.board_state.check_for_victor() {
                if victor == game_state.player1 {
                    println!("You won!");
                } else if victor == game_state.player2 {
                    println!("I won! Better luck next time.");
                } else {
                    panic!("Well who won then?");
                }

                //just quit the game for now.
                buffer = String::from("quit");
            } else if game_state.board_state.is_draw() {
                println!("It's a draw!");
                //just quit the game for now.
                buffer = String::from("quit");
            } else {
                println!("{}", game_state.board_state.to_string());
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

    let player2 = player1.get_opp();

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
        board_state: BoardState::new(),
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
