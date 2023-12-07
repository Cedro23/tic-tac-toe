mod enums;

use enums::GameStatus;
use std::io;

const EMPTY_SPACE_VALUE: char = ' ';

#[derive(Debug)]
struct Move {
    position: usize,
    child: Vec<char>,
}

struct MinimaxResult {
    position: usize,
    eval: isize,
}

fn main() {
    play_game();
}

fn play_game() {
    let mut board = vec![EMPTY_SPACE_VALUE; 9];
    print_board(&board);
    let mut current_player = 'X';
    loop {
        take_turn(&mut board, current_player);
        print_board(&board);

        if !check_game_over(&board) {
            // Switch to the other player
            current_player = if current_player == 'X' { 'O' } else { 'X' };
        } else {
            println!("Game is over. Press any key to close the console...");
            break;
        }
    }

    io::stdin().read_line(&mut String::new()).unwrap();
}

fn take_turn(board: &mut Vec<char>, player: char) {
    println!("{}'s turn.", player);
    let ai = 'X';

    let position: usize = if player != ai {
        player_move(board)
    } else {
        ai_move(board, &ai)
    };
    board[position] = player;
}

fn player_move(board: &Vec<char>) -> usize {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Trim whitespace and parse input as usize
        match input.trim().parse::<usize>() {
            Ok(value) => {
                let position: usize = value - 1;

                if position < 9 && board[position] == EMPTY_SPACE_VALUE {
                    break position;
                } else {
                    println!(
                        "Position already taken or invalid. Choose a different position from 1-9:"
                    );
                }
            }
            Err(_) => {
                println!("Invalid input. Choose a position from 1-9:");
            }
        }
    }
}

//Random AI moves
fn ai_move(board: &Vec<char>, ai: &char) -> usize {
    let maximasing = ai == &'X';
    return minimax(&board, 2, isize::MIN, isize::MAX, maximasing).position;
}

fn minimax(
    board: &Vec<char>,
    depth: usize,
    alpha: isize,
    beta: isize,
    maximizing_player: bool,
) -> MinimaxResult {
    if depth == 0 || check_game_over(&board) {
        return MinimaxResult {
            position: 0,
            eval: evaluate_position(board),
        };
    }

    if maximizing_player {
        let mut max_eval = isize::MIN;
        let mut position = 0;

        for child in get_position_children(&board, 'X') {
            // Do something with each child position
            let result = minimax(&child.child, depth - 1, alpha, beta, false);
            if result.eval > max_eval {
                max_eval = result.eval;
                position = child.position;
            }
            let alpha = alpha.max(result.eval);
            if beta <= alpha {
                break;
            }
        }
        return MinimaxResult {
            position: position,
            eval: max_eval,
        };
    } else {
        let mut min_eval = isize::MAX;
        let mut position = 0;

        for child in get_position_children(&board, 'O') {
            // Do something with each child position
            let result = minimax(&child.child, depth - 1, alpha, beta, true);
            if result.eval < min_eval {
                min_eval = result.eval;
                position = child.position;
            }
            let beta: isize = beta.min(result.eval);
            if beta <= alpha {
                break;
            }
        }
        return MinimaxResult {
            position: position,
            eval: min_eval,
        };
    }
}

fn get_position_children(board: &Vec<char>, player: char) -> Vec<Move> {
    let mut children: Vec<Move> = Vec::new();

    for i in 0..board.len() {
        if board[i] == ' ' {
            let mut child = board.clone();
            child[i] = player;
            let move_info = Move { position: i, child };
            children.push(move_info);
        }
    }

    children
}

