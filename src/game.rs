use std::{io, collections::HashSet};
use rand::random;

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


enum FeedbackColors {
    White,
    Black,
    Empty,
}

pub struct Board {
    guesses: Vec<Vec<CodeColors>>,
    feedback: Vec<Vec<CodeColors>>,
    code: Vec<CodeColors>,
}

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

    pub fn game_step(&mut self) -> Vec<CodeColors> {
        let mut guess: Vec<CodeColors> = Vec::new();
        
        println!("Enter your guess (one color at a time):");
        println!("Valid colors: Red 🔴, Orange 🟠, Yellow 🟡, Green 🟢, Blue 🔵, Brown 🟤, White ⚪️, Black ⚫️");
        
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
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

        self.get_feedback(guess.clone());
        guess
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

        // checking for same color ad same position
        for (index, color) in guess.iter().enumerate(){
            if *color == self.code[index]{
                feedback.push(FeedbackColors::Black);
                guess_remaining.remove(index);
                code_remaining.remove(index);
            }
        }

        // check if guess is correct
        if feedback.len() == 5 {
            return feedback
            
        }

        // check for same color, different position
        for guess_color in guess_remaining.into_iter() {
            let result = code_remaining.clone().into_iter().enumerate().find(|(_,code_color)| *code_color==guess_color);
            match result {
                Some((index, _)) => {
                    feedback.push(FeedbackColors::White);
                    code_remaining.remove(index);
                },
                None => continue,
            }
        }

        feedback

    }

    pub fn print_board(&self) {
        for guess in &self.guesses {
            let mut guess_string = "".to_string();
            for color in guess {
                match color {
                    CodeColors::Red => guess_string.push_str("🔴"),
                    CodeColors::Orange => guess_string.push_str("🟠"),
                    CodeColors::Yellow => guess_string.push_str("🟡"),
                    CodeColors::Green => guess_string.push_str("🟢"),
                    CodeColors::Blue => guess_string.push_str("🔵"),
                    CodeColors::Brown => guess_string.push_str("🟤"),
                    CodeColors::White => guess_string.push_str("⚪️"),
                    CodeColors::Black => guess_string.push_str("⚫️"),
                }
            }
            print!("{}\n", guess_string);
        }
}
pub fn reveal_code(&self) -> Vec<CodeColors> {
    return self.code.clone();
}
}