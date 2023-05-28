use rand::random;
use std::io;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum CodeColors {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Brown,
    White,
    Black,
}

#[derive(Copy, Clone)]
enum FeedbackColors {
    White,
    Black,
    Empty,
}

pub struct Board {
    guesses: Vec<Vec<CodeColors>>,
    feedback: Vec<Vec<FeedbackColors>>,
    code: Vec<CodeColors>,
}

#[allow(clippy::new_without_default)]
impl Board {
    pub fn new() -> Board {
        let mut code = Vec::new();
        for _ in 0..5 {
            let random_number = random::<u8>();
            let color = match random_number % 8 {
                0 => CodeColors::Red,
                1 => CodeColors::Orange,
                2 => CodeColors::Yellow,
                3 => CodeColors::Green,
                4 => CodeColors::Blue,
                5 => CodeColors::Brown,
                6 => CodeColors::White,
                7 => CodeColors::Black,
                _ => panic!("This should never happen!"),
            };
            code.push(color);
        }
        Board {
            guesses: Vec::new(),
            feedback: Vec::new(),
            code,
        }
    }

    pub fn game_step(&mut self) -> bool {
        let mut guess: Vec<CodeColors> = Vec::new();

        println!("Enter your guess (one color at a time):");
        println!("Valid colors: Red 🔴, Orange 🟠, Yellow 🟡, Green 🟢, Blue 🔵, Brown 🟤, White ⚪️, Black ⚫️");

        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input = input.trim().to_lowercase();

            let color = match input.as_str() {
                "red" => CodeColors::Red,
                "orange" => CodeColors::Orange,
                "yellow" => CodeColors::Yellow,
                "green" => CodeColors::Green,
                "blue" => CodeColors::Blue,
                "brown" => CodeColors::Brown,
                "white" => CodeColors::White,
                "black" => CodeColors::Black,
                _ => {
                    println!("Invalid color. Please try again.");
                    continue;
                }
            };

            guess.push(color);

            if guess.len() == 5 {
                break;
            }
        }

        self.guesses.push(guess.clone());
        let feedback = self.get_feedback(guess.clone());
        self.feedback.push(feedback.clone());
        let mut won_game = true;
        for color in feedback {
            match color {
                FeedbackColors::Black => continue,
                _ => won_game = false,
            }
        }

        won_game
    }

    // pub fn add_guess(&mut self, guess: Vec<CodeColors>) {
    //     if guess.len() != 5 {
    //         panic!("Guess must be 5 colors long!");
    //     }
    //     self.guesses.push(guess);
    // }

    fn get_feedback(&mut self, guess: Vec<CodeColors>) -> Vec<FeedbackColors> {
        let mut guess_remaining = guess.clone();
        let mut code_remaining = self.code.clone();
        let mut feedback = vec![];

        // checking for same color and same position
        let mut indices_to_remove = vec![];
        for (index, color) in guess.iter().enumerate() {
            if *color == self.code[index] {
                feedback.push(FeedbackColors::Black);
                indices_to_remove.push(index);
            }
        }

        // remove correct answers.
        // Do so by reversing the order of indices to remove since they were created smallest to biggest.
        for index in indices_to_remove.into_iter().rev() {
            guess_remaining.remove(index);
            code_remaining.remove(index);
        }

        // check for same color, different position
        while !guess_remaining.is_empty() {
            let mut end_of_matches = false;
            for (guess_index, guess_color) in guess_remaining.clone().into_iter().enumerate() {
                let result = code_remaining
                    .clone()
                    .into_iter()
                    .enumerate()
                    .find(|(_, code_color)| *code_color == guess_color);
                match result {
                    Some((index, _)) => {
                        guess_remaining.remove(guess_index);
                        code_remaining.remove(index);
                        feedback.push(FeedbackColors::White);
                        break;
                    }
                    None => {
                        if guess_index == guess_remaining.len() - 1 {
                            end_of_matches = true;
                        }
                        continue;
                    }
                }
            }
            if end_of_matches {
                break;
            }
        }

        // add 'empty' to any remaining blank spaces
        let remaining_spaces = 5 - feedback.len();
        for _ in 0..remaining_spaces {
            feedback.push(FeedbackColors::Empty);
        }

        assert!(feedback.len() == 5);

        feedback
    }

    pub fn print_board(&self) {
        for (guess, feedback) in self.guesses.iter().zip(self.feedback.iter()) {
            let mut guess_string = "".to_string();
            for guess_color in guess {
                match guess_color {
                    CodeColors::Red => guess_string.push('🔴'),
                    CodeColors::Orange => guess_string.push('🟠'),
                    CodeColors::Yellow => guess_string.push('🟡'),
                    CodeColors::Green => guess_string.push('🟢'),
                    CodeColors::Blue => guess_string.push('🔵'),
                    CodeColors::Brown => guess_string.push('🟤'),
                    CodeColors::White => guess_string.push_str("⚪️"),
                    CodeColors::Black => guess_string.push_str("⚫️"),
                }
            }

            let mut feedback_string = "".to_string();
            for feedback_color in feedback {
                match feedback_color {
                    FeedbackColors::White => feedback_string.push_str("⬜️"),
                    FeedbackColors::Black => feedback_string.push_str("⬛️"),
                    FeedbackColors::Empty => continue,
                }
            }
            println!("{} | {}\n", guess_string, feedback_string);
        }
    }

    pub fn reveal_code(&self) -> Vec<CodeColors> {
        self.code.clone()
    }
}
