mod enums;

use enums::GameStatus;
use std::io;

fn main() {
    play_game();
}

fn play_game() {
    let mut board = ['-'; 9];
    print_board(board);
    let mut current_player = 'X';
    let mut is_game_over = false;

    loop {
        if is_game_over {
            break;
        }

        take_turn(&mut board, current_player);
        print_board(board);

        let game_result: GameStatus = check_game_over(board);

        if matches!(game_result, GameStatus::Over) {
            println!("{current_player} wins!");
            is_game_over = true;
        } else if matches!(game_result, GameStatus::Tie) {
            println!("It's a tie!");
            is_game_over = true;
        } else {
            // Switch to the other player
            current_player = if current_player == 'X' { 'O' } else { 'X' };
        }
    }

    println!("Game is over. Press any key to close the console...");
    io::stdin().read_line(&mut String::new()).unwrap();
}

fn take_turn(board: &mut [char; 9], player: char) {
    println!("{}'s turn.", player);

    let position: usize = loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Trim whitespace and parse input as usize
        match input.trim().parse::<usize>() {
            Ok(value) => {
                let position: usize = value - 1;

                if position < 9 && board[position] == '-' {
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
    };

    board[position] = player;
}

fn print_board(board: [char; 9]) {
    println!("{} | {} | {}", board[0], board[1], board[2]);
    println!("{} | {} | {}", board[3], board[4], board[5]);
    println!("{} | {} | {}", board[6], board[7], board[8]);
    println!();
}

fn check_game_over(board: [char; 9]) -> GameStatus {
    // Check for a win
    if (board[0] == board[1] && board[1] == board[2] && board[0..=2].iter().all(|&x| x != '-'))
        || (board[3] == board[4] && board[4] == board[5] && board[3..=5].iter().all(|&x| x != '-'))
        || (board[6] == board[7] && board[7] == board[8] && board[6..=8].iter().all(|&x| x != '-'))
        || (board[0] == board[3]
            && board[3] == board[6]
            && [0, 3, 6].iter().all(|&index| board[index] != '-'))
        || (board[1] == board[4]
            && board[4] == board[7]
            && [1, 4, 7].iter().all(|&index| board[index] != '-'))
        || (board[2] == board[5]
            && board[5] == board[8]
            && [2, 5, 8].iter().all(|&index| board[index] != '-'))
        || (board[0] == board[4]
            && board[4] == board[8]
            && [0, 4, 8].iter().all(|&index| board[index] != '-'))
        || (board[2] == board[4]
            && board[4] == board[6]
            && [2, 4, 6].iter().all(|&index| board[index] != '-'))
    {
        return GameStatus::Over;
    }
    // Check for a tie
    else if !board.contains(&'-') {
        return GameStatus::Tie;
    }
    // Game is not over
    else {
        return GameStatus::Play;
    }
}
