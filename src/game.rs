use std::io;
use rand::random;

#[derive(Debug, Clone, Copy)]
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

    pub fn read_guess(&mut self) -> Vec<CodeColors> {
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
        guess
    }
    

    pub fn add_guess(&mut self, guess: Vec<CodeColors>) {
        if guess.len() != 5 {
            panic!("Guess must be 5 colors long!");
        }
        self.guesses.push(guess);
    }

    fn add_feedback(&mut self, feedback: [FeedbackColors; 5]) {
        todo!()
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