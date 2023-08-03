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

    // new is an associated function of the String type
    // associated functions are implemented on types, rather than on instances of types
    // new function creates a new, empty string
    let mut guess = String::new();

    // &mut guess is a reference to guess that allows the function to modify guess
    // references are immutable by default
    io::stdin()
        .read_line(&mut guess) // read_line returns a value of type io::Result
        .expect("Failed to read line"); // expect is a method of io::Result types

    println!("You guessed: {guess}");
}