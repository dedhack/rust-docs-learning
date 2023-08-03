// bring io input/output library into scope, which comes from the standard library
use std::io;

// main function is the entry point of the program
fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // let creates a variable
    // mut makes the variable mutable

    // let apples = 5; // immutable
    // let mut bananas = 5; // mutable

    
    let mut guess = String::new();

    // &mut guess is a reference to guess that allows the function to modify guess
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}