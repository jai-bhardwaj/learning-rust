use std::io;
use rand::Rng;
use colour::{red_ln,green_ln, prnt_ln};

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..101);  // generate random number
    
    loop {
        prnt_ln!("Please input your guess.");
        
        let mut guess = String::new();  // mutable variable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        prnt_ln!("You guessed: {guess}");
        if guess == secret_number {
            green_ln!("You win!");
            break;
        }else if guess > secret_number {
            red_ln!("Too big!");
        }else {
            red_ln!("Too small!");
        }
    }
}