fn evaluate_position(board: &Vec<char>) -> isize {
    const X_PLAYER: char = 'X';
    const O_PLAYER: char = 'O';
    const BLANK: char = ' ';

    let mut x2 = 0;
    let mut x1 = 0;
    let mut o2 = 0;
    let mut o1 = 0;

    // Winning combinations
    let winning_combinations = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8], // Rows
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8], // Columns
        [0, 4, 8],
        [2, 4, 6], // Diagonals
    ];

    for &combination in &winning_combinations {
        let line = [
            board[combination[0]],
            board[combination[1]],
            board[combination[2]],
        ];

        // Check for X wins
        if line == [X_PLAYER, X_PLAYER, X_PLAYER] {
            return isize::MAX; // 'X' wins
        }

        // Check for O wins
        if line == [O_PLAYER, O_PLAYER, O_PLAYER] {
            return isize::MIN; // 'O' wins
        }

        // Count patterns for evaluation
        if line == [X_PLAYER, X_PLAYER, BLANK] {
            x2 += 1;
        } else if line == [X_PLAYER, BLANK, X_PLAYER] {
            x2 += 1;
        } else if line == [BLANK, X_PLAYER, X_PLAYER] {
            x2 += 1;
        } else if line == [X_PLAYER, BLANK, BLANK] {
            x1 += 1;
        } else if line == [BLANK, X_PLAYER, BLANK] {
            x1 += 1;
        } else if line == [BLANK, BLANK, X_PLAYER] {
            x1 += 1;
        } else if line == [O_PLAYER, O_PLAYER, BLANK] {
            o2 += 1;
        } else if line == [O_PLAYER, BLANK, O_PLAYER] {
            o2 += 1;
        } else if line == [BLANK, O_PLAYER, O_PLAYER] {
            o2 += 1;
        } else if line == [O_PLAYER, BLANK, BLANK] {
            o1 += 1;
        } else if line == [BLANK, O_PLAYER, BLANK] {
            o1 += 1;
        } else if line == [BLANK, BLANK, O_PLAYER] {
            o1 += 1;
        }
    }

    // Calculate the evaluation score
    3 * x2 + x1 - (3 * o2 + o1)
}

fn print_board(board: &Vec<char>) {
    println!("{} | {} | {}", board[0], board[1], board[2]);
    println!("{} | {} | {}", board[3], board[4], board[5]);
    println!("{} | {} | {}", board[6], board[7], board[8]);
    println!();
    println!("==========");
    println!();
}

fn check_game_status(board: &Vec<char>) -> GameStatus {
    // Check for a win
    if (board[0] == board[1]
        && board[1] == board[2]
        && board[0..=2].iter().all(|&x| x != EMPTY_SPACE_VALUE))
        || (board[3] == board[4]
            && board[4] == board[5]
            && board[3..=5].iter().all(|&x| x != EMPTY_SPACE_VALUE))
        || (board[6] == board[7]
            && board[7] == board[8]
            && board[6..=8].iter().all(|&x| x != EMPTY_SPACE_VALUE))
        || (board[0] == board[3]
            && board[3] == board[6]
            && [0, 3, 6]
                .iter()
                .all(|&index| board[index] != EMPTY_SPACE_VALUE))
        || (board[1] == board[4]
            && board[4] == board[7]
            && [1, 4, 7]
                .iter()
                .all(|&index| board[index] != EMPTY_SPACE_VALUE))
        || (board[2] == board[5]
            && board[5] == board[8]
            && [2, 5, 8]
                .iter()
                .all(|&index| board[index] != EMPTY_SPACE_VALUE))
        || (board[0] == board[4]
            && board[4] == board[8]
            && [0, 4, 8]
                .iter()
                .all(|&index| board[index] != EMPTY_SPACE_VALUE))
        || (board[2] == board[4]
            && board[4] == board[6]
            && [2, 4, 6]
                .iter()
                .all(|&index| board[index] != EMPTY_SPACE_VALUE))
    {
        return GameStatus::Over;
    }
    // Check for a tie
    else if !board.contains(&EMPTY_SPACE_VALUE) {
        return GameStatus::Tie;
    }
    // Game is not over
    else {
        return GameStatus::Play;
    }
}

fn check_game_over(board: &Vec<char>) -> bool {
    let game_result: GameStatus = check_game_status(board);
    if matches!(game_result, GameStatus::Over) || matches!(game_result, GameStatus::Tie) {
        return true;
    } else {
        return false;
    }
}
