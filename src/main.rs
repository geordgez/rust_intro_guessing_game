use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1, 5);

    println!("The secret number is: {}", secret_number);
    
    loop {

        println!("Guess the number!\nPlease input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(msg) => {
                println!("Please type a number!");
                println!("You can't give me \"{}\"! {}\n", guess, msg);
                continue;
            },
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small... guess something bigger"),
            Ordering::Equal => {
                println!("You got it! The correct number was {}", secret_number);
                break;
            },
            Ordering::Greater => println!("Too big... guess something smaller"),
        }

    }
}
