// this is the prelude
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is: {}", secret_number);

    loop {
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

        // we can redefine the guess variable
        // this is called "shadowing", we can use this intead of converting the variable
        // to a new type
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too big."),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

