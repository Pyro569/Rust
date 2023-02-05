use std::io;

fn main() {
    println!("Guess the Number!"); //prints
    println!("Please input your guess."); //prints a prompt for guess
    let mut guess: String = String::new(); //sets variable guess as new string
    io::stdin() //input output method
        .read_line(&mut guess)//gets user input and stores the input as str guess
        .expect("Failed to read line"); //if user inputs nothing/wrong input type
    println!("You guessed: {guess}"); //print out guess string

    let _guess : i32 = guess.trim().parse().unwrap(); //sets new 32 bit int _guess to the parsed guess string

    if _guess == 3 {
        println!("You guessed the right number!")
    }else {
        println!("You guessed the wrong number!")
    }
}

