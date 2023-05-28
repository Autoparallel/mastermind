pub mod game;
use game::*;

fn main() {
    println!("Welcome to Mastermind!");
    let mut board = Board::new();

    let mut number_of_guesses = 0;
    loop {
        let status = board.game_step();
        board.print_board();
        number_of_guesses += 1;
        // TODO: Remove guess limit?
        if number_of_guesses == 12 {
            println!("You lost! The code was: {:?}", board.reveal_code());
            break;
        }
        if status {
            println!("🎉 You won! 🎉 The code was: {:?}", board.reveal_code());
            break;
        }
    }
}
