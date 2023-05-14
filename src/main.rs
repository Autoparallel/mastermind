pub mod game;
use game::*;

fn main() {
    println!("Welcome to Mastermind!");
    let mut board = Board::new();

    let mut number_of_guesses = 0;
    loop {
        board.read_guess();
        board.print_board();
        number_of_guesses += 1;
        // println!("Number of guesses: {}", number_of_guesses);
        if number_of_guesses == 12 {
            println!("You lost! The code was: {:?}", board.reveal_code());
            break;
        }
    }
}
