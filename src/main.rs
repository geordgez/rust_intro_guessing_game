use std::io;
use rand::Rng;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1, 5);

    println!("The secret number is: {}", secret_number);

    println!("Guess the number!\nPlease input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
