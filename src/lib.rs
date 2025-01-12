use std::{io, cmp::Ordering, time, fmt, fmt::{Display, Formatter}};
use rand::Rng; 

/// A list of all possible errors
#[derive(Debug)]
pub enum GameError {
    EarlyQuit, // Not an actual error but makes it easier to quit the game without having to break out of all loops
    TimeError(time::SystemTimeError),
    ParseError(std::num::ParseIntError),
    IoError(io::Error)
}

#[derive(Debug)]
enum Difficulty {
    Journalist, 
    Easy,
    Medium,
    Hard,
}

#[derive(Debug)]
struct Game {
    difficulty: Difficulty,
    guesses: u32,
    number: i32, 
    scores: Vec<u32>,
    started: time::SystemTime,
}

impl Game {
    fn new() -> Self {
        Game { difficulty: Difficulty::Medium, guesses: 0, number: 0, scores: Vec::new(), started: time::SystemTime::now() }
    }
    fn gen_number(&mut self) {
        self.number = rand::thread_rng().gen_range(1..=100); 
    }
    fn change_difficulty(&mut self, difficulty: Difficulty) {
        self.difficulty = difficulty; 
    }
    fn max_attempts(&self) -> Option<u32> {
        match self.difficulty {
            Difficulty::Journalist => None, 
            Difficulty::Easy => Some(10), 
            Difficulty::Medium => Some(5),
            Difficulty::Hard => Some(3),
        }
    }
    fn is_over(&self) -> bool {
        if let Some(max_attempts) = self.max_attempts() {
            return self.guesses >= max_attempts
        }
        false
    }
    fn reset(&mut self) {
        self.guesses = 0; 
        self.gen_number();
    }
    fn print_high_score(&self) {
        if !self.scores.is_empty() {
            if let Some(best_score) = self.scores.iter().min() {
                println!("Your current best score is: {}", best_score); 
            }
        }
    }
}

impl Display for GameError {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
            GameError::EarlyQuit => f.write_str("Quitting game..."),
            GameError::TimeError(e) => Display::fmt(e, f),
            GameError::ParseError(e) => Display::fmt(e, f),
            GameError::IoError(e) => Display::fmt(e, f), 
        }
    }
}
impl From<std::num::ParseIntError> for GameError {
	fn from(error: std::num::ParseIntError) -> Self {
		Self::ParseError(error)
	}
}
impl From<io::Error> for GameError {
	fn from(error: io::Error) -> Self {
		Self::IoError(error)
	}
}
impl From<time::SystemTimeError> for GameError {
    fn from(error: time::SystemTimeError) -> Self {
        Self::TimeError(error)
    }
}

fn welcome() {
    println!("Welcome to the Number Guessing Game!\nI'm thinking of a number between 1 and 100.\nYou have a few chances to guess the correct number.\n")
}

fn difficulty_selection() -> Result<Difficulty, GameError> {
    println!("\nPlease select the difficulty level:\n0. Gaming Journalist (unlimited)\n1. Easy (10 chances)\n2. Medium (5 chances)\n3. Hard (3 chances)");
    println!("Enter your choice or q/Ctrl+C to quit at any point: ");
    let mut input = String::new(); 
    io::stdin().read_line(&mut input)?;
    if input.trim() == "q" {
        return Err(GameError::EarlyQuit);
    }
    let int_input = input.trim().parse::<i32>()?; 
    let difficulty = match int_input {
        0 => Difficulty::Journalist,
        1 => Difficulty::Easy,
        2 => Difficulty::Medium,
        3 => Difficulty::Hard,
        _ => {
            println!("Invalid difficulty, defaulting to Medium.");
            Difficulty::Medium
        }
    };
    Ok(difficulty)
}

fn game_loop(game: &mut Game) -> Result<(), GameError> {
    game.started = time::SystemTime::now(); 
    // This loop continuously receives a new number guess from stdin and compares it to the correct answer if it is able to be parsed. 
    loop {
        println!("Enter your guess: "); 
        let mut input = String::new(); 
        io::stdin().read_line(&mut input)?;
        if input.trim() == "q" {
            return Err(GameError::EarlyQuit);
        }
        let guess = input.trim().parse::<i32>();
        if guess.is_err() {
            println!("Invalid number, please try again.");
            continue;
        }
        let guess = guess.unwrap(); 
        game.guesses += 1; 
        match guess.cmp(&game.number) {
            Ordering::Equal => {
                println!("Congratulations! You guessed the correct number in {} attempts.", game.guesses);
                let time_elapsed = game.started.elapsed()?.as_secs(); 
                println!("Additionally, it took you {time_elapsed} seconds to guess correctly."); 
                // Keeping track of score.
                game.scores.push(game.guesses); 
                break; 
            },
            Ordering::Greater => {
                println!("Incorrect! The number is less than {guess}.");
            },
            Ordering::Less => {
                println!("Incorrect! The number is greater than {guess}.");
            },
        }

        if game.is_over() {
            println!("You exceeded the maximum number of attempts ({}) for the selected difficulty level.", game.max_attempts().unwrap());
            println!("The correct number was: {}.", game.number);
            break;
        }
    }

    Ok(())
}

fn go_again() -> Result<bool, GameError> {
    println!("Do you want to keep playing? [y/n]");
    // Continuously prompts the user for a valid input.
    loop {
        let mut input = String::new(); 
        io::stdin().read_line(&mut input)?;
        match input.trim().to_lowercase().as_str() {
            "y" => return Ok(true),
            "yes" => return Ok(true),
            "q" => return Ok(false), 
            "n" => return Ok(false),
            "no" => return Ok(false),
            _ => {
                println!("Invalid option. Please try again (y/n).");
                continue;
            }
        }
    }
}


pub fn run() -> Result<(), GameError> {
    welcome();
    let mut game = Game::new(); 
    game.gen_number();
    // Outer loop if the player wants to start again.
    loop {
        let difficulty = difficulty_selection()?;
        game.change_difficulty(difficulty); 
        println!("Great! You have selected the {:?} difficulty level.", game.difficulty);
        println!("Let's start the game!\n");
        
        // Main game loop, guess and check.
        game_loop(&mut game)?;

        let continue_game = go_again()?; 
        // Check if the player wants to keep playing. 
        match continue_game {
            true => {
                // Reset game stats (current guesses and number)
                game.reset();
                game.print_high_score(); 
            },
            false => {
                println!("Thanks for playing!");
                break
            }
        }
    }
    Ok(())
}