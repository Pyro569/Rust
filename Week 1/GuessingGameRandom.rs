use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); //generate random number 1-100 and store as "secret variable"

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            //converts guess to unsigned 32 bit integer, sets it to a match expression to handle errors
            Ok(num) => num,     //continue
            Err(_) => continue, //restart the loop
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            //cmp compares two values, in this case guess and secret number, match decides what to do
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break; //breaks out of loop
            }
        }
    }
}
