// Number Guessing Game
// A simple Rust program for learning basic concepts

use std::io;
use rand::Rng;


fn main(){
    let correct_number = rand::thread_rng().gen_range(0..=100);

    print!("Enter Your Guessing Number: ");

    let mut guessing = String::new();

    io::stdin()
    .read_line(&mut guessing)
    .expect("Error while taking input.");

    println!("Lets Check Your Guessing");

    let guessing: u32 = guessing.trim().parse().expect("Please enter a valid number!");

    if guessing == correct_number{
        println!("Your Guess is Correct {guessing}.");
    }

    else if guessing > correct_number {
        println!("Your Guess is Too High {guessing}.");
    }

    else if guessing < correct_number{
        println!("Your Guess is Too Low {guessing}.");
    }

    
}