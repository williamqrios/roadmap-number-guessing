use std::process;
use number_guessing::{run, GameError}; 

fn main() {
    if let Err(e) = run() {
        match e {
            GameError::EarlyQuit => println!("{e}"),
            _ => println!("Application error: {e}")
        }
        process::exit(1); 
    }
}
