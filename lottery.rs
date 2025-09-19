// Lottery Number Generator
// A Rust program that generates random lottery numbers

//standard library --helps to take user input
use std::io;
//
use rand::Rng;

fn main(){

    //Print hello world
    println!("Hello, world!");

    //variables
    let  first_name = "Pratik";
    let  last_name = "Acharya";

    
    
    // assign variables using mut
    println!("Enter Your City: ");
    let mut city_name = String::new();
    
    io::stdin()
    .read_line(&mut city_name)
    .expect("Failed to read line");
    println!("{} {} {}", first_name, last_name, city_name);

    println!("Generating Lottery Ticket: ");

    // 
    let random_number = rand::thread_rng().gen_range(0..=100);
    println!("Your Lottery Number is {random_number}");

}