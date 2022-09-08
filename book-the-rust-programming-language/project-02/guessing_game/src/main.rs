// this is the prelude
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    // new() is the associated function
    // or also it is called static method
    // which is a method that does not need an instace
    // to be used
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Result is a enum type
    // an enum is said to have variants, which are the value that 
    // it can assume
    // in the case of Result it can have Ok or Err


    println!("You guessed: {}", guess);
}


// stoped at:
// Comparing the Guess to the Secret Number