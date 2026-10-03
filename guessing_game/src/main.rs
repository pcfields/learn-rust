use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    match guess.cmp(&secret_number) {
        Ordering::Greater => println!("Too big!"),
        Ordering::Less => println!("Too small!"),
        Ordering::Equal => println!("You win!"),
    }

    println!("The secret number is: {secret_number}");
    println!("You guessed: {guess}");
}
