//standard library --helps to take user input
use std::io;


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
}